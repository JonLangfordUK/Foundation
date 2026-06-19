//! Game launcher configuration for PiGame.

use pigame_engine::{run_launcher, LauncherWindowConfig};

/// Human-readable game name used by the standalone game and editor.
pub const GAME_NAME: &str = "PiGame";

/// Title for the standalone game window.
pub const GAME_WINDOW_TITLE: &str = "PiGame";

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
