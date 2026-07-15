//! Game-extension manifest discovery for the Foundation launcher.
//!
//! The manifest layer is intentionally data-only. It knows how to find and parse
//! game extension manifests, but it does not depend on any concrete game crate.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::launch::FoundationLaunchError;

const GAME_MANIFEST_FILE_NAME: &str = "foundation.game.toml";
const GAMES_DIRECTORY_NAME: &str = "games";

/// Parsed game extension manifest.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub(crate) struct FoundationGameManifest {
    /// Game identity section.
    pub(crate) game: FoundationGameManifestGame,
    /// Launch configuration section.
    pub(crate) launch: FoundationGameManifestLaunch,
}

/// Game identity declared by a game extension.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub(crate) struct FoundationGameManifestGame {
    /// Name used by `foundation --game <game-name>`.
    pub(crate) name: String,
}

/// Development launch configuration declared by a game extension.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub(crate) struct FoundationGameManifestLaunch {
    /// Cargo package to run for the current loose-package development mode.
    pub(crate) package: String,
}

/// Game manifest discovered under the workspace `games/` directory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DiscoveredGame {
    /// Filesystem path to the manifest that produced this entry.
    pub(crate) manifest_path: PathBuf,
    /// Parsed manifest contents.
    pub(crate) manifest: FoundationGameManifest,
}

/// Returns the workspace root used for loose game discovery.
///
/// Prefer the process current directory when it is inside this workspace, but
/// fall back to the compile-time crate location so `foundation` can still be
/// launched from a nested directory during development.
pub(crate) fn workspace_root() -> PathBuf {
    let manifest_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fallback_workspace_root = manifest_directory
        .ancestors()
        .nth(2)
        .map(Path::to_path_buf)
        .unwrap_or(manifest_directory);

    std::env::current_dir()
        .ok()
        .and_then(|current_directory| find_workspace_root(&current_directory))
        .unwrap_or(fallback_workspace_root)
}

fn find_workspace_root(start_directory: &Path) -> Option<PathBuf> {
    start_directory
        .ancestors()
        .find(|candidate_directory| {
            candidate_directory.join("Cargo.toml").is_file()
                && candidate_directory.join(GAMES_DIRECTORY_NAME).is_dir()
        })
        .map(Path::to_path_buf)
}

/// Discovers every game manifest in the workspace `games/` directory.
pub(crate) fn discover_games(
    workspace_root: &Path,
) -> Result<Vec<DiscoveredGame>, FoundationLaunchError> {
    let games_directory = workspace_root.join(GAMES_DIRECTORY_NAME);
    let mut discovered_games = Vec::new();

    if !games_directory.is_dir() {
        return Ok(discovered_games);
    }

    for game_directory_entry in
        std::fs::read_dir(games_directory).map_err(FoundationLaunchError::Io)?
    {
        let game_directory_entry = game_directory_entry.map_err(FoundationLaunchError::Io)?;
        let manifest_path = game_directory_entry.path().join(GAME_MANIFEST_FILE_NAME);
        if !manifest_path.is_file() {
            continue;
        }

        let manifest = read_game_manifest(&manifest_path)?;
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

fn read_game_manifest(
    manifest_path: &Path,
) -> Result<FoundationGameManifest, FoundationLaunchError> {
    let manifest_text =
        std::fs::read_to_string(manifest_path).map_err(FoundationLaunchError::Io)?;
    toml::from_str::<FoundationGameManifest>(&manifest_text).map_err(|error| {
        FoundationLaunchError::ManifestParse {
            manifest_path: manifest_path.to_path_buf(),
            error,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_manifest_parses_name_and_package() {
        let manifest_text = r#"
            [game]
            name = "example-game"

            [launch]
            package = "example-game"
        "#;
        let manifest =
            toml::from_str::<FoundationGameManifest>(manifest_text).expect("manifest should parse");

        assert_eq!(manifest.game.name, "example-game");
        assert_eq!(manifest.launch.package, "example-game");
    }

    #[test]
    fn find_workspace_root_walks_up_from_nested_directory() {
        let test_directory_name = format!("foundation-workspace-root-test-{}", std::process::id());
        let test_workspace_root = std::env::temp_dir().join(test_directory_name);
        let nested_directory = test_workspace_root.join("games/example-game/src");
        std::fs::create_dir_all(&nested_directory).expect("nested directory should be created");
        std::fs::write(test_workspace_root.join("Cargo.toml"), "[workspace]\n")
            .expect("workspace manifest should be written");

        let discovered_workspace_root =
            find_workspace_root(&nested_directory).expect("workspace root should be found");

        assert_eq!(discovered_workspace_root, test_workspace_root);
        let _ = std::fs::remove_dir_all(discovered_workspace_root);
    }
}
