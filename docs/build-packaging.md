# Foundation Build And Packaging

Foundation games should be built, run, and packaged through the Foundation build tool instead of the loose development launcher.

Preferred local run command:

```cmd
scripts\foundation-build.cmd run --game template-game --platform windows-x64 --configuration debug --target-kind game-editor
```

Preferred local package command:

```cmd
scripts\foundation-build.cmd package --game template-game --platform windows-x64 --configuration test --target-kind game
```

The older command, `cargo run -p foundation -- --game template-game`, remains useful as loose launcher context, but it is no longer the intended build/package workflow.

## Build Configurations

| Configuration | Meaning | Dev tools | Editor target | Typical use |
| --- | --- | --- | --- | --- |
| `debug` | Full debugging with no optimization-focused profile. | Enabled | Allowed | Local debugging |
| `test` | Optimized development/QA build with diagnostics. | Enabled | Allowed | Internal test builds |
| `shipping` | Store/share build containing only the game and assets. | Disabled | Not allowed | Public distribution |

`shipping` builds pass `--no-default-features` to the game crate and use the `foundation-shipping` Cargo profile. This is the configuration that must exclude console commands, log windows, editor plugins, and other dev-only systems.

## Target Kinds

- `game`: builds the standalone game.
- `game-editor`: builds the game with Foundation editor support. This target kind is rejected for `shipping`.

## Platforms

Initial aliases:

| Alias | Rust target triple |
| --- | --- |
| `windows-x64` | `x86_64-pc-windows-msvc` |
| `linux-x64` | `x86_64-unknown-linux-gnu` |

A Rust target triple can also be passed directly. Cross-compilation support depends on the host toolchain, target toolchain, linker, and native dependencies required by Bevy/wgpu. The build tool validates Foundation's build matrix, but it cannot install platform SDKs for the runner.

## Game Manifest Metadata

Each game declares build/package metadata in `games/<game>/foundation.game.toml`:

```toml
[game]
name = "template-game"

[launch]
package = "template-game"

[package]
executable-name = "template-game"
asset-roots = ["assets"]
```

- `game.name`: Foundation game identifier used by `--game`.
- `launch.package`: Cargo package to build.
- `package.executable-name`: file name used for the packaged executable.
- `package.asset-roots`: game-relative directories copied into the package.

## Local Examples

```cmd
scripts\foundation-build.cmd run --game template-game --platform windows-x64 --configuration debug --target-kind game-editor
scripts\foundation-build.cmd run --game template-game --platform windows-x64 --configuration test --target-kind game
scripts\foundation-build.cmd build --game template-game --platform windows-x64 --configuration debug --target-kind game
scripts\foundation-build.cmd package --game template-game --platform windows-x64 --configuration test --target-kind game-editor
scripts\foundation-build.cmd package --game template-game --platform linux-x64 --configuration shipping --target-kind game
```

Runtime arguments can be passed after `--`:

```cmd
scripts\foundation-build.cmd run --game template-game --platform windows-x64 --configuration debug --target-kind game -- --some-game-argument
```

`game-editor` runs automatically pass `--editor` to the game executable.

Invalid example:

```cmd
scripts\foundation-build.cmd package --game template-game --platform windows-x64 --configuration shipping --target-kind game-editor
```

## Package Layout

Packages are written under:

```text
artifacts/packages/<game>/<platform>/<configuration>/<target-kind>/
```

A package contains:

- the game executable named from `package.executable-name`,
- configured asset roots such as `assets/`,
- `foundation.game.toml`,
- generated `foundation.package.toml`,
- a `.tar.gz` archive next to the package directory.

## CI Usage

GitHub workflows should call the same local command as developers. Self-hosted runners need:

- Rust toolchain installed,
- target triples installed through `rustup target add`,
- platform linkers and SDKs required by the selected targets,
- `tar` available for archive creation.

The workflow in `.github/workflows/foundation-build.yml` provides a starting matrix for validation, packaging, artifact upload, and optional GitHub Release upload.
