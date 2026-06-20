//! Standalone game binary for TemplateGame.

use bevy::{
    image::{ImageAddressMode, ImagePlugin, ImageSamplerDescriptor},
    prelude::*,
};
use foundation_library::prelude::*;
use jackdaw_runtime::prelude::*;

fn main() -> AppExit {
    let _ = ctrlc::set_handler(|| std::process::exit(130));

    App::new()
        // Debug fallback clear color: should usually be black, but blue makes
        // any uncovered frame obvious while tuning splash transitions.
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 1.0)))
        .set_error_handler(bevy::ecs::error::error)
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                address_mode_w: ImageAddressMode::Repeat,
                ..ImageSamplerDescriptor::linear()
            },
        }))
        .add_plugins(JackdawPlugin)
        .add_plugins(FoundationPlugin)
        .add_plugins(template_game::TemplateGamePlugin)
        .add_systems(Startup, spawn_default_camera)
        .run()
}

fn spawn_default_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
