//! Reusable compact asset picker widgets for Jackdaw project assets.
//!
//! The picker is intentionally data-driven: callers provide a stable picker ID,
//! a current value, and a filter. Selection changes are emitted as
//! [`FoundationAssetPicked`] messages so game/editor tools can decide how to
//! store the chosen asset.

use std::path::{Path, PathBuf};

use bevy::prelude::*;

/// Installs systems and messages used by Foundation asset picker widgets.
#[derive(Default)]
pub struct FoundationAssetPickerPlugin;

impl Plugin for FoundationAssetPickerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<FoundationAssetPicked>()
            .add_systems(Update, handle_asset_picker_interactions);
    }
}

/// Filter used by a Foundation asset picker.
///
/// `allowed_extensions` filters by file extension without the leading dot. The
/// optional `required_text` is a lightweight class/type filter for text assets,
/// such as requiring a `.jsn` file to contain a reflected component path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FoundationAssetPickerFilter {
    /// Human-readable filter name shown in file dialogs.
    pub label: String,
    /// Allowed file extensions without leading dots. Empty means all files.
    pub allowed_extensions: Vec<String>,
    /// Optional text that must appear in the file contents.
    pub required_text: Option<String>,
}

impl FoundationAssetPickerFilter {
    /// Creates a filter that accepts every project asset.
    #[must_use]
    pub fn all_assets() -> Self {
        Self {
            label: "All Assets".to_string(),
            allowed_extensions: Vec::new(),
            required_text: None,
        }
    }

    /// Creates a filter that accepts files with the provided extensions.
    #[must_use]
    pub fn extensions(
        label: impl Into<String>,
        extensions: impl IntoIterator<Item = String>,
    ) -> Self {
        Self {
            label: label.into(),
            allowed_extensions: extensions
                .into_iter()
                .map(|extension| normalized_extension(&extension))
                .filter(|extension| !extension.is_empty())
                .collect(),
            required_text: None,
        }
    }

    /// Creates a filter for Jackdaw `.jsn` scene assets.
    #[must_use]
    pub fn jackdaw_scenes() -> Self {
        Self::extensions("Jackdaw Scenes", ["jsn".to_string()])
    }

    /// Adds a text/class requirement to this filter.
    #[must_use]
    pub fn requiring_text(mut self, required_text: impl Into<String>) -> Self {
        self.required_text = Some(required_text.into());
        self
    }

    /// Returns whether the given file is accepted by this filter.
    #[must_use]
    pub fn accepts_file(&self, file_path: &Path) -> bool {
        if !self.accepts_extension(file_path) {
            return false;
        }

        let Some(required_text) = self.required_text.as_deref() else {
            return true;
        };

        std::fs::read_to_string(file_path)
            .is_ok_and(|file_contents| file_contents.contains(required_text))
    }

    fn accepts_extension(&self, file_path: &Path) -> bool {
        if self.allowed_extensions.is_empty() {
            return true;
        }

        let Some(file_extension) = file_path
            .extension()
            .and_then(|extension| extension.to_str())
        else {
            return false;
        };
        let normalized_file_extension = normalized_extension(file_extension);
        self.allowed_extensions
            .iter()
            .any(|allowed_extension| allowed_extension == &normalized_file_extension)
    }
}

impl Default for FoundationAssetPickerFilter {
    fn default() -> Self {
        Self::all_assets()
    }
}

/// Properties used to spawn a compact asset picker widget.
#[derive(Clone, Debug)]
pub struct FoundationAssetPickerProps {
    /// Stable ID included in [`FoundationAssetPicked`] messages.
    pub picker_id: String,
    /// Label shown to the left of the picker.
    pub label: String,
    /// Currently selected project-relative asset path.
    pub selected_asset_path: Option<String>,
    /// Filter limiting which assets can be selected.
    pub filter: FoundationAssetPickerFilter,
}

impl FoundationAssetPickerProps {
    /// Creates picker properties for the given picker ID and row label.
    #[must_use]
    pub fn new(picker_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            picker_id: picker_id.into(),
            label: label.into(),
            selected_asset_path: None,
            filter: FoundationAssetPickerFilter::default(),
        }
    }

    /// Sets the current selected asset path.
    #[must_use]
    pub fn with_selected_asset_path(mut self, selected_asset_path: Option<String>) -> Self {
        self.selected_asset_path = selected_asset_path;
        self
    }

    /// Sets the picker filter.
    #[must_use]
    pub fn with_filter(mut self, filter: FoundationAssetPickerFilter) -> Self {
        self.filter = filter;
        self
    }
}

/// Message emitted when a picker selects or clears an asset.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub struct FoundationAssetPicked {
    /// Picker ID supplied by [`FoundationAssetPickerProps`].
    pub picker_id: String,
    /// Project-relative asset path, or `None` when the picker was cleared.
    pub asset_path: Option<String>,
}

/// Text label showing a picker's selected asset path.
#[derive(Component)]
pub(crate) struct FoundationAssetPickerValueLabel {
    /// Picker ID this label mirrors.
    pub picker_id: String,
}

#[derive(Clone, Debug, Component)]
struct FoundationAssetPickerButton {
    picker_id: String,
    filter: FoundationAssetPickerFilter,
    action: FoundationAssetPickerButtonAction,
}

#[derive(Clone, Debug)]
enum FoundationAssetPickerButtonAction {
    Browse,
    Clear,
    UseOpenScene,
}

/// Spawns a UE-style compact asset picker row.
///
/// The layout mirrors the common Unreal-style picker shape: a small preview
/// tile, a dark selection field, and compact action buttons for browse/reset.
pub fn spawn_foundation_asset_picker(parent: &mut ChildSpawner, props: FoundationAssetPickerProps) {
    let preview_text = props
        .selected_asset_path
        .as_deref()
        .and_then(preview_label_for_asset_path)
        .unwrap_or("None")
        .to_string();
    let value_text = props
        .selected_asset_path
        .clone()
        .unwrap_or_else(|| "None".to_string());
    let browse_button = FoundationAssetPickerButton {
        picker_id: props.picker_id.clone(),
        filter: props.filter.clone(),
        action: FoundationAssetPickerButtonAction::Browse,
    };
    let clear_button = FoundationAssetPickerButton {
        picker_id: props.picker_id.clone(),
        filter: props.filter.clone(),
        action: FoundationAssetPickerButtonAction::Clear,
    };
    let use_open_scene_button = FoundationAssetPickerButton {
        picker_id: props.picker_id.clone(),
        filter: props.filter,
        action: FoundationAssetPickerButtonAction::UseOpenScene,
    };

    parent.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: px(8.0),
            min_height: px(58.0),
            ..default()
        },
        children![
            (
                Text::new(props.label),
                Node {
                    width: px(128.0),
                    ..default()
                },
            ),
            (
                Node {
                    width: px(48.0),
                    height: px(48.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.03, 0.03, 0.035)),
                children![(Text::new(preview_text), TextFont::from_font_size(9.0))],
            ),
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: px(4.0),
                    flex_grow: 1.0,
                    ..default()
                },
                children![
                    (
                        Button,
                        browse_button,
                        Node {
                            height: px(22.0),
                            width: percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            padding: UiRect::horizontal(px(8.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.01, 0.01, 0.012)),
                        children![
                            (
                                FoundationAssetPickerValueLabel {
                                    picker_id: props.picker_id,
                                },
                                Text::new(value_text),
                            ),
                            (Text::new("..."),),
                        ],
                    ),
                    (
                        Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: px(6.0),
                            ..default()
                        },
                        children![
                            (
                                Button,
                                clear_button,
                                Node {
                                    width: px(52.0),
                                    height: px(18.0),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::NONE),
                                children![(Text::new("Clear"), TextFont::from_font_size(10.0))],
                            ),
                            (
                                Button,
                                use_open_scene_button,
                                Node {
                                    width: px(72.0),
                                    height: px(18.0),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::NONE),
                                children![(Text::new("Use Open"), TextFont::from_font_size(10.0))],
                            ),
                        ],
                    ),
                ],
            ),
        ],
    ));
}

fn handle_asset_picker_interactions(
    buttons: Query<(Entity, &Interaction, &FoundationAssetPickerButton), Changed<Interaction>>,
    scene_file_path: Option<Res<jackdaw::scene_io::SceneFilePath>>,
    mut picked: MessageWriter<FoundationAssetPicked>,
) {
    for (_button_entity, interaction, button) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let selected_asset_path = match button.action {
            FoundationAssetPickerButtonAction::Browse => browse_for_asset(&button.filter),
            FoundationAssetPickerButtonAction::Clear => Some(String::new()),
            FoundationAssetPickerButtonAction::UseOpenScene => {
                let Some(asset_path) = current_open_scene_asset_path(scene_file_path.as_deref())
                else {
                    continue;
                };
                let project_root = current_project_root();
                let file_path = project_root.join("assets").join(&asset_path);
                if button.filter.accepts_file(&file_path) {
                    Some(asset_path)
                } else {
                    None
                }
            }
        };

        if let Some(selected_asset_path) = selected_asset_path {
            let asset_path = non_empty_string(&selected_asset_path);
            picked.write(FoundationAssetPicked {
                picker_id: button.picker_id.clone(),
                asset_path,
            });
        }
    }
}

fn browse_for_asset(filter: &FoundationAssetPickerFilter) -> Option<String> {
    let project_root = current_project_root();
    let asset_root = project_root.join("assets");
    let mut dialog = rfd::FileDialog::new().set_directory(&asset_root);
    if !filter.allowed_extensions.is_empty() {
        let extensions = filter
            .allowed_extensions
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        dialog = dialog.add_filter(&filter.label, &extensions);
    }

    let selected_file_path = dialog.pick_file()?;
    if !filter.accepts_file(&selected_file_path) {
        return None;
    }

    asset_path_from_file_path(&project_root, &selected_file_path)
}

fn asset_path_from_file_path(project_root: &Path, file_path: &Path) -> Option<String> {
    let asset_root = project_root.join("assets");
    let relative_asset_path = file_path.strip_prefix(asset_root).ok()?;
    non_empty_string(&relative_asset_path.to_string_lossy().replace('\\', "/"))
}

fn current_open_scene_asset_path(
    scene_file_path: Option<&jackdaw::scene_io::SceneFilePath>,
) -> Option<String> {
    let scene_path = scene_file_path?.path.as_deref()?;
    scene_path_to_asset_path(scene_path)
}

fn scene_path_to_asset_path(scene_path: &str) -> Option<String> {
    let normalized_scene_path = scene_path.replace('\\', "/");
    let asset_marker = "/assets/";
    if let Some((_, asset_relative_path)) = normalized_scene_path.rsplit_once(asset_marker) {
        return non_empty_string(asset_relative_path);
    }

    let scene_file_name = Path::new(scene_path).file_name()?.to_string_lossy();
    non_empty_string(scene_file_name.as_ref())
}

fn preview_label_for_asset_path(asset_path: &str) -> Option<&str> {
    Path::new(asset_path)
        .file_stem()
        .and_then(|file_stem| file_stem.to_str())
        .and_then(|file_stem| file_stem.get(..file_stem.len().min(8)))
}

fn normalized_extension(extension: &str) -> String {
    extension
        .trim()
        .trim_start_matches('.')
        .to_ascii_lowercase()
}

fn non_empty_string(value: &str) -> Option<String> {
    let trimmed_value = value.trim();
    if trimmed_value.is_empty() {
        return None;
    }

    Some(trimmed_value.to_string())
}

fn current_project_root() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_filter_accepts_matching_extension() {
        let filter = FoundationAssetPickerFilter::jackdaw_scenes();

        assert!(filter.accepts_extension(Path::new("main_menu.jsn")));
        assert!(!filter.accepts_extension(Path::new("main_menu.png")));
    }

    #[test]
    fn asset_path_from_file_path_requires_project_assets_directory() {
        let project_root = Path::new("C:/Project");

        assert_eq!(
            asset_path_from_file_path(project_root, Path::new("C:/Project/assets/maps/menu.jsn")),
            Some("maps/menu.jsn".to_string())
        );
        assert_eq!(
            asset_path_from_file_path(project_root, Path::new("C:/Other/assets/menu.jsn")),
            None
        );
    }
}
