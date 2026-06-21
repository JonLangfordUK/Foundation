//! FoundationEditorLibrary provides reusable Jackdaw editor extensions.
//!
//! Runtime/game systems belong in `foundation-runtime-library`; this crate owns
//! editor-shell integrations that depend on the full Jackdaw editor API.

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;
use jackdaw::prelude::*;

/// Unique Jackdaw extension identifier for the Foundation game settings window.
pub const FOUNDATION_GAME_SETTINGS_EXTENSION_ID: &str = "foundation.game_settings";
/// Unique dock-window identifier for the Foundation game settings window.
pub const FOUNDATION_GAME_SETTINGS_WINDOW_ID: &str = "foundation.game_settings.window";

/// Installs reusable Foundation editor systems.
///
/// Add this plugin to game-specific editor binaries alongside Jackdaw's editor
/// plugins. Register [`FoundationGameSettingsExtension`] through Jackdaw's
/// [`ExtensionPlugin`] so the dockable window appears in the editor UI.
#[derive(Default)]
pub struct FoundationEditorPlugin;

impl Plugin for FoundationEditorPlugin {
    fn build(&self, app: &mut App) {
        // Keep the project settings resource populated before editor UI refreshes labels.
        app.add_systems(Startup, load_foundation_game_settings_from_project_root)
            // Project auto-open loads Jackdaw's default `assets/scene.jsn` first.
            // This follow-up replaces it with the configured editor startup map.
            .add_systems(
                OnEnter(jackdaw::AppState::Editor),
                load_editor_startup_scene_from_settings,
            )
            .add_systems(Update, refresh_game_settings_window_labels);
    }
}

/// Jackdaw extension that contributes the Foundation Game Settings window.
#[derive(Default)]
pub struct FoundationGameSettingsExtension;

impl JackdawExtension for FoundationGameSettingsExtension {
    fn id(&self) -> String {
        FOUNDATION_GAME_SETTINGS_EXTENSION_ID.to_string()
    }

    fn label(&self) -> String {
        "Foundation Game Settings".to_string()
    }

    fn description(&self) -> String {
        "Adds a reusable game settings window for Foundation-based games.".to_string()
    }

    fn register(&self, extension_context: &mut ExtensionContext) {
        extension_context
            .init_resource::<FoundationGameSettingsWindowStatus>()
            .register_operator::<SetStartupMapFromOpenSceneOp>()
            .register_operator::<SetEditorStartupMapFromOpenSceneOp>()
            .register_operator::<SaveGameSettingsOp>()
            .register_operator::<ReloadGameSettingsOp>()
            .register_window(
                WindowDescriptor::new(FOUNDATION_GAME_SETTINGS_WINDOW_ID)
                    .with_name("Game Settings")
                    .with_default_area(DefaultArea::RightSidebar)
                    .with_build(spawn_game_settings_window),
            );
    }
}

/// Current status text displayed in the game settings window.
#[derive(Clone, Debug, Resource)]
pub struct FoundationGameSettingsWindowStatus {
    message: String,
}

impl Default for FoundationGameSettingsWindowStatus {
    fn default() -> Self {
        Self {
            message: "Settings are loaded from the current project.".to_string(),
        }
    }
}

#[derive(Component)]
struct StartupMapValueLabel;

#[derive(Component)]
struct EditorStartupMapValueLabel;

#[derive(Component)]
struct GameSettingsStatusLabel;

type EditorStartupMapLabelQuery<'world, 'state> = Query<
    'world,
    'state,
    &'static mut Text,
    (
        With<EditorStartupMapValueLabel>,
        Without<StartupMapValueLabel>,
        Without<GameSettingsStatusLabel>,
    ),
>;

type GameSettingsStatusLabelQuery<'world, 'state> = Query<
    'world,
    'state,
    &'static mut Text,
    (
        With<GameSettingsStatusLabel>,
        Without<StartupMapValueLabel>,
        Without<EditorStartupMapValueLabel>,
    ),
>;

#[operator(
    id = "foundation.game_settings.set_startup_map_from_open_scene",
    label = "Use Open Scene For Startup Map",
    description = "Sets the standalone startup map to the currently open Jackdaw scene."
)]
fn set_startup_map_from_open_scene(
    _: In<OperatorParameters>,
    scene_file_path: Option<Res<jackdaw::scene_io::SceneFilePath>>,
    mut settings: ResMut<FoundationGameSettings>,
    mut status: ResMut<FoundationGameSettingsWindowStatus>,
) -> OperatorResult {
    let Some(open_scene_asset_path) = current_open_scene_asset_path(scene_file_path.as_deref())
    else {
        status.message = "No open .jsn scene is available for the startup map.".to_string();
        return OperatorResult::Finished;
    };

    settings.startup_map = open_scene_asset_path;
    status.message = "Startup map updated from the open scene.".to_string();
    OperatorResult::Finished
}

#[operator(
    id = "foundation.game_settings.set_editor_startup_map_from_open_scene",
    label = "Use Open Scene For Editor Startup Map",
    description = "Sets the editor startup map to the currently open Jackdaw scene."
)]
fn set_editor_startup_map_from_open_scene(
    _: In<OperatorParameters>,
    scene_file_path: Option<Res<jackdaw::scene_io::SceneFilePath>>,
    mut settings: ResMut<FoundationGameSettings>,
    mut status: ResMut<FoundationGameSettingsWindowStatus>,
) -> OperatorResult {
    let Some(open_scene_asset_path) = current_open_scene_asset_path(scene_file_path.as_deref())
    else {
        status.message = "No open .jsn scene is available for the editor startup map.".to_string();
        return OperatorResult::Finished;
    };

    settings.editor_startup_map = open_scene_asset_path;
    status.message = "Editor startup map updated from the open scene.".to_string();
    OperatorResult::Finished
}

#[operator(
    id = "foundation.game_settings.save",
    label = "Save Game Settings",
    description = "Saves Foundation game settings to the current project."
)]
fn save_game_settings(
    _: In<OperatorParameters>,
    settings: Res<FoundationGameSettings>,
    mut status: ResMut<FoundationGameSettingsWindowStatus>,
) -> OperatorResult {
    let project_root = current_project_root();
    match settings.save_to_project_root(&project_root) {
        Ok(()) => {
            status.message = format!(
                "Saved settings to {}.",
                project_root
                    .join(FOUNDATION_GAME_SETTINGS_FILE_NAME)
                    .display()
            );
        }
        Err(error) => {
            status.message = format!("Failed to save settings: {error}");
        }
    }

    OperatorResult::Finished
}

#[operator(
    id = "foundation.game_settings.reload",
    label = "Reload Game Settings",
    description = "Reloads Foundation game settings from the current project."
)]
fn reload_game_settings(
    _: In<OperatorParameters>,
    mut settings: ResMut<FoundationGameSettings>,
    mut status: ResMut<FoundationGameSettingsWindowStatus>,
) -> OperatorResult {
    let project_root = current_project_root();
    match FoundationGameSettings::load_from_project_root(&project_root) {
        Ok(loaded_settings) => {
            *settings = loaded_settings;
            status.message = format!(
                "Reloaded settings from {}.",
                project_root
                    .join(FOUNDATION_GAME_SETTINGS_FILE_NAME)
                    .display()
            );
        }
        Err(error) => {
            status.message = format!("Failed to reload settings: {error}");
        }
    }

    OperatorResult::Finished
}

fn spawn_game_settings_window(window_spawner: &mut ChildSpawner) {
    let row_margin = UiRect::vertical(px(4.0));
    let section_margin = UiRect::all(px(8.0));

    window_spawner.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(8.0),
            margin: section_margin,
            ..default()
        },
        children![
            (Text::new("Foundation Game Settings"),),
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    margin: row_margin,
                    ..default()
                },
                children![
                    (Text::new("Startup map"),),
                    (StartupMapValueLabel, Text::new("<loading>")),
                    button(ButtonProps::from_operator::<SetStartupMapFromOpenSceneOp>()),
                ],
            ),
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    margin: row_margin,
                    ..default()
                },
                children![
                    (Text::new("Editor startup map"),),
                    (EditorStartupMapValueLabel, Text::new("<loading>")),
                    button(ButtonProps::from_operator::<
                        SetEditorStartupMapFromOpenSceneOp,
                    >()),
                ],
            ),
            button(ButtonProps::from_operator::<SaveGameSettingsOp>()),
            button(ButtonProps::from_operator::<ReloadGameSettingsOp>()),
            (GameSettingsStatusLabel, Text::new("<loading>")),
        ],
    ));
}

fn load_foundation_game_settings_from_project_root(
    mut commands: Commands,
    mut status: Option<ResMut<FoundationGameSettingsWindowStatus>>,
) {
    let project_root = current_project_root();
    match FoundationGameSettings::load_from_project_root(&project_root) {
        Ok(settings) => {
            commands.insert_resource(settings);
            if let Some(status) = status.as_deref_mut() {
                status.message = format!(
                    "Loaded settings from {}.",
                    project_root
                        .join(FOUNDATION_GAME_SETTINGS_FILE_NAME)
                        .display()
                );
            }
        }
        Err(error) => {
            if let Some(status) = status.as_deref_mut() {
                status.message = format!("Failed to load settings: {error}");
            }
        }
    }
}

fn load_editor_startup_scene_from_settings(world: &mut World) {
    let project_root = current_project_root();
    let Some(settings) = world.get_resource::<FoundationGameSettings>() else {
        return;
    };
    let Some(editor_startup_scene_path) = editor_startup_scene_file_path(settings, &project_root)
    else {
        return;
    };

    jackdaw::scene_io::load_scene_from_file(world, &editor_startup_scene_path);

    if let Some(mut status) = world.get_resource_mut::<FoundationGameSettingsWindowStatus>() {
        status.message = format!(
            "Loaded editor startup map {}.",
            editor_startup_scene_path.display()
        );
    }
}

fn editor_startup_scene_file_path(
    settings: &FoundationGameSettings,
    project_root: &std::path::Path,
) -> Option<std::path::PathBuf> {
    let configured_scene_path = settings.editor_startup_map_path()?;
    let configured_scene_file_path = std::path::Path::new(configured_scene_path);
    let scene_file_path = if configured_scene_file_path.is_absolute() {
        configured_scene_file_path.to_path_buf()
    } else {
        project_root.join("assets").join(configured_scene_file_path)
    };

    if !scene_file_path.is_file() {
        warn!(
            "Configured editor startup map {} does not exist; keeping Jackdaw's default scene",
            scene_file_path.display()
        );
        return None;
    }

    Some(scene_file_path)
}

fn refresh_game_settings_window_labels(
    settings: Res<FoundationGameSettings>,
    status: Option<Res<FoundationGameSettingsWindowStatus>>,
    mut startup_map_labels: Query<&mut Text, With<StartupMapValueLabel>>,
    mut editor_startup_map_labels: EditorStartupMapLabelQuery,
    mut status_labels: GameSettingsStatusLabelQuery,
) {
    let startup_map_label = settings
        .startup_map_path()
        .unwrap_or("<game default>")
        .to_string();
    let editor_startup_map_label = settings
        .editor_startup_map_path()
        .unwrap_or("<game default>")
        .to_string();
    let status_message = status
        .as_deref()
        .map(|status| status.message.as_str())
        .unwrap_or("Settings window is ready.")
        .to_string();

    for mut startup_map_text in &mut startup_map_labels {
        **startup_map_text = startup_map_label.clone();
    }

    for mut editor_startup_map_text in &mut editor_startup_map_labels {
        **editor_startup_map_text = editor_startup_map_label.clone();
    }

    for mut status_text in &mut status_labels {
        **status_text = status_message.clone();
    }
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

    let scene_file_name = std::path::Path::new(scene_path)
        .file_name()?
        .to_string_lossy();
    non_empty_string(scene_file_name.as_ref())
}

fn non_empty_string(value: &str) -> Option<String> {
    let trimmed_value = value.trim();
    if trimmed_value.is_empty() {
        return None;
    }

    Some(trimmed_value.to_string())
}

fn current_project_root() -> std::path::PathBuf {
    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
}

/// Common imports for game-specific editor binaries using Foundation editor UI.
pub mod prelude {
    pub use crate::{
        FoundationEditorPlugin, FoundationGameSettingsExtension,
        FOUNDATION_GAME_SETTINGS_EXTENSION_ID, FOUNDATION_GAME_SETTINGS_WINDOW_ID,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_metadata_has_stable_ids() {
        let extension = FoundationGameSettingsExtension;

        assert_eq!(extension.id(), FOUNDATION_GAME_SETTINGS_EXTENSION_ID);
        assert_eq!(
            FOUNDATION_GAME_SETTINGS_WINDOW_ID,
            "foundation.game_settings.window"
        );
        assert_eq!(extension.label(), "Foundation Game Settings");
    }

    #[test]
    fn scene_path_to_asset_path_prefers_assets_relative_path() {
        let scene_path = "C:/Project/assets/menus/main_menu.jsn";

        assert_eq!(
            scene_path_to_asset_path(scene_path),
            Some("menus/main_menu.jsn".to_string())
        );
    }

    #[test]
    fn editor_startup_scene_file_path_uses_assets_relative_setting() {
        let project_root = unique_test_directory_path("editor-startup-relative");
        let scene_directory_path = project_root.join("assets").join("menus");
        std::fs::create_dir_all(&scene_directory_path).expect("scene directory should be created");
        let scene_file_path = scene_directory_path.join("main_menu.jsn");
        std::fs::write(&scene_file_path, "{}").expect("scene file should be written");
        let settings = FoundationGameSettings {
            startup_map: String::new(),
            editor_startup_map: "menus/main_menu.jsn".to_string(),
        };

        assert_eq!(
            editor_startup_scene_file_path(&settings, &project_root),
            Some(scene_file_path)
        );

        let _ = std::fs::remove_dir_all(project_root);
    }

    #[test]
    fn editor_startup_scene_file_path_ignores_missing_setting_file() {
        let project_root = unique_test_directory_path("editor-startup-missing");
        let settings = FoundationGameSettings {
            startup_map: String::new(),
            editor_startup_map: "missing.jsn".to_string(),
        };

        assert_eq!(
            editor_startup_scene_file_path(&settings, &project_root),
            None
        );
    }

    #[test]
    fn scene_path_to_asset_path_falls_back_to_file_name() {
        let scene_path = "main_menu.jsn";

        assert_eq!(
            scene_path_to_asset_path(scene_path),
            Some("main_menu.jsn".to_string())
        );
    }

    fn unique_test_directory_path(test_name: &str) -> std::path::PathBuf {
        let process_id = std::process::id();
        let thread_id = format!("{:?}", std::thread::current().id());
        std::env::temp_dir().join(format!(
            "foundation-editor-settings-{test_name}-{process_id}-{thread_id}"
        ))
    }
}
