//! Standalone game binary for TemplateGame.

use bevy::{
    asset::AssetPlugin,
    image::{ImageAddressMode, ImagePlugin, ImageSamplerDescriptor},
    prelude::*,
};
use foundation_runtime_library::prelude::*;
use jackdaw_runtime::prelude::*;

fn main() -> AppExit {
    // Mirror standard Ctrl-C termination so scripts can detect interrupted runs.
    let interrupt_exit_code = 130;
    let _ = ctrlc::set_handler(move || std::process::exit(interrupt_exit_code));

    // Standalone runs load Jackdaw scenes from this game's asset directory.
    let asset_directory_name = "assets";
    let asset_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(asset_directory_name)
        .to_string_lossy()
        .to_string();

    let clear_color = Color::srgb(0.0, 0.0, 0.0);
    let project_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let foundation_game_settings = FoundationGameSettings::load_or_create_from_project_root(
        project_root,
    )
    .unwrap_or_else(|error| {
        warn!("Failed to load or create Foundation game settings: {error}");
        FoundationGameSettings::default()
    });

    App::new()
        .insert_resource(ClearColor(clear_color))
        .insert_resource(foundation_game_settings)
        .set_error_handler(bevy::ecs::error::error)
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: asset_root,
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        address_mode_w: ImageAddressMode::Repeat,
                        ..ImageSamplerDescriptor::linear()
                    },
                }),
        )
        .add_plugins(JackdawPlugin)
        .add_plugins(FoundationPlugin)
        .add_plugins(template_game::TemplateGamePlugin)
        .add_systems(Startup, spawn_default_camera)
        .run()
}

fn spawn_default_camera(mut commands: Commands) {
    // Keep this UI camera above default world cameras while preserving scene clears.
    let camera_order = 100;
    let clear_color = ClearColorConfig::None;

    commands.spawn((
        Camera2d,
        Camera {
            order: camera_order,
            clear_color,
            ..default()
        },
    ));
}
