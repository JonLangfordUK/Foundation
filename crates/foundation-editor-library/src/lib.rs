//! FoundationEditorLibrary contains Bevy-only editor-time extension points.
//!
//! The previous external-editor tools were intentionally removed. This crate stays
//! as the stable home for future Foundation editor features that run inside the
//! Foundation engine's `--editor` mode.

use bevy::prelude::*;

/// Installs Bevy-only Foundation editor-time systems.
///
/// The plugin is intentionally empty for now. Keeping the plugin gives the
/// Foundation engine a stable integration point for future editor tools without
/// pulling in any external editor dependency.
#[derive(Default)]
pub struct FoundationEditorPlugin;

impl Plugin for FoundationEditorPlugin {
    fn build(&self, app: &mut App) {
        // Store editor mode as a resource so games can query it without caring
        // which editor features are currently implemented.
        app.init_resource::<FoundationEditorMode>();
    }
}

/// Runtime flag indicating that the Foundation engine was launched in editor mode.
#[derive(Clone, Copy, Debug, Default, Resource)]
pub struct FoundationEditorMode {
    /// True when the engine was launched with `--editor`.
    pub enabled: bool,
}

/// Common imports for Foundation editor-time integrations.
pub mod prelude {
    pub use crate::{FoundationEditorMode, FoundationEditorPlugin};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_plugin_registers_editor_mode_resource() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(FoundationEditorPlugin);

        let editor_mode = app.world().resource::<FoundationEditorMode>();
        assert!(!editor_mode.enabled);
    }
}
