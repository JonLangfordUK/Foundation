//! Temporary Foundation bridge for loading Bevy Scene Notation (`.bsn`) assets.
//!
//! Bevy 0.19 ships the `bsn!` macro and scene runtime, but not the official
//! `.bsn` asset loader. This module keeps Foundation's early file-backed level
//! and prefab support isolated so it can be removed when Bevy provides its own
//! loader.

use bevy::{
    asset::{AssetEvent, AssetPath, AssetServer, Handle},
    ecs::hierarchy::ChildOf,
    prelude::*,
    scene::{ResolvedSceneRoot, ScenePatch},
};

use std::sync::Arc;

use crate::{
    dynamic_bsn::DynamicBsnLoader,
    scene_stack::{SceneLoadRequested, SceneOwner, SceneSource},
};

/// Installs temporary `.bsn` asset loading and hot-reload replacement support.
///
/// The plugin registers a `.bsn` loader for Bevy [`ScenePatch`] assets and
/// bridges Foundation [`SceneSource::BsnScene`] requests to asset-backed scene
/// instances. Existing live instances are fully despawned and respawned when
/// their source `.bsn` asset is reloaded.
#[derive(Default)]
pub struct FoundationBsnAssetPlugin;

impl Plugin for FoundationBsnAssetPlugin {
    fn build(&self, app: &mut App) {
        // The loader mirrors Bevy's in-progress dynamic BSN asset work and is
        // intentionally registered from one isolated Foundation plugin.
        app.init_asset_loader::<DynamicBsnLoader>()
            .init_resource::<FoundationBsnSceneRegistry>()
            .register_type::<FoundationBsnInstance>()
            .add_systems(
                Update,
                (
                    spawn_requested_bsn_scenes,
                    apply_pending_bsn_instances,
                    propagate_loaded_bsn_scene_owners,
                    replace_reloaded_bsn_instances,
                )
                    .chain(),
            );
    }
}

/// Maps scene-stack BSN keys to asset paths.
///
/// If a key is not registered, Foundation treats the key as a direct asset path.
/// This keeps the initial bridge simple while allowing games to preserve stable
/// scene keys such as `last-beacon/main_menu`.
#[derive(Debug, Default, Resource)]
pub struct FoundationBsnSceneRegistry {
    scene_asset_paths: std::collections::HashMap<String, String>,
}

impl FoundationBsnSceneRegistry {
    /// Registers the `.bsn` asset path used for a scene-stack key.
    pub fn register_scene(&mut self, scene_key: impl Into<String>, asset_path: impl Into<String>) {
        self.scene_asset_paths
            .insert(scene_key.into(), asset_path.into());
    }

    /// Resolves a scene key into a `.bsn` asset path.
    pub fn resolve_scene_path(&self, scene_key: &str) -> String {
        self.scene_asset_paths
            .get(scene_key)
            .cloned()
            .unwrap_or_else(|| scene_key.to_string())
    }
}

/// Tracks one live `.bsn` scene or prefab instance.
///
/// This component is stored on the root entity that receives the loaded
/// [`ScenePatch`]. On hot reload, Foundation despawns this root recursively and
/// creates a replacement from the same asset and ownership context.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component)]
pub struct FoundationBsnInstance {
    /// Asset path used to load the `.bsn` content.
    pub asset_path: String,
    /// Owning Foundation scene, when the instance came from the scene stack.
    pub scene_owner: Option<SceneOwner>,
    /// Parent entity to reattach to during hot reload, when one exists.
    #[reflect(ignore)]
    pub parent: Option<Entity>,
    /// Loaded scene patch handle used by the temporary Foundation apply path.
    #[reflect(ignore)]
    pub scene_handle: Handle<ScenePatch>,
}

/// Marks a tracked BSN root whose loaded scene patch has not been applied yet.
#[derive(Clone, Copy, Debug, Component)]
struct FoundationBsnApplyPending;

/// Extension methods for spawning `.bsn` prefab or level assets from commands.
pub trait FoundationBsnCommandsExt {
    /// Queues a `.bsn` asset spawn as a new tracked root entity.
    fn spawn_bsn_asset(&mut self, asset_path: impl Into<String>);
}

impl<'world, 'state> FoundationBsnCommandsExt for Commands<'world, 'state> {
    fn spawn_bsn_asset(&mut self, asset_path: impl Into<String>) {
        let asset_path = asset_path.into();
        spawn_bsn_instance(self, asset_path, None, None);
    }
}

fn spawn_requested_bsn_scenes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    registry: Res<FoundationBsnSceneRegistry>,
    mut scene_requests: MessageReader<SceneLoadRequested>,
) {
    for scene_request in scene_requests.read() {
        let SceneSource::BsnScene { key } = &scene_request.source else {
            continue;
        };

        let asset_path = registry.resolve_scene_path(key);
        let scene_owner = SceneOwner {
            scene_id: scene_request.scene_id,
        };
        spawn_bsn_instance_with_asset_server(
            &mut commands,
            &asset_server,
            asset_path,
            Some(scene_owner),
            None,
        );
    }
}

fn spawn_bsn_instance(
    commands: &mut Commands,
    asset_path: String,
    scene_owner: Option<SceneOwner>,
    parent: Option<Entity>,
) {
    commands.queue(move |world: &mut World| {
        let asset_server = world.resource::<AssetServer>().clone();
        let mut commands = world.commands();
        spawn_bsn_instance_with_asset_server(
            &mut commands,
            &asset_server,
            asset_path,
            scene_owner,
            parent,
        );
    });
}

fn spawn_bsn_instance_with_asset_server(
    commands: &mut Commands,
    asset_server: &AssetServer,
    asset_path: String,
    scene_owner: Option<SceneOwner>,
    parent: Option<Entity>,
) -> Entity {
    let scene_handle: Handle<ScenePatch> = asset_server.load(asset_path.clone());
    let mut scene_entity = commands.spawn((
        Name::new(format!("BSN {asset_path}")),
        FoundationBsnInstance {
            asset_path,
            scene_owner,
            parent,
            scene_handle,
        },
        FoundationBsnApplyPending,
    ));

    if let Some(scene_owner) = scene_owner {
        scene_entity.insert(scene_owner);
    }

    if let Some(parent_entity) = parent {
        scene_entity.insert(ChildOf(parent_entity));
    }

    scene_entity.id()
}

fn apply_pending_bsn_instances(world: &mut World) {
    let pending_instances = {
        let mut pending_query = world
            .query_filtered::<(Entity, &FoundationBsnInstance), With<FoundationBsnApplyPending>>();
        pending_query
            .iter(world)
            .map(|(instance_entity, bsn_instance)| {
                (instance_entity, bsn_instance.scene_handle.clone())
            })
            .collect::<Vec<_>>()
    };

    for (instance_entity, scene_handle) in pending_instances {
        let scene_patch_id = scene_handle.id();
        let scene_is_ready = world.resource_scope(
            |world, mut scene_patches: Mut<Assets<ScenePatch>>| -> bool {
                let Some(scene_patch) = scene_patches.get(scene_patch_id) else {
                    return false;
                };

                if scene_patch.resolved.is_some() {
                    return true;
                }

                let scene = scene_patches
                    .get_mut(scene_patch_id)
                    .and_then(|mut scene_patch| scene_patch.scene.take());
                let Some(scene) = scene else {
                    return false;
                };

                let asset_server = world.resource::<AssetServer>();
                match ResolvedSceneRoot::resolve(scene, asset_server, &scene_patches) {
                    Ok(resolved_scene_root) => {
                        if let Some(mut scene_patch) = scene_patches.get_mut(scene_patch_id) {
                            scene_patch.resolved = Some(Arc::new(resolved_scene_root));
                        }
                        true
                    }
                    Err(resolve_error) => {
                        error!(
                            "Failed to resolve Foundation BSN scene {scene_patch_id}: {resolve_error}"
                        );
                        false
                    }
                }
            },
        );

        if !scene_is_ready {
            continue;
        }

        let scene_applied = world.resource_scope(
            |world, scene_patches: Mut<Assets<ScenePatch>>| -> bool {
                let Some(scene_patch) = scene_patches.get(scene_patch_id) else {
                    return false;
                };
                let Ok(mut instance_entity_mut) = world.get_entity_mut(instance_entity) else {
                    return false;
                };

                match scene_patch.apply(&mut instance_entity_mut) {
                    Ok(()) => true,
                    Err(apply_error) => {
                        error!(
                            "Failed to apply Foundation BSN scene {scene_patch_id} to {instance_entity}: {apply_error}"
                        );
                        false
                    }
                }
            },
        );

        if scene_applied {
            if let Ok(mut instance_entity_mut) = world.get_entity_mut(instance_entity) {
                instance_entity_mut.remove::<FoundationBsnApplyPending>();
            }
        }
    }
}

fn replace_reloaded_bsn_instances(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_events: MessageReader<AssetEvent<ScenePatch>>,
    scene_instances: Query<(Entity, &FoundationBsnInstance, Option<&ChildOf>)>,
) {
    let reloaded_asset_ids = scene_events
        .read()
        .filter_map(|asset_event| match asset_event {
            AssetEvent::LoadedWithDependencies { id } => Some(*id),
            AssetEvent::Modified { id } => Some(*id),
            _ => None,
        })
        .collect::<Vec<_>>();

    if reloaded_asset_ids.is_empty() {
        return;
    }

    for (instance_entity, bsn_instance, parent_link) in &scene_instances {
        if !reloaded_asset_ids.contains(&bsn_instance.scene_handle.id()) {
            continue;
        }

        let parent_entity = parent_link.map(|parent_link| parent_link.0);
        let replacement_parent = parent_entity.or(bsn_instance.parent);
        let replacement_asset_path = bsn_instance.asset_path.clone();
        let replacement_scene_owner = bsn_instance.scene_owner;

        // The accepted hot-reload policy is intentionally simple: remove the
        // entire authored instance and spawn a fresh replacement from disk.
        commands.entity(instance_entity).despawn();
        spawn_bsn_instance_with_asset_server(
            &mut commands,
            &asset_server,
            replacement_asset_path,
            replacement_scene_owner,
            replacement_parent,
        );
    }
}

fn propagate_loaded_bsn_scene_owners(
    mut commands: Commands,
    scene_instances: Query<(Entity, &FoundationBsnInstance)>,
    children: Query<&Children>,
    scene_owners: Query<&SceneOwner>,
) {
    for (root_entity, bsn_instance) in &scene_instances {
        let Some(scene_owner) = bsn_instance.scene_owner else {
            continue;
        };

        // Foundation applies pending BSN scene content before this propagation pass so authored
        // roots and children participate in scene cleanup, visibility, and scene-scoped runtime
        // systems such as splashes.
        insert_scene_owner_recursively(
            &mut commands,
            &children,
            &scene_owners,
            root_entity,
            scene_owner,
        );
    }
}

fn insert_scene_owner_recursively(
    commands: &mut Commands,
    children: &Query<&Children>,
    scene_owners: &Query<&SceneOwner>,
    entity: Entity,
    scene_owner: SceneOwner,
) {
    let should_insert_owner = scene_owners
        .get(entity)
        .map(|current_owner| *current_owner != scene_owner)
        .unwrap_or(true);

    if should_insert_owner {
        commands.entity(entity).insert(scene_owner);
    }

    if let Ok(child_entities) = children.get(entity) {
        for child_entity in child_entities.iter() {
            insert_scene_owner_recursively(
                commands,
                children,
                scene_owners,
                child_entity,
                scene_owner,
            );
        }
    }
}

fn _asset_path_is_bsn(asset_path: &str) -> bool {
    AssetPath::parse(asset_path)
        .path()
        .extension()
        .is_some_and(|extension| extension == "bsn")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hot_reload_replaces_old_root_and_children() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::asset::AssetPlugin {
            file_path: ".".to_string(),
            ..default()
        });
        app.init_asset::<ScenePatch>();
        app.add_message::<AssetEvent<ScenePatch>>();
        app.add_systems(Update, replace_reloaded_bsn_instances);

        let scene_handle = app
            .world_mut()
            .resource_mut::<Assets<ScenePatch>>()
            .add(ScenePatch {
                scene: None,
                dependencies: Vec::new(),
                resolved: None,
            });
        let scene_asset_id = scene_handle.id();
        let root_entity = app
            .world_mut()
            .spawn((FoundationBsnInstance {
                asset_path: "scenes/reload-test.bsn".to_string(),
                scene_owner: None,
                parent: None,
                scene_handle,
            },))
            .with_child(())
            .id();
        let child_entity = app.world().get::<Children>(root_entity).unwrap()[0];

        app.world_mut()
            .write_message(AssetEvent::LoadedWithDependencies { id: scene_asset_id });
        app.update();

        assert!(app.world().get_entity(root_entity).is_err());
        assert!(app.world().get_entity(child_entity).is_err());

        let mut instances = app.world_mut().query::<&FoundationBsnInstance>();
        let replacement_count = instances.iter(app.world()).count();
        assert_eq!(replacement_count, 1);
    }

    #[test]
    fn scene_registry_uses_registered_paths() {
        let mut registry = FoundationBsnSceneRegistry::default();
        registry.register_scene("last-beacon/main_menu", "scenes/main_menu.bsn");

        assert_eq!(
            registry.resolve_scene_path("last-beacon/main_menu"),
            "scenes/main_menu.bsn"
        );
    }

    #[test]
    fn scene_registry_falls_back_to_key_as_path() {
        let registry = FoundationBsnSceneRegistry::default();

        assert_eq!(
            registry.resolve_scene_path("levels/intro.bsn"),
            "levels/intro.bsn"
        );
    }
}
