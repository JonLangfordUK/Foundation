//! TemplateGame gameplay shared between the standalone binary (`cargo run`) and
//! the editor binary (`cargo editor`).
//!
//! Scene content lives in `.jsn` files authored in Jackdaw Editor. Game behavior
//! lives in [`TemplateGamePlugin`].

use bevy::prelude::*;
use foundation_library::prelude::*;
use jackdaw_runtime::prelude::*;

/// Jackdaw scene path for the persistent startup background.
pub const SPLASH_BACKGROUND_SCENE: &str = "splash_background.jsn";
/// Jackdaw scene path for the first startup splash screen.
pub const PIXEL_PERFECT_SPLASH_SCENE: &str = "splash_pixel_perfect.jsn";
/// Jackdaw scene path for the second startup splash screen.
pub const BEVY_SPLASH_SCENE: &str = "splash_bevy.jsn";
/// Jackdaw scene path for the example main menu.
pub const MAIN_MENU_SCENE: &str = "main_menu.jsn";

/// TemplateGame's Bevy plugin.
#[derive(Default)]
pub struct TemplateGamePlugin;

impl Plugin for TemplateGamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SpinningCube>()
            .register_type::<TemplateFullscreenBackground>()
            .register_type::<TemplateMainMenu>()
            .add_systems(Startup, open_initial_scene)
            .add_systems(
                Update,
                (
                    spawn_requested_jackdaw_scenes,
                    initialize_fullscreen_backgrounds,
                    cleanup_orphaned_fullscreen_backgrounds,
                    initialize_main_menus,
                    advance_main_menu_prompt,
                    update_main_menu_button_interactions,
                    spin_cube.run_if(play_gate::is_playing),
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
struct TemplateMainMenuRuntime {
    root: Entity,
    state: TemplateMainMenuState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TemplateMainMenuState {
    Prompt,
    Buttons,
}

#[derive(Component)]
struct TemplateMenuButton;

type MenuButtonInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static mut BackgroundColor),
    (Changed<Interaction>, With<TemplateMenuButton>),
>;

/// Marker for TemplateGame's example main menu scene.
///
/// The marker is authored in the main-menu `.jsn` file. Runtime game code uses
/// it to create the visible menu UI while the scene stack owns when the menu is
/// loaded and cleaned up.
#[derive(Component, Reflect)]
#[reflect(Component, @EditorCategory::new("TemplateGame"))]
pub struct TemplateMainMenu {
    /// Title text shown in the middle of the example menu.
    pub title: String,
    /// Smaller hint text shown under the title.
    pub hint: String,
}

impl Default for TemplateMainMenu {
    fn default() -> Self {
        Self {
            title: "Template Game".to_string(),
            hint: "Press any button".to_string(),
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

fn open_initial_scene(mut scene_commands: MessageWriter<SceneCommand>) {
    for command in initial_scene_commands() {
        scene_commands.write(command);
    }
}

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

fn initialize_fullscreen_backgrounds(
    mut commands: Commands,
    backgrounds: Query<
        (Entity, &TemplateFullscreenBackground),
        Added<TemplateFullscreenBackground>,
    >,
) {
    for (background_entity, background) in &backgrounds {
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
            ))
            .id();

        debug_assert_ne!(background_entity, ui_root);
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

fn initialize_main_menus(
    mut commands: Commands,
    menus: Query<(Entity, &TemplateMainMenu), Added<TemplateMainMenu>>,
) {
    for (menu_entity, menu) in &menus {
        let ui_root = spawn_main_menu_prompt(&mut commands, menu);
        commands
            .entity(menu_entity)
            .insert(TemplateMainMenuRuntime {
                root: ui_root,
                state: TemplateMainMenuState::Prompt,
            });
    }
}

fn advance_main_menu_prompt(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    gamepad: Option<Res<ButtonInput<GamepadButton>>>,
    mut menus: Query<(&TemplateMainMenu, &mut TemplateMainMenuRuntime)>,
) {
    if !any_button_just_pressed(&keyboard, &mouse, gamepad.as_deref()) {
        return;
    }

    for (_menu, mut runtime) in &mut menus {
        if runtime.state != TemplateMainMenuState::Prompt {
            continue;
        }

        commands.entity(runtime.root).despawn();
        runtime.root = spawn_main_menu_buttons(&mut commands);
        runtime.state = TemplateMainMenuState::Buttons;
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

fn spawn_main_menu_prompt(commands: &mut Commands, menu: &TemplateMainMenu) -> Entity {
    let title = commands
        .spawn((
            Text::new(menu.title.clone()),
            TextFont::from_font_size(72.0),
            TextColor(Color::WHITE),
        ))
        .id();
    let hint = commands
        .spawn((
            Text::new(menu.hint.clone()),
            TextFont::from_font_size(24.0),
            TextColor(Color::srgb(0.75, 0.8, 1.0)),
        ))
        .id();

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.02, 0.025, 0.04)),
        ))
        .add_child(title)
        .add_child(hint)
        .id()
}

fn spawn_main_menu_buttons(commands: &mut Commands) -> Entity {
    let root = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(18.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.02, 0.025, 0.04)),
        ))
        .id();

    for label in ["New Game", "Load Game", "Options", "Quit"] {
        let button = spawn_menu_button(commands, label);
        commands.entity(root).add_child(button);
    }

    root
}

fn spawn_menu_button(commands: &mut Commands, label: &str) -> Entity {
    let text = commands
        .spawn((
            Text::new(label),
            TextFont::from_font_size(28.0),
            TextColor(Color::WHITE),
        ))
        .id();

    commands
        .spawn((
            Button,
            Node {
                width: Val::Px(260.0),
                height: Val::Px(56.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.12, 0.14, 0.25)),
            TemplateMenuButton,
        ))
        .add_child(text)
        .id()
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
    fn default_main_menu_starts_with_template_game_prompt() {
        let menu = TemplateMainMenu::default();
        assert_eq!(menu.title, "Template Game");
        assert_eq!(menu.hint, "Press any button");
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
        assert!(registry.contains(std::any::TypeId::of::<TemplateMainMenu>()));
    }
}
