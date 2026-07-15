//! BSN-authored gameplay scenes for TemplateGame.

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

use crate::scenes::PAUSE_MENU_SCENE;

/// Returns the sample gameplay scene.
pub fn gameplay_level_scene(scene_owner: SceneOwner) -> impl Scene {
    bsn! {
        #GameplayLevel
        FoundationSimpleGameplayLevel { cube_size: 2.0 }
        FoundationPauseOpener {
            pause_scene_key: { PAUSE_MENU_SCENE.to_string() },
            pause_stack_key: { "pause-menu".to_string() },
        }
        template_value(scene_owner)
    }
}
