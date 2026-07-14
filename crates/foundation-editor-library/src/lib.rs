//! FoundationEditorLibrary provides reusable Jackdaw editor integrations.
//!
//! Runtime/game systems belong in `foundation-runtime-library`; this crate owns
//! editor-shell integrations that depend on the full Jackdaw editor API.

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

pub mod asset_picker;

use asset_picker::FoundationAssetPickerPlugin;

/// Installs reusable Foundation editor systems.
///
/// Add this plugin to game-specific editor binaries alongside Jackdaw's editor
/// plugins. Foundation settings are edited manually in
/// `foundation.settings.toml`; this plugin creates that file with defaults when
/// it is missing and loads the configured editor startup map.
#[derive(Default)]
pub struct FoundationEditorPlugin;

impl Plugin for FoundationEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FoundationAssetPickerPlugin)
            // Keep the project settings resource populated for editor-only systems.
            .add_systems(Startup, load_foundation_game_settings_from_project_root)
            // Project auto-open loads Jackdaw's default `assets/scene.jsn` first.
            // This follow-up waits for the Outliner before replacing it so UI-only roots are
            // indexed.
            .init_resource::<FoundationEditorStartupSceneState>()
            .add_systems(
                Update,
                load_editor_startup_scene_from_settings.run_if(in_state(jackdaw::AppState::Editor)),
            );
    }
}

fn load_foundation_game_settings_from_project_root(mut commands: Commands) {
    let project_root = current_project_root();
    match FoundationGameSettings::load_or_create_from_project_root(&project_root) {
        Ok(settings) => {
            commands.insert_resource(settings);
        }
        Err(error) => {
            warn!(
                "Failed to load or create Foundation game settings at {}: {error}",
                project_root
                    .join(FOUNDATION_GAME_SETTINGS_FILE_NAME)
                    .display()
            );
        }
    }
}

#[derive(Default, Resource)]
struct FoundationEditorStartupSceneState {
    has_loaded_configured_scene: bool,
}

fn load_editor_startup_scene_from_settings(world: &mut World) {
    if world
        .resource::<FoundationEditorStartupSceneState>()
        .has_loaded_configured_scene
    {
        return;
    }

    if !has_hierarchy_tree_container(world) {
        return;
    }

    world
        .resource_mut::<FoundationEditorStartupSceneState>()
        .has_loaded_configured_scene = true;

    let project_root = current_project_root();
    let Some(settings) = world.get_resource::<FoundationGameSettings>() else {
        info!("No Foundation game settings resource is available; creating an empty editor scene.");
        create_empty_editor_scene(world);
        return;
    };

    match editor_startup_scene_file_path(settings, &project_root) {
        Some(editor_startup_scene_path) => {
            // Jackdaw indexes UI-only root entities through spawn observers when the Outliner
            // already exists. Loading before the tree container is mounted leaves those roots
            // invisible there.
            jackdaw::scene_io::load_scene_from_file(world, &editor_startup_scene_path);
        }
        None => {
            info!(
                "No valid editor_startup_map is configured; creating an empty editor scene instead."
            );
            create_empty_editor_scene(world);
        }
    }
}

fn create_empty_editor_scene(world: &mut World) {
    let empty_scene_file_path = temporary_empty_scene_file_path();
    let empty_scene_contents = r#"{
  "jsn": {
    "format_version": [3, 0, 0],
    "editor_version": "0.4.0",
    "bevy_version": "0.18"
  },
  "metadata": {
    "name": "Untitled",
    "description": "",
    "author": "",
    "created": "",
    "modified": ""
  },
  "assets": {},
  "editor": null,
  "scene": []
}"#;

    if let Err(error) = std::fs::write(&empty_scene_file_path, empty_scene_contents) {
        warn!(
            "Failed to create temporary empty editor scene at {}: {error}",
            empty_scene_file_path.display()
        );
        return;
    }

    // Loading through Jackdaw keeps all editor scene bookkeeping consistent while clearing the
    // project auto-open scene. Reset the path afterward so the scene remains a new unsaved scene.
    jackdaw::scene_io::load_scene_from_file(world, &empty_scene_file_path);
    world
        .resource_mut::<jackdaw::scene_io::SceneFilePath>()
        .path = None;

    if let Err(error) = std::fs::remove_file(&empty_scene_file_path) {
        warn!(
            "Failed to remove temporary empty editor scene at {}: {error}",
            empty_scene_file_path.display()
        );
    }
}

fn temporary_empty_scene_file_path() -> std::path::PathBuf {
    let process_id = std::process::id();
    std::env::temp_dir().join(format!("foundation-empty-editor-scene-{process_id}.jsn"))
}

fn has_hierarchy_tree_container(world: &mut World) -> bool {
    let mut hierarchy_container_query =
        world.query_filtered::<Entity, With<jackdaw::hierarchy::HierarchyTreeContainer>>();
    hierarchy_container_query.iter(world).next().is_some()
}

fn editor_startup_scene_file_path(
    settings: &FoundationGameSettings,
    project_root: &std::path::Path,
) -> Option<std::path::PathBuf> {
    let configured_scene_path = settings.editor_startup_map_path()?;
    let configured_scene_file_path = std::path::Path::new(configured_scene_path);
    let scene_file_path = if configured_scene_file_path.is_absolute() {
        configured_scene_file_path.to_path_buf()
    } else {
        project_root.join("assets").join(configured_scene_file_path)
    };

    if !scene_file_path.is_file() {
        info!(
            "Configured editor_startup_map {} does not exist.",
            scene_file_path.display()
        );
        return None;
    }

    Some(scene_file_path)
}

#[cfg(test)]
fn scene_path_to_asset_path(scene_path: &str) -> Option<String> {
    let normalized_scene_path = scene_path.replace('\\', "/");
    let asset_marker = "/assets/";
    if let Some((_, asset_relative_path)) = normalized_scene_path.rsplit_once(asset_marker) {
        return non_empty_string(asset_relative_path);
    }

    let scene_file_name = std::path::Path::new(scene_path)
        .file_name()?
        .to_string_lossy();
    non_empty_string(scene_file_name.as_ref())
}

#[cfg(test)]
fn non_empty_string(value: &str) -> Option<String> {
    let trimmed_value = value.trim();
    if trimmed_value.is_empty() {
        return None;
    }

    Some(trimmed_value.to_string())
}

fn current_project_root() -> std::path::PathBuf {
    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
}

/// Common imports for game-specific editor binaries using Foundation editor UI.
pub mod prelude {
    pub use crate::asset_picker::{
        spawn_foundation_asset_picker, FoundationAssetPicked, FoundationAssetPickerFilter,
        FoundationAssetPickerPlugin, FoundationAssetPickerProps,
    };
    pub use crate::FoundationEditorPlugin;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_path_to_asset_path_prefers_assets_relative_path() {
        let scene_path = "C:/Project/assets/menus/main_menu.jsn";

        assert_eq!(
            scene_path_to_asset_path(scene_path),
            Some("menus/main_menu.jsn".to_string())
        );
    }

    #[test]
    fn editor_startup_scene_file_path_uses_assets_relative_setting() {
        let project_root = unique_test_directory_path("editor-startup-relative");
        let scene_directory_path = project_root.join("assets").join("menus");
        std::fs::create_dir_all(&scene_directory_path).expect("scene directory should be created");
        let scene_file_path = scene_directory_path.join("main_menu.jsn");
        std::fs::write(&scene_file_path, "{}").expect("scene file should be written");
        let settings = FoundationGameSettings {
            startup_map: String::new(),
            editor_startup_map: "menus/main_menu.jsn".to_string(),
        };

        assert_eq!(
            editor_startup_scene_file_path(&settings, &project_root),
            Some(scene_file_path)
        );

        let _ = std::fs::remove_dir_all(project_root);
    }

    #[test]
    fn editor_startup_scene_file_path_ignores_missing_setting_file() {
        let project_root = unique_test_directory_path("editor-startup-missing");
        let settings = FoundationGameSettings {
            startup_map: String::new(),
            editor_startup_map: "missing.jsn".to_string(),
        };

        assert_eq!(
            editor_startup_scene_file_path(&settings, &project_root),
            None
        );
    }

    #[test]
    fn scene_path_to_asset_path_falls_back_to_file_name() {
        let scene_path = "main_menu.jsn";

        assert_eq!(
            scene_path_to_asset_path(scene_path),
            Some("main_menu.jsn".to_string())
        );
    }

    fn unique_test_directory_path(test_name: &str) -> std::path::PathBuf {
        let process_id = std::process::id();
        let thread_id = format!("{:?}", std::thread::current().id());
        std::env::temp_dir().join(format!(
            "foundation-editor-settings-{test_name}-{process_id}-{thread_id}"
        ))
    }
}
