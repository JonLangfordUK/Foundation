//! BSN-authored Pixel Perfect splash scene for TemplateGame.

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

use crate::scenes::BEVY_SPLASH_SCENE;

/// Spawns the Pixel Perfect splash scene and its Foundation transition driver.
pub fn spawn_pixel_perfect_splash_scene(commands: &mut Commands, scene_owner: SceneOwner) {
    let splash_timings = FoundationSplashTimings::new(0.75, 1.0, 0.75);
    let splash_screen = FoundationSplashScreen {
        timings: splash_timings,
        font_size: 72.0,
        next_scene_key: BEVY_SPLASH_SCENE.to_string(),
        reset_stack_for_next_scene: false,
        replace_current_scene: true,
    };

    commands.spawn_scene(pixel_perfect_splash_scene(scene_owner));
    commands.spawn((Name::new("Pixel Perfect"), splash_screen, scene_owner));
}

fn pixel_perfect_splash_scene(scene_owner: SceneOwner) -> impl Scene {
    let transparent_white_text = Color::srgba(1.0, 1.0, 1.0, 0.0);
    bsn! {
        #TemplateGamePixelPerfectSplash
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
            Text("Pixel Perfect")
            TextFont { font_size: FontSize::Px(72.0) }
            TextColor(transparent_white_text)
            FoundationSplashText
            template_value(scene_owner)
        )]
    }
}
