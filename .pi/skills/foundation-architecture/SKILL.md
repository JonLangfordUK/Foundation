---
name: foundation-architecture
description: Foundation crate boundaries for runtime systems, editor extensions, game integration, settings, and Jackdaw-authored assets.
---

# Foundation Architecture

## Purpose
Use this skill when planning, implementing, or reviewing Foundation runtime/editor library work, TemplateGame integration with Foundation crates, game settings, scene-stack behavior, Jackdaw editor windows, or Jackdaw-authored Foundation components.

## Crate Boundaries
- `crates/foundation-runtime-library` owns reusable runtime/game systems, reflected components, shared settings data, scene-stack APIs, splash/menu runtime behavior, and persistence helpers that standalone games can use.
- `crates/foundation-editor-library` owns reusable Jackdaw editor extensions, dockable editor windows, editor-only UI, editor operators, and tooling that depends on the full `jackdaw` editor crate.
- `games/template-game` owns concrete game assets, concrete startup sequences, game-specific plugin glue, and game-specific editor binary wiring.
- `crates/jackdaw-editor` is a generic Jackdaw launcher/editor shell and should not accumulate game-specific Foundation behavior.

## Dependency Rules
- `foundation-runtime-library` may depend on Bevy, `jackdaw_runtime`, and data/persistence crates needed by standalone runtime code.
- `foundation-runtime-library` must not depend on the full `jackdaw` editor crate unless the user explicitly approves a major architecture change.
- `foundation-editor-library` may depend on `jackdaw`, `jackdaw_api`, `bevy_enhanced_input`, Bevy, and `foundation-runtime-library`.
- Game-specific editor binaries may depend on both Foundation crates, but standalone game binaries should only require `foundation-runtime-library`.

## Settings Ownership
- Shared settings data, defaults, validation, and file persistence belong in `foundation-runtime-library` when standalone games need to read them.
- Settings editing UI, Jackdaw operators, and dockable settings windows belong in `foundation-editor-library`.
- Game crates should define game-specific fallback behavior when a generic Foundation setting is unset.

## Jackdaw Asset Rules
- When reflected component crate paths change, update `.jsn` serialized type paths at the same time.
- Prefer generic Foundation reflected components for reusable behavior.
- Keep concrete game scene paths and authored assets in the game crate.

## Verification
- Check `Cargo.toml` workspace members and package names before making crate-boundary changes.
- Run focused checks for changed Foundation crates and TemplateGame integration before full workspace validation.
- Confirm generated docs include both Foundation crates when public APIs change.
