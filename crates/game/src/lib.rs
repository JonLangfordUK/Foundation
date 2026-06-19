//! Game launcher configuration for PiGame.

use bevy::prelude::*;
use pigame_engine::{run_launcher, LauncherWindowConfig};

/// Human-readable game name used by the standalone game and editor.
pub const GAME_NAME: &str = "PiGame";

/// Title for the standalone game window.
pub const GAME_WINDOW_TITLE: &str = "PiGame";

/// Minimal game plugin shared by the standalone game and PillarEditor.
///
/// Gameplay systems, resources, and reflected authoring components should be
/// registered here as the project grows. Ambient engine/editor plugins such as
/// `DefaultPlugins`, Jackdaw physics, and enhanced input remain in the host
/// binaries to avoid duplicate plugin registration.
#[derive(Default)]
pub struct PiGamePlugin;

impl Plugin for PiGamePlugin {
    fn build(&self, _app: &mut App) {}
}

/// Builds the game window configuration.
pub fn game_window_config() -> LauncherWindowConfig {
    LauncherWindowConfig::new(GAME_WINDOW_TITLE)
}

/// Runs the standalone game application.
pub fn run_game() {
    run_launcher(game_window_config());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_window_has_expected_title() {
        assert_eq!(game_window_config().title, GAME_WINDOW_TITLE);
    }
}
