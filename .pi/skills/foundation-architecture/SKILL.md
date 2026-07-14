---
name: foundation-architecture
description: Foundation crate boundaries for the Bevy engine launcher, runtime systems, editor-time extension points, game settings, BSN scenes, and TemplateGame integration.
---

# Foundation Architecture

## Purpose
Use this skill when planning, implementing, or reviewing Foundation engine work, runtime systems, editor-time extension points, game settings, scene-stack behavior, BSN scene catalogs, reusable game components, or TemplateGame integration with Foundation crates.

## Crate Boundaries
- `crates/foundation` owns the Foundation engine executable. It discovers game extension manifests, parses launch arguments such as `--game template-game` and `--editor`, and launches the selected game without depending on concrete game crates.
- `crates/foundation-runtime-library` owns reusable runtime/game systems, reflected components, shared settings data, scene-stack APIs, splash/menu runtime behavior, credits, and persistence helpers that games can use.
- `crates/foundation-editor-library` owns Bevy-only editor-time extension points for Foundation engine `--editor` mode. It is intentionally cleared for now and must not depend on an external editor.
- `games/template-game` owns TemplateGame's concrete game plugin, BSN scene functions, scene-key catalog, and game-specific glue.

## Dependency Rules
- The project is Bevy-only. Do not add Jackdaw dependencies.
- `foundation-runtime-library` may depend on Bevy and data/persistence crates needed by standalone runtime code.
- `foundation-editor-library` may depend on Bevy and `foundation-runtime-library`, but should remain a lightweight editor-time shell until a concrete Bevy editor feature is planned.
- `crates/foundation` must not depend on concrete game crates. Games are extensions discovered through manifests or future module-loading mechanisms.
- Game crates should depend on Foundation libraries as needed and expose a launch manifest consumed by the Foundation engine.

## Scene And Game Launch Rules
- Use Bevy 0.19 BSN (`bsn!`) scene functions in Rust for concrete game scenes until first-party `.bsn` asset loading exists.
- Foundation runtime owns the scene stack and lifecycle messages.
- Game crates own scene keys and the catalog that maps those keys to BSN scene content.
- The primary launch shape is `cargo run -p foundation -- --game template-game`.
- Editor-time mode is a Foundation engine runtime flag: `cargo run -p foundation -- --game template-game --editor`.
- Keep `--game` meaningful even when only one game is statically registered, so future loose-module game loading can be added.

## Build Mode Direction
- Loose manifest-based game launching is the current implementation path for development and multi-game engine installs.
- Static bundled builds are a future distribution mode for single-executable releases.
- Do not hard-code template-game as the only possible engine target except as a default requested game name.

## Settings Ownership
- Shared settings data, defaults, validation, and file persistence belong in `foundation-runtime-library` when runtime games need them.
- Editor-time settings UI belongs in `foundation-editor-library` only after a concrete Bevy editor feature is planned.
- Game crates should define game-specific fallback behavior when a generic Foundation setting is unset.

## Verification
- Check `Cargo.toml` workspace members and package names before making crate-boundary changes.
- Run focused checks for changed Foundation crates and TemplateGame integration before full workspace validation.
- Confirm `cargo tree --workspace | rg "jackdaw|jackdaw_runtime|jackdaw_api|jackdaw_jsn"` returns no dependency matches after dependency changes.
- Confirm generated docs include Foundation crates when public APIs change.
