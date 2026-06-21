# PiGame

PiGame is a multi-project Rust repository for Jackdaw Editor, reusable Foundation runtime/editor libraries, and Jackdaw-style games.

## Repository layout
- `Cargo.toml` - root workspace manifest for tooling/editor crates
- `crates/jackdaw-editor` - Jackdaw Editor, a Jackdaw launcher/editor subproject
- `crates/foundation-runtime-library` - FoundationRuntimeLibrary, reusable shared Bevy/Jackdaw-compatible game code
- `crates/foundation-editor-library` - FoundationEditorLibrary, reusable Jackdaw editor extensions and windows
- `games/template-game` - a Jackdaw static-game subproject shaped like Jackdaw's generated game template
- `AGENTS.md` - project instructions for Pi
- `.pi/skills/` - reusable skills for Rust work, feature planning, tracker updates, review handoff, and Git workflow
- `.pi/prompts/` - prompt templates for planning, implementation, review, and validation
- `docs/plans/` - feature plans, trackers, and templates
- `scripts/` - Windows wrappers for root Cargo validation commands and optional feature-plan scaffolding

The current architecture is **Editor / Game / Runtime Library / Editor Library**:

- **Editor**: `crates/jackdaw-editor` launches Jackdaw.
- **Game**: `games/template-game` is a concrete Jackdaw-style game project.
- **Runtime Library**: `crates/foundation-runtime-library` contains reusable runtime/game code shared by games and their game-specific editor binaries. It depends on `jackdaw_runtime` so shared components can expose Jackdaw-compatible editor metadata without depending on the full Jackdaw editor app.
- **Editor Library**: `crates/foundation-editor-library` contains reusable Jackdaw editor extensions, dockable windows, editor operators, and editor-only UI. It may depend on the full `jackdaw` editor crate.

`games/template-game` is a root workspace member so it can be launched from the repository root with `cargo run -p template-game`, while retaining Jackdaw's generated static-game source layout.

## Running Jackdaw Editor
From the repository root:

```cmd
cargo run -p jackdaw-editor
```

Jackdaw Editor can create/open Jackdaw projects. For static game projects, Jackdaw builds the project's own editor binary and hands off to it.

## Running TemplateGame
From the repository root:

```cmd
cargo run -p template-game
```

Open TemplateGame's Jackdaw editor binary from the repository root:

```cmd
cargo run -p template-game --bin editor --features editor
```

From `games/template-game`:

```cmd
cargo run
```

or:

```cmd
cargo play
```

Open TemplateGame's Jackdaw editor binary from `games/template-game`:

```cmd
cargo editor
```

Equivalent explicit command:

```cmd
cargo run --bin editor --features editor
```

## Jackdaw static-game shape
TemplateGame follows Jackdaw's generated static template:

- `src/lib.rs` contains game-specific behavior in `TemplateGamePlugin`
- `src/main.rs` is the standalone game runner
- `src/bin/editor.rs` is the feature-gated editor + game runner
- `assets/scene.jsn` is the authored scene
- `.jsn/project.jsn` is Jackdaw project metadata/layout
- `jackdaw.toml` configures Jackdaw Editor/Jackdaw Play-button run modes
- `.cargo/config.toml` defines `cargo editor` and `cargo play`
- root workspace membership allows `cargo run -p template-game` and `cargo run -p template-game --bin editor --features editor` from the repository root
- `foundation-runtime-library` is added to both the standalone game and game-specific editor binary before `TemplateGamePlugin`
- `foundation-editor-library` is added only to the game-specific editor binary and contributes reusable Jackdaw editor extensions such as the Game Settings window

## FoundationRuntimeLibrary shared components
Reusable components can live in `crates/foundation-runtime-library` when they should be available to multiple games and their Jackdaw editor binaries. Components intended for editor authoring should derive Bevy reflection traits, include Jackdaw editor metadata, and be registered by `FoundationPlugin`.

Example pattern:

```rust
use bevy::prelude::*;
use jackdaw_runtime::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component, @EditorCategory::new("Foundation"))]
pub struct MySharedComponent {
    pub value: f32,
}
```

Register shared types from `FoundationPlugin` so both `cargo run -p template-game` and `cargo run -p template-game --bin editor --features editor` see the same reusable API.

## FoundationEditorLibrary editor tools
`crates/foundation-editor-library` owns editor-only integrations that require Jackdaw's full editor API. TemplateGame registers `FoundationGameSettingsExtension` in its editor binary so Jackdaw exposes a dockable **Game Settings** window.

The Game Settings window edits `foundation.settings.toml` in the game project root. Current settings are:

```toml
startup_map = ""
editor_startup_map = ""
```

An empty value means the game uses its built-in default flow. `startup_map` controls the first scene loaded during normal standalone game startup. `editor_startup_map` controls the scene Jackdaw loads when the game-specific editor opens.

The settings window uses the reusable `FoundationAssetPicker` widget. The picker has a UE-style compact asset field with a preview tile, drop-down/browse area, reset action, and current-scene action. Reuse it from `foundation_editor_library::prelude` for other editor tools, and pass a `FoundationAssetPickerFilter` to restrict choices by file extension or by required text/class content in text assets.

## Setup
Ensure Rust is installed and `cargo`/`rustc` are on `PATH`, then validate:

```cmd
scripts\validate-env.cmd
```

or:

```cmd
npm run validate-env
```

Jackdaw project scaffolding also needs `cargo-generate` available on `PATH`. With the current Rust 1.92 toolchain, install the compatible version with:

```cmd
cargo install cargo-generate --version 0.22.0 --locked
```

## Commands
### Root workspace validation
From the repository root:

```cmd
scripts\format-project.cmd
scripts\lint-project.cmd
scripts\test-project.cmd
scripts\compile-project.cmd
scripts\doc-project.cmd
```

### Foundation library validation
From the repository root:

```cmd
cargo test -p foundation-runtime-library
cargo doc -p foundation-runtime-library --no-deps
cargo test -p foundation-editor-library
cargo doc -p foundation-editor-library --no-deps
```

### TemplateGame validation
From `games/template-game`:

```cmd
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo build --all-features
cargo doc --all-features --no-deps
```

## Pi workflow
Feature workflow enforcement:
- planning a feature must use the `feature-plan-docs` skill and `gpt-5.5`
- implementing a feature must use `feature-plan-docs`, `feature-tracker-update`, and `gpt-5.4`
- optional final review must use `feature-review-handoff`, `feature-tracker-update`, and `gpt-5.5`
- implementation must not begin until `docs/plans/<new-feature>/plan.md` and `docs/plans/<new-feature>/tracker.md` exist and the user has approved moving forward
- work should be on a dedicated `feature/*` branch from `dev`
- every completed task and phase should be committed
- when remote `origin` exists, every commit and merge checkpoint should be pushed to `origin`
- if no `origin` is configured, push status should be recorded as `N/A (local-only repository)`
