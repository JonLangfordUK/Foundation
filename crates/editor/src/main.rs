use pigame_engine::{run_launcher, LauncherWindowConfig};
use pigame_game::GAME_NAME;

fn main() {
    run_launcher(editor_window_config());
}

fn editor_window_config() -> LauncherWindowConfig {
    LauncherWindowConfig::new(format!("{GAME_NAME} Editor"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_window_has_expected_title() {
        assert_eq!(editor_window_config().title, "PiGame Editor");
    }
}
