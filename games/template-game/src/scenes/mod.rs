//! TemplateGame BSN scene catalog.
//!
//! Foundation owns the generic scene stack. TemplateGame owns these concrete scene
//! keys and maps them to BSN scene functions.

mod gameplay;
mod menu;
mod splash;

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

pub use gameplay::gameplay_level_scene;
pub use menu::{main_menu_scene, options_menu_scene, pause_menu_scene};
pub use splash::splash_screen_scene;

/// Scene key for the first startup splash screen.
pub const PIXEL_PERFECT_SPLASH_SCENE: &str = "template-game/splash_pixel_perfect";
/// Scene key for the second startup splash screen.
pub const BEVY_SPLASH_SCENE: &str = "template-game/splash_bevy";
/// Scene key for the example main menu.
pub const MAIN_MENU_SCENE: &str = "template-game/main_menu";
/// Scene key for the stack-based options menu.
pub const OPTIONS_MENU_SCENE: &str = "template-game/options_menu";
/// Scene key for the small sample gameplay level.
pub const GAMEPLAY_LEVEL_SCENE: &str = "template-game/gameplay_level";
/// Scene key for the gameplay pause menu.
pub const PAUSE_MENU_SCENE: &str = "template-game/pause_menu";

/// Opens the first TemplateGame scene-stack entry.
pub fn open_initial_scene(mut scene_commands: MessageWriter<SceneCommand>) {
    let startup_scene_source = SceneSource::bsn_scene(PIXEL_PERFECT_SPLASH_SCENE);
    let startup_scene_options = OpenSceneOptions::default()
        .with_key("startup-splash")
        .with_presentation(ScenePresentation::FULLSCREEN);

    scene_commands.write(SceneCommand::Clear);
    scene_commands.write(SceneCommand::open_with_options(
        startup_scene_source,
        startup_scene_options,
    ));
}

/// Spawns requested TemplateGame scenes from Foundation scene-load messages.
pub fn spawn_requested_template_game_scenes(
    mut commands: Commands,
    mut scene_requests: MessageReader<SceneLoadRequested>,
) {
    for scene_request in scene_requests.read() {
        let scene_owner = SceneOwner {
            scene_id: scene_request.scene_id,
        };
        let scene_key = scene_source_key(&scene_request.source);

        match scene_key.as_deref() {
            Some(PIXEL_PERFECT_SPLASH_SCENE) => spawn_splash_scene(
                &mut commands,
                scene_owner,
                "Pixel Perfect",
                BEVY_SPLASH_SCENE,
                false,
            ),
            Some(BEVY_SPLASH_SCENE) => {
                spawn_splash_scene(&mut commands, scene_owner, "Bevy", MAIN_MENU_SCENE, true)
            }
            Some(MAIN_MENU_SCENE) => {
                commands.spawn_scene(main_menu_scene(scene_owner));
            }
            Some(OPTIONS_MENU_SCENE) => {
                commands.spawn_scene(options_menu_scene(scene_owner));
            }
            Some(GAMEPLAY_LEVEL_SCENE) => {
                commands.spawn_scene(gameplay_level_scene(scene_owner));
            }
            Some(PAUSE_MENU_SCENE) => {
                commands.spawn_scene(pause_menu_scene(scene_owner));
            }
            Some(unknown_scene_key) => {
                warn!("Unknown TemplateGame scene requested: {unknown_scene_key}");
            }
            None => {
                warn!("TemplateGame received a scene source without a scene key");
            }
        }
    }
}

fn scene_source_key(scene_source: &SceneSource) -> Option<String> {
    match scene_source {
        SceneSource::BsnScene { path } => Some(path.clone()),
        SceneSource::Runtime { key } => Some(key.0.clone()),
    }
}

fn spawn_splash_scene(
    commands: &mut Commands,
    scene_owner: SceneOwner,
    splash_text: &'static str,
    next_scene_key: &'static str,
    reset_stack_for_next_scene: bool,
) {
    let splash_timings = FoundationSplashTimings::new(0.75, 1.0, 0.75);
    let splash_screen = FoundationSplashScreen {
        timings: splash_timings,
        font_size: 72.0,
        next_scene_path: next_scene_key.to_string(),
        reset_stack_for_next_scene,
        replace_current_scene: !reset_stack_for_next_scene,
    };

    commands.spawn_scene(splash_screen_scene(scene_owner, splash_text));
    commands.spawn((Name::new(splash_text), splash_screen, scene_owner));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_scene_keys_are_stable() {
        assert_eq!(
            PIXEL_PERFECT_SPLASH_SCENE,
            "template-game/splash_pixel_perfect"
        );
        assert_eq!(BEVY_SPLASH_SCENE, "template-game/splash_bevy");
        assert_eq!(MAIN_MENU_SCENE, "template-game/main_menu");
        assert_eq!(OPTIONS_MENU_SCENE, "template-game/options_menu");
        assert_eq!(GAMEPLAY_LEVEL_SCENE, "template-game/gameplay_level");
        assert_eq!(PAUSE_MENU_SCENE, "template-game/pause_menu");
    }
}
