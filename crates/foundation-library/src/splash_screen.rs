//! Reusable splash-screen primitives for Foundation scene-stack flows.
//!
//! Splash scene data can live in Jackdaw `.jsn` files as reflected
//! [`FoundationSplashScreen`] components. At runtime this module creates the
//! authored UI text, drives a fade-in/hold/fade-out sequence, and emits scene
//! stack commands when the sequence completes.

use bevy::prelude::*;
use jackdaw_runtime::prelude::*;

use crate::scene_stack::{
    OpenSceneOptions, SceneCommand, SceneOwner, ScenePresentation, SceneSource,
};

/// Installs reusable Foundation splash-screen types and systems.
#[derive(Default)]
pub struct FoundationSplashScreenPlugin;

/// Optional UI camera target for generated splash UI.
///
/// Editor integrations can insert this resource before gameplay starts so
/// splash UI renders into an editor viewport camera instead of covering the
/// editor window chrome.
#[derive(Clone, Copy, Debug, Resource)]
pub struct FoundationSplashUiTargetCamera(pub Entity);

/// Optional parent entity for generated splash UI roots.
///
/// This is a fallback for integrations that do not provide a UI target camera.
/// If [`FoundationSplashUiTargetCamera`] is present, generated splash UI remains
/// a root UI tree and targets that camera directly because Bevy only honors
/// [`UiTargetCamera`] on root UI nodes.
#[derive(Clone, Copy, Debug, Resource)]
pub struct FoundationSplashUiParent(pub Entity);

/// Marks the authored UI root controlled by a [`FoundationSplashScreen`].
///
/// Add this to a root UI entity in a Jackdaw `.jsn` splash scene when the scene
/// should be visually editable. Runtime systems target this root to the editor
/// viewport or standalone game window and fade the marked text child.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, @EditorCategory::new("Foundation/Splash"))]
pub struct FoundationSplashUiRoot;

/// Marks the authored text entity faded by a [`FoundationSplashScreen`].
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, @EditorCategory::new("Foundation/Splash"))]
pub struct FoundationSplashText;

/// Runtime policy for reusable Foundation splash systems.
///
/// Standalone games use the default policy: splash systems are enabled and any
/// authored [`FoundationSplashScreen`] component may drive runtime UI. Editors
/// should disable this while authoring and enable it only during Play. Editors
/// that keep the authoring scene alive during Play should also require
/// [`SceneOwner`] so systems process only scene-stack runtime copies.
#[derive(Clone, Copy, Debug, Resource)]
pub struct FoundationSplashRuntimeSettings {
    /// Whether splash systems may spawn/update gameplay UI and transitions.
    pub enabled: bool,
    /// Whether splash systems should ignore splash components without
    /// [`SceneOwner`]. This is useful in editors where the authoring scene and
    /// runtime scene-stack copy coexist.
    pub require_scene_owner: bool,
}

impl Default for FoundationSplashRuntimeSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            require_scene_owner: false,
        }
    }
}

impl Plugin for FoundationSplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FoundationSplashRuntimeSettings>()
            .register_type::<FoundationSplashScreen>()
            .register_type::<FoundationSplashTimings>()
            .register_type::<FoundationSplashUiRoot>()
            .register_type::<FoundationSplashText>()
            .add_systems(
                Update,
                (
                    cleanup_disabled_splash_screens,
                    (initialize_splash_screens, advance_splash_screens)
                        .chain()
                        .run_if(splash_runtime_enabled),
                ),
            );
    }
}

/// Scene-authored splash-screen configuration.
///
/// Attach this component to an entity in a Jackdaw `.jsn` scene to fade an
/// authored [`FoundationSplashText`] child, hold it, fade it out, and optionally
/// transition to another scene-stack entry. Visible copy is owned by the Bevy
/// [`Text`] component on the marked text entity, not by this configuration
/// component.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, @EditorCategory::new("Foundation/Splash"))]
pub struct FoundationSplashScreen {
    /// Timing values for the splash sequence, in seconds.
    pub timings: FoundationSplashTimings,
    /// Font size used only for the empty generated fallback text when no
    /// authored [`FoundationSplashText`] exists.
    pub font_size: f32,
    /// Optional Jackdaw `.jsn` scene path to open after fade-out completes.
    ///
    /// Leave empty for a terminal splash that does not transition.
    pub next_scene_path: String,
    /// If true, reset the stack before opening [`next_scene_path`](Self::next_scene_path).
    pub reset_stack_for_next_scene: bool,
    /// If true and `reset_stack_for_next_scene` is false, close the current
    /// splash scene while opening the next one.
    pub replace_current_scene: bool,
}

impl FoundationSplashScreen {
    /// Creates a splash screen with default Foundation timings.
    pub fn new() -> Self {
        Self {
            timings: FoundationSplashTimings::default(),
            font_size: 72.0,
            next_scene_path: String::new(),
            reset_stack_for_next_scene: false,
            replace_current_scene: true,
        }
    }

    /// Returns true when this splash has a configured next scene.
    pub fn has_next_scene(&self) -> bool {
        !self.next_scene_path.trim().is_empty()
    }

    fn completion_command(&self) -> Option<SceneCommand> {
        if !self.has_next_scene() {
            return None;
        }

        let source = SceneSource::jsn_level(self.next_scene_path.trim());
        if self.reset_stack_for_next_scene {
            Some(SceneCommand::ClearAndOpen {
                source,
                options: OpenSceneOptions::default()
                    .with_presentation(ScenePresentation::FULLSCREEN),
            })
        } else {
            let mut options = OpenSceneOptions::default()
                .with_presentation(ScenePresentation::INPUT_BLOCKING_OVERLAY);
            if self.replace_current_scene {
                options = options.close_current();
            }
            Some(SceneCommand::open_with_options(source, options))
        }
    }
}

impl Default for FoundationSplashScreen {
    fn default() -> Self {
        Self::new()
    }
}

/// Adjustable splash sequence timings, expressed in seconds.
#[derive(Clone, Copy, Debug, Reflect)]
pub struct FoundationSplashTimings {
    /// Seconds spent fading from transparent to fully visible.
    pub fade_in_seconds: f32,
    /// Seconds spent fully visible after fade-in.
    pub hold_seconds: f32,
    /// Seconds spent fading from fully visible to transparent.
    pub fade_out_seconds: f32,
}

impl FoundationSplashTimings {
    /// Creates timing values in seconds.
    pub const fn new(fade_in_seconds: f32, hold_seconds: f32, fade_out_seconds: f32) -> Self {
        Self {
            fade_in_seconds,
            hold_seconds,
            fade_out_seconds,
        }
    }
}

impl Default for FoundationSplashTimings {
    fn default() -> Self {
        Self::new(1.5, 2.0, 1.5)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SplashPhase {
    FadeIn,
    Hold,
    FadeOut,
    Complete,
}

#[derive(Component, Debug)]
struct FoundationSplashRuntime {
    phase: SplashPhase,
    phase_elapsed: f32,
    ui_root: Entity,
    text_entity: Entity,
    generated_ui: bool,
}

#[derive(Component, Debug)]
struct FoundationSplashGeneratedUi;

fn splash_runtime_enabled(settings: Res<FoundationSplashRuntimeSettings>) -> bool {
    settings.enabled
}

fn cleanup_disabled_splash_screens(
    mut commands: Commands,
    settings: Res<FoundationSplashRuntimeSettings>,
    runtimes: Query<(Entity, &FoundationSplashRuntime)>,
) {
    if settings.enabled {
        return;
    }

    for (splash_entity, runtime) in &runtimes {
        if runtime.generated_ui {
            commands.entity(runtime.ui_root).despawn();
        }
        commands
            .entity(splash_entity)
            .remove::<FoundationSplashRuntime>();
    }
}

fn initialize_splash_screens(
    mut commands: Commands,
    settings: Res<FoundationSplashRuntimeSettings>,
    splashes: Query<
        (Entity, &FoundationSplashScreen, Option<&SceneOwner>),
        Without<FoundationSplashRuntime>,
    >,
    authored_roots: Query<(Entity, Option<&SceneOwner>), With<FoundationSplashUiRoot>>,
    authored_texts: Query<(Entity, Option<&SceneOwner>), With<FoundationSplashText>>,
    ui_target_camera: Option<Res<FoundationSplashUiTargetCamera>>,
    ui_parent: Option<Res<FoundationSplashUiParent>>,
) {
    for (splash_entity, splash, scene_owner) in &splashes {
        if settings.require_scene_owner && scene_owner.is_none() {
            continue;
        }

        let scene_owner = scene_owner.copied();
        let authored_root = matching_authored_entity(scene_owner, &authored_roots);
        let authored_text = matching_authored_entity(scene_owner, &authored_texts);
        let (ui_root, text_entity, generated_ui) =
            if let (Some(ui_root), Some(text_entity)) = (authored_root, authored_text) {
                (ui_root, text_entity, false)
            } else {
                spawn_generated_splash_ui(
                    &mut commands,
                    splash,
                    scene_owner,
                    ui_target_camera.as_deref(),
                    ui_parent.as_deref(),
                )
            };

        if let Some(ui_target_camera) = ui_target_camera.as_ref() {
            commands
                .entity(ui_root)
                .insert(UiTargetCamera(ui_target_camera.0));
        } else if generated_ui {
            if let Some(ui_parent) = ui_parent.as_ref() {
                safe_add_child(&mut commands, ui_parent.0, ui_root);
            }
        }

        commands
            .entity(splash_entity)
            .insert(FoundationSplashRuntime {
                phase: SplashPhase::FadeIn,
                phase_elapsed: 0.0,
                ui_root,
                text_entity,
                generated_ui,
            });
    }
}

fn matching_authored_entity<F: bevy::ecs::query::QueryFilter>(
    scene_owner: Option<SceneOwner>,
    query: &Query<(Entity, Option<&SceneOwner>), F>,
) -> Option<Entity> {
    query
        .iter()
        .find(|(_, owner)| match (scene_owner, owner.copied()) {
            (Some(expected), Some(actual)) => expected == actual,
            (None, None) => true,
            (None, Some(_)) => true,
            (Some(_), None) => false,
        })
        .map(|(entity, _)| entity)
}

fn spawn_generated_splash_ui(
    commands: &mut Commands,
    splash: &FoundationSplashScreen,
    scene_owner: Option<SceneOwner>,
    ui_target_camera: Option<&FoundationSplashUiTargetCamera>,
    ui_parent: Option<&FoundationSplashUiParent>,
) -> (Entity, Entity, bool) {
    let text_entity = commands
        .spawn((
            Text::new(String::new()),
            TextFont::from_font_size(splash.font_size),
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
            FoundationSplashText,
            FoundationSplashGeneratedUi,
        ))
        .id();

    let ui_root = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                overflow: Overflow::clip(),
                ..default()
            },
            FoundationSplashUiRoot,
            FoundationSplashGeneratedUi,
        ))
        .id();
    safe_add_child(commands, ui_root, text_entity);

    if let Some(scene_owner) = scene_owner {
        commands.entity(text_entity).insert(scene_owner);
        commands.entity(ui_root).insert(scene_owner);
    }

    if let Some(ui_target_camera) = ui_target_camera {
        commands
            .entity(ui_root)
            .insert(UiTargetCamera(ui_target_camera.0));
    } else if let Some(ui_parent) = ui_parent {
        safe_add_child(commands, ui_parent.0, ui_root);
    }

    (ui_root, text_entity, true)
}

fn safe_add_child(commands: &mut Commands, parent: Entity, child: Entity) {
    commands.queue(move |world: &mut World| {
        if world.get_entity(parent).is_err() || world.get_entity(child).is_err() {
            return;
        }

        if let Ok(mut parent_entity) = world.get_entity_mut(parent) {
            parent_entity.add_child(child);
        }
    });
}

fn advance_splash_screens(
    mut commands: Commands,
    time: Res<Time>,
    mut splashes: Query<(&FoundationSplashScreen, &mut FoundationSplashRuntime)>,
    mut text_colors: Query<&mut TextColor>,
    mut scene_commands: MessageWriter<SceneCommand>,
) {
    for (splash, mut runtime) in &mut splashes {
        if runtime.phase == SplashPhase::Complete {
            continue;
        }

        let mut phase = runtime.phase;
        let mut phase_elapsed = runtime.phase_elapsed + time.delta_secs();
        let alpha = advance_phase(&mut phase, &mut phase_elapsed, splash.timings);
        runtime.phase = phase;
        runtime.phase_elapsed = phase_elapsed;

        if let Ok(mut text_color) = text_colors.get_mut(runtime.text_entity) {
            text_color.0 = Color::srgba(1.0, 1.0, 1.0, alpha);
        }

        if runtime.phase == SplashPhase::Complete {
            if runtime.generated_ui {
                commands.entity(runtime.ui_root).despawn();
            }
            if let Some(command) = splash.completion_command() {
                scene_commands.write(command);
            }
        }
    }
}

fn advance_phase(
    phase: &mut SplashPhase,
    elapsed: &mut f32,
    timings: FoundationSplashTimings,
) -> f32 {
    loop {
        match *phase {
            SplashPhase::FadeIn => {
                let duration = timings.fade_in_seconds.max(0.0);
                if duration == 0.0 || *elapsed >= duration {
                    *elapsed -= duration;
                    *phase = SplashPhase::Hold;
                    continue;
                }
                return (*elapsed / duration).clamp(0.0, 1.0);
            }
            SplashPhase::Hold => {
                let duration = timings.hold_seconds.max(0.0);
                if duration == 0.0 || *elapsed >= duration {
                    *elapsed -= duration;
                    *phase = SplashPhase::FadeOut;
                    continue;
                }
                return 1.0;
            }
            SplashPhase::FadeOut => {
                let duration = timings.fade_out_seconds.max(0.0);
                if duration == 0.0 || *elapsed >= duration {
                    *elapsed = 0.0;
                    *phase = SplashPhase::Complete;
                    return 0.0;
                }
                return (1.0 - (*elapsed / duration)).clamp(0.0, 1.0);
            }
            SplashPhase::Complete => return 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_splash_timings_match_requested_flow() {
        let timings = FoundationSplashTimings::default();
        assert_eq!(timings.fade_in_seconds, 1.5);
        assert_eq!(timings.hold_seconds, 2.0);
        assert_eq!(timings.fade_out_seconds, 1.5);
    }

    #[test]
    fn splash_completion_command_can_replace_current_scene() {
        let mut splash = FoundationSplashScreen::new();
        splash.next_scene_path = "splash_bevy.jsn".to_string();

        assert_eq!(
            splash.completion_command(),
            Some(SceneCommand::Open {
                source: SceneSource::jsn_level("splash_bevy.jsn"),
                options: OpenSceneOptions::default()
                    .with_presentation(ScenePresentation::INPUT_BLOCKING_OVERLAY)
                    .close_current(),
            })
        );
    }

    #[test]
    fn splash_completion_command_can_reset_stack_for_next_scene() {
        let mut splash = FoundationSplashScreen::new();
        splash.next_scene_path = "main_menu.jsn".to_string();
        splash.reset_stack_for_next_scene = true;

        assert_eq!(
            splash.completion_command(),
            Some(SceneCommand::ClearAndOpen {
                source: SceneSource::jsn_level("main_menu.jsn"),
                options: OpenSceneOptions::default()
                    .with_presentation(ScenePresentation::FULLSCREEN),
            })
        );
    }

    #[test]
    fn phase_alpha_follows_fade_hold_fade() {
        let timings = FoundationSplashTimings::new(1.5, 2.0, 1.5);
        let mut phase = SplashPhase::FadeIn;
        let mut elapsed = 0.75;
        assert_eq!(advance_phase(&mut phase, &mut elapsed, timings), 0.5);
        assert_eq!(phase, SplashPhase::FadeIn);

        elapsed = 1.5;
        assert_eq!(advance_phase(&mut phase, &mut elapsed, timings), 1.0);
        assert_eq!(phase, SplashPhase::Hold);

        elapsed = 2.0;
        assert_eq!(advance_phase(&mut phase, &mut elapsed, timings), 1.0);
        assert_eq!(phase, SplashPhase::FadeOut);

        elapsed = 0.75;
        assert_eq!(advance_phase(&mut phase, &mut elapsed, timings), 0.5);
        assert_eq!(phase, SplashPhase::FadeOut);

        elapsed = 1.5;
        assert_eq!(advance_phase(&mut phase, &mut elapsed, timings), 0.0);
        assert_eq!(phase, SplashPhase::Complete);
    }
}
