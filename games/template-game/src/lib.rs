//! TemplateGame gameplay shared between the standalone binary (`cargo run`) and
//! the editor binary (`cargo editor`).
//!
//! Scene content lives in `.jsn` files authored in Jackdaw Editor. Game behavior
//! lives in [`TemplateGamePlugin`].

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
            .add_systems(Update, spin_cube.run_if(play_gate::is_playing));

        #[cfg(feature = "editor")]
        app.add_systems(Update, target_editor_authored_gameplay_ui_roots)
            .insert_resource(FoundationSplashRuntimeSettings {
                enabled: false,
                require_scene_owner: true,
            })
            .add_systems(
                OnEnter(jackdaw::prelude::PlayState::Playing),
                open_initial_scene,
            )
            .add_systems(
                OnExit(jackdaw::prelude::PlayState::Playing),
                clear_scene_stack,
            )
            .add_systems(
                Update,
                (
                    spawn_requested_jackdaw_scenes,
                    detach_scene_stack_ui_roots,
                    complete_authored_ui_text_components,
                    initialize_fullscreen_backgrounds,
                    cleanup_orphaned_fullscreen_backgrounds,
                    initialize_landing_pages,
                    advance_landing_pages,
                    initialize_main_menus,
                    update_main_menu_button_interactions,
                )
                    .run_if(play_gate::is_playing),
            );

        #[cfg(not(feature = "editor"))]
        app.add_systems(Startup, open_initial_scene).add_systems(
            Update,
            (
                spawn_requested_jackdaw_scenes,
                detach_scene_stack_ui_roots,
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
    Entity,
    (
        Added<Node>,
        Or<(
            With<TemplateGameplayUiRoot>,
            With<FoundationSplashUiRoot>,
            With<Text>,
        )>,
    ),
>;

type AuthoredUiTextCompletionQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static Text,
        Option<&'static TextFont>,
        Option<&'static ChildOf>,
    ),
    (With<Text>, Without<TemplateUiTextCompleted>),
>;

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
fn target_editor_authored_gameplay_ui_roots(
    mut commands: Commands,
    active_viewport: Option<Res<jackdaw::viewport::ActiveViewport>>,
    cameras: Query<Entity, With<jackdaw::viewport::MainViewportCamera>>,
    template_roots: Query<Entity, With<TemplateGameplayUiRoot>>,
    splash_roots: Query<Entity, With<FoundationSplashUiRoot>>,
) {
    let target_camera = active_viewport
        .as_deref()
        .and_then(|viewport| viewport.camera)
        .or_else(|| cameras.iter().next());
    let Some(target_camera) = target_camera else {
        return;
    };

    for root in template_roots.iter().chain(splash_roots.iter()) {
        commands.entity(root).insert(UiTargetCamera(target_camera));
    }
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

    if let Some(target_camera) = target_camera {
        world.insert_resource(FoundationSplashUiTargetCamera(target_camera));
    } else {
        world.remove_resource::<FoundationSplashUiTargetCamera>();
        warn!("No Jackdaw viewport camera found; gameplay UI will use Bevy's default UI camera");
    }

    if let Some(viewport_parent) = viewport_parent {
        if let Some(mut node) = world.get_mut::<Node>(viewport_parent) {
            node.overflow = Overflow::clip();
        }
        world.insert_resource(FoundationSplashUiParent(viewport_parent));
    } else {
        world.remove_resource::<FoundationSplashUiParent>();
        warn!(
            "No Jackdaw viewport UI node found; gameplay UI roots will not be parented into the viewport"
        );
    }
}

#[cfg(feature = "editor")]
fn editor_play_scene_commands(world: &World) -> Vec<SceneCommand> {
    let current_scene = world
        .get_resource::<jackdaw::scene_io::SceneFilePath>()
        .and_then(|scene_file| scene_file.path.as_deref())
        .and_then(scene_asset_name)
        .unwrap_or(PIXEL_PERFECT_SPLASH_SCENE);

    match current_scene {
        SPLASH_BACKGROUND_SCENE => initial_scene_commands().into_iter().collect(),
        PIXEL_PERFECT_SPLASH_SCENE | BEVY_SPLASH_SCENE => vec![
            SceneCommand::open_with_options(
                SceneSource::jsn_level(SPLASH_BACKGROUND_SCENE),
                OpenSceneOptions::default()
                    .with_key("splash-background")
                    .with_presentation(ScenePresentation::FULLSCREEN),
            ),
            SceneCommand::open_with_options(
                SceneSource::jsn_level(current_scene),
                OpenSceneOptions::default()
                    .with_key(editor_scene_key(current_scene))
                    .with_presentation(ScenePresentation::INPUT_BLOCKING_OVERLAY),
            ),
        ],
        LANDING_PAGE_SCENE => vec![SceneCommand::clear_and_open(SceneSource::jsn_level(
            LANDING_PAGE_SCENE,
        ))],
        MAIN_MENU_SCENE => vec![SceneCommand::clear_and_open(SceneSource::jsn_level(
            MAIN_MENU_SCENE,
        ))],
        _ => initial_scene_commands().into_iter().collect(),
    }
}

#[cfg(feature = "editor")]
fn scene_asset_name(path: &str) -> Option<&str> {
    std::path::Path::new(path)
        .file_name()
        .and_then(|file_name| file_name.to_str())
}

#[cfg(feature = "editor")]
fn editor_scene_key(path: &str) -> &'static str {
    match path {
        SPLASH_BACKGROUND_SCENE => "splash-background",
        PIXEL_PERFECT_SPLASH_SCENE => "pixel-perfect-splash",
        BEVY_SPLASH_SCENE => "bevy-splash",
        LANDING_PAGE_SCENE => "landing-page",
        MAIN_MENU_SCENE => "main-menu",
        _ => "editor-scene",
    }
}

#[cfg(feature = "editor")]
fn clear_scene_stack(world: &mut World) {
    world.write_message(SceneCommand::Clear);
    world.insert_resource(FoundationSplashRuntimeSettings {
        enabled: false,
        require_scene_owner: true,
    });
    world.remove_resource::<FoundationSplashUiTargetCamera>();
    world.remove_resource::<FoundationSplashUiParent>();
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
            let local_assets = std::collections::HashMap::new();
            let spawned = jackdaw::scene_io::load_scene_from_jsn(
                world,
                &jsn.scene,
                parent_path,
                &local_assets,
            );
            for entity in spawned {
                if let Ok(mut entity) = world.get_entity_mut(entity) {
                    entity.insert(SceneOwner { scene_id });
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

fn complete_authored_ui_text_components(
    mut commands: Commands,
    ui_nodes: AuthoredUiNodeCompletionQuery,
    texts: AuthoredUiTextCompletionQuery,
    child_links: Query<(Entity, &ChildOf)>,
) {
    for entity in &ui_nodes {
        commands
            .entity(entity)
            .remove::<(Transform, GlobalTransform)>();
    }

    let mut parents_to_rebuild = std::collections::HashSet::new();
    for (entity, text, font, parent) in &texts {
        // Jackdaw runtime inserts reflected components one-by-one, which can
        // bypass Bevy's typed `Text` required-components path. Add the UI text
        // measure/layout components that `commands.spawn(Text::new(...))` would
        // normally provide, while preserving the authored Text/TextFont/TextColor.
        let font_size = font.map(|font| font.font_size).unwrap_or(20.0);
        let width = (text.0.chars().count() as f32 * font_size * 0.7).max(font_size);
        if let Some(parent) = parent {
            parents_to_rebuild.insert(parent.0);
        }
        commands.entity(entity).insert((
            TemplateUiTextCompleted,
            Node {
                width: Val::Px(width),
                height: Val::Px(font_size * 1.4),
                ..default()
            },
            TextLayout::default(),
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
            .filter_map(|(child, child_of)| (child_of.0 == parent).then_some(child))
            .collect::<Vec<_>>();
        children.sort_by_key(|child| child.index_u32());
        commands.entity(parent).replace_children(&children);
    }
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
    if let Some(ui_target_camera) = ui_target_camera {
        commands
            .entity(root)
            .insert(UiTargetCamera(ui_target_camera));
    } else if let Some(ui_parent) = ui_parent {
        commands.entity(ui_parent).add_child(root);
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
