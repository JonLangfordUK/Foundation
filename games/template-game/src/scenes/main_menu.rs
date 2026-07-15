//! BSN-authored main menu scene for TemplateGame.

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

use crate::scenes::{CREDITS_SCENE, GAMEPLAY_LEVEL_SCENE, OPTIONS_MENU_SCENE};

/// Returns the main menu scene.
pub fn main_menu_scene(scene_owner: SceneOwner) -> impl Scene {
    bsn! {
        #TemplateGameMainMenu
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(16.0),
        }
        BackgroundColor(Color::srgba(0.02, 0.02, 0.04, 0.92))
        template_value(scene_owner)
        Children [
            main_menu_title(scene_owner),
            main_menu_button(
                "New Game",
                FoundationMenuButton::clear_and_open_scene(GAMEPLAY_LEVEL_SCENE, "gameplay"),
                scene_owner,
            ),
            main_menu_button(
                "Options",
                FoundationMenuButton::open_overlay_scene(OPTIONS_MENU_SCENE, "options"),
                scene_owner,
            ),
            main_menu_button(
                "Credits",
                FoundationMenuButton::open_scene(CREDITS_SCENE, "credits"),
                scene_owner,
            ),
            main_menu_button("Exit", FoundationMenuButton::exit(), scene_owner),
        ]
    }
}

fn main_menu_title(scene_owner: SceneOwner) -> impl Scene {
    bsn! {
        Text("Template Game")
        TextFont { font_size: FontSize::Px(64.0) }
        TextColor(Color::WHITE)
        template_value(scene_owner)
    }
}

fn main_menu_button(
    label: &'static str,
    action: FoundationMenuButton,
    scene_owner: SceneOwner,
) -> impl Scene {
    bsn! {
        Button
        Node {
            width: Val::Px(260.0),
            height: Val::Px(56.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        BackgroundColor(Color::srgb(0.15, 0.15, 0.18))
        template_value(action)
        template_value(scene_owner)
        Children [(
            Text(label)
            TextFont { font_size: FontSize::Px(28.0) }
            TextColor(Color::WHITE)
            template_value(scene_owner)
        )]
    }
}
