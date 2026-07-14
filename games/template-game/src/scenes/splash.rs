//! BSN-authored splash scenes for TemplateGame.

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

/// Returns a splash UI scene that Foundation splash systems fade and advance.
pub fn splash_screen_scene(scene_owner: SceneOwner, splash_text: &'static str) -> impl Scene {
    let transparent_white_text = Color::srgba(1.0, 1.0, 1.0, 0.0);
    bsn! {
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
            Text(splash_text)
            TextFont { font_size: FontSize::Px(72.0) }
            TextColor(transparent_white_text)
            FoundationSplashText
            template_value(scene_owner)
        )]
    }
}
