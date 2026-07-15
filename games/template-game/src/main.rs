//! Thin binary wrapper for the TemplateGame library.
//!
//! Development should normally launch through Foundation:
//! `cargo run -p foundation -- --game template-game`.

fn main() -> bevy::prelude::AppExit {
    template_game::run()
}
