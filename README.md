# Template Game / Foundation

Template Game is a Bevy 0.19 workspace centered on **Foundation**, a small engine wrapper around Bevy plus reusable Foundation runtime/editor libraries.

## Repository layout
- `crates/foundation` - Foundation engine executable. It parses launch arguments such as `--game template-game` and `--editor`.
- `crates/foundation-runtime-library` - reusable runtime systems: scene stack, splash flow, menu primitives, settings, credits, and gameplay helpers.
- `crates/foundation-editor-library` - Bevy-only editor-time extension point. It is intentionally cleared for now after removing the external editor dependency.
- `games/template-game` - the current registered game, exposed to the Foundation engine as `template-game`.
- `docs/scene-system.md` - Foundation scene-stack and BSN scene catalog guide.
- `docs/plans/` - feature plans, trackers, and templates.
- `scripts/` - validation wrappers.

## Running the game
Run Template Game through the Foundation engine:

```cmd
cargo run -p foundation -- --game template-game
```

Run Template Game with Foundation editor-time mode enabled:

```cmd
cargo run -p foundation -- --game template-game --editor
```

The editor-time crate currently provides only a cleared Bevy-only shell. Future editor tools should attach through `foundation-editor-library` and the engine `--editor` mode.

A direct game launcher remains useful during development:

```cmd
cargo run -p template-game
```

## Scene authoring
Scenes are now code-authored with Bevy 0.19 BSN (`bsn!`) in Rust. Bevy does not currently ship a first-party `.bsn` asset loader, so scene definitions live in Rust scene functions for now.

Foundation owns the scene stack and scene lifecycle. TemplateGame owns concrete BSN scene functions and maps scene keys such as `template-game/main_menu` to spawned content.

TemplateGame scene modules live under:

```text
games/template-game/src/scenes/mod.rs
games/template-game/src/scenes/splash.rs
games/template-game/src/scenes/menu.rs
games/template-game/src/scenes/gameplay.rs
```

Current required flow:

```text
splash_pixel_perfect -> splash_bevy -> main_menu -> gameplay_level
                                      \-> options_menu

gameplay_level -- Escape --> pause_menu
```

## Game linking direction
Foundation is intended to support two game-linking modes:

1. **Bundled/static mode** - selected games are compiled into the Foundation executable for a single-exe distributed build.
2. **Loose module mode** - future development mode where the engine can load/select separately built game modules for debugging or multi-game installs.

The current implementation uses static registration and keeps the registry shape ready for future dynamic module loading.

## Validation
From the repository root:

```cmd
scripts\format-project.cmd
scripts\lint-project.cmd
scripts\test-project.cmd
scripts\compile-project.cmd
scripts\doc-project.cmd
```

Focused checks:

```cmd
cargo check -p foundation
cargo test -p foundation
cargo check -p foundation-runtime-library
cargo test -p foundation-runtime-library
cargo check -p foundation-editor-library
cargo test -p foundation-editor-library
cargo check -p template-game
cargo test -p template-game
```

Confirm the external editor dependency is gone:

```cmd
cargo tree --workspace | rg "jackdaw|jackdaw_runtime|jackdaw_api|jackdaw_jsn"
```
