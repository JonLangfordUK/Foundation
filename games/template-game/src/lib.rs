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
use foundation_runtime_library::prelude::*;
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
        // Register reflected gameplay types before systems can load them from `.jsn` scenes.
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
        // Standalone builds turn menu exit requests into process exits.
        app.add_systems(Update, exit_game_on_foundation_exit_request);

        #[cfg(feature = "editor")]
        // Editor builds stop Play mode instead of closing the editor process.
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
        // Standalone runs open the startup scene stack immediately.
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
fn open_initial_scene(
    settings: Res<FoundationGameSettings>,
    mut scene_commands: MessageWriter<SceneCommand>,
) {
    // Clear any stale stack entries before replaying the configured startup sequence.
    scene_commands.write(SceneCommand::Clear);
    for command in standalone_startup_scene_commands(&settings) {
        scene_commands.write(command);
    }
}

#[cfg(not(feature = "editor"))]
fn standalone_startup_scene_commands(settings: &FoundationGameSettings) -> Vec<SceneCommand> {
    if let Some(startup_map_path) = settings.startup_map_path() {
        // A configured startup map intentionally bypasses the example splash flow.
        return vec![SceneCommand::clear_and_open(SceneSource::jsn_level(
            startup_map_path,
        ))];
    }

    initial_scene_commands().into_iter().collect()
}

#[cfg(feature = "editor")]
fn hide_editor_authored_scene_for_play(world: &mut World) {
    // Hide authored UI roots so Play mode shows only scene-stack runtime copies.
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
        // Preserve the previous visibility so stopping Play can restore editor state.
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
        // Restore exactly the visibility state that existed before Play mode began.
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
    // Configure UI routing before splash/menu scenes spawn runtime UI.
    configure_editor_gameplay_ui_target(world);
    world.insert_resource(FoundationSplashRuntimeSettings {
        enabled: true,
        require_scene_owner: true,
    });

    let commands = editor_play_scene_commands(world);
    // Rebuild the scene stack from the editor's current file each time Play starts.
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
    let runtime_scene_entity = trigger.event_target();
    // Runtime entities are hidden from the editor hierarchy but still render in Play.
    commands
        .entity(runtime_scene_entity)
        .insert(jackdaw::EditorHidden);
    if let Ok(mut camera) = cameras.get_mut(runtime_scene_entity) {
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

    for (ui_root_entity, child_link) in &roots {
        if viewport_parent.is_some_and(|viewport_parent| {
            child_link.map(|parent_link| parent_link.0) == Some(viewport_parent)
        }) {
            // Earlier builds parented edit-mode UI roots under the editor viewport.
            // Jackdaw treats viewport descendants as editor-owned, so scene-open
            // cleanup can miss those roots and leave stale UI rendered in the
            // viewport after switching .jsn files. Remove any such legacy roots.
            commands.entity(ui_root_entity).despawn();
            continue;
        }

        if let Some(target_camera) = target_camera {
            // Edit-mode UI roots target the viewport camera when they are not parented.
            commands
                .entity(ui_root_entity)
                .insert(UiTargetCamera(target_camera));
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
        // Parent runtime UI into the viewport node when Jackdaw exposes one.
        for (ui_root_entity, child_link) in &roots {
            safely_parent_ui_root_to_viewport(
                &mut commands,
                ui_root_entity,
                child_link,
                viewport_parent,
            );
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

    for (ui_root_entity, _) in &roots {
        // Fall back to camera targeting when there is no viewport UI parent.
        commands
            .entity(ui_root_entity)
            .insert(UiTargetCamera(target_camera));
    }
}

fn safely_parent_ui_root_to_viewport(
    commands: &mut Commands,
    ui_root_entity: Entity,
    current_parent_link: Option<&ChildOf>,
    viewport_parent_entity: Entity,
) {
    let is_already_viewport_child =
        current_parent_link.map(|parent_link| parent_link.0) == Some(viewport_parent_entity);
    if ui_root_entity == viewport_parent_entity || is_already_viewport_child {
        return;
    }

    commands.queue(move |world: &mut World| {
        if world.get_entity(ui_root_entity).is_err()
            || world.get_entity(viewport_parent_entity).is_err()
        {
            return;
        }

        if let Ok(mut root_entity) = world.get_entity_mut(ui_root_entity) {
            // Viewport parenting clips authored UI to the editor play surface.
            root_entity.remove::<UiTargetCamera>();
            root_entity.insert(ChildOf(viewport_parent_entity));
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
        // Clip the viewport parent so gameplay UI cannot spill into editor chrome.
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
        // No useful current scene means Play starts from the configured editor startup map.
        None | Some(SPLASH_BACKGROUND_SCENE) => editor_default_startup_scene_commands(world),
        // Splash scenes need the persistent background below the selected overlay.
        Some(splash_scene @ (PIXEL_PERFECT_SPLASH_SCENE | BEVY_SPLASH_SCENE)) => {
            splash_scene_commands(splash_scene)
        }
        // Any other open scene plays directly so authors can test the current file.
        Some(scene_path) => direct_scene_commands(scene_path),
    }
}

#[cfg(feature = "editor")]
fn editor_default_startup_scene_commands(world: &World) -> Vec<SceneCommand> {
    if let Some(editor_startup_map_path) = world
        .get_resource::<FoundationGameSettings>()
        .and_then(|settings| settings.editor_startup_map_path())
    {
        // The editor startup map is only a fallback; an open authoring scene still wins.
        return editor_configured_scene_commands(editor_startup_map_path);
    }

    initial_scene_commands().into_iter().collect()
}

#[cfg(feature = "editor")]
fn editor_configured_scene_commands(scene_path: &str) -> Vec<SceneCommand> {
    match scene_path {
        PIXEL_PERFECT_SPLASH_SCENE | BEVY_SPLASH_SCENE => splash_scene_commands(scene_path),
        configured_scene_path => direct_scene_commands(configured_scene_path),
    }
}

#[cfg(feature = "editor")]
fn splash_scene_commands(splash_scene_path: &str) -> Vec<SceneCommand> {
    vec![
        SceneCommand::open_with_options(
            SceneSource::jsn_level(SPLASH_BACKGROUND_SCENE),
            OpenSceneOptions::default()
                .with_key("splash-background")
                .with_presentation(ScenePresentation::FULLSCREEN),
        ),
        SceneCommand::open_with_options(
            SceneSource::jsn_level(splash_scene_path),
            OpenSceneOptions::default()
                .with_key(editor_scene_key(splash_scene_path))
                .with_presentation(ScenePresentation::INPUT_BLOCKING_OVERLAY),
        ),
    ]
}

#[cfg(feature = "editor")]
fn direct_scene_commands(scene_path: &str) -> Vec<SceneCommand> {
    vec![SceneCommand::clear_and_open(SceneSource::jsn_level(
        scene_path,
    ))]
}

#[cfg(feature = "editor")]
fn editor_current_scene_asset_path(world: &World) -> Option<String> {
    let raw_scene_file_path = world
        .get_resource::<jackdaw::scene_io::SceneFilePath>()
        .and_then(|scene_file| scene_file.path.as_deref())?;
    let asset_directory_name = "assets";
    let asset_root = std::env::current_dir()
        .unwrap_or_default()
        .join(asset_directory_name);
    scene_asset_path_from_path(raw_scene_file_path, &asset_root)
}

#[cfg(feature = "editor")]
fn scene_asset_path_from_path(
    scene_file_path: &str,
    asset_root: &std::path::Path,
) -> Option<String> {
    let trimmed_scene_file_path = scene_file_path.trim();
    if trimmed_scene_file_path.is_empty() {
        return None;
    }

    let scene_path = std::path::Path::new(trimmed_scene_file_path);
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
fn editor_scene_key(scene_asset_path: &str) -> &'static str {
    match scene_asset_path {
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
    let owned_scene_entities = owned_entities
        .iter(world)
        .map(|(owned_entity, parent_link)| {
            let parent_entity = parent_link.map(|parent_link| parent_link.0);
            (owned_entity, parent_entity)
        })
        .collect::<Vec<_>>();
    let owned_scene_entity_set = owned_scene_entities
        .iter()
        .map(|(owned_entity, _)| *owned_entity)
        .collect::<std::collections::HashSet<_>>();

    for (owned_entity, parent_entity) in owned_scene_entities {
        // Despawn only roots so hierarchy cleanup removes runtime children once.
        if parent_entity
            .is_some_and(|parent_entity| owned_scene_entity_set.contains(&parent_entity))
        {
            continue;
        }
        if let Ok(owned_entity_mut) = world.get_entity_mut(owned_entity) {
            owned_entity_mut.despawn();
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
        // Capture the editor viewport target before mutably iterating runtime cameras.
        let viewport_cameras = cameras.p0();
        active_viewport
            .as_deref()
            .and_then(|viewport| viewport.camera)
            .and_then(|camera| viewport_cameras.get(camera).ok())
            .or_else(|| viewport_cameras.iter().next())
            .map(|(_, camera, target)| (target.clone(), camera.order))
    };
    let Some((viewport_render_target, viewport_camera_order)) = viewport_target else {
        return;
    };

    let mut has_runtime_camera = false;
    {
        let mut runtime_cameras = cameras.p2();
        for (runtime_camera_index, (camera_entity, mut camera, render_target)) in
            runtime_cameras.iter_mut().enumerate()
        {
            // Runtime cameras borrow the viewport target and disable the editor camera.
            has_runtime_camera = true;
            camera.order = viewport_camera_order + runtime_camera_index as isize;
            camera.is_active = runtime_camera_index == 0 && render_target.is_some();
            if let Some(mut render_target) = render_target {
                *render_target = viewport_render_target.clone();
            } else {
                commands
                    .entity(camera_entity)
                    .insert(viewport_render_target.clone());
            }
        }
    }

    for mut viewport_camera in &mut cameras.p1() {
        // Reactivate editor viewport cameras when no runtime scene camera is available.
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
        let SceneSource::JsnLevel {
            path: scene_asset_path,
        } = &request.source
        else {
            continue;
        };

        // Standalone runtime can load Jackdaw scenes directly through the asset server.
        let scene_handle = asset_server.load(scene_asset_path.clone());
        let scene_owner = SceneOwner {
            scene_id: request.scene_id,
        };
        commands.spawn((JackdawSceneRoot(scene_handle), scene_owner));
    }
}

#[cfg(feature = "editor")]
fn spawn_requested_jackdaw_scenes(
    mut commands: Commands,
    mut load_requests: MessageReader<SceneLoadRequested>,
) {
    for request in load_requests.read() {
        let SceneSource::JsnLevel {
            path: scene_asset_path,
        } = &request.source
        else {
            continue;
        };

        let scene_id = request.scene_id;
        let scene_asset_path = scene_asset_path.clone();
        info!(
            "Editor Play loading scene-stack scene `{scene_asset_path}` for scene {}",
            scene_id.0
        );
        commands.queue(move |world: &mut World| {
            // Editor Play loads `.jsn` manually so runtime entities can be tagged immediately.
            let scene_path = std::env::current_dir()
                .unwrap_or_default()
                .join("assets")
                .join(&scene_asset_path);
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
            // Inline assets are loaded relative to the scene file being played.
            let local_assets =
                jackdaw::scene_io::load_inline_assets(world, &jsn.assets, parent_path);
            let original_parent_indices = jsn
                .scene
                .iter()
                .map(|scene_entity| scene_entity.parent)
                .collect::<Vec<_>>();
            let mut root_scene = jsn.scene.clone();
            for scene_entity in &mut root_scene {
                // Load roots first so scene ownership can be applied before parenting is restored.
                scene_entity.parent = None;
            }

            let spawned_entities = jackdaw::scene_io::load_scene_from_jsn(
                world,
                &root_scene,
                parent_path,
                &local_assets,
            );
            info!(
                "Editor Play spawned {} entities for scene-stack scene `{scene_asset_path}`",
                spawned_entities.len()
            );
            for spawned_entity in spawned_entities.iter().copied() {
                // Tag every spawned entity before parent links are restored.
                if let Ok(mut spawned_entity_mut) = world.get_entity_mut(spawned_entity) {
                    let scene_owner = SceneOwner { scene_id };
                    spawned_entity_mut.insert((scene_owner, jackdaw::EditorHidden));
                    if let Some(mut camera) = spawned_entity_mut.get_mut::<Camera>() {
                        camera.is_active = false;
                    }
                }
            }
            for (child_scene_index, parent_scene_index) in
                original_parent_indices.into_iter().enumerate()
            {
                // Rebuild authored hierarchy after ownership and editor-hidden tags are applied.
                let Some(parent_scene_index) = parent_scene_index else {
                    continue;
                };
                let (Some(&child_entity), Some(&parent_entity)) = (
                    spawned_entities.get(child_scene_index),
                    spawned_entities.get(parent_scene_index),
                ) else {
                    continue;
                };
                if world.get_entity(child_entity).is_ok() && world.get_entity(parent_entity).is_ok()
                {
                    world
                        .entity_mut(child_entity)
                        .insert(ChildOf(parent_entity));
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
    scene_owners: Query<&SceneOwner>,
) {
    for (ui_root_entity, child_link) in &roots {
        if let Ok(scene_owner) = scene_owners.get(child_link.0) {
            // Detach scene UI roots so Bevy UI treats them as top-level runtime UI.
            commands
                .entity(ui_root_entity)
                .insert(*scene_owner)
                .remove::<ChildOf>();
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
    for (ui_root_entity, scene_owner, has_global_z_index) in &roots {
        if !has_global_z_index {
            // Space UI layers by scene ID so later stack entries render above earlier ones.
            let z_index_spacing = 10;
            let root_z_index = scene_owner.scene_id.0.saturating_mul(z_index_spacing) as i32;
            commands
                .entity(ui_root_entity)
                .insert(GlobalZIndex(root_z_index));
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
    for (ui_node_entity, scene_owner) in &ui_nodes {
        // Jackdaw-authored UI nodes should not keep transform components at runtime.
        if !should_process_runtime_scene_entity(scene_owner) {
            continue;
        }
        commands
            .entity(ui_node_entity)
            .remove::<(Transform, GlobalTransform)>();
    }

    let mut parents_to_rebuild = std::collections::HashSet::new();
    for (text_entity, parent_link, scene_owner) in &texts {
        if !should_process_runtime_scene_entity(scene_owner) {
            continue;
        }
        // Jackdaw runtime inserts reflected components one-by-one, which can
        // bypass Bevy's typed `Text` required-components path. Add the UI text
        // measure/layout components that `commands.spawn(Text::new(...))` would
        // normally provide, while preserving the authored Text/TextFont/TextColor.
        if let Some(parent_link) = parent_link {
            parents_to_rebuild.insert(parent_link.0);
        }
        commands.entity(text_entity).insert((
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

    for parent_entity in parents_to_rebuild {
        // Rebuild child order from authored metadata so UI layout is deterministic.
        let mut ordered_children = child_links
            .iter()
            .filter_map(|(child_entity, child_link, authored_order)| {
                let child_order = authored_order.map(|authored_order| authored_order.order);
                (child_link.0 == parent_entity).then_some((child_entity, child_order))
            })
            .collect::<Vec<_>>();
        ordered_children.sort_by_key(|(child_entity, child_order)| {
            (child_order.unwrap_or(u32::MAX), child_entity.index_u32())
        });
        let children = ordered_children
            .into_iter()
            .map(|(child_entity, _)| child_entity)
            .collect::<Vec<_>>();
        safe_replace_children(&mut commands, parent_entity, children);
    }
}

fn safe_replace_children(commands: &mut Commands, parent_entity: Entity, children: Vec<Entity>) {
    commands.queue(move |world: &mut World| {
        if world.get_entity(parent_entity).is_err() {
            return;
        }

        // Discard children that were despawned while the replacement command was queued.
        let existing_children = children
            .into_iter()
            .filter(|child_entity| world.get_entity(*child_entity).is_ok())
            .collect::<Vec<_>>();
        if let Ok(mut parent_entity_mut) = world.get_entity_mut(parent_entity) {
            parent_entity_mut.replace_children(&existing_children);
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
            // Authored background roots already contain the UI node we need to target.
            background_entity
        } else {
            // Generate a fullscreen UI root when the scene only authored a marker entity.
            let root_edge_offset = Val::Px(0.0);
            let root_size = Val::Percent(100.0);
            let background_color = Color::srgb(background.red, background.green, background.blue);
            let background_z_index = GlobalZIndex(-1000);

            let ui_root = commands
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: root_edge_offset,
                        right: root_edge_offset,
                        top: root_edge_offset,
                        bottom: root_edge_offset,
                        width: root_size,
                        height: root_size,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(background_color),
                    background_z_index,
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

        // Attach or target the root after it exists so editor and standalone paths share code.
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
            // Remove generated UI once its authored marker source has been despawned.
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
    // Cache target resources once so every landing page uses the same viewport routing.
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
        // Remove the runtime marker before clearing so repeated input cannot double-open scenes.
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
    // Cache target resources once so every main menu uses the same viewport routing.
    let ui_target_camera = ui_target_camera.as_ref().map(|target| target.0);
    let ui_parent = ui_parent.as_ref().map(|parent| parent.0);
    for (menu_entity, scene_owner) in &menus {
        if !should_process_runtime_scene_entity(scene_owner) {
            continue;
        }
        attach_gameplay_ui_root(&mut commands, menu_entity, ui_target_camera, ui_parent);
    }
}

fn update_main_menu_button_interactions(mut buttons: MenuButtonInteractionQuery) {
    let pressed_button_color = Color::srgb(0.45, 0.50, 0.85);
    let hovered_button_color = Color::srgb(0.28, 0.32, 0.62);
    let normal_button_color = Color::srgb(0.12, 0.14, 0.25);

    for (interaction, mut background) in &mut buttons {
        background.0 = match *interaction {
            Interaction::Pressed => pressed_button_color,
            Interaction::Hovered => hovered_button_color,
            Interaction::None => normal_button_color,
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
    let delta_seconds = time.delta_secs();
    for (cube, mut transform) in &mut cubes {
        transform.rotate_y(cube.speed * delta_seconds);
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
    fn scene_path_constants_match_existing_assets() {
        let scene_asset_paths = [
            SPLASH_BACKGROUND_SCENE,
            PIXEL_PERFECT_SPLASH_SCENE,
            BEVY_SPLASH_SCENE,
            LANDING_PAGE_SCENE,
            MAIN_MENU_SCENE,
            OPTIONS_MENU_SCENE,
            LOAD_GAME_SCENE,
            GAMEPLAY_LEVEL_SCENE,
            PAUSE_MENU_SCENE,
        ];
        let asset_directory_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");

        for scene_asset_path in scene_asset_paths {
            let full_scene_asset_path = asset_directory_path.join(scene_asset_path);
            assert!(
                full_scene_asset_path.is_file(),
                "Scene constant `{scene_asset_path}` must point at an existing asset file",
            );
        }
    }

    #[test]
    fn jackdaw_run_config_targets_template_game_binary() {
        let jackdaw_config_path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("jackdaw.toml");
        let jackdaw_config = std::fs::read_to_string(&jackdaw_config_path)
            .expect("jackdaw.toml should be readable during tests");

        assert!(
            jackdaw_config.contains("bin = \"template-game\""),
            "Jackdaw run config should target Cargo's `template-game` binary name",
        );
    }

    #[test]
    fn authored_scene_assets_reference_known_scene_paths() {
        let pixel_perfect_splash_scene = include_str!("../assets/splash_pixel_perfect.jsn");
        let bevy_splash_scene = include_str!("../assets/splash_bevy.jsn");
        let landing_page_scene = include_str!("../assets/landing_page.jsn");
        let main_menu_scene = include_str!("../assets/main_menu.jsn");
        let gameplay_level_scene = include_str!("../assets/gameplay_level.jsn");
        let pause_menu_scene = include_str!("../assets/pause_menu.jsn");

        assert!(pixel_perfect_splash_scene.contains(BEVY_SPLASH_SCENE));
        assert!(bevy_splash_scene.contains(LANDING_PAGE_SCENE));
        assert!(landing_page_scene.contains(MAIN_MENU_SCENE));
        assert!(main_menu_scene.contains(GAMEPLAY_LEVEL_SCENE));
        assert!(main_menu_scene.contains(LOAD_GAME_SCENE));
        assert!(main_menu_scene.contains(OPTIONS_MENU_SCENE));
        assert!(gameplay_level_scene.contains(PAUSE_MENU_SCENE));
        assert!(pause_menu_scene.contains(OPTIONS_MENU_SCENE));
        assert!(pause_menu_scene.contains(MAIN_MENU_SCENE));
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
    fn editor_play_command_uses_editor_startup_map_when_no_scene_is_open() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(FoundationGameSettings {
            startup_map: String::new(),
            editor_startup_map: MAIN_MENU_SCENE.to_string(),
        });

        assert_eq!(
            editor_play_scene_commands(app.world()),
            vec![SceneCommand::clear_and_open(SceneSource::jsn_level(
                MAIN_MENU_SCENE
            ))]
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
