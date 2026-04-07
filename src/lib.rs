//! Blank Rust project template.
//!
//! Replace this module with project-specific code when creating a new Rust project
//! from this template.

/// Returns the template crate name.
pub fn template_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_template_name() {
        assert_eq!(template_name(), "pi-rust-template");
    }
}
