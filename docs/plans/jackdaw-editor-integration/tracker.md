# Jackdaw Editor Integration Tracker

## Metadata
- Feature slug: `jackdaw-editor-integration`
- Feature area: `editor`
- Primary area: `editor`
- Branch: `feature/jackdaw-editor-integration`
- Overall status: `Root workspace TemplateGame commands awaiting commit/push`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Root workspace package commands implemented and validated with gpt-5.4`
- Created: `2026-06-19`
- Last updated: `2026-06-19`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- Push after every commit and merge checkpoint when `origin` is configured. If push fails, record the failure and do not treat the checkpoint as complete until remediated.

## Branch And Push State
- Active planning branch: `feature/jackdaw-editor-integration`
- Branch creation: Created locally from `dev` on 2026-06-19 during planning.
- Branch-base verification: `git merge-base --is-ancestor dev HEAD` passed before implementation edits.
- Remote: `origin` is configured as `https://github.com/JonLangfordUK/Foundation.git`.
- Push status: Planning commit `ae3fcb3`, implementation commit `dc0828f`, tracker checkpoint `1d19323`, corrective refactor commit `f6945e3`, Jackdaw Editor rename commit `bf89b5e`, cleanup commit `0f062b7`, and root alias commit `64f0716` pushed to `origin/feature/jackdaw-editor-integration`; workspace command replacement commit pending.

## Phase 1: Rename Editor Subproject To Jackdaw Editor
**Status:** Complete  
**Goal:** Rename the current editor subproject and user-facing editor identity to Jackdaw Editor while preserving Cargo compatibility.

### Tasks
- [x] Rename the editor workspace member path from `crates/editor` to a Jackdaw Editor path.
  - Status: Complete
  - Notes: Renamed the workspace member to `crates/jackdaw-editor`.
- [x] Rename the editor package from `pigame-editor` to `jackdaw-editor`.
  - Status: Complete
  - Notes: Used Cargo-compatible lowercase package name `jackdaw-editor`; user-facing window/product title is `Jackdaw Editor`.
- [x] Update workspace membership and dependencies for the renamed editor package.
  - Status: Complete
  - Notes: Root `Cargo.toml` now lists `crates/jackdaw-editor` and adds workspace dependencies for Jackdaw integration.
- [x] Update README and run commands for the new editor name.
  - Status: Complete
  - Notes: README now documents `cargo run -p jackdaw-editor` and describes Jackdaw Editor as the Jackdaw editor host.

### Validation
- Format: Passed (`cargo fmt --all -- --check`; also via full PowerShell validation wrapper)
- Lint: Passed (`cargo clippy --workspace --all-targets --all-features -- -D warnings`; also via full PowerShell validation wrapper)
- Tests: Passed (`cargo test --workspace --all-features`; 4 unit tests passed plus doc-tests)
- Build: Passed (`cargo build --workspace --all-features`; also via full PowerShell validation wrapper)
- Documentation generation: Passed (`cargo doc --workspace --all-features --no-deps`; also via full PowerShell validation wrapper)
- Full validation wrapper: Passed via `powershell -ExecutionPolicy Bypass -File scripts/Invoke-RustWorkspace.ps1 validate-project`
- User confirmation: Received approval to implement on 2026-06-19 (`Looks good to me, commit the feature planning docs first, then implement`).

### Notes
- Cargo package/executable naming does not use `Jackdaw Editor` capitalization; `Jackdaw Editor` is used as the user-facing window/product title and `jackdaw-editor` as the package/run target.

## Phase 2: Jackdaw Editor Host Integration
**Status:** Cleanup awaiting commit/push  
**Goal:** Replace the plain Bevy editor window with a Jackdaw-backed editor host.

### Tasks
- [x] Add Jackdaw dependencies to the editor crate/workspace.
  - Status: Complete
  - Notes: Added `jackdaw = { version = "0.4.1", default-features = false }` and `ctrlc = "3"` as workspace dependencies; `jackdaw-editor` depends on `bevy`, `ctrlc`, and `jackdaw`.
- [x] Adjust Bevy feature configuration if needed for Jackdaw.
  - Status: Complete
  - Notes: Updated workspace Bevy dependency to enable `file_watcher`, `reflect_documentation`, `serialize`, and `experimental_bevy_feathers`, matching Jackdaw integration needs.
- [x] Add a minimal game plugin if needed for editor hosting.
  - Status: Superseded by corrected architecture
  - Notes: Root `crates/game` was removed. Game-specific behavior now lives in the nested Jackdaw-style `games/template-game` project, primarily `games/template-game/src/lib.rs` as `TemplateGamePlugin`.
- [x] Implement Jackdaw Editor app startup with Jackdaw plugins.
  - Status: Complete after corrective refactor
  - Notes: `jackdaw-editor` is now a Jackdaw launcher/editor subproject, not a game-host-specific static editor. It uses Jackdaw launcher-style startup with `EditorPlugins::default()` and `DylibLoaderPlugin::default()`. Game-specific static editors live inside game subprojects.
- [x] Preserve the game launcher.
  - Status: Superseded by corrected architecture
  - Notes: The root `pigame-game` launcher was removed. `TemplateGame` is now launched from `games/template-game` with `cargo run` or `cargo play`, matching Jackdaw's generated project shape.
- [x] Add non-interactive tests where practical.
  - Status: Complete
  - Notes: Existing title/config tests were preserved and updated for `Jackdaw Editor`; tests do not open a GPU window.

### Validation
- Format: Passed (`cargo fmt --all -- --check`; also via full PowerShell validation wrapper)
- Lint: Passed (`cargo clippy --workspace --all-targets --all-features -- -D warnings`)
- Tests: Passed (`cargo test --workspace --all-features`; 4 unit tests passed plus doc-tests)
- Build: Passed (`cargo build --workspace --all-features`)
- Documentation generation: Passed (`cargo doc --workspace --all-features --no-deps`)
- Manual game window check: Superseded by corrected architecture; TemplateGame should be launched from `games/template-game` with `cargo run` or `cargo play`.
- Manual Jackdaw Editor Jackdaw launch check: Passed by running `cargo run -p jackdaw-editor`; Bevy logged `Creating new window Jackdaw Editor (0v0)` and Jackdaw logged loading built-in extensions such as `jackdaw.asset_browser`, `jackdaw.core`, `jackdaw.inspector`, and `jackdaw.viewport_panel`. Command was allowed to time out because the editor remains open.
- Full validation wrapper: Passed via `powershell -ExecutionPolicy Bypass -File scripts/Invoke-RustWorkspace.ps1 validate-project`
- User confirmation: Not required until implementation review.

### Notes
- Added an empty tracked `assets/.gitkeep` directory for `crates/jackdaw-editor`. The old root `crates/game` project and its assets directory were removed during the corrected architecture refactor.
- Jackdaw Editor uses static Jackdaw `EditorPlugins::default()` and does not enable Jackdaw dylib loading.
- The editor uses `ctrlc::set_handler`; Bevy logs that it skips installing its own handler because one already exists. This matches the Jackdaw migration advice to install Ctrl+C handling before wgpu/gilrs.

## Phase 3: Documentation, Validation, And Checkpoints
**Status:** Complete  
**Goal:** Document the Jackdaw/Jackdaw Editor workflow and complete validation/commit/push checkpoints.

### Tasks
- [x] Update README with Jackdaw Editor and Jackdaw usage.
  - Status: Complete
  - Notes: README documents `cargo run -p jackdaw-editor` and states Jackdaw Editor uses Jackdaw.
- [x] Run required validation wrappers.
  - Status: Complete
  - Notes: Full validation passed via the PowerShell wrapper.
- [x] Generate documentation.
  - Status: Complete
  - Notes: `cargo doc --workspace --all-features --no-deps` passed and generated documentation under `target/doc/`.
- [x] Commit completed tasks/phases and push to `origin`.
  - Status: Complete
  - Notes: Implementation commit `dc0828f` was pushed to `origin/feature/jackdaw-editor-integration`; this tracker update records the checkpoint.

### Validation
- Format: Passed
- Lint: Passed
- Tests: Passed
- Build: Passed
- Documentation generation: Passed
- Full validation wrapper: Passed
- Push state: Implementation commit `dc0828f` pushed; final tracker push-state update pending.
- User confirmation: Pending final user review.

### Notes
- Phase completion requires validation evidence or documented user-approved waivers.

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Active branch confirmed as `feature/jackdaw-editor-integration`.
- `dev` ancestry was verified before implementation edits.
- User-facing editor name: `Jackdaw Editor`.
- Cargo package/run command: `jackdaw-editor` / `cargo run -p jackdaw-editor`.
- Jackdaw integration is static/minimal and does not enable Jackdaw `dylib`.
- Awaiting cleanup validation and commit/push checkpoint, then ready for optional `gpt-5.5` sanity review or final user review.

## Postponed Work
- Full custom game content beyond the generated TemplateGame-style `SpinningCube` example is postponed.
- Advanced custom components, editor panels, and Jackdaw Editor-specific branding assets/icons are postponed unless needed for the initial Jackdaw host.

## Progress Log
- `2026-06-19`: Read required feature planning, Rust workspace, and Gitflow skills.
- `2026-06-19`: Inspected root workspace manifest, current editor manifest/source, engine launcher code, game crate, README, Git branch state, and remotes.
- `2026-06-19`: Created branch `feature/jackdaw-editor-integration` from local `dev` for feature planning.
- `2026-06-19`: Researched Jackdaw 0.4.1 README, docs, migration guidance, custom component guidance, configuration docs, and source exports at commit `5e20671dbf6851180b9e032ee95039cdc1adab4d`.
- `2026-06-19`: Plan and tracker created; awaiting user review/approval before implementation.
- `2026-06-19`: User approved the plan, planning commit `ae3fcb3` was created and pushed to `origin/feature/jackdaw-editor-integration`, and implementation started with gpt-5.4.
- `2026-06-19`: Verified `dev` is an ancestor of `feature/jackdaw-editor-integration` before implementation edits.
- `2026-06-19`: Renamed editor crate to `crates/jackdaw-editor` / `jackdaw-editor`, added Jackdaw dependencies, added `PiGamePlugin`, implemented Jackdaw Editor startup, and updated README.
- `2026-06-19`: Validation passed: format, clippy, tests, build, docs, and full PowerShell validation wrapper.
- `2026-06-19`: Manual launch checks confirmed `PiGame` and `Jackdaw Editor` windows are created; Jackdaw Editor also loaded Jackdaw built-in extensions.
- `2026-06-19`: Implementation commit `dc0828f` was created and pushed to `origin/feature/jackdaw-editor-integration`.
- `2026-06-19`: User reported the prior architecture still did not match Jackdaw's generated game flow. Inspected `E:\GameDev\test\my_game` and confirmed Jackdaw static games are self-contained packages with their own `src/lib.rs`, `src/main.rs`, `src/bin/editor.rs`, `.cargo/config.toml`, `.jsn/project.jsn`, `assets/scene.jsn`, and `jackdaw.toml`.
- `2026-06-19`: User clarified the intended repo architecture: a Jackdaw/Jackdaw Editor subproject crate plus a TemplateGame subproject set up like Jackdaw's generated static game. Corrected the implementation accordingly: root workspace now contains `crates/jackdaw-editor`; `games/template-game` is a nested independent Cargo project excluded from the root workspace.
- `2026-06-19`: Root Jackdaw Editor validation passed: format, clippy, tests, build, and docs. TemplateGame validation passed format and clippy, then test/build/doc validation was blocked by disk space (`os error 112`, no space on device) while compiling dependencies. Removed `games/template-game/target` to recover space; root `target` remains large (~46G).
- `2026-06-19`: User chose to keep the wrapper identity as Jackdaw rather than PillarEditor because it is just Jackdaw functionality. Renamed `crates/pillar-editor` to `crates/jackdaw-editor`, package `pillar-editor` to `jackdaw-editor`, and window title to `Jackdaw Editor`. Confirmed Jackdaw is supplied through Cargo dependencies, not a cloned Git submodule.
- `2026-06-19`: Root validation after rename passed: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`, and `cargo doc --workspace --all-features --no-deps`.
- `2026-06-19`: Cleaned lingering old references after the Jackdaw Editor rename: restored root workspace member to `crates/jackdaw-editor`, updated TemplateGame source comments to say Jackdaw Editor, removed the untracked `ref/` screenshot directory, and removed stale build artifacts where possible. Root `target/` was reduced from ~46G before cleanup and regenerated during validation.
- `2026-06-19`: Full validation after cleanup passed for root Jackdaw Editor workspace: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`, and `cargo doc --workspace --all-features --no-deps`.
- `2026-06-19`: Full validation after cleanup passed for nested TemplateGame: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-features`, `cargo build --all-features`, and `cargo doc --all-features --no-deps`.
- `2026-06-19`: Added root Cargo aliases in `.cargo/config.toml`: `cargo template-game` runs `games/template-game`, and `cargo template-game-editor` runs the TemplateGame editor binary with the `editor` feature. Updated README command examples.
- `2026-06-19`: Alias validation passed: `cargo template-game --help`, `cargo template-game-editor --help`, `cargo fmt --all -- --check`, and root `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- `2026-06-19`: User preferred package-style root commands instead of aliases. Removed root `.cargo/config.toml`, made `games/template-game` a root workspace member, renamed its package to `template-game`, removed its nested `[workspace]`, moved dev profile settings to the root manifest, and updated README examples for `cargo run -p template-game` and `cargo run -p jackdaw-editor`.
- `2026-06-19`: Root workspace command validation passed: `cargo fmt --all -- --check`, `cargo check -p jackdaw-editor`, `cargo check -p template-game`, `cargo check -p template-game --bin editor --features editor`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
