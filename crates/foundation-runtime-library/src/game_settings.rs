//! Project-level settings shared by Foundation runtime and editor integrations.
//!
//! The runtime crate owns the plain data and persistence helpers so standalone
//! games can read settings without depending on Jackdaw editor APIs.

use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Default Foundation settings file name under a game project root.
pub const FOUNDATION_GAME_SETTINGS_FILE_NAME: &str = "foundation.settings.toml";

/// Shared project settings for Foundation-based games.
///
/// Empty map paths mean the game should use its own built-in fallback. This
/// keeps the settings generic and avoids putting TemplateGame-specific scene
/// names inside the reusable runtime library.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Reflect, Resource, Serialize)]
#[reflect(Resource)]
pub struct FoundationGameSettings {
    /// Startup map used by standalone game runs when set.
    pub startup_map: String,
    /// Startup map preferred by editor Play/default editor workflows when set.
    pub editor_startup_map: String,
}

impl FoundationGameSettings {
    /// Returns the configured standalone startup map when one has been set.
    #[must_use]
    pub fn startup_map_path(&self) -> Option<&str> {
        non_empty_setting(&self.startup_map)
    }

    /// Returns the configured editor startup map when one has been set.
    #[must_use]
    pub fn editor_startup_map_path(&self) -> Option<&str> {
        non_empty_setting(&self.editor_startup_map)
    }

    /// Loads settings from the default file under `project_root`.
    pub fn load_from_project_root(
        project_root: impl AsRef<Path>,
    ) -> Result<Self, FoundationGameSettingsIoError> {
        let settings_file_path = project_root
            .as_ref()
            .join(FOUNDATION_GAME_SETTINGS_FILE_NAME);
        Self::load_from_file(settings_file_path)
    }

    /// Loads settings from a TOML file.
    ///
    /// A missing file is not an error and returns default settings.
    pub fn load_from_file(
        settings_file_path: impl AsRef<Path>,
    ) -> Result<Self, FoundationGameSettingsIoError> {
        let settings_file_path = settings_file_path.as_ref();
        let settings_contents = match fs::read_to_string(settings_file_path) {
            Ok(settings_contents) => settings_contents,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(error) => {
                return Err(FoundationGameSettingsIoError::Read {
                    path: settings_file_path.to_path_buf(),
                    source: error,
                });
            }
        };

        toml::from_str(&settings_contents).map_err(|source| FoundationGameSettingsIoError::Parse {
            path: settings_file_path.to_path_buf(),
            source,
        })
    }

    /// Saves settings to the default file under `project_root`.
    pub fn save_to_project_root(
        &self,
        project_root: impl AsRef<Path>,
    ) -> Result<(), FoundationGameSettingsIoError> {
        let settings_file_path = project_root
            .as_ref()
            .join(FOUNDATION_GAME_SETTINGS_FILE_NAME);
        self.save_to_file(settings_file_path)
    }

    /// Saves settings as a pretty TOML file.
    pub fn save_to_file(
        &self,
        settings_file_path: impl AsRef<Path>,
    ) -> Result<(), FoundationGameSettingsIoError> {
        let settings_file_path = settings_file_path.as_ref();
        let settings_contents = toml::to_string_pretty(self).map_err(|source| {
            FoundationGameSettingsIoError::Serialize {
                path: settings_file_path.to_path_buf(),
                source,
            }
        })?;

        if let Some(settings_directory_path) = settings_file_path.parent() {
            fs::create_dir_all(settings_directory_path).map_err(|source| {
                FoundationGameSettingsIoError::CreateDirectory {
                    path: settings_directory_path.to_path_buf(),
                    source,
                }
            })?;
        }

        fs::write(settings_file_path, settings_contents).map_err(|source| {
            FoundationGameSettingsIoError::Write {
                path: settings_file_path.to_path_buf(),
                source,
            }
        })
    }
}

/// Error produced while reading or writing Foundation game settings.
#[derive(Debug)]
pub enum FoundationGameSettingsIoError {
    /// The settings file could not be read.
    Read { path: PathBuf, source: io::Error },
    /// The settings file could not be parsed as TOML.
    Parse {
        path: PathBuf,
        source: toml::de::Error,
    },
    /// The settings value could not be serialized as TOML.
    Serialize {
        path: PathBuf,
        source: toml::ser::Error,
    },
    /// The settings directory could not be created.
    CreateDirectory { path: PathBuf, source: io::Error },
    /// The settings file could not be written.
    Write { path: PathBuf, source: io::Error },
}

impl fmt::Display for FoundationGameSettingsIoError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => {
                write!(formatter, "failed to read {}: {source}", path.display())
            }
            Self::Parse { path, source } => {
                write!(formatter, "failed to parse {}: {source}", path.display())
            }
            Self::Serialize { path, source } => {
                write!(
                    formatter,
                    "failed to serialize {}: {source}",
                    path.display()
                )
            }
            Self::CreateDirectory { path, source } => {
                write!(formatter, "failed to create {}: {source}", path.display())
            }
            Self::Write { path, source } => {
                write!(formatter, "failed to write {}: {source}", path.display())
            }
        }
    }
}

impl std::error::Error for FoundationGameSettingsIoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Read { source, .. }
            | Self::CreateDirectory { source, .. }
            | Self::Write { source, .. } => Some(source),
            Self::Parse { source, .. } => Some(source),
            Self::Serialize { source, .. } => Some(source),
        }
    }
}

fn non_empty_setting(setting_value: &str) -> Option<&str> {
    let trimmed_setting_value = setting_value.trim();
    if trimmed_setting_value.is_empty() {
        return None;
    }

    Some(trimmed_setting_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_game_settings_have_no_configured_maps() {
        let settings = FoundationGameSettings::default();

        assert_eq!(settings.startup_map_path(), None);
        assert_eq!(settings.editor_startup_map_path(), None);
    }

    #[test]
    fn blank_map_values_are_treated_as_missing() {
        let settings = FoundationGameSettings {
            startup_map: "  ".to_string(),
            editor_startup_map: "main_menu.jsn".to_string(),
        };

        assert_eq!(settings.startup_map_path(), None);
        assert_eq!(settings.editor_startup_map_path(), Some("main_menu.jsn"));
    }

    #[test]
    fn missing_settings_file_loads_defaults() {
        let settings_directory_path = unique_test_directory_path("missing-settings");
        let settings = FoundationGameSettings::load_from_project_root(&settings_directory_path)
            .expect("missing settings should load defaults");

        assert_eq!(settings, FoundationGameSettings::default());
    }

    #[test]
    fn settings_round_trip_through_toml_file() {
        let settings_directory_path = unique_test_directory_path("round-trip");
        let settings = FoundationGameSettings {
            startup_map: "landing_page.jsn".to_string(),
            editor_startup_map: "main_menu.jsn".to_string(),
        };

        settings
            .save_to_project_root(&settings_directory_path)
            .expect("settings should save");
        let loaded_settings =
            FoundationGameSettings::load_from_project_root(&settings_directory_path)
                .expect("settings should load");

        assert_eq!(loaded_settings, settings);

        let _ = fs::remove_dir_all(settings_directory_path);
    }

    fn unique_test_directory_path(test_name: &str) -> PathBuf {
        let process_id = std::process::id();
        let thread_id = format!("{:?}", std::thread::current().id());
        std::env::temp_dir().join(format!(
            "foundation-game-settings-{test_name}-{process_id}-{thread_id}"
        ))
    }
}
