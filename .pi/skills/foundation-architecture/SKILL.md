---
name: foundation-architecture
description: Foundation crate boundaries for the Bevy engine launcher, runtime systems, editor-time extension points, game settings, BSN scenes, and TemplateGame/PiGame integration.
---

# Foundation Architecture

## Purpose
Use this skill when planning, implementing, or reviewing Foundation engine work, runtime systems, editor-time extension points, game settings, scene-stack behavior, BSN scene catalogs, reusable game components, or TemplateGame/PiGame integration with Foundation crates.

## Crate Boundaries
- `crates/foundation` owns the Foundation engine executable. It wraps Bevy app construction, parses launch arguments such as `--game PiGame` and `--editor`, installs Foundation runtime/editor plugins, and selects the registered game to run.
- `crates/foundation-runtime-library` owns reusable runtime/game systems, reflected components, shared settings data, scene-stack APIs, splash/menu runtime behavior, credits, and persistence helpers that games can use.
- `crates/foundation-editor-library` owns Bevy-only editor-time extension points for Foundation engine `--editor` mode. It is intentionally cleared for now and must not depend on an external editor.
- `games/template-game` owns PiGame's concrete game plugin, BSN scene functions, scene-key catalog, and game-specific glue.

## Dependency Rules
- The project is Bevy-only. Do not add Jackdaw dependencies.
- `foundation-runtime-library` may depend on Bevy and data/persistence crates needed by standalone runtime code.
- `foundation-editor-library` may depend on Bevy and `foundation-runtime-library`, but should remain a lightweight editor-time shell until a concrete Bevy editor feature is planned.
- `crates/foundation` may depend on both Foundation libraries and statically registered game crates.
- Game crates should depend on `foundation-runtime-library` and expose a plugin/registration surface consumed by the Foundation engine.

## Scene And Game Launch Rules
- Use Bevy 0.19 BSN (`bsn!`) scene functions in Rust for concrete game scenes until first-party `.bsn` asset loading exists.
- Foundation runtime owns the scene stack and lifecycle messages.
- Game crates own scene keys and the catalog that maps those keys to BSN scene content.
- The primary launch shape is `cargo run -p foundation -- --game PiGame`.
- Editor-time mode is a Foundation engine runtime flag: `cargo run -p foundation -- --game PiGame --editor`.
- Keep `--game` meaningful even when only one game is statically registered, so future loose-module game loading can be added.

## Build Mode Direction
- Static bundled registration is the current implementation path for distributed single-executable builds.
- Loose dynamic game modules are a future direction for development and multi-game engine installs.
- Do not hard-code PiGame as the only possible engine target except as a default/registered game entry.

## Settings Ownership
- Shared settings data, defaults, validation, and file persistence belong in `foundation-runtime-library` when runtime games need them.
- Editor-time settings UI belongs in `foundation-editor-library` only after a concrete Bevy editor feature is planned.
- Game crates should define game-specific fallback behavior when a generic Foundation setting is unset.

## Verification
- Check `Cargo.toml` workspace members and package names before making crate-boundary changes.
- Run focused checks for changed Foundation crates and TemplateGame/PiGame integration before full workspace validation.
- Confirm `cargo tree --workspace | rg "jackdaw|jackdaw_runtime|jackdaw_api|jackdaw_jsn"` returns no dependency matches after dependency changes.
- Confirm generated docs include Foundation crates when public APIs change.
