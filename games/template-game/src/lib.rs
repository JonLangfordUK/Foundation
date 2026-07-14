//! PiGame gameplay plugin and Foundation engine integration.
//!
//! The Foundation engine launches this crate as a registered game. Concrete BSN
//! scenes live in [`scenes`], while reusable scene-stack, splash, menu, and
//! gameplay systems live in `foundation-runtime-library`.

use std::path::PathBuf;

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

pub mod scenes;

/// Foundation game name used by the engine `--game` argument.
pub const GAME_NAME: &str = "PiGame";

/// Returns PiGame's asset root.
///
/// Foundation uses this when launching PiGame as a statically registered game.
pub fn asset_root() -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets")
}

/// PiGame's Bevy plugin.
#[derive(Default)]
pub struct PiGamePlugin;

impl Plugin for PiGamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SpinningCube>()
            .add_systems(Startup, scenes::open_initial_scene)
            .add_systems(
                Update,
                (
                    scenes::spawn_requested_pigame_scenes,
                    exit_game_on_foundation_exit_request,
                    spin_cube.run_if(foundation_is_not_paused),
                ),
            );
    }
}

/// Backwards-compatible alias while the project still uses the `template-game` package name.
pub type TemplateGamePlugin = PiGamePlugin;

fn exit_game_on_foundation_exit_request(
    mut exit_requests: MessageReader<FoundationExitRequested>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for _exit_request in exit_requests.read() {
        app_exit.write(AppExit::Success);
    }
}

/// Example gameplay component used by PiGame-specific systems.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct SpinningCube;

fn spin_cube(time: Res<Time>, mut spinning_entities: Query<&mut Transform, With<SpinningCube>>) {
    for mut transform in &mut spinning_entities {
        let spin_speed_radians_per_second = 0.8;
        let spin_delta = spin_speed_radians_per_second * time.delta_secs();
        transform.rotate_y(spin_delta);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_name_matches_foundation_launch_argument() {
        assert_eq!(GAME_NAME, "PiGame");
    }
}
