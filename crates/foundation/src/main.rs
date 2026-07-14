//! Foundation engine launcher.
//!
//! Foundation discovers game extensions from game manifests and launches the
//! selected game by name. The engine launcher intentionally does not depend on
//! concrete game crates.

use std::{
    fmt,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

use serde::Deserialize;

fn main() -> ExitCode {
    let interrupt_exit_code = 130;
    let _ = ctrlc::set_handler(move || std::process::exit(interrupt_exit_code));

    let launch_arguments = match FoundationLaunchArguments::parse(std::env::args().skip(1)) {
        Ok(launch_arguments) => launch_arguments,
        Err(argument_error) => {
            eprintln!("{argument_error}");
            eprintln!("Usage: cargo run -p foundation -- --game template-game [--editor]");
            return ExitCode::FAILURE;
        }
    };

    match launch_selected_game(&launch_arguments) {
        Ok(game_exit_code) => game_exit_code,
        Err(launch_error) => {
            eprintln!("{launch_error}");
            ExitCode::FAILURE
        }
    }
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

        let game = game.unwrap_or_else(|| "template-game".to_string());
        Ok(Self {
            game,
            editor_enabled,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
struct FoundationGameManifest {
    game: FoundationGameManifestGame,
    launch: FoundationGameManifestLaunch,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
struct FoundationGameManifestGame {
    name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
struct FoundationGameManifestLaunch {
    package: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DiscoveredGame {
    manifest_path: PathBuf,
    manifest: FoundationGameManifest,
}

#[derive(Debug)]
enum FoundationLaunchError {
    Io(std::io::Error),
    ManifestParse {
        manifest_path: PathBuf,
        error: toml::de::Error,
    },
    GameNotFound {
        requested_game: String,
        available_games: Vec<String>,
    },
    GameProcessFailed(std::io::Error),
}

impl fmt::Display for FoundationLaunchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "Failed to discover Foundation games: {error}"),
            Self::ManifestParse {
                manifest_path,
                error,
            } => write!(
                formatter,
                "Failed to parse game manifest {}: {error}",
                manifest_path.display()
            ),
            Self::GameNotFound {
                requested_game,
                available_games,
            } => write!(
                formatter,
                "Unknown game `{requested_game}`. Available games: {}",
                available_games.join(", ")
            ),
            Self::GameProcessFailed(error) => {
                write!(formatter, "Failed to launch selected game: {error}")
            }
        }
    }
}

fn launch_selected_game(
    launch_arguments: &FoundationLaunchArguments,
) -> Result<ExitCode, FoundationLaunchError> {
    let workspace_root = workspace_root();
    let discovered_games = discover_games(&workspace_root)?;
    let available_games = discovered_games
        .iter()
        .map(|discovered_game| discovered_game.manifest.game.name.clone())
        .collect::<Vec<_>>();

    let Some(discovered_game) = discovered_games.into_iter().find(|discovered_game| {
        discovered_game
            .manifest
            .game
            .name
            .eq_ignore_ascii_case(&launch_arguments.game)
    }) else {
        return Err(FoundationLaunchError::GameNotFound {
            requested_game: launch_arguments.game.clone(),
            available_games,
        });
    };

    launch_game_process(&discovered_game.manifest, launch_arguments)
}

fn workspace_root() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}

fn discover_games(workspace_root: &Path) -> Result<Vec<DiscoveredGame>, FoundationLaunchError> {
    let games_directory = workspace_root.join("games");
    let mut discovered_games = Vec::new();

    if !games_directory.is_dir() {
        return Ok(discovered_games);
    }

    for game_directory_entry in
        std::fs::read_dir(games_directory).map_err(FoundationLaunchError::Io)?
    {
        let game_directory_entry = game_directory_entry.map_err(FoundationLaunchError::Io)?;
        let manifest_path = game_directory_entry.path().join("foundation.game.toml");
        if !manifest_path.is_file() {
            continue;
        }

        let manifest_text =
            std::fs::read_to_string(&manifest_path).map_err(FoundationLaunchError::Io)?;
        let manifest =
            toml::from_str::<FoundationGameManifest>(&manifest_text).map_err(|error| {
                FoundationLaunchError::ManifestParse {
                    manifest_path: manifest_path.clone(),
                    error,
                }
            })?;

        discovered_games.push(DiscoveredGame {
            manifest_path,
            manifest,
        });
    }

    discovered_games.sort_by(|left_game, right_game| {
        left_game
            .manifest
            .game
            .name
            .cmp(&right_game.manifest.game.name)
    });
    Ok(discovered_games)
}

fn launch_game_process(
    manifest: &FoundationGameManifest,
    launch_arguments: &FoundationLaunchArguments,
) -> Result<ExitCode, FoundationLaunchError> {
    let mut command = Command::new("cargo");
    command.args(["run", "-p", manifest.launch.package.as_str(), "--"]);
    if launch_arguments.editor_enabled {
        command.arg("--editor");
    }

    let status = command
        .status()
        .map_err(FoundationLaunchError::GameProcessFailed)?;
    Ok(ExitCode::from(status.code().unwrap_or(1) as u8))
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

        assert_eq!(launch_arguments.game, "template-game");
        assert!(!launch_arguments.editor_enabled);
    }

    #[test]
    fn game_manifest_parses_name_and_package() {
        let manifest_text = r#"
            [game]
            name = "template-game"

            [launch]
            package = "template-game"
        "#;
        let manifest =
            toml::from_str::<FoundationGameManifest>(manifest_text).expect("manifest should parse");

        assert_eq!(manifest.game.name, "template-game");
        assert_eq!(manifest.launch.package, "template-game");
    }
}
