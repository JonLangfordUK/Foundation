//! FoundationLibrary provides reusable Bevy building blocks for Foundation games.
//!
//! The crate is intentionally small at this baseline stage. Jackdaw supplies the
//! editor and scene-authoring layer; FoundationLibrary supplies shared game and
//! editor-compatible code that multiple Jackdaw-style games can compose.
//!
//! Game crates should add [`FoundationPlugin`] before their game-specific plugin
//! so shared resources, systems, and reflected types are available first.

use bevy::prelude::*;
use jackdaw_runtime::prelude::*;

pub mod menu;
pub mod scene_stack;
pub mod splash_screen;

/// Shared baseline plugin for Foundation games.
///
/// Add this plugin to both standalone game binaries and game-specific Jackdaw
/// editor binaries. Future reusable components, resources, and systems should be
/// registered here when they are intended to be available across games.
#[derive(Default)]
pub struct FoundationPlugin;

impl Plugin for FoundationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            scene_stack::FoundationSceneStackPlugin,
            splash_screen::FoundationSplashScreenPlugin,
            menu::FoundationMenuPlugin,
        ))
        .register_type::<FoundationSettings>()
        .register_type::<FoundationActor>()
        .init_resource::<FoundationSettings>();
    }
}

/// Shared baseline settings resource for Foundation games.
///
/// This currently records the library's display name and gives the baseline
/// plugin a concrete, testable effect. Future shared configuration can grow from
/// this resource or move into more focused resources as needed.
#[derive(Clone, Debug, Reflect, Resource)]
#[reflect(Resource)]
pub struct FoundationSettings {
    /// Human-readable library name for diagnostics and editor display.
    pub display_name: String,
}

impl Default for FoundationSettings {
    fn default() -> Self {
        Self {
            display_name: "FoundationLibrary".to_string(),
        }
    }
}

/// Baseline shared component for Foundation-authored actors.
///
/// This component demonstrates the pattern reusable FoundationLibrary components
/// should follow to be available to both games and Jackdaw editor binaries:
/// derive [`Component`] and [`Reflect`], add Jackdaw editor metadata, and
/// register the type from [`FoundationPlugin`].
#[derive(Clone, Debug, Default, Component, Reflect)]
#[reflect(Component, @EditorCategory::new("Foundation"))]
pub struct FoundationActor {
    /// Optional human-readable actor label for diagnostics and editor display.
    pub label: String,
}

/// Common imports for games using FoundationLibrary.
pub mod prelude {
    pub use crate::menu::{
        foundation_is_not_paused, foundation_is_paused, FoundationCloseOnEscape,
        FoundationExitRequested, FoundationGeneratedMenuUi, FoundationMenuButton,
        FoundationMenuPlugin, FoundationOptionsMenu, FoundationPauseOpener, FoundationPauseState,
        FoundationPlaceholderMenu, FoundationSimpleGameplayLevel,
    };
    pub use crate::scene_stack::{
        FoundationSceneStackPlugin, OpenSceneOptions, SceneAdded, SceneCommand, SceneCommandsExt,
        SceneFocused, SceneId, SceneKey, SceneLoadRequested, SceneOwner, ScenePresentation,
        SceneRemoved, SceneRuntimeFlags, SceneSource, SceneStack, SceneStackEntry, SceneTarget,
        SceneUnfocused,
    };
    pub use crate::splash_screen::{
        FoundationSplashRuntimeSettings, FoundationSplashScreen, FoundationSplashScreenPlugin,
        FoundationSplashText, FoundationSplashTimings, FoundationSplashUiParent,
        FoundationSplashUiRoot, FoundationSplashUiTargetCamera,
    };
    pub use crate::{FoundationActor, FoundationPlugin, FoundationSettings};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foundation_plugin_registers_settings_resource_and_actor_type() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(FoundationPlugin);

        let settings = app.world().resource::<FoundationSettings>();
        assert_eq!(settings.display_name, "FoundationLibrary");

        let registry = app
            .world()
            .resource::<bevy::ecs::reflect::AppTypeRegistry>()
            .read();
        assert!(registry.contains(std::any::TypeId::of::<FoundationActor>()));
    }
}
