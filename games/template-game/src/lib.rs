//! TemplateGame gameplay shared between the standalone binary (`cargo run`) and
//! the editor binary (`cargo editor`).
//!
//! Scene content lives in `.jsn` files authored in Jackdaw Editor. Game behavior
//! lives in [`TemplateGamePlugin`].

#[cfg(feature = "editor")]
use bevy::camera::RenderTarget;
use bevy::{
    prelude::*,
    text::{ComputedTextBlock, FontHinting, LineHeight, TextLayout, TextLayoutInfo},
    ui::{ContentSize, widget::TextNodeFlags},
};
use foundation_library::prelude::*;
use jackdaw_runtime::prelude::*;

/// Jackdaw scene path for the persistent startup background.
pub const SPLASH_BACKGROUND_SCENE: &str = "splash_background.jsn";
/// Jackdaw scene path for the first startup splash screen.
pub const PIXEL_PERFECT_SPLASH_SCENE: &str = "splash_pixel_perfect.jsn";
/// Jackdaw scene path for the second startup splash screen.
pub const BEVY_SPLASH_SCENE: &str = "splash_bevy.jsn";
/// Jackdaw scene path for the press-any-button landing page.
pub const LANDING_PAGE_SCENE: &str = "landing_page.jsn";
/// Jackdaw scene path for the example main menu.
pub const MAIN_MENU_SCENE: &str = "main_menu.jsn";
/// Jackdaw scene path for the stack-based options menu.
pub const OPTIONS_MENU_SCENE: &str = "options_menu.jsn";
/// Jackdaw scene path for the dummy load-game menu.
pub const LOAD_GAME_SCENE: &str = "load_game.jsn";
/// Jackdaw scene path for the small sample gameplay level.
pub const GAMEPLAY_LEVEL_SCENE: &str = "gameplay_level.jsn";
/// Jackdaw scene path for the gameplay pause menu.
pub const PAUSE_MENU_SCENE: &str = "pause_menu.jsn";

/// TemplateGame's Bevy plugin.
#[derive(Default)]
pub struct TemplateGamePlugin;

impl Plugin for TemplateGamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SpinningCube>()
            .register_type::<TemplateFullscreenBackground>()
            .register_type::<TemplateGameplayUiRoot>()
            .register_type::<TemplateLandingPage>()
            .register_type::<TemplateMainMenu>()
            .register_type::<TemplateMenuButton>()
            .add_systems(
                Update,
                spin_cube
                    .run_if(play_gate::is_playing)
                    .run_if(foundation_is_not_paused),
            );

        #[cfg(not(feature = "editor"))]
        app.add_systems(Update, exit_game_on_foundation_exit_request);

        #[cfg(feature = "editor")]
        app.add_systems(Update, stop_editor_play_on_foundation_exit_request);

        #[cfg(feature = "editor")]
        app.add_systems(
            Update,
            target_editor_open_scene_ui_roots_to_viewport.run_if(play_gate::is_not_playing),
        )
        .add_systems(Update, target_editor_authored_gameplay_ui_roots)
        .add_observer(mark_editor_runtime_scene_entity)
        .insert_resource(FoundationSplashRuntimeSettings {
            enabled: false,
            require_scene_owner: true,
        })
        .insert_resource(FoundationMenuRuntimeSettings {
            require_scene_owner: true,
        })
        .add_systems(
            OnEnter(jackdaw::prelude::PlayState::Playing),
            (hide_editor_authored_scene_for_play, open_initial_scene).chain(),
        )
        .add_systems(
            OnExit(jackdaw::prelude::PlayState::Playing),
            (
                clear_scene_stack,
                restore_editor_viewport_cameras,
                restore_editor_authored_scene_after_play,
            ),
        )
        .add_systems(
            Update,
            (
                spawn_requested_jackdaw_scenes,
                detach_scene_stack_ui_roots,
                update_scene_stack_ui_root_z_indices,
                complete_authored_ui_text_components,
                initialize_fullscreen_backgrounds,
                cleanup_orphaned_fullscreen_backgrounds,
                initialize_landing_pages,
                advance_landing_pages,
                initialize_main_menus,
                update_main_menu_button_interactions,
            )
                .run_if(play_gate::is_playing),
        )
        .add_systems(
            PostUpdate,
            (
                deactivate_new_editor_runtime_cameras,
                target_editor_runtime_cameras_to_viewport,
            )
                .chain()
                .run_if(play_gate::is_playing),
        );

        #[cfg(not(feature = "editor"))]
        app.add_systems(Startup, open_initial_scene).add_systems(
            Update,
            (
                spawn_requested_jackdaw_scenes,
                detach_scene_stack_ui_roots,
                update_scene_stack_ui_root_z_indices,
                complete_authored_ui_text_components,
                initialize_fullscreen_backgrounds,
                cleanup_orphaned_fullscreen_backgrounds,
                initialize_landing_pages,
                advance_landing_pages,
                initialize_main_menus,
                update_main_menu_button_interactions,
            ),
        );
    }
}

/// Marker for a TemplateGame full-screen background scene.
///
/// This is authored in `.jsn` so splash screens can be transparent UI overlays
/// above a persistent scene-stack background.
#[derive(Component, Reflect)]
#[reflect(Component, @EditorCategory::new("TemplateGame"))]
pub struct TemplateFullscreenBackground {
    /// Red channel in sRGB color space.
    pub red: f32,
    /// Green channel in sRGB color space.
    pub green: f32,
    /// Blue channel in sRGB color space.
    pub blue: f32,
}

impl Default for TemplateFullscreenBackground {
    fn default() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
}

#[derive(Component)]
struct GeneratedFullscreenBackground {
    source: Entity,
}

#[derive(Component)]
struct TemplateLandingPageRuntime;

#[derive(Component)]
struct TemplateUiTextCompleted;

/// Marks a gameplay UI root that should fill the game surface.
///
/// In standalone runs this is a normal root UI node. In Jackdaw edit/play mode
/// TemplateGame retargets these roots to the active editor viewport camera so
/// authored UI remains contained in the viewport.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, @EditorCategory::new("TemplateGame/UI"))]
pub struct TemplateGameplayUiRoot;

/// Marker for a TemplateGame main-menu button authored in `.jsn`.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, @EditorCategory::new("TemplateGame/UI"))]
pub struct TemplateMenuButton;

type MenuButtonInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static mut BackgroundColor),
    (Changed<Interaction>, With<TemplateMenuButton>),
>;

type FullscreenBackgroundQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static TemplateFullscreenBackground,
        Option<&'static SceneOwner>,
        Has<TemplateGameplayUiRoot>,
    ),
    Added<TemplateFullscreenBackground>,
>;

type AuthoredUiNodeCompletionQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, Option<&'static SceneOwner>),
    (
        Added<Node>,
        Or<(
            With<TemplateGameplayUiRoot>,
            With<FoundationSplashUiRoot>,
            With<Text>,
        )>,
        Without<FoundationGeneratedMenuUi>,
    ),
>;

type AuthoredUiTextCompletionQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        Option<&'static ChildOf>,
        Option<&'static SceneOwner>,
    ),
    (
        With<Text>,
        Without<TemplateUiTextCompleted>,
        Without<FoundationGeneratedMenuUi>,
    ),
>;

#[cfg(feature = "editor")]
type EditorGameplayUiRootTargetQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, Option<&'static ChildOf>),
    (
        With<SceneOwner>,
        Or<(With<TemplateGameplayUiRoot>, With<FoundationSplashUiRoot>)>,
    ),
>;

#[cfg(feature = "editor")]
type EditorOpenSceneUiRootTargetQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, Option<&'static ChildOf>),
    (
        Without<SceneOwner>,
        Or<(With<TemplateGameplayUiRoot>, With<FoundationSplashUiRoot>)>,
    ),
>;

#[cfg(feature = "editor")]
type EditorViewportCameraReadQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static Camera, &'static RenderTarget),
    With<jackdaw::viewport::MainViewportCamera>,
>;

#[cfg(feature = "editor")]
type EditorViewportCameraWriteQuery<'w, 's> =
    Query<'w, 's, &'static mut Camera, With<jackdaw::viewport::MainViewportCamera>>;

#[cfg(feature = "editor")]
type EditorRuntimeCameraQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static mut Camera,
        Option<&'static mut RenderTarget>,
    ),
    (
        With<SceneOwner>,
        Without<jackdaw::viewport::MainViewportCamera>,
    ),
>;

#[cfg(feature = "editor")]
#[derive(Component)]
struct EditorAuthoredSceneHiddenForPlay {
    previous_visibility: Option<Visibility>,
}

/// Marker for TemplateGame's press-any-button landing page scene.
///
/// The marker is authored in `landing_page.jsn`. Runtime game code uses it to
/// create the visible prompt while the scene stack owns when the page is loaded
/// and cleaned up.
#[derive(Component, Reflect)]
#[reflect(Component, @EditorCategory::new("TemplateGame"))]
pub struct TemplateLandingPage {
    /// Title text shown in the middle of the landing page.
    pub title: String,
    /// Smaller prompt text shown under the title.
    pub hint: String,
    /// Jackdaw `.jsn` scene path to open when any button is pressed.
    pub next_scene_path: String,
}

impl Default for TemplateLandingPage {
    fn default() -> Self {
        Self {
            title: "Template Game".to_string(),
            hint: "Press any button".to_string(),
            next_scene_path: MAIN_MENU_SCENE.to_string(),
        }
    }
}

/// Marker for TemplateGame's example main menu scene.
///
/// The marker is authored in `main_menu.jsn`. Runtime game code uses it to
/// create the visible menu buttons while the scene stack owns when the menu is
/// loaded and cleaned up.
#[derive(Component, Reflect)]
#[reflect(Component, @EditorCategory::new("TemplateGame"))]
pub struct TemplateMainMenu {
    /// Title text shown above the menu buttons.
    pub title: String,
}

impl Default for TemplateMainMenu {
    fn default() -> Self {
        Self {
            title: "Main Menu".to_string(),
        }
    }
}

/// Creates the startup scene-stack commands for TemplateGame.
pub fn initial_scene_commands() -> [SceneCommand; 2] {
    [
        SceneCommand::open_with_options(
            SceneSource::jsn_level(SPLASH_BACKGROUND_SCENE),
            OpenSceneOptions::default()
                .with_key("splash-background")
                .with_presentation(ScenePresentation::FULLSCREEN),
        ),
        SceneCommand::open_with_options(
            SceneSource::jsn_level(PIXEL_PERFECT_SPLASH_SCENE),
            OpenSceneOptions::default()
                .with_key("pixel-perfect-splash")
                .with_presentation(ScenePresentation::INPUT_BLOCKING_OVERLAY),
        ),
    ]
}

#[cfg(not(feature = "editor"))]
fn open_initial_scene(mut scene_commands: MessageWriter<SceneCommand>) {
    scene_commands.write(SceneCommand::Clear);
    for command in initial_scene_commands() {
        scene_commands.write(command);
    }
}

#[cfg(feature = "editor")]
fn hide_editor_authored_scene_for_play(world: &mut World) {
    let mut roots = world.query_filtered::<(Entity, Option<&Visibility>), (
        Without<SceneOwner>,
        Without<EditorAuthoredSceneHiddenForPlay>,
        Or<(With<TemplateGameplayUiRoot>, With<FoundationSplashUiRoot>)>,
    )>();
    let roots = roots
        .iter(world)
        .map(|(entity, visibility)| (entity, visibility.copied()))
        .collect::<Vec<_>>();

    for (entity, previous_visibility) in roots {
        if let Ok(mut entity) = world.get_entity_mut(entity) {
            entity.insert((
                Visibility::Hidden,
                EditorAuthoredSceneHiddenForPlay {
                    previous_visibility,
                },
            ));
        }
    }
}

#[cfg(feature = "editor")]
fn restore_editor_authored_scene_after_play(
    mut commands: Commands,
    hidden_roots: Query<(Entity, &EditorAuthoredSceneHiddenForPlay)>,
) {
    for (entity, hidden) in &hidden_roots {
        let mut entity_commands = commands.entity(entity);
        entity_commands.remove::<EditorAuthoredSceneHiddenForPlay>();
        if let Some(previous_visibility) = hidden.previous_visibility {
            entity_commands.insert(previous_visibility);
        } else {
            entity_commands.remove::<Visibility>();
        }
    }
}

#[cfg(feature = "editor")]
fn open_initial_scene(world: &mut World) {
    configure_editor_gameplay_ui_target(world);
    world.insert_resource(FoundationSplashRuntimeSettings {
        enabled: true,
        require_scene_owner: true,
    });

    let commands = editor_play_scene_commands(world);
    world.write_message(SceneCommand::Clear);
    for command in commands {
        world.write_message(command);
    }
}

#[cfg(feature = "editor")]
fn mark_editor_runtime_scene_entity(
    trigger: On<Add, SceneOwner>,
    mut commands: Commands,
    mut cameras: Query<&mut Camera>,
) {
    let entity = trigger.event_target();
    commands.entity(entity).insert(jackdaw::EditorHidden);
    if let Ok(mut camera) = cameras.get_mut(entity) {
        camera.is_active = false;
    }
}

#[cfg(feature = "editor")]
fn target_editor_open_scene_ui_roots_to_viewport(
    mut commands: Commands,
    active_viewport: Option<Res<jackdaw::viewport::ActiveViewport>>,
    cameras: Query<Entity, With<jackdaw::viewport::MainViewportCamera>>,
    viewports: Query<Entity, With<jackdaw::viewport::SceneViewport>>,
    roots: EditorOpenSceneUiRootTargetQuery,
) {
    let viewport_parent = active_viewport
        .as_deref()
        .and_then(|viewport| viewport.ui_node)
        .or_else(|| viewports.iter().next());
    let target_camera = active_viewport
        .as_deref()
        .and_then(|viewport| viewport.camera)
        .or_else(|| cameras.iter().next());

    for (root, child_of) in &roots {
        if viewport_parent
            .is_some_and(|viewport_parent| child_of.map(|parent| parent.0) == Some(viewport_parent))
        {
            // Earlier builds parented edit-mode UI roots under the editor viewport.
            // Jackdaw treats viewport descendants as editor-owned, so scene-open
            // cleanup can miss those roots and leave stale UI rendered in the
            // viewport after switching .jsn files. Remove any such legacy roots.
            commands.entity(root).despawn();
            continue;
        }

        if let Some(target_camera) = target_camera {
            commands.entity(root).insert(UiTargetCamera(target_camera));
        }
    }
}

#[cfg(feature = "editor")]
fn target_editor_authored_gameplay_ui_roots(
    mut commands: Commands,
    active_viewport: Option<Res<jackdaw::viewport::ActiveViewport>>,
    cameras: Query<Entity, With<jackdaw::viewport::MainViewportCamera>>,
    roots: EditorGameplayUiRootTargetQuery,
) {
    let viewport_parent = active_viewport
        .as_deref()
        .and_then(|viewport| viewport.ui_node);

    if let Some(viewport_parent) = viewport_parent {
        for (root, child_of) in &roots {
            safely_parent_ui_root_to_viewport(&mut commands, root, child_of, viewport_parent);
        }
        return;
    }

    let target_camera = active_viewport
        .as_deref()
        .and_then(|viewport| viewport.camera)
        .or_else(|| cameras.iter().next());
    let Some(target_camera) = target_camera else {
        return;
    };

    for (root, _) in &roots {
        commands.entity(root).insert(UiTargetCamera(target_camera));
    }
}

fn safely_parent_ui_root_to_viewport(
    commands: &mut Commands,
    root: Entity,
    current_parent: Option<&ChildOf>,
    viewport_parent: Entity,
) {
    if root == viewport_parent || current_parent.map(|parent| parent.0) == Some(viewport_parent) {
        return;
    }

    commands.queue(move |world: &mut World| {
        if world.get_entity(root).is_err() || world.get_entity(viewport_parent).is_err() {
            return;
        }

        if let Ok(mut root_entity) = world.get_entity_mut(root) {
            root_entity.remove::<UiTargetCamera>();
            root_entity.insert(ChildOf(viewport_parent));
        }
    });
}

#[cfg(feature = "editor")]
fn configure_editor_gameplay_ui_target(world: &mut World) {
    let active_viewport = world
        .get_resource::<jackdaw::viewport::ActiveViewport>()
        .copied()
        .unwrap_or_default();

    let target_camera = active_viewport.camera.or_else(|| {
        let mut cameras =
            world.query_filtered::<Entity, With<jackdaw::viewport::MainViewportCamera>>();
        cameras.iter(world).next()
    });
    let viewport_parent = active_viewport.ui_node.or_else(|| {
        let mut viewports =
            world.query_filtered::<Entity, With<jackdaw::viewport::SceneViewport>>();
        viewports.iter(world).next()
    });

    if let Some(viewport_parent) = viewport_parent {
        if let Some(mut node) = world.get_mut::<Node>(viewport_parent) {
            node.overflow = Overflow::clip();
        }
        world.insert_resource(FoundationSplashUiParent(viewport_parent));
        world.remove_resource::<FoundationSplashUiTargetCamera>();
    } else {
        world.remove_resource::<FoundationSplashUiParent>();
        if let Some(target_camera) = target_camera {
            world.insert_resource(FoundationSplashUiTargetCamera(target_camera));
        } else {
            world.remove_resource::<FoundationSplashUiTargetCamera>();
            warn!(
                "No Jackdaw viewport camera found; gameplay UI will use Bevy's default UI camera"
            );
        }
        warn!(
            "No Jackdaw viewport UI node found; gameplay UI roots will not be parented into the viewport"
        );
    }
}

#[cfg(feature = "editor")]
fn editor_play_scene_commands(world: &World) -> Vec<SceneCommand> {
    let current_scene = editor_current_scene_asset_path(world);

    match current_scene.as_deref() {
        None | Some(SPLASH_BACKGROUND_SCENE) => initial_scene_commands().into_iter().collect(),
        Some(splash_scene @ (PIXEL_PERFECT_SPLASH_SCENE | BEVY_SPLASH_SCENE)) => vec![
            SceneCommand::open_with_options(
                SceneSource::jsn_level(SPLASH_BACKGROUND_SCENE),
                OpenSceneOptions::default()
                    .with_key("splash-background")
                    .with_presentation(ScenePresentation::FULLSCREEN),
            ),
            SceneCommand::open_with_options(
                SceneSource::jsn_level(splash_scene),
                OpenSceneOptions::default()
                    .with_key(editor_scene_key(splash_scene))
                    .with_presentation(ScenePresentation::INPUT_BLOCKING_OVERLAY),
            ),
        ],
        Some(scene_path) => vec![SceneCommand::clear_and_open(SceneSource::jsn_level(
            scene_path,
        ))],
    }
}

#[cfg(feature = "editor")]
fn editor_current_scene_asset_path(world: &World) -> Option<String> {
    let raw_path = world
        .get_resource::<jackdaw::scene_io::SceneFilePath>()
        .and_then(|scene_file| scene_file.path.as_deref())?;
    let asset_root = std::env::current_dir().unwrap_or_default().join("assets");
    scene_asset_path_from_path(raw_path, &asset_root)
}

#[cfg(feature = "editor")]
fn scene_asset_path_from_path(path: &str, asset_root: &std::path::Path) -> Option<String> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return None;
    }

    let scene_path = std::path::Path::new(trimmed);
    let relative = scene_path
        .strip_prefix(asset_root)
        .ok()
        .or_else(|| scene_path.strip_prefix("assets").ok())
        .unwrap_or_else(|| {
            scene_path
                .file_name()
                .and_then(|file_name| file_name.to_str())
                .map(std::path::Path::new)
                .unwrap_or(scene_path)
        });

    Some(relative.to_string_lossy().replace('\\', "/"))
}

#[cfg(feature = "editor")]
fn editor_scene_key(path: &str) -> &'static str {
    match path {
        SPLASH_BACKGROUND_SCENE => "splash-background",
        PIXEL_PERFECT_SPLASH_SCENE => "pixel-perfect-splash",
        BEVY_SPLASH_SCENE => "bevy-splash",
        LANDING_PAGE_SCENE => "landing-page",
        MAIN_MENU_SCENE => "main-menu",
        OPTIONS_MENU_SCENE => "options-menu",
        LOAD_GAME_SCENE => "load-game",
        GAMEPLAY_LEVEL_SCENE => "gameplay-level",
        PAUSE_MENU_SCENE => "pause-menu",
        _ => "editor-scene",
    }
}

#[cfg(feature = "editor")]
fn clear_scene_stack(world: &mut World) {
    world.write_message(SceneCommand::Clear);
    despawn_editor_runtime_scene_entities(world);
    world.insert_resource(FoundationSplashRuntimeSettings {
        enabled: false,
        require_scene_owner: true,
    });
    world.remove_resource::<FoundationSplashUiTargetCamera>();
    world.remove_resource::<FoundationSplashUiParent>();
    if let Some(mut pause_state) = world.get_resource_mut::<FoundationPauseState>() {
        pause_state.paused = false;
    }
}

#[cfg(feature = "editor")]
fn despawn_editor_runtime_scene_entities(world: &mut World) {
    let mut owned_entities = world.query_filtered::<(Entity, Option<&ChildOf>), With<SceneOwner>>();
    let owned = owned_entities
        .iter(world)
        .map(|(entity, parent)| (entity, parent.map(|parent| parent.0)))
        .collect::<Vec<_>>();
    let owned_set = owned
        .iter()
        .map(|(entity, _)| *entity)
        .collect::<std::collections::HashSet<_>>();

    for (entity, parent) in owned {
        if parent.is_some_and(|parent| owned_set.contains(&parent)) {
            continue;
        }
        if let Ok(entity) = world.get_entity_mut(entity) {
            entity.despawn();
        }
    }
}

#[cfg(feature = "editor")]
fn deactivate_new_editor_runtime_cameras(mut cameras: EditorRuntimeCameraQuery) {
    for (_, mut camera, _) in &mut cameras {
        camera.is_active = false;
    }
}

#[cfg(feature = "editor")]
fn target_editor_runtime_cameras_to_viewport(
    mut commands: Commands,
    active_viewport: Option<Res<jackdaw::viewport::ActiveViewport>>,
    mut cameras: ParamSet<(
        EditorViewportCameraReadQuery,
        EditorViewportCameraWriteQuery,
        EditorRuntimeCameraQuery,
    )>,
) {
    let viewport_target = {
        let viewport_cameras = cameras.p0();
        active_viewport
            .as_deref()
            .and_then(|viewport| viewport.camera)
            .and_then(|camera| viewport_cameras.get(camera).ok())
            .or_else(|| viewport_cameras.iter().next())
            .map(|(_, camera, target)| (target.clone(), camera.order))
    };
    let Some((target, order)) = viewport_target else {
        return;
    };

    let mut has_runtime_camera = false;
    {
        let mut runtime_cameras = cameras.p2();
        for (index, (entity, mut camera, render_target)) in runtime_cameras.iter_mut().enumerate() {
            has_runtime_camera = true;
            camera.order = order + index as isize;
            camera.is_active = index == 0 && render_target.is_some();
            if let Some(mut render_target) = render_target {
                *render_target = target.clone();
            } else {
                commands.entity(entity).insert(target.clone());
            }
        }
    }

    for mut viewport_camera in &mut cameras.p1() {
        viewport_camera.is_active = !has_runtime_camera;
    }
}

#[cfg(feature = "editor")]
fn restore_editor_viewport_cameras(mut cameras: EditorViewportCameraWriteQuery) {
    for mut camera in &mut cameras {
        camera.is_active = true;
    }
}

#[cfg(not(feature = "editor"))]
fn spawn_requested_jackdaw_scenes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut load_requests: MessageReader<SceneLoadRequested>,
) {
    for request in load_requests.read() {
        let SceneSource::JsnLevel { path } = &request.source else {
            continue;
        };

        commands.spawn((
            JackdawSceneRoot(asset_server.load(path.clone())),
            SceneOwner {
                scene_id: request.scene_id,
            },
        ));
    }
}

#[cfg(feature = "editor")]
fn spawn_requested_jackdaw_scenes(
    mut commands: Commands,
    mut load_requests: MessageReader<SceneLoadRequested>,
) {
    for request in load_requests.read() {
        let SceneSource::JsnLevel { path } = &request.source else {
            continue;
        };

        let scene_id = request.scene_id;
        let path = path.clone();
        info!(
            "Editor Play loading scene-stack scene `{path}` for scene {}",
            scene_id.0
        );
        commands.queue(move |world: &mut World| {
            let scene_path = std::env::current_dir()
                .unwrap_or_default()
                .join("assets")
                .join(&path);
            let Ok(json) = std::fs::read_to_string(&scene_path) else {
                warn!(
                    "Failed to read scene stack .jsn scene {}",
                    scene_path.display()
                );
                return;
            };
            let Ok(jsn) = serde_json::from_str::<jackdaw_jsn::format::JsnScene>(&json) else {
                warn!(
                    "Failed to parse scene stack .jsn scene {}",
                    scene_path.display()
                );
                return;
            };

            let parent_path = scene_path
                .parent()
                .unwrap_or_else(|| std::path::Path::new(""));
            let local_assets =
                jackdaw::scene_io::load_inline_assets(world, &jsn.assets, parent_path);
            let original_parents = jsn
                .scene
                .iter()
                .map(|entity| entity.parent)
                .collect::<Vec<_>>();
            let mut root_scene = jsn.scene.clone();
            for entity in &mut root_scene {
                entity.parent = None;
            }

            let spawned = jackdaw::scene_io::load_scene_from_jsn(
                world,
                &root_scene,
                parent_path,
                &local_assets,
            );
            info!(
                "Editor Play spawned {} entities for scene-stack scene `{path}`",
                spawned.len()
            );
            for entity in spawned.iter().copied() {
                if let Ok(mut entity) = world.get_entity_mut(entity) {
                    entity.insert((SceneOwner { scene_id }, jackdaw::EditorHidden));
                    if let Some(mut camera) = entity.get_mut::<Camera>() {
                        camera.is_active = false;
                    }
                }
            }
            for (child_index, parent_index) in original_parents.into_iter().enumerate() {
                let Some(parent_index) = parent_index else {
                    continue;
                };
                let (Some(&child), Some(&parent)) =
                    (spawned.get(child_index), spawned.get(parent_index))
                else {
                    continue;
                };
                if world.get_entity(child).is_ok() && world.get_entity(parent).is_ok() {
                    world.entity_mut(child).insert(ChildOf(parent));
                }
            }
        });
    }
}

type GameplayUiRootQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static ChildOf),
    Or<(With<TemplateGameplayUiRoot>, With<FoundationSplashUiRoot>)>,
>;

fn detach_scene_stack_ui_roots(
    mut commands: Commands,
    roots: GameplayUiRootQuery,
    owners: Query<&SceneOwner>,
) {
    for (root, child_of) in &roots {
        if let Ok(owner) = owners.get(child_of.0) {
            commands.entity(root).insert(*owner).remove::<ChildOf>();
        }
    }
}

type SceneStackUiRootZIndexQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static SceneOwner, Has<GlobalZIndex>),
    (
        With<TemplateGameplayUiRoot>,
        Without<GeneratedFullscreenBackground>,
    ),
>;

fn update_scene_stack_ui_root_z_indices(
    mut commands: Commands,
    roots: SceneStackUiRootZIndexQuery,
) {
    for (root, owner, has_global_z_index) in &roots {
        if !has_global_z_index {
            commands
                .entity(root)
                .insert(GlobalZIndex(owner.scene_id.0.saturating_mul(10) as i32));
        }
    }
}

fn complete_authored_ui_text_components(
    mut commands: Commands,
    ui_nodes: AuthoredUiNodeCompletionQuery,
    texts: AuthoredUiTextCompletionQuery,
    child_links: Query<
        (Entity, &ChildOf, Option<&FoundationUiOrder>),
        Without<FoundationGeneratedMenuUi>,
    >,
) {
    for (entity, scene_owner) in &ui_nodes {
        if !should_process_runtime_scene_entity(scene_owner) {
            continue;
        }
        commands
            .entity(entity)
            .remove::<(Transform, GlobalTransform)>();
    }

    let mut parents_to_rebuild = std::collections::HashSet::new();
    for (entity, parent, scene_owner) in &texts {
        if !should_process_runtime_scene_entity(scene_owner) {
            continue;
        }
        // Jackdaw runtime inserts reflected components one-by-one, which can
        // bypass Bevy's typed `Text` required-components path. Add the UI text
        // measure/layout components that `commands.spawn(Text::new(...))` would
        // normally provide, while preserving the authored Text/TextFont/TextColor.
        if let Some(parent) = parent {
            parents_to_rebuild.insert(parent.0);
        }
        commands.entity(entity).insert((
            TemplateUiTextCompleted,
            Node::default(),
            TextLayout::new_with_justify(Justify::Center),
            ComputedTextBlock::default(),
            TextLayoutInfo::default(),
            LineHeight::default(),
            TextNodeFlags::default(),
            ContentSize::default(),
            FontHinting::Disabled,
        ));
    }

    for parent in parents_to_rebuild {
        let mut children = child_links
            .iter()
            .filter_map(|(child, child_of, order)| {
                (child_of.0 == parent).then_some((child, order.map(|order| order.order)))
            })
            .collect::<Vec<_>>();
        children.sort_by_key(|(child, order)| (order.unwrap_or(u32::MAX), child.index_u32()));
        let children = children
            .into_iter()
            .map(|(child, _)| child)
            .collect::<Vec<_>>();
        safe_replace_children(&mut commands, parent, children);
    }
}

fn safe_replace_children(commands: &mut Commands, parent: Entity, children: Vec<Entity>) {
    commands.queue(move |world: &mut World| {
        if world.get_entity(parent).is_err() {
            return;
        }

        let existing_children = children
            .into_iter()
            .filter(|child| world.get_entity(*child).is_ok())
            .collect::<Vec<_>>();
        if let Ok(mut parent_entity) = world.get_entity_mut(parent) {
            parent_entity.replace_children(&existing_children);
        }
    });
}

fn initialize_fullscreen_backgrounds(
    mut commands: Commands,
    backgrounds: FullscreenBackgroundQuery,
    ui_target_camera: Option<Res<FoundationSplashUiTargetCamera>>,
    ui_parent: Option<Res<FoundationSplashUiParent>>,
) {
    for (background_entity, background, scene_owner, has_authored_root) in &backgrounds {
        if !should_process_runtime_scene_entity(scene_owner) {
            continue;
        }

        let ui_root = if has_authored_root {
            background_entity
        } else {
            let ui_root = commands
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.0),
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(
                        background.red,
                        background.green,
                        background.blue,
                    )),
                    GlobalZIndex(-1000),
                    GeneratedFullscreenBackground {
                        source: background_entity,
                    },
                    TemplateGameplayUiRoot,
                ))
                .id();

            if let Some(scene_owner) = scene_owner.copied() {
                commands.entity(ui_root).insert(scene_owner);
            }
            debug_assert_ne!(background_entity, ui_root);
            ui_root
        };

        attach_gameplay_ui_root(
            &mut commands,
            ui_root,
            ui_target_camera.as_ref().map(|target| target.0),
            ui_parent.as_ref().map(|parent| parent.0),
        );
    }
}

fn cleanup_orphaned_fullscreen_backgrounds(
    mut commands: Commands,
    generated_backgrounds: Query<(Entity, &GeneratedFullscreenBackground)>,
    background_sources: Query<(), With<TemplateFullscreenBackground>>,
) {
    for (generated_entity, generated_background) in &generated_backgrounds {
        if background_sources.get(generated_background.source).is_err() {
            commands.entity(generated_entity).despawn();
        }
    }
}

fn initialize_landing_pages(
    mut commands: Commands,
    landing_pages: Query<(Entity, Option<&SceneOwner>), Added<TemplateLandingPage>>,
    ui_target_camera: Option<Res<FoundationSplashUiTargetCamera>>,
    ui_parent: Option<Res<FoundationSplashUiParent>>,
) {
    let ui_target_camera = ui_target_camera.as_ref().map(|target| target.0);
    let ui_parent = ui_parent.as_ref().map(|parent| parent.0);
    for (landing_entity, scene_owner) in &landing_pages {
        if !should_process_runtime_scene_entity(scene_owner) {
            continue;
        }
        attach_gameplay_ui_root(&mut commands, landing_entity, ui_target_camera, ui_parent);
        commands
            .entity(landing_entity)
            .insert(TemplateLandingPageRuntime);
    }
}

fn advance_landing_pages(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    gamepad: Option<Res<ButtonInput<GamepadButton>>>,
    landing_pages: Query<(Entity, &TemplateLandingPage), With<TemplateLandingPageRuntime>>,
    mut scene_commands: MessageWriter<SceneCommand>,
) {
    if !any_button_just_pressed(&keyboard, &mouse, gamepad.as_deref()) {
        return;
    }

    for (landing_entity, landing_page) in &landing_pages {
        commands
            .entity(landing_entity)
            .remove::<TemplateLandingPageRuntime>();
        scene_commands.write(SceneCommand::clear_and_open(SceneSource::jsn_level(
            landing_page.next_scene_path.trim(),
        )));
    }
}

fn initialize_main_menus(
    mut commands: Commands,
    menus: Query<(Entity, Option<&SceneOwner>), Added<TemplateMainMenu>>,
    ui_target_camera: Option<Res<FoundationSplashUiTargetCamera>>,
    ui_parent: Option<Res<FoundationSplashUiParent>>,
) {
    let ui_target_camera = ui_target_camera.as_ref().map(|target| target.0);
    let ui_parent = ui_parent.as_ref().map(|parent| parent.0);
    for (menu_entity, scene_owner) in &menus {
        if !should_process_runtime_scene_entity(scene_owner) {
            continue;
        }
        attach_gameplay_ui_root(&mut commands, menu_entity, ui_target_camera, ui_parent);
    }
}

#[allow(dead_code)]
fn update_main_menu_button_interactions(mut buttons: MenuButtonInteractionQuery) {
    for (interaction, mut background) in &mut buttons {
        background.0 = match *interaction {
            Interaction::Pressed => Color::srgb(0.45, 0.50, 0.85),
            Interaction::Hovered => Color::srgb(0.28, 0.32, 0.62),
            Interaction::None => Color::srgb(0.12, 0.14, 0.25),
        };
    }
}

fn any_button_just_pressed(
    keyboard: &ButtonInput<KeyCode>,
    mouse: &ButtonInput<MouseButton>,
    gamepad: Option<&ButtonInput<GamepadButton>>,
) -> bool {
    keyboard.get_just_pressed().next().is_some()
        || mouse.get_just_pressed().next().is_some()
        || gamepad.is_some_and(|gamepad| gamepad.get_just_pressed().next().is_some())
}

fn attach_gameplay_ui_root(
    commands: &mut Commands,
    root: Entity,
    ui_target_camera: Option<Entity>,
    ui_parent: Option<Entity>,
) {
    if let Some(ui_parent) = ui_parent {
        safely_parent_ui_root_to_viewport(commands, root, None, ui_parent);
    } else if let Some(ui_target_camera) = ui_target_camera {
        commands
            .entity(root)
            .insert(UiTargetCamera(ui_target_camera));
    }
}

#[cfg(not(feature = "editor"))]
fn exit_game_on_foundation_exit_request(
    mut exit_requests: MessageReader<FoundationExitRequested>,
    mut app_exit: MessageWriter<AppExit>,
) {
    if exit_requests.read().next().is_some() {
        app_exit.write(AppExit::Success);
    }
}

#[cfg(feature = "editor")]
fn stop_editor_play_on_foundation_exit_request(
    mut exit_requests: MessageReader<FoundationExitRequested>,
    mut next_play_state: ResMut<NextState<jackdaw::prelude::PlayState>>,
) {
    if exit_requests.read().next().is_some() {
        next_play_state.set(jackdaw::prelude::PlayState::Stopped);
    }
}

fn should_process_runtime_scene_entity(scene_owner: Option<&SceneOwner>) -> bool {
    if cfg!(feature = "editor") {
        scene_owner.is_some()
    } else {
        true
    }
}

/// Spin-rate in radians per second. Attach in the inspector while authoring.
#[derive(Component, Reflect)]
#[reflect(Component, @EditorCategory::new("Actor"))]
pub struct SpinningCube {
    /// Rotation speed around the Y axis, in radians per second.
    pub speed: f32,
}

fn spin_cube(time: Res<Time>, mut cubes: Query<(&SpinningCube, &mut Transform)>) {
    let dt = time.delta_secs();
    for (cube, mut transform) in &mut cubes {
        transform.rotate_y(cube.speed * dt);
    }
}

/// Bridges Jackdaw Play mode to gameplay without forcing a `jackdaw`
/// dependency in standalone builds.
pub mod play_gate {
    #[cfg(feature = "editor")]
    pub fn is_playing(
        state: bevy::prelude::Res<bevy::state::state::State<jackdaw::prelude::PlayState>>,
    ) -> bool {
        matches!(*state.get(), jackdaw::prelude::PlayState::Playing)
    }

    #[cfg(feature = "editor")]
    pub fn is_not_playing(
        state: bevy::prelude::Res<bevy::state::state::State<jackdaw::prelude::PlayState>>,
    ) -> bool {
        !matches!(*state.get(), jackdaw::prelude::PlayState::Playing)
    }

    #[cfg(not(feature = "editor"))]
    pub fn is_playing() -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_paths_match_stack_example_assets() {
        assert_eq!(SPLASH_BACKGROUND_SCENE, "splash_background.jsn");
        assert_eq!(PIXEL_PERFECT_SPLASH_SCENE, "splash_pixel_perfect.jsn");
        assert_eq!(BEVY_SPLASH_SCENE, "splash_bevy.jsn");
        assert_eq!(LANDING_PAGE_SCENE, "landing_page.jsn");
        assert_eq!(MAIN_MENU_SCENE, "main_menu.jsn");
        assert_eq!(OPTIONS_MENU_SCENE, "options_menu.jsn");
        assert_eq!(LOAD_GAME_SCENE, "load_game.jsn");
        assert_eq!(GAMEPLAY_LEVEL_SCENE, "gameplay_level.jsn");
        assert_eq!(PAUSE_MENU_SCENE, "pause_menu.jsn");
    }

    #[test]
    fn initial_scene_commands_open_background_then_pixel_perfect_splash() {
        assert_eq!(
            initial_scene_commands(),
            [
                SceneCommand::Open {
                    source: SceneSource::jsn_level(SPLASH_BACKGROUND_SCENE),
                    options: OpenSceneOptions::default()
                        .with_key("splash-background")
                        .with_presentation(ScenePresentation::FULLSCREEN),
                },
                SceneCommand::Open {
                    source: SceneSource::jsn_level(PIXEL_PERFECT_SPLASH_SCENE),
                    options: OpenSceneOptions::default()
                        .with_key("pixel-perfect-splash")
                        .with_presentation(ScenePresentation::INPUT_BLOCKING_OVERLAY),
                },
            ]
        );
    }

    #[test]
    fn default_landing_page_leads_to_main_menu() {
        let landing_page = TemplateLandingPage::default();
        assert_eq!(landing_page.title, "Template Game");
        assert_eq!(landing_page.hint, "Press any button");
        assert_eq!(landing_page.next_scene_path, MAIN_MENU_SCENE);
    }

    #[test]
    fn default_main_menu_has_menu_title() {
        let menu = TemplateMainMenu::default();
        assert_eq!(menu.title, "Main Menu");
    }

    #[cfg(feature = "editor")]
    #[test]
    fn editor_scene_asset_path_prefers_asset_relative_paths() {
        let asset_root = std::path::Path::new("C:/project/assets");

        assert_eq!(
            scene_asset_path_from_path("C:/project/assets/menus/custom.jsn", asset_root),
            Some("menus/custom.jsn".to_string())
        );
        assert_eq!(
            scene_asset_path_from_path("assets/main_menu.jsn", asset_root),
            Some("main_menu.jsn".to_string())
        );
        assert_eq!(
            scene_asset_path_from_path("D:/other/custom_scene.jsn", asset_root),
            Some("custom_scene.jsn".to_string())
        );
    }

    #[cfg(feature = "editor")]
    #[test]
    fn editor_play_command_opens_unknown_current_scene_directly() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<jackdaw::scene_io::SceneFilePath>();
        app.world_mut()
            .resource_mut::<jackdaw::scene_io::SceneFilePath>()
            .path = Some("assets/custom_scene.jsn".to_string());

        assert_eq!(
            editor_play_scene_commands(app.world()),
            vec![SceneCommand::clear_and_open(SceneSource::jsn_level(
                "custom_scene.jsn"
            ))]
        );
    }

    #[test]
    fn template_game_plugin_registers_menu_marker() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(TemplateGamePlugin);

        let registry = app
            .world()
            .resource::<bevy::ecs::reflect::AppTypeRegistry>()
            .read();
        assert!(registry.contains(std::any::TypeId::of::<TemplateFullscreenBackground>()));
        assert!(registry.contains(std::any::TypeId::of::<TemplateLandingPage>()));
        assert!(registry.contains(std::any::TypeId::of::<TemplateMainMenu>()));
    }
}
