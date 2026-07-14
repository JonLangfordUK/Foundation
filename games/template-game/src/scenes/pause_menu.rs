//! BSN-authored pause menu scene for TemplateGame.

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

use crate::scenes::{MAIN_MENU_SCENE, OPTIONS_MENU_SCENE};

/// Returns the pause menu scene.
pub fn pause_menu_scene(scene_owner: SceneOwner) -> impl Scene {
    bsn! {
        #TemplateGamePauseMenu
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
            pause_menu_title(scene_owner),
            pause_menu_button("Resume", FoundationMenuButton::resume(), scene_owner),
            pause_menu_button(
                "Options",
                FoundationMenuButton::open_overlay_scene(OPTIONS_MENU_SCENE, "options"),
                scene_owner,
            ),
            pause_menu_button(
                "Main Menu",
                FoundationMenuButton::clear_and_open_scene(MAIN_MENU_SCENE, "main-menu"),
                scene_owner,
            ),
            (
                #PauseEscapeHandler
                FoundationCloseOnEscape
                FoundationResumeOnEscape
                template_value(scene_owner)
            ),
        ]
    }
}

fn pause_menu_title(scene_owner: SceneOwner) -> impl Scene {
    bsn! {
        Text("Paused")
        TextFont { font_size: FontSize::Px(64.0) }
        TextColor(Color::WHITE)
        template_value(scene_owner)
    }
}

fn pause_menu_button(
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
