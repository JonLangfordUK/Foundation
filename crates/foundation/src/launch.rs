//! Launch argument parsing and selected-game process spawning.
//!
//! This module owns the command-line contract for the Foundation executable. It
//! selects a discovered manifest by game name and forwards runtime flags to the
//! selected game package for the current loose-package development mode.

use std::{
    fmt,
    path::PathBuf,
    process::{Command, ExitCode},
};

use crate::manifest::{discover_games, workspace_root, FoundationGameManifest};

/// Parsed Foundation launcher arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FoundationLaunchArguments {
    /// Requested game extension name.
    pub(crate) game: String,
    /// Enables the Foundation editor-time shell in the selected game process.
    pub(crate) editor_enabled: bool,
    /// Requests visible game log output in non-shipping game builds.
    pub(crate) log_window_requested: bool,
    /// Requests visible game logs in the parent terminal instead of a separate log window.
    pub(crate) inline_log_requested: bool,
}

/// Successful parse outcome for the Foundation command line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum FoundationLaunchCommand {
    /// Launch the selected game with the parsed arguments.
    Launch(FoundationLaunchArguments),
    /// Print usage and exit successfully; requested via `--help`/`-h`.
    ShowHelp,
}

impl FoundationLaunchArguments {
    /// Parses command-line arguments after the executable name.
    pub(crate) fn parse(
        arguments: impl IntoIterator<Item = String>,
    ) -> Result<FoundationLaunchCommand, String> {
        let mut game = None;
        let mut editor_enabled = false;
        let mut log_window_requested = false;
        let mut inline_log_requested = false;
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
                "--log" => {
                    log_window_requested = true;
                }
                "--log-inline" => {
                    inline_log_requested = true;
                }
                "--help" | "-h" => {
                    return Ok(FoundationLaunchCommand::ShowHelp);
                }
                unknown_argument => {
                    return Err(format!("Unknown Foundation argument `{unknown_argument}`."));
                }
            }
        }

        let Some(game) = game else {
            return Err("Expected `--game <game-name>`.".to_string());
        };
        Ok(FoundationLaunchCommand::Launch(Self {
            game,
            editor_enabled,
            log_window_requested,
            inline_log_requested,
        }))
    }
}

/// Error raised while discovering or launching a selected game.
#[derive(Debug)]
pub(crate) enum FoundationLaunchError {
    /// Filesystem error while discovering or reading manifests.
    Io(std::io::Error),
    /// TOML parse error for a discovered manifest.
    ManifestParse {
        /// Path to the manifest that failed to parse.
        manifest_path: PathBuf,
        /// TOML parse error.
        error: toml::de::Error,
    },
    /// The requested game name did not match any discovered manifest.
    GameNotFound {
        /// Requested game name.
        requested_game: String,
        /// Sorted discovered game names.
        available_games: Vec<String>,
    },
    /// Error while spawning or waiting for the selected game process.
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

/// Launches the selected game process for the parsed arguments.
pub(crate) fn launch_selected_game(
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

    launch_game_process(&discovered_game.manifest, launch_arguments, &workspace_root)
}

fn launch_game_process(
    manifest: &FoundationGameManifest,
    launch_arguments: &FoundationLaunchArguments,
    workspace_root: &std::path::Path,
) -> Result<ExitCode, FoundationLaunchError> {
    let mut command = Command::new("cargo");
    command.current_dir(workspace_root);
    command.args(["run", "-p", manifest.launch.package.as_str(), "--"]);
    if launch_arguments.editor_enabled {
        command.arg("--editor");
    }
    if launch_arguments.log_window_requested {
        command.arg("--log");
    }
    if launch_arguments.inline_log_requested {
        command.arg("--log-inline");
    }

    let status = command
        .status()
        .map_err(FoundationLaunchError::GameProcessFailed)?;
    Ok(ExitCode::from(game_exit_code(status.code())))
}

/// Maps a game process exit code onto the launcher's 8-bit exit code.
///
/// Game exit codes are full `i32`s (Windows crash codes are far outside u8
/// range), so any failure code that does not fit is reported as `1` rather than
/// being truncated, which could otherwise turn a crash into apparent success.
fn game_exit_code(game_status_code: Option<i32>) -> u8 {
    match game_status_code {
        Some(0) => 0,
        Some(failure_code) => u8::try_from(failure_code)
            .ok()
            .filter(|code| *code != 0)
            .unwrap_or(1),
        None => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game_editor_and_log_arguments() {
        let arguments = [
            "--game",
            "example-game",
            "--editor",
            "--log",
            "--log-inline",
        ]
        .map(str::to_string);
        let launch_command =
            FoundationLaunchArguments::parse(arguments).expect("arguments should parse");

        assert_eq!(
            launch_command,
            FoundationLaunchCommand::Launch(FoundationLaunchArguments {
                game: "example-game".to_string(),
                editor_enabled: true,
                log_window_requested: true,
                inline_log_requested: true,
            })
        );
    }

    #[test]
    fn help_flag_requests_usage_without_an_error() {
        let arguments = ["--help"].map(str::to_string);
        let launch_command =
            FoundationLaunchArguments::parse(arguments).expect("help should not be an error");

        assert_eq!(launch_command, FoundationLaunchCommand::ShowHelp);
    }

    #[test]
    fn game_exit_codes_map_to_nonzero_launcher_codes_without_truncation() {
        assert_eq!(game_exit_code(Some(0)), 0);
        assert_eq!(game_exit_code(Some(5)), 5);
        // 256 truncates to 0 through `as u8`; a crashed game must never report success.
        assert_eq!(game_exit_code(Some(256)), 1);
        // Windows crash codes such as STATUS_ACCESS_VIOLATION are far outside u8 range.
        assert_eq!(game_exit_code(Some(-1073741819)), 1);
        assert_eq!(game_exit_code(None), 1);
    }

    #[test]
    fn game_argument_is_required() {
        let arguments = std::iter::empty::<String>();
        let launch_error = FoundationLaunchArguments::parse(arguments)
            .expect_err("empty arguments should require a game name");

        assert_eq!(launch_error, "Expected `--game <game-name>`.");
    }
}
