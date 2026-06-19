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
        .add_systems(Startup, (spawn_initial_scene, spawn_default_camera))
        .run()
}

fn spawn_initial_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(JackdawSceneRoot(asset_server.load("scene.jsn")));
}

fn spawn_default_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
