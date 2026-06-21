use bevy::{
    asset::AssetPlugin,
    ecs::error::ErrorContext,
    image::{ImageAddressMode, ImagePlugin, ImageSamplerDescriptor},
    prelude::*,
    window::{ExitCondition, WindowPlugin},
};
use jackdaw::prelude::*;

const JACKDAW_EDITOR_TITLE: &str = "Jackdaw Editor";

fn main() -> AppExit {
    let _ = ctrlc::set_handler(|| {
        error!("Jackdaw Editor: received Ctrl+C, exiting");
        std::process::exit(130);
    });

    let project_root = jackdaw::project::read_last_project()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    let respawn_skip_build = std::env::var_os(jackdaw::restart::ENV_SKIP_INITIAL_BUILD).is_some();
    let auto_open_opt_in = std::env::var_os("JACKDAW_AUTO_OPEN").is_some();
    let auto_open = if respawn_skip_build {
        jackdaw::project::read_last_project().map(|path| jackdaw::project_select::PendingAutoOpen {
            path,
            skip_build: true,
        })
    } else if auto_open_opt_in {
        jackdaw::project::read_last_project()
            .filter(|p| p.is_dir() && p.join("Cargo.toml").is_file())
            .map(|path| jackdaw::project_select::PendingAutoOpen {
                path,
                skip_build: false,
            })
    } else {
        None
    };

    let mut app = App::new();
    app.set_error_handler(error_handler)
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: project_root.join("assets").to_string_lossy().to_string(),
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        address_mode_w: ImageAddressMode::Repeat,
                        ..ImageSamplerDescriptor::linear()
                    },
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: JACKDAW_EDITOR_TITLE.into(),
                        ..default()
                    }),
                    exit_condition: ExitCondition::DontExit,
                    close_when_requested: false,
                    ..default()
                }),
        )
        .add_plugins((PhysicsPlugins::default(), EnhancedInputPlugin))
        .add_plugins(editor_plugins)
        .add_systems(OnEnter(jackdaw::AppState::Editor), spawn_scene);

    if let Some(pending) = auto_open {
        app.insert_resource(pending);
    }

    app.run()
}

fn editor_plugins(app: &mut App) {
    app.add_plugins(EditorPlugins::default());
    app.add_plugins(DylibLoaderPlugin::default());
}

fn spawn_scene(mut commands: Commands) {
    commands.queue(|world: &mut World| {
        jackdaw::scene_io::spawn_default_lighting(world);
    });
}

#[track_caller]
#[inline]
fn error_handler(error: BevyError, ctx: ErrorContext) {
    let msg = format!("{error}");
    if msg.contains(
        "Note that interacting with a despawned entity is the most common cause of this error but there are others",
    ) {
        bevy::ecs::error::debug(error, ctx);
        return;
    }
    bevy::ecs::error::error(error, ctx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_title_is_jackdaw_editor() {
        assert_eq!(JACKDAW_EDITOR_TITLE, "Jackdaw Editor");
    }
}
