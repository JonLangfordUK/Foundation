//! Shared Bevy setup for PiGame workspace applications.
//!
//! The game and editor binaries use this crate for their common window and app
//! configuration so the workspace launchers stay linked through shared code.

use bevy::prelude::*;
use bevy::window::WindowPlugin;

/// Default launcher window width in physical pixels.
pub const DEFAULT_WINDOW_WIDTH: f32 = 1280.0;

/// Default launcher window height in physical pixels.
pub const DEFAULT_WINDOW_HEIGHT: f32 = 720.0;

/// Configuration for a Bevy launcher window.
#[derive(Clone, Debug, PartialEq)]
pub struct LauncherWindowConfig {
    /// Title shown in the native window title bar.
    pub title: String,
    /// Width of the primary window.
    pub width: f32,
    /// Height of the primary window.
    pub height: f32,
}

impl LauncherWindowConfig {
    /// Creates a window configuration with the workspace default resolution.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            width: DEFAULT_WINDOW_WIDTH,
            height: DEFAULT_WINDOW_HEIGHT,
        }
    }

    /// Converts this configuration into Bevy's primary [`Window`] component.
    pub fn into_window(self) -> Window {
        Window {
            title: self.title,
            resolution: (self.width, self.height).into(),
            ..default()
        }
    }
}

/// Adds the default Bevy plugins configured with a primary launcher window.
pub fn add_launcher_plugins(app: &mut App, window_config: LauncherWindowConfig) -> &mut App {
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window_config.into_window()),
        ..default()
    }))
}

/// Runs a Bevy application with the shared launcher window configuration.
pub fn run_launcher(window_config: LauncherWindowConfig) {
    let mut app = App::new();
    add_launcher_plugins(&mut app, window_config);
    app.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config_uses_default_resolution() {
        let config = LauncherWindowConfig::new("Test Window");

        assert_eq!(config.title, "Test Window");
        assert_eq!(config.width, DEFAULT_WINDOW_WIDTH);
        assert_eq!(config.height, DEFAULT_WINDOW_HEIGHT);
    }

    #[test]
    fn config_can_override_resolution() {
        let config = LauncherWindowConfig {
            title: "Custom".to_string(),
            width: 800.0,
            height: 600.0,
        };

        assert_eq!(config.width, 800.0);
        assert_eq!(config.height, 600.0);
    }
}
