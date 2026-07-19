//! Startup scene-stack override helpers for non-shipping Foundation builds.
//!
//! Games can use this module during startup to honor development-time command
//! line scene overrides while keeping their normal default scene flow for
//! shipping and ordinary launches.

use std::fmt;

use crate::scene_stack::SceneCommand;
#[cfg(feature = "dev-tools")]
use crate::scene_stack::{OpenSceneOptions, SceneSource};

/// Runtime argument that requests a non-shipping startup scene override.
pub const FOUNDATION_STARTUP_SCENE_ARGUMENT: &str = "--scene";

/// Error returned when startup scene override arguments are malformed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FoundationStartupSceneOverrideError {
    /// The `--scene` argument was passed more than once.
    DuplicateSceneArgument,
    /// The `--scene` argument did not have a following value.
    MissingSceneValue,
    /// The supplied scene value was empty after trimming whitespace.
    EmptySceneValue,
    /// A bracketed list contained an empty item.
    EmptySceneListItem,
    /// A scene list used only one bracket or had brackets in the wrong location.
    InvalidSceneListSyntax { value: String },
}

impl fmt::Display for FoundationStartupSceneOverrideError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateSceneArgument => write!(
                formatter,
                "Expected at most one `{FOUNDATION_STARTUP_SCENE_ARGUMENT}` argument."
            ),
            Self::MissingSceneValue => write!(
                formatter,
                "Expected a scene key, asset path, or bracketed list after `{FOUNDATION_STARTUP_SCENE_ARGUMENT}`."
            ),
            Self::EmptySceneValue => write!(
                formatter,
                "Expected `{FOUNDATION_STARTUP_SCENE_ARGUMENT}` to contain a non-empty scene key, asset path, or bracketed list."
            ),
            Self::EmptySceneListItem => write!(
                formatter,
                "Startup scene lists cannot contain empty items."
            ),
            Self::InvalidSceneListSyntax { value } => write!(
                formatter,
                "Invalid startup scene list `{value}`. Use a single scene value or a bracketed list such as `[scene_a, scene_b]`."
            ),
        }
    }
}

impl std::error::Error for FoundationStartupSceneOverrideError {}

/// Builds startup scene commands from a non-shipping `--scene` override or defaults.
///
/// The override accepts either a single scene key/path, such as
/// `last-beacon/main_menu`, or a bracketed ordered list, such as
/// `[last-beacon/gameplay_level, scenes/testing_mode.bsn]`. Spaces before and
/// after commas are ignored. The first override scene clears the stack, and
/// later scenes are opened in order above it.
///
/// When the `dev-tools` feature is disabled, this function always returns the
/// provided default commands so shipping-compatible builds do not honor startup
/// scene overrides.
pub fn startup_scene_commands_or_default(
    arguments: impl IntoIterator<Item = String>,
    default_commands: impl IntoIterator<Item = SceneCommand>,
) -> Result<Vec<SceneCommand>, FoundationStartupSceneOverrideError> {
    let default_commands = default_commands.into_iter().collect::<Vec<_>>();

    #[cfg(not(feature = "dev-tools"))]
    {
        let _arguments = arguments;
        return Ok(default_commands);
    }

    #[cfg(feature = "dev-tools")]
    {
        let Some(startup_scene_keys) = parse_startup_scene_override(arguments)? else {
            return Ok(default_commands);
        };

        Ok(startup_scene_override_commands(startup_scene_keys))
    }
}

#[cfg(feature = "dev-tools")]
fn parse_startup_scene_override(
    arguments: impl IntoIterator<Item = String>,
) -> Result<Option<Vec<String>>, FoundationStartupSceneOverrideError> {
    let mut argument_iterator = arguments.into_iter();
    let mut startup_scene_value = None;

    while let Some(argument) = argument_iterator.next() {
        if argument != FOUNDATION_STARTUP_SCENE_ARGUMENT {
            continue;
        }

        if startup_scene_value.is_some() {
            return Err(FoundationStartupSceneOverrideError::DuplicateSceneArgument);
        }

        let Some(scene_value) = argument_iterator.next() else {
            return Err(FoundationStartupSceneOverrideError::MissingSceneValue);
        };
        startup_scene_value = Some(scene_value);
    }

    startup_scene_value
        .map(parse_startup_scene_value)
        .transpose()
}

#[cfg(feature = "dev-tools")]
fn parse_startup_scene_value(
    startup_scene_value: String,
) -> Result<Vec<String>, FoundationStartupSceneOverrideError> {
    let trimmed_scene_value = startup_scene_value.trim();
    if trimmed_scene_value.is_empty() {
        return Err(FoundationStartupSceneOverrideError::EmptySceneValue);
    }

    let has_opening_bracket = trimmed_scene_value.starts_with('[');
    let has_closing_bracket = trimmed_scene_value.ends_with(']');
    if has_opening_bracket || has_closing_bracket {
        return parse_startup_scene_list(trimmed_scene_value);
    }

    Ok(vec![trimmed_scene_value.to_string()])
}

#[cfg(feature = "dev-tools")]
fn parse_startup_scene_list(
    startup_scene_list: &str,
) -> Result<Vec<String>, FoundationStartupSceneOverrideError> {
    if !startup_scene_list.starts_with('[') || !startup_scene_list.ends_with(']') {
        return Err(
            FoundationStartupSceneOverrideError::InvalidSceneListSyntax {
                value: startup_scene_list.to_string(),
            },
        );
    }

    let list_content_end = startup_scene_list.len() - 1;
    let list_content = &startup_scene_list[1..list_content_end];
    if list_content.trim().is_empty() {
        return Err(FoundationStartupSceneOverrideError::EmptySceneValue);
    }

    let mut startup_scene_keys = Vec::new();
    for scene_list_item in list_content.split(',') {
        let startup_scene_key = scene_list_item.trim();
        if startup_scene_key.is_empty() {
            return Err(FoundationStartupSceneOverrideError::EmptySceneListItem);
        }
        startup_scene_keys.push(startup_scene_key.to_string());
    }

    Ok(startup_scene_keys)
}

#[cfg(feature = "dev-tools")]
fn startup_scene_override_commands(startup_scene_keys: Vec<String>) -> Vec<SceneCommand> {
    startup_scene_keys
        .into_iter()
        .enumerate()
        .map(|(startup_scene_position, startup_scene_key)| {
            let startup_scene_source = SceneSource::bsn_scene(startup_scene_key);
            if startup_scene_position == 0 {
                let clear_stack_options = OpenSceneOptions::default().clear_stack();
                SceneCommand::open_with_options(startup_scene_source, clear_stack_options)
            } else {
                SceneCommand::open(startup_scene_source)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene_stack::{OpenSceneOptions, SceneCommand, ScenePresentation, SceneSource};

    #[test]
    fn no_scene_override_returns_default_commands() {
        let default_scene_source = SceneSource::bsn_scene("last-beacon/splash_pixel_perfect");
        let default_command = SceneCommand::clear_and_open(default_scene_source.clone());
        let arguments = ["--editor"].map(str::to_string);

        let startup_commands = startup_scene_commands_or_default(arguments, [default_command])
            .expect("default startup commands should be returned");

        assert_eq!(
            startup_commands,
            vec![SceneCommand::clear_and_open(default_scene_source)]
        );
    }

    #[test]
    #[cfg(feature = "dev-tools")]
    fn single_scene_override_clears_stack_and_opens_scene() {
        let arguments = ["--scene", "last-beacon/main_menu"].map(str::to_string);

        let startup_commands = startup_scene_commands_or_default(arguments, [])
            .expect("single startup scene override should produce scene stack commands");

        let expected_options = OpenSceneOptions::default().clear_stack();
        assert_eq!(
            startup_commands,
            vec![SceneCommand::open_with_options(
                SceneSource::bsn_scene("last-beacon/main_menu"),
                expected_options,
            )]
        );
    }

    #[test]
    #[cfg(feature = "dev-tools")]
    fn scene_list_override_trims_commas_and_preserves_order() {
        let arguments = [
            "--scene",
            "[last-beacon/gameplay_level , scenes/testing_mode.bsn]",
        ]
        .map(str::to_string);

        let startup_commands = startup_scene_commands_or_default(arguments, [])
            .expect("scene list override should produce ordered scene stack commands");

        let expected_options = OpenSceneOptions::default().clear_stack();
        assert_eq!(
            startup_commands,
            vec![
                SceneCommand::open_with_options(
                    SceneSource::bsn_scene("last-beacon/gameplay_level"),
                    expected_options,
                ),
                SceneCommand::open(SceneSource::bsn_scene("scenes/testing_mode.bsn")),
            ]
        );
    }

    #[test]
    #[cfg(feature = "dev-tools")]
    fn duplicate_scene_argument_is_rejected() {
        let arguments = ["--scene", "scene_a", "--scene", "scene_b"].map(str::to_string);

        let startup_error = startup_scene_commands_or_default(arguments, [])
            .expect_err("duplicate startup scene arguments should fail");

        assert_eq!(
            startup_error,
            FoundationStartupSceneOverrideError::DuplicateSceneArgument
        );
    }

    #[test]
    #[cfg(feature = "dev-tools")]
    fn empty_scene_list_item_is_rejected() {
        let arguments = ["--scene", "[scene_a, , scene_b]"].map(str::to_string);

        let startup_error = startup_scene_commands_or_default(arguments, [])
            .expect_err("empty scene list items should fail");

        assert_eq!(
            startup_error,
            FoundationStartupSceneOverrideError::EmptySceneListItem
        );
    }

    #[test]
    #[cfg(feature = "dev-tools")]
    fn invalid_scene_list_brackets_are_rejected() {
        let arguments = ["--scene", "[scene_a, scene_b"].map(str::to_string);

        let startup_error = startup_scene_commands_or_default(arguments, [])
            .expect_err("unbalanced scene list brackets should fail");

        assert_eq!(
            startup_error,
            FoundationStartupSceneOverrideError::InvalidSceneListSyntax {
                value: "[scene_a, scene_b".to_string(),
            }
        );
    }

    #[test]
    fn explicit_default_options_are_preserved_without_overrides() {
        let default_scene_options = OpenSceneOptions::default()
            .with_key("startup-splash")
            .with_presentation(ScenePresentation::FULLSCREEN);
        let default_command = SceneCommand::open_with_options(
            SceneSource::bsn_scene("last-beacon/splash_pixel_perfect"),
            default_scene_options.clone(),
        );

        let startup_commands = startup_scene_commands_or_default([], [default_command])
            .expect("default startup commands should keep caller-provided options");

        assert_eq!(
            startup_commands,
            vec![SceneCommand::open_with_options(
                SceneSource::bsn_scene("last-beacon/splash_pixel_perfect"),
                default_scene_options,
            )]
        );
    }
}
