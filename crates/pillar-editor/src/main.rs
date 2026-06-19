use bevy::prelude::*;
use jackdaw::prelude::*;
use pigame_engine::{add_launcher_plugins, LauncherWindowConfig};
use pigame_game::PiGamePlugin;

const PILLAR_EDITOR_TITLE: &str = "PillarEditor";

fn main() -> AppExit {
    let _ = ctrlc::set_handler(|| std::process::exit(130));

    let mut app = App::new();
    add_launcher_plugins(&mut app, editor_window_config());
    app.add_plugins((PhysicsPlugins::default(), EnhancedInputPlugin))
        .add_plugins(EditorPlugins::default())
        .add_plugins(PiGamePlugin)
        .run()
}

fn editor_window_config() -> LauncherWindowConfig {
    LauncherWindowConfig::new(PILLAR_EDITOR_TITLE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_window_has_expected_title() {
        assert_eq!(editor_window_config().title, PILLAR_EDITOR_TITLE);
    }
}
