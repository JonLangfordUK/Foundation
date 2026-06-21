//! Editor + game binary for TemplateGame, built only with the `editor` feature.

use std::path::PathBuf;

use bevy::{
    asset::AssetPlugin,
    image::{ImageAddressMode, ImagePlugin, ImageSamplerDescriptor},
    prelude::*,
};
use foundation_library::prelude::*;
use jackdaw::prelude::*;
use jackdaw::project_select::PendingAutoOpen;

fn main() -> AppExit {
    let _ = ctrlc::set_handler(|| std::process::exit(130));

    let project_root = std::env::var_os("JACKDAW_PROJECT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));

    if let Err(error) = std::env::set_current_dir(&project_root) {
        error!(
            "Failed to set TemplateGame editor working directory to {}: {error}",
            project_root.display()
        );
    }

    let asset_root = project_root.join("assets").to_string_lossy().to_string();

    let mut app = App::new();
    app.set_error_handler(bevy::ecs::error::error)
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
        .add_plugins((PhysicsPlugins::default(), EnhancedInputPlugin))
        .add_plugins(EditorPlugins::default())
        .add_plugins(FoundationPlugin)
        .add_plugins(template_game::TemplateGamePlugin);

    if project_root.is_dir() {
        app.insert_resource(PendingAutoOpen {
            path: project_root,
            skip_build: true,
        });
    }

    app.run()
}
