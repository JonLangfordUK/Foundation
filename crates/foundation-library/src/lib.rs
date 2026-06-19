//! FoundationLibrary provides reusable Bevy building blocks for Foundation games.
//!
//! The crate is intentionally small at this baseline stage. Jackdaw supplies the
//! editor and scene-authoring layer; FoundationLibrary supplies shared game and
//! editor-compatible code that multiple Jackdaw-style games can compose.
//!
//! Game crates should add [`FoundationPlugin`] before their game-specific plugin
//! so shared resources, systems, and reflected types are available first.

use bevy::prelude::*;

/// Shared baseline plugin for Foundation games.
///
/// Add this plugin to both standalone game binaries and game-specific Jackdaw
/// editor binaries. Future reusable components, resources, and systems should be
/// registered here when they are intended to be available across games.
#[derive(Default)]
pub struct FoundationPlugin;

impl Plugin for FoundationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FoundationSettings>()
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

/// Common imports for games using FoundationLibrary.
pub mod prelude {
    pub use crate::{FoundationPlugin, FoundationSettings};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foundation_plugin_registers_settings_resource() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(FoundationPlugin);

        let settings = app.world().resource::<FoundationSettings>();
        assert_eq!(settings.display_name, "FoundationLibrary");
    }
}
