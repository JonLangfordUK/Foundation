//! BSN-authored options menu scene for TemplateGame.

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

/// Returns the options menu scene.
pub fn options_menu_scene(scene_owner: SceneOwner) -> impl Scene {
    bsn! {
        #TemplateGameOptionsMenu
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(16.0),
        }
        BackgroundColor(Color::srgba(0.02, 0.02, 0.04, 0.92))
        FoundationOptionsMenu { title: { "Options".to_string() } }
        FoundationCloseOnEscape
        template_value(scene_owner)
    }
}
