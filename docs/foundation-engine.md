# Foundation Engine Launcher

## Purpose

Foundation is the Bevy-based engine launcher. It is intentionally game-agnostic:
`crates/foundation` does not depend on game crates and does not hard-code game
names. Games are extensions discovered from manifests.

## Launch Command

Run a game extension by name:

```cmd
cargo run -p foundation -- --game <game-name>
```

Enable the current editor-time shell for that game:

```cmd
cargo run -p foundation -- --game <game-name> --editor
```

For external games, prefer the build workflow documented in [`build-packaging.md`](build-packaging.md). The loose launcher remains available for in-repo manifests when a repository intentionally carries a local game fixture.

## Game Manifests

During in-repo development, Foundation discovers game extensions under `games/*` by
looking for:

```text
games/<game-folder>/foundation.game.toml
```

A manifest declares the user-facing game name and the current loose Cargo
package launch target:

```toml
[game]
name = "template-game"

[launch]
package = "template-game"
```

The engine only reads this data. The selected game process owns its concrete
Bevy plugins, BSN scene catalog, assets, and gameplay systems. TemplateGame now
lives in its own repository as the reference external Foundation game.

## Current Development Mode

The current implementation is a loose Cargo-package launch mode:

1. `foundation` parses `--game <game-name>` and optional `--editor`.
2. `foundation` discovers manifests from `games/*/foundation.game.toml` when local game manifests exist.
3. It selects the matching manifest by `[game].name`.
4. It launches the manifest's `[launch].package` with `cargo run -p <package>`.
5. It forwards `--editor` to the selected game process.

This keeps the engine crate independent from concrete games while preserving a
single stable command shape for users.

## Future Build Modes

Foundation is expected to support additional build modes later:

- **Bundled single executable:** a shipping build links one selected game into a
  Foundation executable so distribution can be one binary.
- **Loose module loading:** a development or multi-game install loads game
  modules/DLLs without recompiling the engine.

The current manifest model is the bridge toward both modes. It gives games a
stable identity now without making the engine depend on a specific game crate.
Packaged non-Cargo launching is intentionally deferred until a dedicated
build/distribution feature defines executable layout, module search paths, and
bundled-game selection.

## Editor Mode

`--editor` is a runtime mode flag. Today it enables the cleared
`FoundationEditorPlugin` shell in the selected game process. Future editor
features should remain in `crates/foundation-editor-library` or game-owned
editor integration, not in concrete-game branches inside `crates/foundation`.

## Runtime Debug Console

Foundation games include a reusable debug console through `FoundationPlugin`.
See [`debug-console.md`](debug-console.md) for command usage, macro examples,
autocomplete behavior, history persistence, and command registration rules.

## Crate Boundaries

- `crates/foundation`: game-agnostic engine launcher and manifest discovery.
- `crates/foundation-runtime-library`: reusable runtime systems such as scene
  stack, splash flow, menu primitives, settings, and credits.
- `crates/foundation-editor-library`: Bevy-only editor-time extension shell.
- External game repositories: concrete games, manifests, assets, plugins, and BSN scene catalogs.
- Optional `games/*` fixtures: local engine-development game manifests when intentionally added.
