//! Foundation engine launcher.
//!
//! Foundation discovers game extensions from game manifests and launches the
//! selected game by name. The engine launcher intentionally does not depend on
//! concrete game crates.

mod launch;
mod manifest;

use std::process::ExitCode;

use launch::{launch_selected_game, FoundationLaunchArguments, FoundationLaunchCommand};

const USAGE: &str = "Usage: cargo run -p foundation -- --game <game-name> [--editor]";

fn main() -> ExitCode {
    let interrupt_exit_code = 130;
    let _ = ctrlc::set_handler(move || std::process::exit(interrupt_exit_code));

    let launch_arguments = match FoundationLaunchArguments::parse(std::env::args().skip(1)) {
        Ok(FoundationLaunchCommand::Launch(launch_arguments)) => launch_arguments,
        Ok(FoundationLaunchCommand::ShowHelp) => {
            println!("Foundation engine launcher.");
            println!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        Err(argument_error) => {
            eprintln!("{argument_error}");
            eprintln!("{USAGE}");
            return ExitCode::FAILURE;
        }
    };

    match launch_selected_game(&launch_arguments) {
        Ok(game_exit_code) => game_exit_code,
        Err(launch_error) => {
            eprintln!("{launch_error}");
            ExitCode::FAILURE
        }
    }
}
