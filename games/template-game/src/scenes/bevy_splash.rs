//! BSN-authored Bevy splash scene for TemplateGame.

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

use crate::scenes::MAIN_MENU_SCENE;

/// Spawns the Bevy splash scene and its Foundation transition driver.
pub fn spawn_bevy_splash_scene(commands: &mut Commands, scene_owner: SceneOwner) {
    let splash_timings = FoundationSplashTimings::new(0.75, 1.0, 0.75);
    let splash_screen = FoundationSplashScreen {
        timings: splash_timings,
        font_size: 72.0,
        next_scene_key: MAIN_MENU_SCENE.to_string(),
        reset_stack_for_next_scene: true,
        replace_current_scene: false,
    };

    commands.spawn_scene(bevy_splash_scene(scene_owner));
    commands.spawn((Name::new("Bevy"), splash_screen, scene_owner));
}

fn bevy_splash_scene(scene_owner: SceneOwner) -> impl Scene {
    let transparent_white_text = Color::srgba(1.0, 1.0, 1.0, 0.0);
    bsn! {
        #TemplateGameBevySplash
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        BackgroundColor(Color::BLACK)
        FoundationSplashUiRoot
        template_value(scene_owner)
        Children [(
            Text("Bevy")
            TextFont { font_size: FontSize::Px(72.0) }
            TextColor(transparent_white_text)
            FoundationSplashText
            template_value(scene_owner)
        )]
    }
}
