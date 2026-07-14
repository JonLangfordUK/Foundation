//! Standalone development launcher for template-game.
//!
//! Distributed and editor-mode launches should use the Foundation engine crate:
//! `cargo run -p foundation -- --game template-game`.

use bevy::{asset::AssetPlugin, prelude::*};
use foundation_editor_library::prelude::*;
use foundation_runtime_library::prelude::*;

fn main() -> AppExit {
    let interrupt_exit_code = 130;
    let _ = ctrlc::set_handler(move || std::process::exit(interrupt_exit_code));

    let asset_directory_name = "assets";
    let asset_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(asset_directory_name)
        .to_string_lossy()
        .to_string();

    let editor_enabled = std::env::args().any(|argument| argument == "--editor");

    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK))
        .set_error_handler(bevy::ecs::error::error)
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: asset_root,
            ..default()
        }))
        .add_plugins(FoundationPlugin)
        .add_plugins(template_game::TemplateGamePlugin)
        .add_systems(Startup, spawn_default_camera);

    if editor_enabled {
        app.add_plugins(FoundationEditorPlugin);
        app.insert_resource(FoundationEditorMode { enabled: true });
        info!("Foundation editor mode enabled for TemplateGame.");
    }

    app.run()
}

fn spawn_default_camera(mut commands: Commands) {
    let camera_order = 100;
    commands.spawn((
        Camera2d,
        Camera {
            order: camera_order,
            ..default()
        },
    ));
}
