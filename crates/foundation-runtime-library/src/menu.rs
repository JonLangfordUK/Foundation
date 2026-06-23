//! Reusable scene-stack menu primitives for Foundation games.
//!
//! These components intentionally describe game-agnostic menu behavior: opening
//! scenes, closing the current scene, requesting application exit, rendering a
//! dummy options menu, and rendering placeholder stack scenes. Game crates can
//! author these components in Jackdaw `.jsn` assets without duplicating menu
//! systems in game-specific Rust code.

use bevy::prelude::*;
use jackdaw_runtime::prelude::*;

use crate::scene_stack::{
    OpenSceneOptions, SceneCommand, SceneOwner, ScenePresentation, SceneSource, SceneStack,
};

/// Installs reusable Foundation menu components and systems.
#[derive(Default)]
pub struct FoundationMenuPlugin;

impl Plugin for FoundationMenuPlugin {
    fn build(&self, app: &mut App) {
        // Runtime resources come first so menu systems can read stable defaults immediately.
        app.init_resource::<FoundationPauseState>()
            .init_resource::<FoundationMenuRuntimeSettings>()
            .add_message::<FoundationExitRequested>()
            .register_type::<FoundationMenuButton>()
            .register_type::<FoundationOptionsMenu>()
            .register_type::<FoundationPlaceholderMenu>()
            .register_type::<FoundationCloseOnEscape>()
            .register_type::<FoundationPauseOpener>()
            .register_type::<FoundationSimpleGameplayLevel>()
            .register_type::<FoundationSpin>()
            .register_type::<FoundationUiOrder>()
            .register_type::<FoundationPauseState>()
            // Menu systems run together because generated UI and actions share scene ownership.
            .add_systems(
                Update,
                (
                    initialize_simple_gameplay_levels,
                    initialize_options_menus,
                    initialize_placeholder_menus,
                    open_pause_menus,
                    spin_foundation_entities.run_if(foundation_is_not_paused),
                    update_foundation_menu_button_interactions,
                    update_options_tab_button_interactions,
                    inherit_scene_owner_to_generated_menu_ui,
                    close_on_escape,
                ),
            );
    }
}

/// Global pause state for Foundation scene-stack gameplay.
///
/// Games can read this resource directly or use [`foundation_is_paused`] and
/// [`foundation_is_not_paused`] as run conditions for systems that should react
/// to pause menus.
#[derive(Clone, Copy, Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct FoundationPauseState {
    /// True while gameplay is paused by a Foundation pause menu.
    pub paused: bool,
}

/// Returns true when Foundation gameplay is currently paused.
pub fn foundation_is_paused(pause: Res<FoundationPauseState>) -> bool {
    pause.paused
}

/// Returns true when Foundation gameplay is not currently paused.
pub fn foundation_is_not_paused(pause: Res<FoundationPauseState>) -> bool {
    !pause.paused
}

/// Runtime policy for reusable Foundation menu systems.
///
/// Standalone games use the default policy: menu systems process any authored
/// menu entity. Editors that keep the authored scene alive during Play should
/// require [`SceneOwner`] so menu actions only run for scene-stack runtime
/// copies, not the open editor scene.
#[derive(Clone, Copy, Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct FoundationMenuRuntimeSettings {
    /// If true, menu systems ignore entities that are not owned by the scene stack.
    pub require_scene_owner: bool,
}

/// Message emitted when reusable menu UI requests application exit.
///
/// Standalone games should translate this into [`AppExit`]. Editor integrations
/// can translate it into a stop-play action so the editor process remains open.
#[derive(Clone, Copy, Debug, Default, Message, PartialEq, Eq)]
pub struct FoundationExitRequested;

/// Reusable button action component for stack-based menus.
///
/// Supported `action` values are:
/// - `none`
/// - `open_scene`
/// - `open_overlay_scene`
/// - `clear_and_open_scene`
/// - `close_current`
/// - `resume`
/// - `exit`
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, @EditorCategory::new("Foundation/Menu"))]
pub struct FoundationMenuButton {
    /// Action identifier. Use `open_scene`, `close_current`, `exit`, or `none`.
    pub action: String,
    /// Jackdaw `.jsn` scene path used by the `open_scene` action.
    pub scene_path: String,
    /// Optional scene-stack key used by the `open_scene` action.
    pub scene_key: String,
}

impl FoundationMenuButton {
    /// Creates a button that does nothing when pressed.
    pub fn none() -> Self {
        // Empty scene fields keep inert buttons safe to author in placeholder menus.
        Self {
            action: "none".to_string(),
            scene_path: String::new(),
            scene_key: String::new(),
        }
    }

    /// Creates a button that opens a full-screen Jackdaw scene on the stack.
    pub fn open_scene(scene_path: impl Into<String>, scene_key: impl Into<String>) -> Self {
        // Full-screen opens cover earlier entries and become the focused scene.
        Self {
            action: "open_scene".to_string(),
            scene_path: scene_path.into(),
            scene_key: scene_key.into(),
        }
    }

    /// Creates a button that opens an input-blocking overlay scene on the stack.
    pub fn open_overlay_scene(scene_path: impl Into<String>, scene_key: impl Into<String>) -> Self {
        // Overlay opens preserve lower scene visibility while blocking lower input.
        Self {
            action: "open_overlay_scene".to_string(),
            scene_path: scene_path.into(),
            scene_key: scene_key.into(),
        }
    }

    /// Creates a button that clears the stack and opens a Jackdaw scene.
    pub fn clear_and_open_scene(
        scene_path: impl Into<String>,
        scene_key: impl Into<String>,
    ) -> Self {
        // Clear-and-open is used by main menu flows that should discard old gameplay state.
        Self {
            action: "clear_and_open_scene".to_string(),
            scene_path: scene_path.into(),
            scene_key: scene_key.into(),
        }
    }

    /// Creates a button that closes the current scene-stack entry.
    pub fn close_current() -> Self {
        Self {
            action: "close_current".to_string(),
            scene_path: String::new(),
            scene_key: String::new(),
        }
    }

    /// Creates a button that resumes gameplay by closing the current scene and
    /// clearing [`FoundationPauseState`].
    pub fn resume() -> Self {
        Self {
            action: "resume".to_string(),
            scene_path: String::new(),
            scene_key: String::new(),
        }
    }

    /// Creates a button that requests application exit.
    pub fn exit() -> Self {
        Self {
            action: "exit".to_string(),
            scene_path: String::new(),
            scene_key: String::new(),
        }
    }
}

impl Default for FoundationMenuButton {
    fn default() -> Self {
        Self::none()
    }
}

/// Marks an options menu root that should be populated with reusable dummy UI.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, @EditorCategory::new("Foundation/Menu"))]
pub struct FoundationOptionsMenu {
    /// Title shown above the tabs.
    pub title: String,
}

impl Default for FoundationOptionsMenu {
    fn default() -> Self {
        Self {
            title: "Options".to_string(),
        }
    }
}

/// Marks a placeholder menu root that should show title/body text and a Back button.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, @EditorCategory::new("Foundation/Menu"))]
pub struct FoundationPlaceholderMenu {
    /// Title shown at the top of the placeholder menu.
    pub title: String,
    /// Body copy shown under the title.
    pub body: String,
}

impl Default for FoundationPlaceholderMenu {
    fn default() -> Self {
        Self {
            title: "Placeholder".to_string(),
            body: "This scene is not implemented yet.".to_string(),
        }
    }
}

/// Closes the current scene-stack entry when Escape is pressed.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, @EditorCategory::new("Foundation/Menu"))]
pub struct FoundationCloseOnEscape;

/// Opens a pause menu scene when Escape is pressed while gameplay is unpaused.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, @EditorCategory::new("Foundation/Menu"))]
pub struct FoundationPauseOpener {
    /// Jackdaw `.jsn` scene path for the pause menu.
    pub pause_scene_path: String,
    /// Optional scene-stack key for the pause menu.
    pub pause_scene_key: String,
}

impl Default for FoundationPauseOpener {
    fn default() -> Self {
        Self {
            pause_scene_path: String::new(),
            pause_scene_key: "pause-menu".to_string(),
        }
    }
}

/// Runtime-authored starter gameplay level with a centered cube and light.
///
/// Add this component to a Jackdaw scene entity when a project needs a small
/// placeholder level without hand-authoring mesh/material handles in `.jsn`.
#[derive(Clone, Copy, Debug, Component, Reflect)]
#[reflect(Component, @EditorCategory::new("Foundation/Gameplay"))]
pub struct FoundationSimpleGameplayLevel {
    /// Edge length of the generated cube in world units.
    pub cube_size: f32,
}

impl Default for FoundationSimpleGameplayLevel {
    fn default() -> Self {
        Self { cube_size: 2.0 }
    }
}

/// Rotates an entity around its local Y axis while Foundation gameplay is not paused.
#[derive(Clone, Copy, Debug, Component, Reflect)]
#[reflect(Component, @EditorCategory::new("Foundation/Gameplay"))]
pub struct FoundationSpin {
    /// Rotation speed around the Y axis, in radians per second.
    pub radians_per_second: f32,
}

impl Default for FoundationSpin {
    fn default() -> Self {
        Self {
            radians_per_second: 1.0,
        }
    }
}

#[derive(Component, Debug)]
struct FoundationGeneratedGameplayLevel;

/// Stable authored sibling order for UI entities loaded from `.jsn` assets.
///
/// Jackdaw-authored UI can include this component so runtime repair systems can
/// rebuild Bevy `Children` lists without relying on ECS query or entity order.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, @EditorCategory::new("Foundation/UI"))]
pub struct FoundationUiOrder {
    /// Zero-based order of this entity within the authored scene file.
    pub order: u32,
}

#[derive(Component, Debug)]
struct FoundationOptionsRuntime {
    active_tab: usize,
}

/// Marks UI entities generated by [`FoundationMenuPlugin`] systems.
///
/// Game-specific authored-UI repair systems can use this marker to avoid
/// treating Foundation-generated runtime UI as Jackdaw-authored scene data.
#[derive(Component, Debug)]
pub struct FoundationGeneratedMenuUi;

#[derive(Component, Debug)]
struct FoundationOptionsTabButton {
    tab: usize,
}

#[derive(Component, Debug)]
struct FoundationOptionsSettingLabel {
    index: usize,
}

#[derive(Component, Debug)]
struct FoundationOptionsSettingValue {
    index: usize,
}

const OPTIONS_TABS: [&str; 4] = ["Gameplay", "Display", "Graphics", "Accessibility"];

type SimpleGameplayLevelInitQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static FoundationSimpleGameplayLevel,
        Option<&'static SceneOwner>,
        Option<&'static ChildOf>,
    ),
    Added<FoundationSimpleGameplayLevel>,
>;

fn initialize_simple_gameplay_levels(
    mut commands: Commands,
    settings: Res<FoundationMenuRuntimeSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    gameplay_levels: SimpleGameplayLevelInitQuery,
    scene_owners: Query<&SceneOwner>,
) {
    for (level_entity, level, scene_owner, parent_link) in &gameplay_levels {
        // Scene roots own generated gameplay entities so stack cleanup removes them.
        let scene_owner = effective_scene_owner(scene_owner, parent_link, &scene_owners);
        if should_skip_menu_runtime_entity(&settings, scene_owner.as_ref()) {
            continue;
        }
        info!(
            "Initializing FoundationSimpleGameplayLevel on {level_entity:?} with scene_owner={scene_owner:?}"
        );

        // Keep generated geometry minimal so projects can replace it with authored content.
        let cube_size = level.cube_size;
        let cube_mesh = meshes.add(Cuboid::from_size(Vec3::splat(cube_size)));
        let cube_base_color = Color::srgb(0.45, 0.55, 0.90);
        let cube_material = materials.add(StandardMaterial {
            base_color: cube_base_color,
            ..default()
        });
        let cube_position = Vec3::new(0.0, cube_size * 0.5, 0.0);

        let cube_entity = commands
            .spawn((
                Mesh3d(cube_mesh),
                MeshMaterial3d(cube_material),
                Transform::from_translation(cube_position),
                FoundationSpin::default(),
                FoundationGeneratedGameplayLevel,
                Name::new("Foundation Gameplay Cube"),
            ))
            .id();

        // A single directional light is enough for the starter cube and avoids asset setup.
        let light_illuminance = 12_000.0;
        let light_position = Vec3::new(3.0, 5.0, 3.0);
        let light_target = Vec3::ZERO;

        let light_entity = commands
            .spawn((
                DirectionalLight {
                    illuminance: light_illuminance,
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_translation(light_position).looking_at(light_target, Vec3::Y),
                FoundationGeneratedGameplayLevel,
                Name::new("Foundation Gameplay Directional Light"),
            ))
            .id();

        // The generated camera frames the starter cube from an angled gameplay view.
        let camera_position = Vec3::new(4.0, 3.0, 6.0);
        let camera_target = Vec3::new(0.0, 0.75, 0.0);

        let camera_entity = commands
            .spawn((
                Camera3d::default(),
                Transform::from_translation(camera_position).looking_at(camera_target, Vec3::Y),
                FoundationGeneratedGameplayLevel,
                Name::new("Foundation Gameplay Camera"),
            ))
            .id();

        for generated_entity in [cube_entity, light_entity, camera_entity] {
            // Ownership is required so closing the gameplay scene removes generated children.
            if let Some(scene_owner) = scene_owner {
                commands.entity(generated_entity).insert(scene_owner);
            }
        }
    }
}

fn spin_foundation_entities(
    time: Res<Time>,
    mut spinners: Query<(&FoundationSpin, &mut Transform)>,
) {
    let delta_seconds = time.delta_secs();
    for (spin, mut transform) in &mut spinners {
        // Use frame delta time so spin speed stays stable across frame rates.
        transform.rotate_y(spin.radians_per_second * delta_seconds);
    }
}

fn initialize_options_menus(
    mut commands: Commands,
    settings: Res<FoundationMenuRuntimeSettings>,
    menus: Query<
        (Entity, &FoundationOptionsMenu, Option<&SceneOwner>),
        Without<FoundationOptionsRuntime>,
    >,
) {
    for (menu_entity, menu, scene_owner) in &menus {
        if should_skip_menu_runtime_entity(&settings, scene_owner) {
            continue;
        }
        if spawn_options_menu_children(&mut commands, menu_entity, menu, scene_owner.copied())
            .is_none()
        {
            // Skip runtime insertion if child generation fails to produce content.
            continue;
        }
        commands
            .entity(menu_entity)
            .insert(FoundationOptionsRuntime { active_tab: 0 });
    }
}

fn initialize_placeholder_menus(
    mut commands: Commands,
    settings: Res<FoundationMenuRuntimeSettings>,
    menus: Query<
        (Entity, &FoundationPlaceholderMenu, Option<&SceneOwner>),
        Added<FoundationPlaceholderMenu>,
    >,
) {
    for (menu_entity, menu, scene_owner) in &menus {
        if should_skip_menu_runtime_entity(&settings, scene_owner) {
            continue;
        }
        spawn_placeholder_menu_children(&mut commands, menu_entity, menu, scene_owner.copied());
    }
}

fn spawn_options_menu_children(
    commands: &mut Commands,
    parent_entity: Entity,
    menu: &FoundationOptionsMenu,
    scene_owner: Option<SceneOwner>,
) -> Option<Entity> {
    // Build generated menu children once and attach them to the authored menu marker.
    let title_font_size = 48.0;
    let title_entity = spawn_text(commands, &menu.title, title_font_size, scene_owner);
    let tab_button_gap = 12.0;

    let tabs_entity = commands
        .spawn((
            row_node(tab_button_gap),
            FoundationGeneratedMenuUi,
            Name::new("Options Tabs"),
        ))
        .id();
    insert_owner(commands, tabs_entity, scene_owner);

    // Generate tab buttons from the static tab list so future tabs keep one path.
    let tab_buttons = OPTIONS_TABS
        .iter()
        .enumerate()
        .map(|(tab_index, tab_label)| {
            let tab_button_entity = spawn_button(commands, tab_label, scene_owner);
            commands
                .entity(tab_button_entity)
                .insert(FoundationOptionsTabButton { tab: tab_index });
            tab_button_entity
        })
        .collect::<Vec<_>>();
    commands.entity(tabs_entity).replace_children(&tab_buttons);

    let options_content_width = Val::Px(640.0);
    let options_row_gap = Val::Px(10.0);

    let content_entity = commands
        .spawn((
            Node {
                width: options_content_width,
                flex_direction: FlexDirection::Column,
                row_gap: options_row_gap,
                ..default()
            },
            FoundationGeneratedMenuUi,
            Name::new("Options Content"),
        ))
        .id();
    insert_owner(commands, content_entity, scene_owner);

    let default_tab_index = 0;
    spawn_setting_rows(commands, content_entity, default_tab_index, scene_owner);

    let back_button_entity = spawn_button(commands, "Back", scene_owner);
    commands
        .entity(back_button_entity)
        .insert(FoundationMenuButton::close_current());

    commands.entity(parent_entity).replace_children(&[
        title_entity,
        tabs_entity,
        content_entity,
        back_button_entity,
    ]);

    Some(content_entity)
}

fn spawn_placeholder_menu_children(
    commands: &mut Commands,
    parent_entity: Entity,
    menu: &FoundationPlaceholderMenu,
    scene_owner: Option<SceneOwner>,
) {
    // Placeholder menus only need text and a Back button to participate in the stack.
    let title_font_size = 48.0;
    let body_font_size = 24.0;
    let title_entity = spawn_text(commands, &menu.title, title_font_size, scene_owner);
    let body_entity = spawn_text(commands, &menu.body, body_font_size, scene_owner);
    let back_button_entity = spawn_button(commands, "Back", scene_owner);
    commands
        .entity(back_button_entity)
        .insert(FoundationMenuButton::close_current());
    commands.entity(parent_entity).replace_children(&[
        title_entity,
        body_entity,
        back_button_entity,
    ]);
}

fn spawn_setting_rows(
    commands: &mut Commands,
    content_entity: Entity,
    selected_tab_index: usize,
    scene_owner: Option<SceneOwner>,
) {
    let tab_name = OPTIONS_TABS
        .get(selected_tab_index)
        .copied()
        .unwrap_or("Options");
    let mut setting_row_entities = Vec::new();
    let first_setting_number = 1;
    let last_setting_number = 5;
    for setting_number in first_setting_number..=last_setting_number {
        // Settings are generated as simple label/value rows until real options exist.
        let row_width = Val::Percent(100.0);
        let row_column_gap = Val::Px(24.0);
        let setting_row_entity = commands
            .spawn((
                Node {
                    width: row_width,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    column_gap: row_column_gap,
                    ..default()
                },
                FoundationGeneratedMenuUi,
                Name::new(format!("{tab_name} Setting Row {setting_number}")),
            ))
            .id();
        insert_owner(commands, setting_row_entity, scene_owner);

        let setting_text_font_size = 22.0;
        let setting_label_text = format!("{tab_name} Property {setting_number}");
        let setting_label_entity = spawn_text(
            commands,
            &setting_label_text,
            setting_text_font_size,
            scene_owner,
        );
        commands
            .entity(setting_label_entity)
            .insert(FoundationOptionsSettingLabel {
                index: setting_number,
            });

        let setting_value_text = format!("< Value {setting_number} >");
        let setting_value_entity = spawn_text(
            commands,
            &setting_value_text,
            setting_text_font_size,
            scene_owner,
        );
        commands
            .entity(setting_value_entity)
            .insert(FoundationOptionsSettingValue {
                index: setting_number,
            });
        commands
            .entity(setting_row_entity)
            .replace_children(&[setting_label_entity, setting_value_entity]);
        setting_row_entities.push(setting_row_entity);
    }
    commands
        .entity(content_entity)
        .replace_children(&setting_row_entities);
}

fn spawn_button(
    commands: &mut Commands,
    button_label: &str,
    scene_owner: Option<SceneOwner>,
) -> Entity {
    let button_text_font_size = 24.0;
    let text_entity = spawn_text(commands, button_label, button_text_font_size, scene_owner);
    let button_width = Val::Px(220.0);
    let button_height = Val::Px(52.0);
    let button_padding = UiRect::all(Val::Px(8.0));

    let button_entity = commands
        .spawn((
            Button,
            Node {
                width: button_width,
                height: button_height,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: button_padding,
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON),
            FoundationGeneratedMenuUi,
            Name::new(format!("{button_label} Button")),
        ))
        .id();
    safe_add_child(commands, button_entity, text_entity);
    insert_owner(commands, button_entity, scene_owner);
    button_entity
}

fn spawn_text(
    commands: &mut Commands,
    text_value: &str,
    font_size: f32,
    scene_owner: Option<SceneOwner>,
) -> Entity {
    let text_entity = commands
        .spawn((
            Text::new(text_value.to_string()),
            TextFont::from_font_size(font_size),
            TextColor(Color::WHITE),
            FoundationGeneratedMenuUi,
            Name::new(text_value.to_string()),
        ))
        .id();
    insert_owner(commands, text_entity, scene_owner);
    text_entity
}

fn row_node(column_gap: f32) -> Node {
    let row_column_gap = Val::Px(column_gap);
    Node {
        flex_direction: FlexDirection::Row,
        column_gap: row_column_gap,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }
}

fn insert_owner(commands: &mut Commands, owned_entity: Entity, scene_owner: Option<SceneOwner>) {
    if let Some(scene_owner) = scene_owner {
        commands.entity(owned_entity).insert(scene_owner);
    }
}

fn effective_scene_owner(
    scene_owner: Option<&SceneOwner>,
    parent_link: Option<&ChildOf>,
    scene_owners: &Query<&SceneOwner>,
) -> Option<SceneOwner> {
    // Generated marker entities may inherit ownership from their loaded scene root.
    scene_owner.copied().or_else(|| {
        parent_link.and_then(|parent_link| scene_owners.get(parent_link.0).ok().copied())
    })
}

fn safe_add_child(commands: &mut Commands, parent_entity: Entity, child_entity: Entity) {
    safe_add_children(commands, parent_entity, vec![child_entity]);
}

fn safe_add_children(commands: &mut Commands, parent_entity: Entity, children: Vec<Entity>) {
    commands.queue(move |world: &mut World| {
        if world.get_entity(parent_entity).is_err() {
            return;
        }

        // Ignore stale children because scene-stack cleanup can race queued UI wiring.
        let existing_children = children
            .into_iter()
            .filter(|child_entity| world.get_entity(*child_entity).is_ok())
            .collect::<Vec<_>>();
        if existing_children.is_empty() {
            return;
        }

        if let Ok(mut parent_entity_mut) = world.get_entity_mut(parent_entity) {
            parent_entity_mut.add_children(&existing_children);
        }
    });
}

const NORMAL_BUTTON: Color = Color::srgb(0.12, 0.14, 0.25);
const HOVERED_BUTTON: Color = Color::srgb(0.28, 0.32, 0.62);
const PRESSED_BUTTON: Color = Color::srgb(0.45, 0.50, 0.85);
const SELECTED_BUTTON: Color = Color::srgb(0.30, 0.42, 0.80);

type FoundationMenuButtonInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static FoundationMenuButton,
        &'static mut BackgroundColor,
        Option<&'static SceneOwner>,
        Option<&'static ChildOf>,
    ),
    (Changed<Interaction>, With<Button>),
>;

fn open_pause_menus(
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<FoundationMenuRuntimeSettings>,
    scene_stack: Option<Res<SceneStack>>,
    mut pause_state: ResMut<FoundationPauseState>,
    pause_openers: Query<(
        &FoundationPauseOpener,
        Option<&SceneOwner>,
        Option<&ChildOf>,
    )>,
    scene_owners: Query<&SceneOwner>,
    mut scene_commands: MessageWriter<SceneCommand>,
) {
    if pause_state.paused || !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }

    // Use the first active opener so scene-authored levels decide which pause menu opens.
    let Some((opener, _, _)) = pause_openers.iter().find(|(_, scene_owner, parent_link)| {
        let opener_scene_owner = effective_scene_owner(*scene_owner, *parent_link, &scene_owners);
        !should_skip_scene_stack_menu_input(
            &settings,
            scene_stack.as_deref(),
            opener_scene_owner.as_ref(),
        )
    }) else {
        return;
    };
    let pause_scene_path = opener.pause_scene_path.trim();
    if pause_scene_path.is_empty() {
        warn!("FoundationPauseOpener has an empty pause_scene_path");
        return;
    }

    // Set the pause state before opening the overlay so gameplay systems stop immediately.
    pause_state.paused = true;
    let mut options =
        OpenSceneOptions::default().with_presentation(ScenePresentation::PAUSE_OVERLAY);
    let pause_scene_key = opener.pause_scene_key.trim();
    if !pause_scene_key.is_empty() {
        options = options.with_key(pause_scene_key);
    }
    scene_commands.write(SceneCommand::open_with_options(
        SceneSource::jsn_level(pause_scene_path),
        options,
    ));
}

fn update_foundation_menu_button_interactions(
    settings: Res<FoundationMenuRuntimeSettings>,
    scene_stack: Option<Res<SceneStack>>,
    mut buttons: FoundationMenuButtonInteractionQuery,
    scene_owners: Query<&SceneOwner>,
    mut scene_commands: MessageWriter<SceneCommand>,
    mut exit_requested: MessageWriter<FoundationExitRequested>,
    mut pause_state: ResMut<FoundationPauseState>,
) {
    for (interaction, button, mut background, scene_owner, parent_link) in &mut buttons {
        let button_scene_owner = effective_scene_owner(scene_owner, parent_link, &scene_owners);
        if should_skip_scene_stack_menu_input(
            &settings,
            scene_stack.as_deref(),
            button_scene_owner.as_ref(),
        ) {
            continue;
        }
        background.0 = match *interaction {
            Interaction::Pressed => {
                // Button actions are emitted through messages so scene-stack mutation stays centralized.
                perform_menu_action(
                    button,
                    &mut scene_commands,
                    &mut exit_requested,
                    &mut pause_state,
                );
                PRESSED_BUTTON
            }
            Interaction::Hovered => HOVERED_BUTTON,
            Interaction::None => NORMAL_BUTTON,
        };
    }
}

fn perform_menu_action(
    button: &FoundationMenuButton,
    scene_commands: &mut MessageWriter<SceneCommand>,
    exit_requested: &mut MessageWriter<FoundationExitRequested>,
    pause_state: &mut FoundationPauseState,
) {
    match button.action.trim().to_ascii_lowercase().as_str() {
        "open_scene" => {
            // Full-screen menu actions replace visual focus with the requested scene.
            open_configured_scene(button, scene_commands, false, ScenePresentation::FULLSCREEN)
        }
        "open_overlay_scene" => open_configured_scene(
            button,
            scene_commands,
            false,
            ScenePresentation::INPUT_BLOCKING_OVERLAY,
        ),
        "clear_and_open_scene" => {
            // Clearing the stack always resumes gameplay state before loading the next scene.
            pause_state.paused = false;
            open_configured_scene(button, scene_commands, true, ScenePresentation::FULLSCREEN);
        }
        "close_current" => {
            scene_commands.write(SceneCommand::CloseCurrent);
        }
        "resume" => {
            // Resume closes the pause overlay and re-enables gameplay systems.
            pause_state.paused = false;
            scene_commands.write(SceneCommand::CloseCurrent);
        }
        "exit" => {
            pause_state.paused = false;
            exit_requested.write(FoundationExitRequested);
        }
        "none" | "" => {}
        unknown => warn!("Unknown FoundationMenuButton action `{unknown}`"),
    }
}

fn open_configured_scene(
    button: &FoundationMenuButton,
    scene_commands: &mut MessageWriter<SceneCommand>,
    should_clear_stack: bool,
    presentation: ScenePresentation,
) {
    let scene_path = button.scene_path.trim();
    if scene_path.is_empty() {
        warn!(
            "FoundationMenuButton `{}` action has an empty scene_path",
            button.action
        );
        return;
    }
    let mut options = OpenSceneOptions::default().with_presentation(presentation);
    let scene_key = button.scene_key.trim();
    if !scene_key.is_empty() {
        options = options.with_key(scene_key);
    }
    info!(
        "FoundationMenuButton `{}` opening scene `{scene_path}` (clear_stack={should_clear_stack})",
        button.action
    );
    if should_clear_stack {
        // Clear-and-open prevents previous gameplay/menu scenes from leaking into the new flow.
        scene_commands.write(SceneCommand::ClearAndOpen {
            source: SceneSource::jsn_level(scene_path),
            options,
        });
    } else {
        scene_commands.write(SceneCommand::open_with_options(
            SceneSource::jsn_level(scene_path),
            options,
        ));
    }
}

type OptionsTabInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static FoundationOptionsTabButton,
        &'static mut BackgroundColor,
        Option<&'static SceneOwner>,
        Option<&'static ChildOf>,
    ),
    (Changed<Interaction>, With<Button>),
>;

type GeneratedMenuUiWithoutOwnerQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static ChildOf),
    (With<FoundationGeneratedMenuUi>, Without<SceneOwner>),
>;

type OptionsSettingTextQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Text,
        Option<&'static FoundationOptionsSettingLabel>,
        Option<&'static FoundationOptionsSettingValue>,
        Option<&'static SceneOwner>,
    ),
    Or<(
        With<FoundationOptionsSettingLabel>,
        With<FoundationOptionsSettingValue>,
    )>,
>;

fn update_options_tab_button_interactions(
    settings: Res<FoundationMenuRuntimeSettings>,
    scene_stack: Option<Res<SceneStack>>,
    mut buttons: OptionsTabInteractionQuery,
    mut menus: Query<(&mut FoundationOptionsRuntime, Option<&SceneOwner>)>,
    scene_owners: Query<&SceneOwner>,
    mut setting_texts: OptionsSettingTextQuery,
) {
    for (interaction, tab_button, mut background, scene_owner, parent_link) in &mut buttons {
        let tab_scene_owner = effective_scene_owner(scene_owner, parent_link, &scene_owners);
        if should_skip_scene_stack_menu_input(
            &settings,
            scene_stack.as_deref(),
            tab_scene_owner.as_ref(),
        ) {
            continue;
        }
        let mut selected = false;
        if *interaction == Interaction::Pressed {
            // Updating all menu runtimes keeps generated labels in sync with the active tab.
            for (mut runtime, scene_owner) in &mut menus {
                runtime.active_tab = tab_button.tab;
                selected = true;
                update_setting_texts(tab_button.tab, scene_owner.copied(), &mut setting_texts);
            }
        }

        background.0 = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON,
            Interaction::Hovered => HOVERED_BUTTON,
            Interaction::None if selected => SELECTED_BUTTON,
            Interaction::None => NORMAL_BUTTON,
        };
    }
}

fn update_setting_texts(
    selected_tab_index: usize,
    menu_scene_owner: Option<SceneOwner>,
    setting_texts: &mut OptionsSettingTextQuery,
) {
    let tab_name = OPTIONS_TABS
        .get(selected_tab_index)
        .copied()
        .unwrap_or("Options");
    for (mut text, label, value, text_scene_owner) in setting_texts.iter_mut() {
        // Scene-owner matching prevents one overlay's options text from updating another overlay.
        if !scene_owners_match(menu_scene_owner, text_scene_owner.copied()) {
            continue;
        }

        if let Some(setting_label) = label {
            text.0 = format!("{tab_name} Property {}", setting_label.index);
        } else if let Some(setting_value) = value {
            text.0 = format!("< Value {} >", setting_value.index);
        }
    }
}

fn scene_owners_match(
    expected_scene_owner: Option<SceneOwner>,
    actual_scene_owner: Option<SceneOwner>,
) -> bool {
    match (expected_scene_owner, actual_scene_owner) {
        (Some(expected_scene_owner), Some(actual_scene_owner)) => {
            expected_scene_owner == actual_scene_owner
        }
        (None, None) => true,
        (None, Some(_)) => true,
        (Some(_), None) => false,
    }
}

fn inherit_scene_owner_to_generated_menu_ui(
    mut commands: Commands,
    generated_children: GeneratedMenuUiWithoutOwnerQuery,
    scene_owners: Query<&SceneOwner>,
) {
    for (generated_entity, child_link) in &generated_children {
        if let Ok(scene_owner) = scene_owners.get(child_link.0) {
            commands.entity(generated_entity).insert(*scene_owner);
        }
    }
}

fn close_on_escape(
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<FoundationMenuRuntimeSettings>,
    scene_stack: Option<Res<SceneStack>>,
    close_markers: Query<(Option<&SceneOwner>, Option<&ChildOf>), With<FoundationCloseOnEscape>>,
    scene_owners: Query<&SceneOwner>,
    mut scene_commands: MessageWriter<SceneCommand>,
) {
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }

    let has_close_marker = close_markers.iter().any(|(scene_owner, parent_link)| {
        let close_marker_scene_owner =
            effective_scene_owner(scene_owner, parent_link, &scene_owners);
        !should_skip_scene_stack_menu_input(
            &settings,
            scene_stack.as_deref(),
            close_marker_scene_owner.as_ref(),
        )
    });
    if has_close_marker {
        scene_commands.write(SceneCommand::CloseCurrent);
    }
}

fn should_skip_menu_runtime_entity(
    settings: &FoundationMenuRuntimeSettings,
    scene_owner: Option<&SceneOwner>,
) -> bool {
    settings.require_scene_owner && scene_owner.is_none()
}

fn should_skip_scene_stack_menu_input(
    settings: &FoundationMenuRuntimeSettings,
    scene_stack: Option<&SceneStack>,
    scene_owner: Option<&SceneOwner>,
) -> bool {
    if should_skip_menu_runtime_entity(settings, scene_owner) {
        return true;
    }

    let Some(scene_owner) = scene_owner else {
        return false;
    };

    let Some(scene_stack) = scene_stack else {
        return false;
    };

    // Scene-stack presentation flags are the runtime source of truth for covered menus.
    scene_stack
        .get(scene_owner.scene_id)
        .is_none_or(|scene_entry| !scene_entry.flags.interactive)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_button_constructors_set_expected_actions() {
        assert_eq!(FoundationMenuButton::none().action, "none");
        let open = FoundationMenuButton::open_scene("options_menu.jsn", "options-menu");
        assert_eq!(open.action, "open_scene");
        assert_eq!(open.scene_path, "options_menu.jsn");
        assert_eq!(open.scene_key, "options-menu");
        let overlay = FoundationMenuButton::open_overlay_scene("options_menu.jsn", "options-menu");
        assert_eq!(overlay.action, "open_overlay_scene");
        assert_eq!(overlay.scene_path, "options_menu.jsn");
        assert_eq!(overlay.scene_key, "options-menu");
        let clear =
            FoundationMenuButton::clear_and_open_scene("gameplay_level.jsn", "gameplay-level");
        assert_eq!(clear.action, "clear_and_open_scene");
        assert_eq!(clear.scene_path, "gameplay_level.jsn");
        assert_eq!(clear.scene_key, "gameplay-level");
        assert_eq!(
            FoundationMenuButton::close_current().action,
            "close_current"
        );
        assert_eq!(FoundationMenuButton::resume().action, "resume");
        assert_eq!(FoundationMenuButton::exit().action, "exit");
    }

    #[test]
    fn pause_state_defaults_to_unpaused() {
        assert!(!FoundationPauseState::default().paused);
    }

    #[test]
    fn simple_gameplay_level_defaults_to_two_unit_cube() {
        assert_eq!(FoundationSimpleGameplayLevel::default().cube_size, 2.0);
    }

    #[test]
    fn foundation_spin_defaults_to_one_radian_per_second() {
        assert_eq!(FoundationSpin::default().radians_per_second, 1.0);
    }

    #[test]
    fn scene_stack_menu_input_skips_covered_scene_buttons() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(crate::scene_stack::FoundationSceneStackPlugin);
        app.world_mut()
            .write_message(SceneCommand::open(SceneSource::runtime("main-menu")));
        app.world_mut()
            .write_message(SceneCommand::open(SceneSource::runtime("credits")));
        app.update();

        let scene_stack = app.world().resource::<SceneStack>();
        let settings = FoundationMenuRuntimeSettings::default();
        let covered_menu_owner = SceneOwner {
            scene_id: crate::scene_stack::SceneId(1),
        };
        let focused_credits_owner = SceneOwner {
            scene_id: crate::scene_stack::SceneId(2),
        };

        assert!(should_skip_scene_stack_menu_input(
            &settings,
            Some(scene_stack),
            Some(&covered_menu_owner),
        ));
        assert!(!should_skip_scene_stack_menu_input(
            &settings,
            Some(scene_stack),
            Some(&focused_credits_owner),
        ));
    }

    #[test]
    fn options_tabs_match_requested_order() {
        assert_eq!(
            OPTIONS_TABS,
            ["Gameplay", "Display", "Graphics", "Accessibility"]
        );
    }
}
