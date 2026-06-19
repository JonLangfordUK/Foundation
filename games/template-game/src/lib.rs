//! TemplateGame gameplay shared between the standalone binary (`cargo run`) and
//! the editor binary (`cargo editor`).
//!
//! Scene content lives in `.jsn` files authored in PillarEditor. Game behavior
//! lives in [`TemplateGamePlugin`].

use bevy::prelude::*;
use jackdaw_runtime::prelude::*;

/// TemplateGame's Bevy plugin.
#[derive(Default)]
pub struct TemplateGamePlugin;

impl Plugin for TemplateGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spin_cube.run_if(play_gate::is_playing));
    }
}

/// Spin-rate in radians per second. Attach in the inspector while authoring.
#[derive(Component, Reflect)]
#[reflect(Component, @EditorCategory::new("Actor"))]
pub struct SpinningCube {
    /// Rotation speed around the Y axis, in radians per second.
    pub speed: f32,
}

fn spin_cube(time: Res<Time>, mut cubes: Query<(&SpinningCube, &mut Transform)>) {
    let dt = time.delta_secs();
    for (cube, mut transform) in &mut cubes {
        transform.rotate_y(cube.speed * dt);
    }
}

/// Bridges Jackdaw Play mode to gameplay without forcing a `jackdaw`
/// dependency in standalone builds.
pub mod play_gate {
    #[cfg(feature = "editor")]
    pub fn is_playing(
        state: bevy::prelude::Res<bevy::state::state::State<jackdaw::prelude::PlayState>>,
    ) -> bool {
        matches!(*state.get(), jackdaw::prelude::PlayState::Playing)
    }

    #[cfg(not(feature = "editor"))]
    pub fn is_playing() -> bool {
        true
    }
}
