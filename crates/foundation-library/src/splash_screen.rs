//! Reusable splash-screen primitives for Foundation scene-stack flows.
//!
//! Splash scene data can live in Jackdaw `.jsn` files as reflected
//! [`FoundationSplashScreen`] components. At runtime this module creates the
//! centered UI text, drives a fade-in/hold/fade-out sequence, and emits scene
//! stack commands when the sequence completes.

use bevy::prelude::*;
use jackdaw_runtime::prelude::*;

use crate::scene_stack::{OpenSceneOptions, SceneCommand, ScenePresentation, SceneSource};

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
/// Editor integrations can parent generated splash UI under a viewport UI node
/// so percentage-sized roots are laid out inside the viewport instead of the
/// whole editor window.
#[derive(Clone, Copy, Debug, Resource)]
pub struct FoundationSplashUiParent(pub Entity);

impl Plugin for FoundationSplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FoundationSplashScreen>()
            .register_type::<FoundationSplashTimings>()
            .add_systems(
                Update,
                (initialize_splash_screens, advance_splash_screens).chain(),
            );
    }
}

/// Scene-authored splash-screen configuration.
///
/// Attach this component to an entity in a Jackdaw `.jsn` scene to display
/// centered text, fade it in, hold it, fade it out, and optionally transition to
/// another scene-stack entry.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, @EditorCategory::new("Foundation/Splash"))]
pub struct FoundationSplashScreen {
    /// Text shown in the middle of the screen.
    pub text: String,
    /// Timing values for the splash sequence, in seconds.
    pub timings: FoundationSplashTimings,
    /// Font size used for the generated centered UI text.
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
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
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
        Self::new("Splash")
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
}

#[derive(Component, Debug)]
struct FoundationSplashGeneratedUi;

fn initialize_splash_screens(
    mut commands: Commands,
    splashes: Query<(Entity, &FoundationSplashScreen), Added<FoundationSplashScreen>>,
    ui_target_camera: Option<Res<FoundationSplashUiTargetCamera>>,
    ui_parent: Option<Res<FoundationSplashUiParent>>,
) {
    for (splash_entity, splash) in &splashes {
        let text_entity = commands
            .spawn((
                Text::new(splash.text.clone()),
                TextFont::from_font_size(splash.font_size),
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
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
                FoundationSplashGeneratedUi,
            ))
            .add_child(text_entity)
            .id();

        if let Some(ui_parent) = ui_parent.as_ref() {
            commands.entity(ui_parent.0).add_child(ui_root);
        } else if let Some(ui_target_camera) = ui_target_camera.as_ref() {
            commands
                .entity(ui_root)
                .insert(UiTargetCamera(ui_target_camera.0));
        }

        commands
            .entity(splash_entity)
            .insert(FoundationSplashRuntime {
                phase: SplashPhase::FadeIn,
                phase_elapsed: 0.0,
                ui_root,
                text_entity,
            });
    }
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
            commands.entity(runtime.ui_root).despawn();
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
        let mut splash = FoundationSplashScreen::new("Pixel Perfect");
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
        let mut splash = FoundationSplashScreen::new("Bevy");
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
