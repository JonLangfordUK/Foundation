//! TemplateGame gameplay plugin and Foundation engine integration.
//!
//! The Foundation engine launches this crate as a registered game. Concrete BSN
//! scenes live in [`scenes`], while reusable scene-stack, splash, menu, and
//! gameplay systems live in `foundation-runtime-library`.

use std::path::PathBuf;

use bevy::{
    asset::AssetPlugin,
    prelude::*,
    render::{
        settings::{Backends, InstanceFlags, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
#[cfg(feature = "editor")]
use foundation_editor_library::prelude::*;
use foundation_runtime_library::prelude::*;

pub mod scenes;

/// Foundation game name used by the engine `--game` argument.
pub const GAME_NAME: &str = "template-game";

/// Returns TemplateGame's asset root.
///
/// Foundation uses this when launching TemplateGame as a statically registered game.
pub fn asset_root() -> PathBuf {
    if let Ok(explicit_asset_root) = std::env::var("FOUNDATION_ASSET_ROOT") {
        return PathBuf::from(explicit_asset_root);
    }

    if let Some(packaged_asset_root) = packaged_asset_root() {
        return packaged_asset_root;
    }

    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets")
}

fn packaged_asset_root() -> Option<PathBuf> {
    let executable_directory = std::env::current_exe()
        .ok()
        .and_then(|executable_path| executable_path.parent().map(std::path::Path::to_path_buf))?;
    let packaged_asset_root = executable_directory.join("assets");
    packaged_asset_root.is_dir().then_some(packaged_asset_root)
}

/// Runs TemplateGame with Foundation runtime systems installed.
///
/// This is shared by the thin Cargo binary wrapper and future packaged launchers.
pub fn run() -> AppExit {
    let asset_root = asset_root().to_string_lossy().to_string();
    let editor_enabled =
        cfg!(feature = "editor") && std::env::args().any(|argument| argument == "--editor");

    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK))
        .set_error_handler(bevy::ecs::error::error)
        .add_plugins(template_game_default_plugins(asset_root))
        .add_plugins(FoundationPlugin)
        .add_plugins(TemplateGamePlugin)
        .add_systems(Startup, spawn_default_camera);

    add_editor_plugins(&mut app, editor_enabled);

    app.run()
}

fn template_game_default_plugins(asset_root: String) -> impl PluginGroup {
    DefaultPlugins
        .build()
        .disable::<GilrsPlugin>()
        .set(AssetPlugin {
            file_path: asset_root,
            ..default()
        })
        .set(RenderPlugin {
            render_creation: RenderCreation::Automatic(Box::new(WgpuSettings {
                backends: platform_render_backends(),
                instance_flags: InstanceFlags::empty().with_env(),
                ..default()
            })),
            ..default()
        })
}

fn platform_render_backends() -> Option<Backends> {
    // Keep the fast Windows path that made gameplay appear immediately, while disabling
    // validation layers separately so local Vulkan SDK warnings do not flood normal logs.
    #[cfg(target_os = "windows")]
    {
        Some(Backends::from_env().unwrap_or(Backends::VULKAN))
    }

    #[cfg(not(target_os = "windows"))]
    {
        Some(Backends::from_env().unwrap_or(Backends::PRIMARY))
    }
}

fn spawn_default_camera(mut commands: Commands) {
    let camera_order = 100;
    commands.spawn((
        Camera2d,
        Camera {
            order: camera_order,
            ..default()
        },
    ));
}

/// TemplateGame's Bevy plugin.
#[derive(Default)]
pub struct TemplateGamePlugin;

impl Plugin for TemplateGamePlugin {
    fn build(&self, app: &mut App) {
        // Credits JSON lives under this game's asset directory; the reusable
        // credits systems only search roots that games register here.
        app.insert_resource(FoundationCreditsAssetRoots {
            roots: vec![asset_root()],
        })
        .register_type::<SpinningCube>()
        .add_systems(Startup, scenes::open_initial_scene)
        .add_systems(
            Update,
            (
                scenes::spawn_requested_template_game_scenes,
                exit_game_on_foundation_exit_request,
                spin_cube.run_if(foundation_is_not_paused),
            ),
        );
    }
}

fn exit_game_on_foundation_exit_request(
    mut exit_requests: MessageReader<FoundationExitRequested>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for _exit_request in exit_requests.read() {
        app_exit.write(AppExit::Success);
    }
}

#[cfg(feature = "editor")]
fn add_editor_plugins(app: &mut App, editor_enabled: bool) {
    if editor_enabled {
        app.add_plugins(FoundationEditorPlugin);
        app.insert_resource(FoundationEditorMode { enabled: true });
        debug!("Foundation editor mode enabled for TemplateGame.");
    }
}

#[cfg(not(feature = "editor"))]
fn add_editor_plugins(_app: &mut App, editor_enabled: bool) {
    if editor_enabled {
        warn!("TemplateGame was built without editor support; ignoring `--editor`.");
    }
}

/// Inputs for TemplateGame's example console greeting command.
#[cfg(feature = "dev-tools")]
#[derive(Clone, Debug, ConsoleCommandInput)]
pub struct TemplateGameConsoleGreetingInputs {
    /// Name that should appear in the console greeting.
    pub name: String,
}

/// Example TemplateGame-authored console command.
#[cfg(feature = "dev-tools")]
#[console_command]
pub fn template_game_greeting(inputs: ConsoleInputs<TemplateGameConsoleGreetingInputs>) {
    info!("TemplateGame console greeting for {}.", inputs.name);
}

/// Inputs for TemplateGame's user-facing `example.say-hello` console command.
#[cfg(feature = "dev-tools")]
#[derive(Clone, Debug, ConsoleCommandInput)]
pub struct TemplateGameSayHelloInputs {
    /// Name that should be greeted by the example command.
    pub name: String,
}

/// Example command that demonstrates using an argument.
#[cfg(feature = "dev-tools")]
#[console_command(name = "example.say-hello")]
pub fn say_hello(inputs: ConsoleInputs<TemplateGameSayHelloInputs>) {
    info!("Hello, {}!", inputs.name);
}

/// Example simple command that has no arguments.
#[cfg(feature = "dev-tools")]
#[console_command(name = "example.say-hello-world")]
pub fn say_hello_world() {
    info!("Hello World!");
}

/// Example gameplay component used by TemplateGame-specific systems.
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
        assert_eq!(GAME_NAME, "template-game");
    }

    #[test]
    fn template_game_registers_its_credits_asset_root() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(FoundationPlugin);
        app.add_plugins(TemplateGamePlugin);

        let credits_asset_roots = app.world().resource::<FoundationCreditsAssetRoots>();
        assert!(
            credits_asset_roots.roots.contains(&asset_root()),
            "TemplateGame should search its own asset directory for credits JSON"
        );
    }

    #[test]
    #[cfg(feature = "dev-tools")]
    fn template_game_console_command_is_linked_into_template_game_binary() {
        let registry = FoundationConsoleRegistry::default();

        assert!(registry
            .commands()
            .iter()
            .any(|command| command.name == "template_game_greeting"));
    }

    #[test]
    #[cfg(feature = "dev-tools")]
    fn say_hello_console_command_uses_overridden_name() {
        let registry = FoundationConsoleRegistry::default();
        let say_hello_command = registry
            .commands()
            .iter()
            .find(|command| command.name == "example.say-hello")
            .expect("say_hello should register with its overridden console name");
        let parameters = (say_hello_command.parameters)();

        assert_eq!(parameters[0].name, "name");
        assert_eq!(parameters[0].type_name, "String");
    }
}
