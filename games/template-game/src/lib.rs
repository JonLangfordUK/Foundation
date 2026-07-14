//! PiGame gameplay plugin and BSN-ready scene catalog.
//!
//! The Foundation engine launches this crate as a registered game. Concrete game
//! scenes live here, while reusable scene-stack, splash, menu, and gameplay
//! systems live in `foundation-runtime-library`.

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

/// Foundation game name used by the engine `--game` argument.
pub const GAME_NAME: &str = "PiGame";

/// Scene key for the first startup splash screen.
pub const PIXEL_PERFECT_SPLASH_SCENE: &str = "pigame/splash_pixel_perfect";
/// Scene key for the second startup splash screen.
pub const BEVY_SPLASH_SCENE: &str = "pigame/splash_bevy";
/// Scene key for the example main menu.
pub const MAIN_MENU_SCENE: &str = "pigame/main_menu";
/// Scene key for the stack-based options menu.
pub const OPTIONS_MENU_SCENE: &str = "pigame/options_menu";
/// Scene key for the small sample gameplay level.
pub const GAMEPLAY_LEVEL_SCENE: &str = "pigame/gameplay_level";
/// Scene key for the gameplay pause menu.
pub const PAUSE_MENU_SCENE: &str = "pigame/pause_menu";

/// PiGame's Bevy plugin.
#[derive(Default)]
pub struct PiGamePlugin;

impl Plugin for PiGamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SpinningCube>()
            .add_systems(Startup, open_initial_scene)
            .add_systems(
                Update,
                (
                    spawn_requested_pigame_scenes,
                    exit_game_on_foundation_exit_request,
                    spin_cube.run_if(foundation_is_not_paused),
                ),
            );
    }
}

/// Backwards-compatible alias while the project still uses the `template-game` package name.
pub type TemplateGamePlugin = PiGamePlugin;

fn open_initial_scene(mut scene_commands: MessageWriter<SceneCommand>) {
    let startup_scene_source = SceneSource::bsn_scene(PIXEL_PERFECT_SPLASH_SCENE);
    let startup_scene_options = OpenSceneOptions::default()
        .with_key("startup-splash")
        .with_presentation(ScenePresentation::FULLSCREEN);

    scene_commands.write(SceneCommand::Clear);
    scene_commands.write(SceneCommand::open_with_options(
        startup_scene_source,
        startup_scene_options,
    ));
}

fn spawn_requested_pigame_scenes(
    mut commands: Commands,
    mut scene_requests: MessageReader<SceneLoadRequested>,
) {
    for scene_request in scene_requests.read() {
        let scene_owner = SceneOwner {
            scene_id: scene_request.scene_id,
        };
        let scene_key = scene_source_key(&scene_request.source);

        match scene_key.as_deref() {
            Some(PIXEL_PERFECT_SPLASH_SCENE) => spawn_splash_scene(
                &mut commands,
                scene_owner,
                "Pixel Perfect",
                BEVY_SPLASH_SCENE,
                false,
            ),
            Some(BEVY_SPLASH_SCENE) => {
                spawn_splash_scene(&mut commands, scene_owner, "Bevy", MAIN_MENU_SCENE, true)
            }
            Some(MAIN_MENU_SCENE) => spawn_main_menu_scene(&mut commands, scene_owner),
            Some(OPTIONS_MENU_SCENE) => spawn_options_menu_scene(&mut commands, scene_owner),
            Some(GAMEPLAY_LEVEL_SCENE) => spawn_gameplay_scene(&mut commands, scene_owner),
            Some(PAUSE_MENU_SCENE) => spawn_pause_menu_scene(&mut commands, scene_owner),
            Some(unknown_scene_key) => {
                warn!("Unknown PiGame scene requested: {unknown_scene_key}");
            }
            None => {
                warn!("PiGame received a scene source without a scene key");
            }
        }
    }
}

fn scene_source_key(scene_source: &SceneSource) -> Option<String> {
    match scene_source {
        SceneSource::BsnScene { path } => Some(path.clone()),
        SceneSource::Runtime { key } => Some(key.0.clone()),
    }
}

fn spawn_splash_scene(
    commands: &mut Commands,
    scene_owner: SceneOwner,
    splash_text: &'static str,
    next_scene_key: &'static str,
    reset_stack_for_next_scene: bool,
) {
    let splash_timings = FoundationSplashTimings::new(0.75, 1.0, 0.75);
    let splash_screen = FoundationSplashScreen {
        timings: splash_timings,
        font_size: 72.0,
        next_scene_path: next_scene_key.to_string(),
        reset_stack_for_next_scene,
        replace_current_scene: !reset_stack_for_next_scene,
    };

    commands.spawn_scene(splash_screen_scene(scene_owner, splash_text));
    commands.spawn((Name::new(splash_text), splash_screen, scene_owner));
}

fn splash_screen_scene(scene_owner: SceneOwner, splash_text: &'static str) -> impl Scene {
    let transparent_white_text = Color::srgba(1.0, 1.0, 1.0, 0.0);
    bsn! {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        BackgroundColor(Color::BLACK)
        FoundationSplashUiRoot
        template_value(scene_owner)
        Children [(
            Text(splash_text)
            TextFont { font_size: FontSize::Px(72.0) }
            TextColor(transparent_white_text)
            FoundationSplashText
            template_value(scene_owner)
        )]
    }
}

fn spawn_main_menu_scene(commands: &mut Commands, scene_owner: SceneOwner) {
    let title = "PiGame";
    let buttons = [
        menu_button_scene(
            "New Game",
            FoundationMenuButton::clear_and_open_scene(GAMEPLAY_LEVEL_SCENE, "gameplay"),
        ),
        menu_button_scene(
            "Options",
            FoundationMenuButton::open_overlay_scene(OPTIONS_MENU_SCENE, "options"),
        ),
        menu_button_scene("Exit", FoundationMenuButton::exit()),
    ];

    spawn_menu_root(commands, scene_owner, title, &buttons);
}

fn spawn_options_menu_scene(commands: &mut Commands, scene_owner: SceneOwner) {
    commands.spawn((
        Name::new("Options Menu"),
        FoundationOptionsMenu {
            title: "Options".to_string(),
        },
        FoundationCloseOnEscape,
        scene_owner,
    ));
}

fn spawn_gameplay_scene(commands: &mut Commands, scene_owner: SceneOwner) {
    commands.spawn((
        Name::new("Gameplay Level"),
        FoundationSimpleGameplayLevel { cube_size: 2.0 },
        FoundationPauseOpener {
            pause_scene_path: PAUSE_MENU_SCENE.to_string(),
            pause_scene_key: "pause-menu".to_string(),
        },
        scene_owner,
    ));
}

fn spawn_pause_menu_scene(commands: &mut Commands, scene_owner: SceneOwner) {
    let title = "Paused";
    let buttons = [
        menu_button_scene("Resume", FoundationMenuButton::resume()),
        menu_button_scene(
            "Options",
            FoundationMenuButton::open_overlay_scene(OPTIONS_MENU_SCENE, "options"),
        ),
        menu_button_scene(
            "Main Menu",
            FoundationMenuButton::clear_and_open_scene(MAIN_MENU_SCENE, "main-menu"),
        ),
    ];

    spawn_menu_root(commands, scene_owner, title, &buttons);
    commands.spawn((
        Name::new("Pause Escape Handler"),
        FoundationCloseOnEscape,
        scene_owner,
    ));
}

fn menu_button_scene(label: &'static str, action: FoundationMenuButton) -> MenuButtonScene {
    MenuButtonScene { label, action }
}

struct MenuButtonScene {
    label: &'static str,
    action: FoundationMenuButton,
}

fn spawn_menu_root(
    commands: &mut Commands,
    scene_owner: SceneOwner,
    title: &'static str,
    buttons: &[MenuButtonScene],
) {
    let root_size = Val::Percent(100.0);
    let root_entity = commands
        .spawn((
            Name::new(title),
            Node {
                width: root_size,
                height: root_size,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.02, 0.04, 0.92)),
            scene_owner,
        ))
        .id();

    let title_entity = commands
        .spawn((
            Text::new(title),
            TextFont::from_font_size(64.0),
            TextColor(Color::WHITE),
            scene_owner,
        ))
        .id();
    commands.entity(root_entity).add_child(title_entity);

    for button_scene in buttons {
        let button_entity = spawn_menu_button(commands, scene_owner, button_scene);
        commands.entity(root_entity).add_child(button_entity);
    }
}

fn spawn_menu_button(
    commands: &mut Commands,
    scene_owner: SceneOwner,
    button_scene: &MenuButtonScene,
) -> Entity {
    let button_size = Vec2::new(260.0, 56.0);
    let button_entity = commands
        .spawn((
            Button,
            Node {
                width: Val::Px(button_size.x),
                height: Val::Px(button_size.y),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.18)),
            button_scene.action.clone(),
            scene_owner,
        ))
        .id();

    let label_entity = commands
        .spawn((
            Text::new(button_scene.label),
            TextFont::from_font_size(28.0),
            TextColor(Color::WHITE),
            scene_owner,
        ))
        .id();
    commands.entity(button_entity).add_child(label_entity);

    button_entity
}

fn exit_game_on_foundation_exit_request(
    mut exit_requests: MessageReader<FoundationExitRequested>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for _exit_request in exit_requests.read() {
        app_exit.write(AppExit::Success);
    }
}

/// Example gameplay component used by PiGame-specific systems.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct SpinningCube;

fn spin_cube(time: Res<Time>, mut spinning_entities: Query<&mut Transform, With<SpinningCube>>) {
    for mut transform in &mut spinning_entities {
        let spin_speed_radians_per_second = 0.8;
        let spin_delta = spin_speed_radians_per_second * time.delta_secs();
        transform.rotate_y(spin_delta);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_name_matches_foundation_launch_argument() {
        assert_eq!(GAME_NAME, "PiGame");
    }

    #[test]
    fn required_scene_keys_are_stable() {
        assert_eq!(PIXEL_PERFECT_SPLASH_SCENE, "pigame/splash_pixel_perfect");
        assert_eq!(BEVY_SPLASH_SCENE, "pigame/splash_bevy");
        assert_eq!(MAIN_MENU_SCENE, "pigame/main_menu");
        assert_eq!(OPTIONS_MENU_SCENE, "pigame/options_menu");
        assert_eq!(GAMEPLAY_LEVEL_SCENE, "pigame/gameplay_level");
        assert_eq!(PAUSE_MENU_SCENE, "pigame/pause_menu");
    }
}
