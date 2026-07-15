//! BSN-authored credits scene for TemplateGame.

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

/// Returns the scrolling credits scene.
pub fn credits_scene(scene_owner: SceneOwner) -> impl Scene {
    bsn! {
        #TemplateGameCredits
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            overflow: Overflow::clip(),
        }
        BackgroundColor(Color::BLACK)
        FoundationCloseOnEscape
        template_value(scene_owner)
        Children [
            credits_roll(scene_owner),
        ]
    }
}

fn credits_roll(scene_owner: SceneOwner) -> impl Scene {
    bsn! {
        #TemplateGameCreditsRoll
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(18.0),
        }
        FoundationCreditsRoll {
            credits_path: { "credits.json".to_string() },
            font_path: { "fonts/NotoSans-Regular.ttf".to_string() },
            scroll_speed_pixels_per_second: 45.0,
            start_offset_pixels: 720.0,
            top_level_header_font_size: 48.0,
            header_font_size_step: 6.0,
            minimum_header_font_size: 24.0,
            person_font_size: 18.0,
            indentation_pixels_per_level: 32.0,
            row_gap_pixels: 12.0,
            group_top_margin_pixels: 0.0,
            group_bottom_margin_pixels: 20.0,
        }
        template_value(scene_owner)
    }
}
