# PiGame

PiGame is a multi-project Rust repository for Jackdaw Editor, reusable FoundationRuntimeLibrary code, and Jackdaw-style games.

## Repository layout
- `Cargo.toml` - root workspace manifest for tooling/editor crates
- `crates/jackdaw-editor` - Jackdaw Editor, a Jackdaw launcher/editor subproject
- `crates/foundation-runtime-library` - FoundationRuntimeLibrary, reusable shared Bevy/Jackdaw-compatible game code
- `games/template-game` - a Jackdaw static-game subproject shaped like Jackdaw's generated game template
- `AGENTS.md` - project instructions for Pi
- `.pi/skills/` - reusable skills for Rust work, feature planning, tracker updates, review handoff, and Git workflow
- `.pi/prompts/` - prompt templates for planning, implementation, review, and validation
- `docs/plans/` - feature plans, trackers, and templates
- `scripts/` - Windows wrappers for root Cargo validation commands and optional feature-plan scaffolding

The current architecture is **Editor / Game / Library**:

- **Editor**: `crates/jackdaw-editor` launches Jackdaw.
- **Game**: `games/template-game` is a concrete Jackdaw-style game project.
- **Library**: `crates/foundation-runtime-library` contains reusable code shared by games and their game-specific editor binaries. It depends on `jackdaw_runtime` so shared components can expose Jackdaw-compatible editor metadata without depending on the full Jackdaw editor app.

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

### FoundationRuntimeLibrary validation
From the repository root:

```cmd
cargo test -p foundation-runtime-library
cargo doc -p foundation-runtime-library --no-deps
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
