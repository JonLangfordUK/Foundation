//! Command-line entry point for Foundation game builds and packages.

use std::process::ExitCode;

fn main() -> ExitCode {
    match foundation_build::run(std::env::args().skip(1)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error_message) => {
            eprintln!("{error_message}");
            ExitCode::FAILURE
        }
    }
}
