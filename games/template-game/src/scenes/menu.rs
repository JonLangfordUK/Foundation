//! BSN-authored menu scenes for PiGame.

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

use crate::scenes::{GAMEPLAY_LEVEL_SCENE, MAIN_MENU_SCENE, OPTIONS_MENU_SCENE};

/// Returns the main menu scene.
pub fn main_menu_scene(scene_owner: SceneOwner) -> impl Scene {
    bsn! {
        #PiGameMenu
        menu_root_style(scene_owner)
        Children [
            title_text("PiGame", scene_owner),
            menu_button(
                "New Game",
                FoundationMenuButton::clear_and_open_scene(GAMEPLAY_LEVEL_SCENE, "gameplay"),
                scene_owner,
            ),
            menu_button(
                "Options",
                FoundationMenuButton::open_overlay_scene(OPTIONS_MENU_SCENE, "options"),
                scene_owner,
            ),
            menu_button("Exit", FoundationMenuButton::exit(), scene_owner),
        ]
    }
}

/// Returns the options menu marker scene.
pub fn options_menu_scene(scene_owner: SceneOwner) -> impl Scene {
    bsn! {
        #OptionsMenu
        FoundationOptionsMenu { title: { "Options".to_string() } }
        FoundationCloseOnEscape
        template_value(scene_owner)
    }
}

/// Returns the pause menu scene.
pub fn pause_menu_scene(scene_owner: SceneOwner) -> impl Scene {
    bsn! {
        #PauseMenu
        menu_root_style(scene_owner)
        Children [
            title_text("Paused", scene_owner),
            menu_button("Resume", FoundationMenuButton::resume(), scene_owner),
            menu_button(
                "Options",
                FoundationMenuButton::open_overlay_scene(OPTIONS_MENU_SCENE, "options"),
                scene_owner,
            ),
            menu_button(
                "Main Menu",
                FoundationMenuButton::clear_and_open_scene(MAIN_MENU_SCENE, "main-menu"),
                scene_owner,
            ),
            (
                #PauseEscapeHandler
                FoundationCloseOnEscape
                template_value(scene_owner)
            ),
        ]
    }
}

fn menu_root_style(scene_owner: SceneOwner) -> impl Scene {
    bsn! {
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
    }
}

fn title_text(title: &'static str, scene_owner: SceneOwner) -> impl Scene {
    bsn! {
        Text(title)
        TextFont { font_size: FontSize::Px(64.0) }
        TextColor(Color::WHITE)
        template_value(scene_owner)
    }
}

fn menu_button(
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
