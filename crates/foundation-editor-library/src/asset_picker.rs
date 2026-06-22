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
        app.add_message::<FoundationAssetPicked>().add_systems(
            Update,
            (
                handle_asset_picker_interactions,
                refresh_asset_picker_previews_after_selection,
                populate_added_asset_picker_previews,
                sync_asset_picker_previews_from_value_labels,
            )
                .chain(),
        );
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

const ASSET_PICKER_FIELD_FONT_SIZE: f32 = 10.0;
const ASSET_PICKER_PREVIEW_FONT_SIZE: f32 = 9.0;

#[derive(Clone, Debug, Component)]
struct FoundationAssetPickerPreview {
    picker_id: String,
    asset_path: Option<String>,
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
                TextFont::from_font_size(ASSET_PICKER_FIELD_FONT_SIZE),
                Node {
                    width: px(128.0),
                    ..default()
                },
            ),
            (
                FoundationAssetPickerPreview {
                    picker_id: props.picker_id.clone(),
                    asset_path: props.selected_asset_path.clone(),
                },
                Node {
                    width: px(48.0),
                    height: px(48.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::clip(),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.03, 0.03, 0.035)),
                children![(
                    Text::new(preview_text),
                    TextFont::from_font_size(ASSET_PICKER_PREVIEW_FONT_SIZE),
                ),],
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
                                TextFont::from_font_size(ASSET_PICKER_FIELD_FONT_SIZE),
                            ),
                            (
                                Text::new("..."),
                                TextFont::from_font_size(ASSET_PICKER_FIELD_FONT_SIZE),
                            ),
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
                                children![(
                                    Text::new("Clear"),
                                    TextFont::from_font_size(ASSET_PICKER_FIELD_FONT_SIZE),
                                ),],
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
                                children![(
                                    Text::new("Use Open"),
                                    TextFont::from_font_size(ASSET_PICKER_FIELD_FONT_SIZE),
                                ),],
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

fn refresh_asset_picker_previews_after_selection(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut picked: MessageReader<FoundationAssetPicked>,
    mut previews: Query<(Entity, &mut FoundationAssetPickerPreview, Option<&Children>)>,
) {
    for picked_asset in picked.read() {
        for (preview_entity, mut preview, preview_children) in &mut previews {
            if preview.picker_id != picked_asset.picker_id {
                continue;
            }

            preview.asset_path.clone_from(&picked_asset.asset_path);
            rebuild_asset_picker_preview(
                &mut commands,
                &asset_server,
                preview_entity,
                preview_children,
                preview.asset_path.as_deref(),
            );
        }
    }
}

fn populate_added_asset_picker_previews(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    previews: Query<
        (Entity, &FoundationAssetPickerPreview, Option<&Children>),
        Added<FoundationAssetPickerPreview>,
    >,
) {
    for (preview_entity, preview, preview_children) in &previews {
        rebuild_asset_picker_preview(
            &mut commands,
            &asset_server,
            preview_entity,
            preview_children,
            preview.asset_path.as_deref(),
        );
    }
}

fn sync_asset_picker_previews_from_value_labels(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    value_labels: Query<(&FoundationAssetPickerValueLabel, &Text), Changed<Text>>,
    mut previews: Query<(Entity, &mut FoundationAssetPickerPreview, Option<&Children>)>,
) {
    for (value_label, value_text) in &value_labels {
        let selected_asset_path = selected_asset_path_from_label(value_text);
        for (preview_entity, mut preview, preview_children) in &mut previews {
            if preview.picker_id != value_label.picker_id {
                continue;
            }
            if preview.asset_path == selected_asset_path {
                continue;
            }

            // Settings windows can refresh labels from persisted settings after the
            // picker row is spawned. Mirror that value so thumbnails are ready as
            // soon as a window opens, not only after the user changes selection.
            preview.asset_path.clone_from(&selected_asset_path);
            rebuild_asset_picker_preview(
                &mut commands,
                &asset_server,
                preview_entity,
                preview_children,
                preview.asset_path.as_deref(),
            );
        }
    }
}

fn selected_asset_path_from_label(value_text: &Text) -> Option<String> {
    let selected_asset_path = value_text.as_str();
    if selected_asset_path == "None" {
        return None;
    }

    non_empty_string(selected_asset_path)
}

fn rebuild_asset_picker_preview(
    commands: &mut Commands,
    asset_server: &AssetServer,
    preview_entity: Entity,
    preview_children: Option<&Children>,
    asset_path: Option<&str>,
) {
    if let Some(preview_children) = preview_children {
        for child_entity in preview_children.iter() {
            commands.entity(child_entity).despawn();
        }
    }

    match preview_content_for_asset_path(asset_path) {
        FoundationAssetPickerPreviewContent::Image(thumbnail_asset_path) => {
            let thumbnail_handle = asset_server.load(thumbnail_asset_path);
            commands.spawn((
                ImageNode::new(thumbnail_handle),
                Node {
                    width: percent(100.0),
                    height: percent(100.0),
                    ..default()
                },
                ChildOf(preview_entity),
            ));
        }
        FoundationAssetPickerPreviewContent::Text(preview_text) => {
            commands.spawn((
                Text::new(preview_text),
                TextFont::from_font_size(ASSET_PICKER_PREVIEW_FONT_SIZE),
                ChildOf(preview_entity),
            ));
        }
    }
}

fn preview_content_for_asset_path(asset_path: Option<&str>) -> FoundationAssetPickerPreviewContent {
    let Some(asset_path) = asset_path.and_then(non_empty_string) else {
        return FoundationAssetPickerPreviewContent::Text("None".to_string());
    };

    if is_image_asset_path(&asset_path) {
        return FoundationAssetPickerPreviewContent::Image(asset_path);
    }

    if is_jackdaw_scene_asset_path(&asset_path) {
        if let Some(thumbnail_asset_path) = jackdaw_scene_thumbnail_asset_path(&asset_path) {
            return FoundationAssetPickerPreviewContent::Image(thumbnail_asset_path);
        }
    }

    FoundationAssetPickerPreviewContent::Text(preview_badge_for_asset_path(&asset_path))
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum FoundationAssetPickerPreviewContent {
    Image(String),
    Text(String),
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

fn preview_badge_for_asset_path(asset_path: &str) -> String {
    let Some(asset_extension) = Path::new(asset_path)
        .extension()
        .and_then(|extension| extension.to_str())
    else {
        return preview_label_for_asset_path(asset_path)
            .unwrap_or("Asset")
            .to_string();
    };

    normalized_extension(asset_extension).to_ascii_uppercase()
}

fn is_image_asset_path(asset_path: &str) -> bool {
    let Some(asset_extension) = Path::new(asset_path)
        .extension()
        .and_then(|extension| extension.to_str())
    else {
        return false;
    };

    matches!(
        normalized_extension(asset_extension).as_str(),
        "png" | "jpg" | "jpeg" | "webp" | "bmp" | "tga" | "ktx2" | "dds"
    )
}

fn is_jackdaw_scene_asset_path(asset_path: &str) -> bool {
    Path::new(asset_path)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| normalized_extension(extension) == "jsn")
}

fn jackdaw_scene_thumbnail_asset_path(scene_asset_path: &str) -> Option<String> {
    let project_root = current_project_root();
    jackdaw_scene_thumbnail_candidates(scene_asset_path)
        .into_iter()
        .find(|thumbnail_asset_path| {
            project_root
                .join("assets")
                .join(thumbnail_asset_path)
                .is_file()
        })
}

fn jackdaw_scene_thumbnail_candidates(scene_asset_path: &str) -> Vec<String> {
    let scene_path = Path::new(scene_asset_path);
    let parent_path = scene_path.parent().unwrap_or_else(|| Path::new(""));
    let scene_file_name = scene_path
        .file_name()
        .and_then(|file_name| file_name.to_str())
        .unwrap_or("scene.jsn");
    let scene_stem = scene_path
        .file_stem()
        .and_then(|file_stem| file_stem.to_str())
        .unwrap_or("scene");

    // Jackdaw can use an exported sidecar thumbnail when the editor or project
    // provides one. Keep the lookup convention broad so generated thumbnails
    // can be dropped next to scenes or under an asset thumbnail directory.
    [
        parent_path.join(format!("{scene_file_name}.png")),
        parent_path.join(format!("{scene_stem}.thumbnail.png")),
        parent_path.join(format!("{scene_stem}.png")),
        Path::new(".thumbnails")
            .join(parent_path)
            .join(format!("{scene_stem}.png")),
        Path::new("thumbnails")
            .join(parent_path)
            .join(format!("{scene_stem}.png")),
    ]
    .into_iter()
    .map(|candidate_path| candidate_path.to_string_lossy().replace('\\', "/"))
    .collect()
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

    #[test]
    fn image_assets_use_their_own_preview_image() {
        assert_eq!(
            preview_content_for_asset_path(Some("textures/button.png")),
            FoundationAssetPickerPreviewContent::Image("textures/button.png".to_string())
        );
    }

    #[test]
    fn jackdaw_scene_thumbnail_candidates_include_sidecar_paths() {
        let candidates = jackdaw_scene_thumbnail_candidates("maps/main_menu.jsn");

        assert_eq!(
            candidates,
            vec![
                "maps/main_menu.jsn.png".to_string(),
                "maps/main_menu.thumbnail.png".to_string(),
                "maps/main_menu.png".to_string(),
                ".thumbnails/maps/main_menu.png".to_string(),
                "thumbnails/maps/main_menu.png".to_string(),
            ]
        );
    }

    #[test]
    fn unknown_assets_fall_back_to_extension_badges() {
        assert_eq!(
            preview_content_for_asset_path(Some("maps/main_menu.jsn")),
            FoundationAssetPickerPreviewContent::Text("JSN".to_string())
        );
        assert_eq!(
            preview_content_for_asset_path(Some("audio/music.ogg")),
            FoundationAssetPickerPreviewContent::Text("OGG".to_string())
        );
    }
}
