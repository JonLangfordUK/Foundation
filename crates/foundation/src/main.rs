//! Foundation engine launcher.
//!
//! Foundation wraps Bevy app construction, installs reusable Foundation systems,
//! and selects the game to run from command-line arguments.

use bevy::{asset::AssetPlugin, prelude::*};
use foundation_editor_library::prelude::*;
use foundation_runtime_library::prelude::*;

fn main() -> AppExit {
    let interrupt_exit_code = 130;
    let _ = ctrlc::set_handler(move || std::process::exit(interrupt_exit_code));

    let launch_arguments = match FoundationLaunchArguments::parse(std::env::args().skip(1)) {
        Ok(launch_arguments) => launch_arguments,
        Err(argument_error) => {
            eprintln!("{argument_error}");
            eprintln!("Usage: cargo run -p foundation -- --game template-game [--editor]");
            return AppExit::error();
        }
    };

    let Some(game_registration) = registered_games().into_iter().find(|registered_game| {
        registered_game
            .name
            .eq_ignore_ascii_case(&launch_arguments.game)
    }) else {
        eprintln!("Unknown game `{}`.", launch_arguments.game);
        eprintln!("Available games: template-game");
        return AppExit::error();
    };

    run_registered_game(game_registration, launch_arguments)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FoundationLaunchArguments {
    game: String,
    editor_enabled: bool,
}

impl FoundationLaunchArguments {
    fn parse(arguments: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let mut game = None;
        let mut editor_enabled = false;
        let mut argument_iterator = arguments.into_iter();

        while let Some(argument) = argument_iterator.next() {
            match argument.as_str() {
                "--game" => {
                    let Some(game_name) = argument_iterator.next() else {
                        return Err("Expected a game name after `--game`.".to_string());
                    };
                    game = Some(game_name);
                }
                "--editor" => {
                    editor_enabled = true;
                }
                "--help" | "-h" => {
                    return Err("Foundation engine launcher.".to_string());
                }
                unknown_argument => {
                    return Err(format!("Unknown Foundation argument `{unknown_argument}`."));
                }
            }
        }

        let game = game.unwrap_or_else(|| template_game::GAME_NAME.to_string());
        Ok(Self {
            game,
            editor_enabled,
        })
    }
}

#[derive(Clone, Copy)]
struct FoundationGameRegistration {
    name: &'static str,
    asset_root: fn() -> std::path::PathBuf,
    install_plugin: fn(&mut App),
}

fn registered_games() -> Vec<FoundationGameRegistration> {
    vec![FoundationGameRegistration {
        name: template_game::GAME_NAME,
        asset_root: template_game::asset_root,
        install_plugin: |app| {
            app.add_plugins(template_game::TemplateGamePlugin);
        },
    }]
}

fn run_registered_game(
    game_registration: FoundationGameRegistration,
    launch_arguments: FoundationLaunchArguments,
) -> AppExit {
    let game_asset_root = (game_registration.asset_root)();
    let asset_root = game_asset_root
        .canonicalize()
        .unwrap_or(game_asset_root)
        .to_string_lossy()
        .to_string();

    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK))
        .set_error_handler(bevy::ecs::error::error)
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: asset_root,
            ..default()
        }))
        .add_plugins(FoundationPlugin)
        .add_systems(Startup, spawn_default_camera);

    if launch_arguments.editor_enabled {
        app.add_plugins(FoundationEditorPlugin);
        app.insert_resource(FoundationEditorMode { enabled: true });
        info!("Foundation editor mode enabled.");
    }

    (game_registration.install_plugin)(&mut app);
    app.run()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game_and_editor_arguments() {
        let arguments = ["--game", "template-game", "--editor"].map(str::to_string);
        let launch_arguments =
            FoundationLaunchArguments::parse(arguments).expect("arguments should parse");

        assert_eq!(launch_arguments.game, "template-game");
        assert!(launch_arguments.editor_enabled);
    }

    #[test]
    fn default_game_is_template_game() {
        let arguments = std::iter::empty::<String>();
        let launch_arguments = FoundationLaunchArguments::parse(arguments)
            .expect("empty arguments should use the default game");

        assert_eq!(launch_arguments.game, template_game::GAME_NAME);
        assert!(!launch_arguments.editor_enabled);
    }
}
