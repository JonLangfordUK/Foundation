//! Foundation debug console command registry and runtime state.
//!
//! The console command registry is assembled from crates linked into the running
//! game binary. Foundation and the selected game can contribute commands; game
//! crates that are not compiled into the current binary cannot contribute
//! command descriptors.

use std::{collections::BTreeMap, fmt, path::PathBuf};

use crate::scene_stack::{
    OpenSceneOptions, SceneAdded, SceneCommand, SceneId, SceneKey, SceneLoadRequested, SceneOwner,
    ScenePresentation, SceneRemoved, SceneSource, SceneTarget,
};
use bevy::{
    input::keyboard::Key,
    prelude::*,
    text::{EditableText, TextCursorStyle, TextEdit, TextLayout},
};
use bevy_feathers::FeathersPlugins;
use bevy_input_focus::{
    tab_navigation::{TabGroup, TabIndex},
    AutoFocus, FocusCause, InputFocus,
};
use linkme::distributed_slice;
use serde::{Deserialize, Serialize};

#[doc(hidden)]
pub mod __private {
    pub use linkme;
}

/// Directory used for persistent console files relative to the game process.
pub const FOUNDATION_CONSOLE_SAVE_DIRECTORY: &str = "saved/console";

/// File name used for persisted command history.
pub const FOUNDATION_CONSOLE_HISTORY_FILE_NAME: &str = "history.json";

/// Scene-stack key used by the debug console overlay scene.
pub const FOUNDATION_CONSOLE_SCENE_KEY: &str = "foundation/debug-console";

/// All console commands linked into the current game binary.
#[distributed_slice]
pub static FOUNDATION_CONSOLE_COMMANDS: [ConsoleCommandDescriptor] = [..];

/// Plugin that installs Foundation debug console resources and systems.
#[derive(Default)]
pub struct FoundationConsolePlugin;

impl Plugin for FoundationConsolePlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<bevy_feathers::FeathersCorePlugin>() {
            app.add_plugins(FeathersPlugins);
        }

        app.init_resource::<FoundationConsoleState>()
            .init_resource::<FoundationConsoleHistory>()
            .init_resource::<FoundationConsoleRegistry>()
            .init_resource::<FoundationConsoleUiState>()
            .register_type::<FoundationConsoleRoot>()
            .register_type::<FoundationConsoleInput>()
            .register_type::<FoundationConsoleOutput>()
            .register_type::<FoundationConsoleSuggestion>()
            .add_systems(
                Update,
                (
                    toggle_console_scene,
                    spawn_console_scene_from_stack_request,
                    track_console_scene_added,
                    track_console_scene_removed,
                    update_console_input_state,
                    handle_console_keyboard_actions,
                    refresh_console_text_nodes,
                ),
            );
    }
}

/// Runtime open/closed state for the Foundation debug console.
#[derive(Clone, Debug, Default, Resource)]
pub struct FoundationConsoleState {
    /// Whether the debug console scene is currently open.
    pub is_open: bool,
    /// Current scene-stack entry that owns the console UI, when open.
    pub scene_id: Option<SceneId>,
}

impl FoundationConsoleState {
    fn mark_open(&mut self, scene_id: SceneId) {
        self.is_open = true;
        self.scene_id = Some(scene_id);
    }

    fn mark_closed_if_scene_matches(&mut self, removed_scene_id: SceneId) -> bool {
        if self.scene_id != Some(removed_scene_id) {
            return false;
        }

        self.is_open = false;
        self.scene_id = None;
        true
    }
}

/// Runtime UI state for the Foundation debug console.
#[derive(Clone, Debug, Default, Resource)]
pub struct FoundationConsoleUiState {
    /// Current editable command line.
    pub input: String,
    /// Console output and status lines displayed above the input.
    pub output_lines: Vec<String>,
    /// Current history cursor used by Up/Down navigation.
    pub history_cursor: Option<usize>,
}

/// Root component for the generated Foundation debug console UI.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct FoundationConsoleRoot;

/// Marker component for the editable console input entity.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct FoundationConsoleInput;

/// Marker component for the console history and output text entity.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct FoundationConsoleOutput;

/// Marker component for the console autocomplete and placeholder text entity.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct FoundationConsoleSuggestion;

/// Persisted command history for the Foundation debug console.
#[derive(Clone, Debug, Default, Resource, Serialize, Deserialize)]
pub struct FoundationConsoleHistory {
    /// Commands executed by the user, oldest first.
    pub commands: Vec<String>,
}

impl FoundationConsoleHistory {
    /// Returns the path used for persisted command history.
    pub fn history_file_path() -> PathBuf {
        PathBuf::from(FOUNDATION_CONSOLE_SAVE_DIRECTORY).join(FOUNDATION_CONSOLE_HISTORY_FILE_NAME)
    }

    /// Adds a non-empty command line to history.
    pub fn push_command(&mut self, command_line: impl Into<String>) {
        let command_line = command_line.into();
        if !command_line.trim().is_empty() {
            self.commands.push(command_line);
        }
    }
}

fn toggle_console_scene(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    console_state: Res<FoundationConsoleState>,
    mut scene_commands: MessageWriter<SceneCommand>,
) {
    if !keyboard_input.just_pressed(KeyCode::Backquote) {
        return;
    }

    let console_scene_key = SceneKey::new(FOUNDATION_CONSOLE_SCENE_KEY);
    if console_state.is_open {
        scene_commands.write(SceneCommand::Close(SceneTarget::Key(console_scene_key)));
    } else {
        let console_scene_source = SceneSource::runtime(console_scene_key.clone());
        let console_scene_options = OpenSceneOptions::default()
            .with_key(console_scene_key)
            .with_presentation(ScenePresentation::INPUT_BLOCKING_OVERLAY);
        scene_commands.write(SceneCommand::open_with_options(
            console_scene_source,
            console_scene_options,
        ));
    }
}

fn spawn_console_scene_from_stack_request(
    mut commands: Commands,
    mut scene_load_requests: MessageReader<SceneLoadRequested>,
    console_history: Res<FoundationConsoleHistory>,
    console_ui_state: Res<FoundationConsoleUiState>,
    mut input_focus: ResMut<InputFocus>,
) {
    for scene_load_request in scene_load_requests.read() {
        if !is_console_scene_source(&scene_load_request.source) {
            continue;
        }

        let input_entity = spawn_console_overlay(
            &mut commands,
            scene_load_request.scene_id,
            &console_history,
            &console_ui_state,
        );
        input_focus.set(input_entity, FocusCause::Navigated);
    }
}

fn track_console_scene_added(
    mut scene_added_messages: MessageReader<SceneAdded>,
    scene_stack: Res<crate::scene_stack::SceneStack>,
    mut console_state: ResMut<FoundationConsoleState>,
) {
    for scene_added_message in scene_added_messages.read() {
        let Some(scene_entry) = scene_stack.get(scene_added_message.scene_id) else {
            continue;
        };
        if is_console_scene_source(&scene_entry.source) {
            console_state.mark_open(scene_added_message.scene_id);
        }
    }
}

fn track_console_scene_removed(
    mut scene_removed_messages: MessageReader<SceneRemoved>,
    mut console_state: ResMut<FoundationConsoleState>,
    mut input_focus: ResMut<InputFocus>,
) {
    for scene_removed_message in scene_removed_messages.read() {
        if console_state.mark_closed_if_scene_matches(scene_removed_message.scene_id) {
            input_focus.clear();
        }
    }
}

fn is_console_scene_source(scene_source: &SceneSource) -> bool {
    matches!(
        scene_source,
        SceneSource::Runtime { key } if key.0 == FOUNDATION_CONSOLE_SCENE_KEY
    )
}

fn update_console_input_state(
    mut console_ui_state: ResMut<FoundationConsoleUiState>,
    console_inputs: Query<&EditableText, (With<FoundationConsoleInput>, Changed<EditableText>)>,
) {
    for editable_text in &console_inputs {
        console_ui_state.input = editable_text_value(editable_text);
        console_ui_state.history_cursor = None;
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_console_keyboard_actions(
    keyboard_input: Res<ButtonInput<Key>>,
    input_focus: Res<InputFocus>,
    mut commands: Commands,
    mut console_ui_state: ResMut<FoundationConsoleUiState>,
    console_history: Res<FoundationConsoleHistory>,
    console_registry: Res<FoundationConsoleRegistry>,
    mut console_inputs: Query<(Entity, &mut EditableText), With<FoundationConsoleInput>>,
    mut scene_commands: MessageWriter<SceneCommand>,
) {
    let Some((input_entity, mut editable_text)) = console_inputs.iter_mut().next() else {
        return;
    };
    if input_focus.get() != Some(input_entity) {
        return;
    }

    if keyboard_input.just_pressed(Key::Escape) {
        let console_scene_key = SceneKey::new(FOUNDATION_CONSOLE_SCENE_KEY);
        scene_commands.write(SceneCommand::Close(SceneTarget::Key(console_scene_key)));
        return;
    }

    if keyboard_input.just_pressed(Key::Tab) {
        if let Some(completed_input) =
            autocomplete_console_input(&console_ui_state.input, &console_registry)
        {
            replace_console_input(&mut editable_text, &completed_input);
            console_ui_state.input = completed_input;
        }
        return;
    }

    if keyboard_input.just_pressed(Key::ArrowUp) {
        if let Some(history_input) = previous_history_input(&mut console_ui_state, &console_history)
        {
            replace_console_input(&mut editable_text, &history_input);
            console_ui_state.input = history_input;
        }
        return;
    }

    if keyboard_input.just_pressed(Key::ArrowDown) {
        let history_input = next_history_input(&mut console_ui_state, &console_history);
        replace_console_input(&mut editable_text, &history_input);
        console_ui_state.input = history_input;
        return;
    }

    if keyboard_input.just_pressed(Key::Enter) {
        let submitted_command_line = console_ui_state.input.trim().to_string();
        if submitted_command_line.is_empty() {
            return;
        }

        replace_console_input(&mut editable_text, "");
        console_ui_state.input.clear();
        console_ui_state.history_cursor = None;

        commands.queue(move |world: &mut World| {
            execute_console_command_from_ui(world, submitted_command_line);
        });
    }
}

fn refresh_console_text_nodes(
    console_ui_state: Res<FoundationConsoleUiState>,
    console_history: Res<FoundationConsoleHistory>,
    console_registry: Res<FoundationConsoleRegistry>,
    mut console_outputs: Query<
        &mut Text,
        (
            With<FoundationConsoleOutput>,
            Without<FoundationConsoleSuggestion>,
        ),
    >,
    mut console_suggestions: Query<
        &mut Text,
        (
            With<FoundationConsoleSuggestion>,
            Without<FoundationConsoleOutput>,
        ),
    >,
) {
    if console_ui_state.is_changed() || console_history.is_changed() {
        let output_text = console_output_text(&console_history, &console_ui_state);
        for mut text in &mut console_outputs {
            text.0 = output_text.clone();
        }
    }

    if console_ui_state.is_changed() || console_registry.is_changed() {
        let suggestion_text = console_suggestion_text(&console_ui_state.input, &console_registry);
        for mut text in &mut console_suggestions {
            text.0 = suggestion_text.clone();
        }
    }
}

fn execute_console_command_from_ui(world: &mut World, submitted_command_line: String) {
    let execution_result = {
        let console_registry = world.resource::<FoundationConsoleRegistry>().clone();
        console_registry.execute_command_line(world, &submitted_command_line)
    };

    let mut output_line = format!("> {submitted_command_line}");
    match execution_result {
        Ok(()) => {
            output_line.push_str("\nCommand completed.");
            let mut console_history = world.resource_mut::<FoundationConsoleHistory>();
            console_history.push_command(submitted_command_line);
        }
        Err(command_error) => {
            output_line.push_str(&format!("\nError: {command_error}"));
        }
    }

    let mut console_ui_state = world.resource_mut::<FoundationConsoleUiState>();
    console_ui_state.output_lines.push(output_line);
    console_ui_state.history_cursor = None;
}

fn editable_text_value(editable_text: &EditableText) -> String {
    let mut value = String::new();
    value.reserve(editable_text.value().into_iter().map(str::len).sum());
    for text_part in editable_text.value() {
        value.push_str(text_part);
    }
    value
}

fn replace_console_input(editable_text: &mut EditableText, replacement: &str) {
    editable_text.clear();
    editable_text.editor.set_text(replacement);
    editable_text.queue_edit(TextEdit::TextEnd(false));
}

fn autocomplete_console_input(
    console_input: &str,
    console_registry: &FoundationConsoleRegistry,
) -> Option<String> {
    let command_prefix = console_input
        .split_whitespace()
        .next()
        .unwrap_or(console_input);
    let candidate = console_registry
        .autocomplete_command_names(command_prefix)
        .into_iter()
        .next()?;

    Some(command_placeholder(
        &candidate.replacement,
        console_registry,
    ))
}

fn console_suggestion_text(
    console_input: &str,
    console_registry: &FoundationConsoleRegistry,
) -> String {
    let Some(suggestion) = autocomplete_console_input(console_input, console_registry) else {
        return String::new();
    };

    if suggestion == console_input {
        String::new()
    } else {
        format!("Tab: {suggestion}")
    }
}

fn command_placeholder(command_name: &str, console_registry: &FoundationConsoleRegistry) -> String {
    let Some(command) = console_registry.find_command(command_name) else {
        return command_name.to_string();
    };

    let parameter_placeholders = (command.parameters)()
        .iter()
        .map(|parameter| format!("{}=<{}>", parameter.name, parameter.type_name))
        .collect::<Vec<_>>();

    if parameter_placeholders.is_empty() {
        command_name.to_string()
    } else {
        format!("{} {}", command_name, parameter_placeholders.join(" "))
    }
}

fn previous_history_input(
    console_ui_state: &mut FoundationConsoleUiState,
    console_history: &FoundationConsoleHistory,
) -> Option<String> {
    if console_history.commands.is_empty() {
        return None;
    }

    let history_entry_index = console_ui_state
        .history_cursor
        .map(|history_cursor| history_cursor.saturating_sub(1))
        .unwrap_or(console_history.commands.len() - 1);
    console_ui_state.history_cursor = Some(history_entry_index);
    console_history.commands.get(history_entry_index).cloned()
}

fn next_history_input(
    console_ui_state: &mut FoundationConsoleUiState,
    console_history: &FoundationConsoleHistory,
) -> String {
    let Some(history_cursor) = console_ui_state.history_cursor else {
        return String::new();
    };

    let next_history_index = history_cursor + 1;
    if next_history_index >= console_history.commands.len() {
        console_ui_state.history_cursor = None;
        String::new()
    } else {
        console_ui_state.history_cursor = Some(next_history_index);
        console_history.commands[next_history_index].clone()
    }
}

fn spawn_console_overlay(
    commands: &mut Commands,
    scene_id: crate::scene_stack::SceneId,
    console_history: &FoundationConsoleHistory,
    console_ui_state: &FoundationConsoleUiState,
) -> Entity {
    let console_root_height = Val::Px(280.0);
    let console_padding = UiRect::all(Val::Px(10.0));
    let console_gap = Val::Px(8.0);
    let console_background = BackgroundColor(Color::srgba(0.02, 0.02, 0.025, 0.92));
    let console_border = BorderColor::all(Color::srgba(0.25, 0.25, 0.30, 1.0));
    let console_text_color = TextColor(Color::srgba(0.82, 0.88, 0.78, 1.0));
    let input_background = BackgroundColor(Color::srgba(0.05, 0.05, 0.06, 1.0));
    let output_text = console_output_text(console_history, console_ui_state);
    let suggestion_text = String::new();

    let root_entity = commands
        .spawn((
            Name::new("Foundation Debug Console"),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                bottom: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: console_root_height,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                row_gap: console_gap,
                padding: console_padding,
                border: UiRect::top(Val::Px(1.0)),
                ..default()
            },
            console_background,
            console_border,
            GlobalZIndex(10_000),
            TabGroup::new(0),
            SceneOwner { scene_id },
            FoundationConsoleRoot,
        ))
        .id();

    let output_entity = commands
        .spawn((
            Name::new("Foundation Debug Console Output"),
            Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                overflow: Overflow::clip_y(),
                ..default()
            },
            Text::new(output_text),
            TextFont {
                font_size: 14.0.into(),
                ..default()
            },
            console_text_color,
            SceneOwner { scene_id },
            FoundationConsoleOutput,
        ))
        .id();

    let suggestion_entity = commands
        .spawn((
            Name::new("Foundation Debug Console Suggestion"),
            Node {
                width: Val::Percent(100.0),
                min_height: Val::Px(20.0),
                ..default()
            },
            Text::new(suggestion_text),
            TextFont {
                font_size: 12.0.into(),
                ..default()
            },
            TextColor(Color::srgba(0.50, 0.62, 0.92, 1.0)),
            SceneOwner { scene_id },
            FoundationConsoleSuggestion,
        ))
        .id();

    let input_container_entity = commands
        .spawn((
            Name::new("Foundation Debug Console Input Container"),
            Node {
                width: Val::Percent(100.0),
                min_height: Val::Px(34.0),
                padding: UiRect::all(Val::Px(6.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            input_background,
            BorderColor::all(Color::srgba(0.35, 0.35, 0.40, 1.0)),
            SceneOwner { scene_id },
        ))
        .id();

    let input_entity = commands
        .spawn((
            Name::new("Foundation Debug Console Input"),
            Node {
                width: Val::Percent(100.0),
                ..default()
            },
            FoundationConsoleInput,
            EditableText::new(console_ui_state.input.clone()),
            TextLayout::no_wrap(),
            TextFont {
                font_size: 14.0.into(),
                ..default()
            },
            console_text_color,
            TextCursorStyle::default(),
            TabIndex(0),
            AutoFocus,
            SceneOwner { scene_id },
        ))
        .id();

    commands
        .entity(input_container_entity)
        .add_child(input_entity);
    commands.entity(root_entity).add_children(&[
        output_entity,
        suggestion_entity,
        input_container_entity,
    ]);

    input_entity
}

fn console_output_text(
    _console_history: &FoundationConsoleHistory,
    console_ui_state: &FoundationConsoleUiState,
) -> String {
    if console_ui_state.output_lines.is_empty() {
        "Foundation debug console ready.".to_string()
    } else {
        console_ui_state.output_lines.join("\n")
    }
}

/// Snapshot of console command descriptors linked into this binary.
#[derive(Clone, Debug, Resource)]
pub struct FoundationConsoleRegistry {
    commands: Vec<&'static ConsoleCommandDescriptor>,
}

impl Default for FoundationConsoleRegistry {
    fn default() -> Self {
        let mut commands = FOUNDATION_CONSOLE_COMMANDS.iter().collect::<Vec<_>>();
        commands.sort_by(|left_command, right_command| left_command.name.cmp(right_command.name));
        Self { commands }
    }
}

impl FoundationConsoleRegistry {
    /// Returns registered commands sorted by command name.
    pub fn commands(&self) -> &[&'static ConsoleCommandDescriptor] {
        &self.commands
    }

    /// Finds a command by exact name.
    pub fn find_command(&self, command_name: &str) -> Option<&'static ConsoleCommandDescriptor> {
        self.commands
            .iter()
            .copied()
            .find(|command| command.name == command_name)
    }

    /// Returns deterministic autocomplete candidates for a command-name prefix.
    pub fn autocomplete_command_names(
        &self,
        command_prefix: &str,
    ) -> Vec<ConsoleAutocompleteCandidate> {
        self.commands
            .iter()
            .filter(|command| command.name.starts_with(command_prefix))
            .map(|command| ConsoleAutocompleteCandidate {
                replacement: command.name.to_string(),
                display: command.name.to_string(),
            })
            .collect()
    }

    /// Executes a parsed console command against the provided Bevy world.
    pub fn execute_command_line(
        &self,
        world: &mut World,
        command_line: &str,
    ) -> ConsoleCommandResult<()> {
        let parsed_command_line = ParsedConsoleCommandLine::parse(command_line)?;
        let Some(command) = self.find_command(&parsed_command_line.command_name) else {
            return Err(ConsoleCommandError::UnknownCommand {
                command_name: parsed_command_line.command_name,
            });
        };

        (command.execute)(world, parsed_command_line.arguments)
    }
}

/// Metadata and executor for one Foundation console command.
#[derive(Clone, Copy)]
pub struct ConsoleCommandDescriptor {
    /// Name typed by users to invoke this command.
    pub name: &'static str,
    /// Function returning user-provided command parameter metadata.
    pub parameters: fn() -> &'static [ConsoleCommandParameter],
    /// Function that parses user inputs and executes the generated command system.
    pub execute: ConsoleCommandExecutor,
}

impl fmt::Debug for ConsoleCommandDescriptor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ConsoleCommandDescriptor")
            .field("name", &self.name)
            .field("parameters", &(self.parameters)())
            .finish_non_exhaustive()
    }
}

/// Function pointer used by generated console command adapters.
pub type ConsoleCommandExecutor =
    fn(&mut World, ConsoleCommandArguments) -> ConsoleCommandResult<()>;

/// Metadata for one named user-provided console command parameter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConsoleCommandParameter {
    /// Name typed by users for this parameter.
    pub name: &'static str,
    /// Rust type name used for placeholder text and diagnostics.
    pub type_name: &'static str,
    /// Whether this parameter is required by the command input struct.
    pub required: bool,
}

/// Parsed user-provided named arguments for a console command.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ConsoleCommandArguments {
    values: BTreeMap<String, String>,
}

impl ConsoleCommandArguments {
    /// Creates arguments from key/value pairs.
    pub fn from_pairs(
        pairs: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        let values = pairs
            .into_iter()
            .map(|(parameter_name, parameter_value)| {
                (parameter_name.into(), parameter_value.into())
            })
            .collect();
        Self { values }
    }

    /// Returns true when no named arguments were provided.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns a required parameter value or a structured command error.
    pub fn required(&self, parameter_name: &'static str) -> ConsoleCommandResult<&str> {
        self.values
            .get(parameter_name)
            .map(String::as_str)
            .ok_or(ConsoleCommandError::MissingParameter { parameter_name })
    }

    /// Returns a named parameter value when it was provided.
    pub fn get(&self, parameter_name: &str) -> Option<&str> {
        self.values.get(parameter_name).map(String::as_str)
    }
}

/// Wrapper around user-provided input structs in console command signatures.
#[derive(Clone, Debug)]
pub struct ConsoleInputs<T> {
    values: T,
}

impl<T> ConsoleInputs<T> {
    /// Creates a console input wrapper from parsed values.
    pub fn new(values: T) -> Self {
        Self { values }
    }

    /// Returns the parsed command input values.
    pub fn values(&self) -> &T {
        &self.values
    }

    /// Consumes the wrapper and returns parsed command input values.
    pub fn into_values(self) -> T {
        self.values
    }
}

impl<T> std::ops::Deref for ConsoleInputs<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

/// Trait implemented by command input structs.
pub trait ConsoleCommandInput: Sized {
    /// Returns named parameter metadata used by placeholder text and autocomplete.
    fn parameters() -> &'static [ConsoleCommandParameter];

    /// Parses command-line arguments into strongly typed input values.
    fn parse(console_command_arguments: &ConsoleCommandArguments) -> ConsoleCommandResult<Self>;
}

impl ConsoleCommandInput for () {
    fn parameters() -> &'static [ConsoleCommandParameter] {
        &[]
    }

    fn parse(console_command_arguments: &ConsoleCommandArguments) -> ConsoleCommandResult<Self> {
        if console_command_arguments.is_empty() {
            Ok(())
        } else {
            Err(ConsoleCommandError::UnexpectedParameters)
        }
    }
}

/// Trait used by generated command adapters to normalize command return values.
pub trait IntoConsoleCommandResult {
    /// Converts a command return value into the standard console command result.
    fn into_console_command_result(self) -> ConsoleCommandResult<()>;
}

impl IntoConsoleCommandResult for () {
    fn into_console_command_result(self) -> ConsoleCommandResult<()> {
        Ok(())
    }
}

impl IntoConsoleCommandResult for ConsoleCommandResult<()> {
    fn into_console_command_result(self) -> ConsoleCommandResult<()> {
        self
    }
}

/// User-facing autocomplete candidate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConsoleAutocompleteCandidate {
    /// Text inserted if the candidate is accepted.
    pub replacement: String,
    /// Text shown in the autocomplete UI.
    pub display: String,
}

/// Result alias used by Foundation console APIs.
pub type ConsoleCommandResult<T> = std::result::Result<T, ConsoleCommandError>;

/// Errors produced while parsing or executing console commands.
#[derive(Debug)]
pub enum ConsoleCommandError {
    /// The command line did not contain a command name.
    EmptyCommand,
    /// A command name did not match the linked command registry.
    UnknownCommand {
        /// Unknown command name typed by the user.
        command_name: String,
    },
    /// A command token could not be parsed as `name=value`.
    InvalidArgumentToken {
        /// Raw token that failed parsing.
        token: String,
    },
    /// A required named parameter was missing.
    MissingParameter {
        /// Missing parameter name.
        parameter_name: &'static str,
    },
    /// A named parameter could not be parsed into its Rust type.
    InvalidParameter {
        /// Parameter name that failed parsing.
        parameter_name: &'static str,
        /// Expected Rust type name.
        expected_type_name: &'static str,
        /// Parse error details.
        reason: String,
    },
    /// A no-input command received one or more parameters.
    UnexpectedParameters,
    /// Bevy failed to run a generated command system.
    SystemFailed {
        /// Bevy system failure details.
        reason: String,
    },
}

impl ConsoleCommandError {
    /// Creates an invalid-parameter error.
    pub fn invalid_parameter(
        parameter_name: &'static str,
        expected_type_name: &'static str,
        reason: String,
    ) -> Self {
        Self::InvalidParameter {
            parameter_name,
            expected_type_name,
            reason,
        }
    }

    /// Converts a Bevy one-shot system failure into a console command error.
    pub fn from_run_system_error(error: bevy::ecs::system::RunSystemError) -> Self {
        Self::SystemFailed {
            reason: error.to_string(),
        }
    }
}

impl fmt::Display for ConsoleCommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCommand => write!(formatter, "Expected a console command."),
            Self::UnknownCommand { command_name } => {
                write!(formatter, "Unknown console command `{command_name}`.")
            }
            Self::InvalidArgumentToken { token } => {
                write!(formatter, "Expected console argument `{token}` to use name=value syntax.")
            }
            Self::MissingParameter { parameter_name } => {
                write!(formatter, "Missing required console parameter `{parameter_name}`.")
            }
            Self::InvalidParameter {
                parameter_name,
                expected_type_name,
                reason,
            } => write!(
                formatter,
                "Failed to parse console parameter `{parameter_name}` as {expected_type_name}: {reason}"
            ),
            Self::UnexpectedParameters => write!(formatter, "This console command does not accept parameters."),
            Self::SystemFailed { reason } => write!(formatter, "Console command system failed: {reason}"),
        }
    }
}

impl std::error::Error for ConsoleCommandError {}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParsedConsoleCommandLine {
    command_name: String,
    arguments: ConsoleCommandArguments,
}

impl ParsedConsoleCommandLine {
    fn parse(command_line: &str) -> ConsoleCommandResult<Self> {
        let mut command_tokens = command_line.split_whitespace();
        let Some(command_name) = command_tokens.next() else {
            return Err(ConsoleCommandError::EmptyCommand);
        };

        let mut argument_values = BTreeMap::new();
        for command_token in command_tokens {
            let Some((parameter_name, parameter_value)) = command_token.split_once('=') else {
                return Err(ConsoleCommandError::InvalidArgumentToken {
                    token: command_token.to_string(),
                });
            };
            argument_values.insert(parameter_name.to_string(), parameter_value.to_string());
        }

        Ok(Self {
            command_name: command_name.to_string(),
            arguments: ConsoleCommandArguments {
                values: argument_values,
            },
        })
    }
}

/// Inputs for the built-in history size command.
#[derive(Clone, Debug, crate::ConsoleCommandInput)]
pub struct FoundationConsoleHistorySizeInputs {
    /// Maximum number of entries that should remain in history.
    pub max_entries: usize,
}

/// Trims persisted console history to a maximum number of entries.
#[allow(unused_mut)]
#[crate::console_command]
pub fn foundation_console_history_size(
    mut console_history: ResMut<FoundationConsoleHistory>,
    inputs: ConsoleInputs<FoundationConsoleHistorySizeInputs>,
) {
    let max_entries = inputs.max_entries;
    if console_history.commands.len() > max_entries {
        let removed_history_entry_count = console_history.commands.len() - max_entries;
        console_history
            .commands
            .drain(0..removed_history_entry_count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, crate::ConsoleCommandInput)]
    struct ExampleInputs {
        amount: usize,
        label: String,
    }

    #[test]
    fn input_metadata_uses_named_struct_fields() {
        let parameters = ExampleInputs::parameters();

        assert_eq!(parameters[0].name, "amount");
        assert_eq!(parameters[0].type_name, "usize");
        assert_eq!(parameters[1].name, "label");
        assert_eq!(parameters[1].type_name, "String");
    }

    #[test]
    fn input_parser_reads_named_arguments() {
        let arguments = ConsoleCommandArguments::from_pairs([("amount", "7"), ("label", "test")]);
        let inputs = ExampleInputs::parse(&arguments).expect("inputs should parse");

        assert_eq!(inputs.amount, 7);
        assert_eq!(inputs.label, "test");
    }

    #[test]
    fn command_line_parser_reads_command_name_and_named_arguments() {
        let parsed_command_line = ParsedConsoleCommandLine::parse("spawn amount=3 label=crate")
            .expect("command line should parse");

        assert_eq!(parsed_command_line.command_name, "spawn");
        assert_eq!(parsed_command_line.arguments.get("amount"), Some("3"));
        assert_eq!(parsed_command_line.arguments.get("label"), Some("crate"));
    }

    #[test]
    fn history_file_path_uses_saved_console_directory() {
        assert_eq!(
            FoundationConsoleHistory::history_file_path(),
            PathBuf::from("saved/console").join("history.json")
        );
    }

    #[test]
    fn console_state_tracks_scene_id_across_reopen_cycles() {
        let mut console_state = FoundationConsoleState::default();
        let first_console_scene_id = SceneId(7);
        let second_console_scene_id = SceneId(8);

        console_state.mark_open(first_console_scene_id);
        assert!(console_state.is_open);
        assert_eq!(console_state.scene_id, Some(first_console_scene_id));

        assert!(console_state.mark_closed_if_scene_matches(first_console_scene_id));
        assert!(!console_state.is_open);
        assert_eq!(console_state.scene_id, None);

        console_state.mark_open(second_console_scene_id);
        assert!(console_state.is_open);
        assert_eq!(console_state.scene_id, Some(second_console_scene_id));
    }

    #[test]
    fn console_state_ignores_unrelated_scene_removals() {
        let mut console_state = FoundationConsoleState::default();
        let console_scene_id = SceneId(11);
        let unrelated_scene_id = SceneId(12);

        console_state.mark_open(console_scene_id);

        assert!(!console_state.mark_closed_if_scene_matches(unrelated_scene_id));
        assert!(console_state.is_open);
        assert_eq!(console_state.scene_id, Some(console_scene_id));
    }

    #[test]
    fn console_output_uses_execution_log_without_reappending_navigation_history() {
        let console_history = FoundationConsoleHistory {
            commands: vec!["example.say-hello name=Jon".to_string()],
        };
        let console_ui_state = FoundationConsoleUiState {
            input: String::new(),
            output_lines: vec![
                "> example.say-hello name=Jon\nCommand completed.".to_string(),
                "> hello world\nError: Expected console argument `world` to use name=value syntax."
                    .to_string(),
            ],
            history_cursor: None,
        };

        let output_text = console_output_text(&console_history, &console_ui_state);

        assert_eq!(
            output_text,
            "> example.say-hello name=Jon\nCommand completed.\n> hello world\nError: Expected console argument `world` to use name=value syntax."
        );
    }

    #[test]
    fn registry_contains_builtin_foundation_command() {
        let registry = FoundationConsoleRegistry::default();

        assert!(registry
            .commands()
            .iter()
            .any(|command| command.name == "foundation_console_history_size"));
    }

    #[test]
    fn registry_executes_builtin_command_as_bevy_system() {
        let mut world = World::new();
        world.insert_resource(FoundationConsoleHistory {
            commands: vec![
                "first".to_string(),
                "second".to_string(),
                "third".to_string(),
            ],
        });
        let registry = FoundationConsoleRegistry::default();

        registry
            .execute_command_line(&mut world, "foundation_console_history_size max_entries=2")
            .expect("command should execute");

        let console_history = world.resource::<FoundationConsoleHistory>();
        assert_eq!(console_history.commands, vec!["second", "third"]);
    }
}
