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
                    initialize_main_menus,
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
            title: "Main Menu".to_string(),
            hint: "Scene stack reset complete".to_string(),
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
            ))
            .id();

        commands.entity(background_entity).add_child(ui_root);
    }
}

fn initialize_main_menus(
    mut commands: Commands,
    menus: Query<(Entity, &TemplateMainMenu), Added<TemplateMainMenu>>,
) {
    for (menu_entity, menu) in &menus {
        let title = commands
            .spawn((
                Text::new(menu.title.clone()),
                TextFont::from_font_size(64.0),
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

        let ui_root = commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.02, 0.025, 0.04)),
            ))
            .add_child(title)
            .add_child(hint)
            .id();

        commands.entity(menu_entity).add_child(ui_root);
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
