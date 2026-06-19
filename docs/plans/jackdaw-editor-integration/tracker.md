# Jackdaw Editor Integration Tracker

## Metadata
- Feature slug: `jackdaw-editor-integration`
- Feature area: `editor`
- Primary area: `editor`
- Branch: `feature/jackdaw-editor-integration`
- Overall status: `Awaiting commit/push checkpoint`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation complete with gpt-5.4; commit/push checkpoint pending`
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
- Push status: Planning commit `ae3fcb3` pushed to `origin/feature/jackdaw-editor-integration`; implementation commit pending.

## Phase 1: Rename Editor Subproject To PillarEditor
**Status:** Complete  
**Goal:** Rename the current editor subproject and user-facing editor identity to PillarEditor while preserving Cargo compatibility.

### Tasks
- [x] Rename the editor workspace member path from `crates/editor` to a PillarEditor path.
  - Status: Complete
  - Notes: Renamed the workspace member to `crates/pillar-editor`.
- [x] Rename the editor package from `pigame-editor` to `pillar-editor`.
  - Status: Complete
  - Notes: Used Cargo-compatible lowercase package name `pillar-editor`; user-facing window/product title is `PillarEditor`.
- [x] Update workspace membership and dependencies for the renamed editor package.
  - Status: Complete
  - Notes: Root `Cargo.toml` now lists `crates/pillar-editor` and adds workspace dependencies for Jackdaw integration.
- [x] Update README and run commands for the new editor name.
  - Status: Complete
  - Notes: README now documents `cargo run -p pillar-editor` and describes PillarEditor as the Jackdaw editor host.

### Validation
- Format: Passed (`cargo fmt --all -- --check`; also via full PowerShell validation wrapper)
- Lint: Passed (`cargo clippy --workspace --all-targets --all-features -- -D warnings`; also via full PowerShell validation wrapper)
- Tests: Passed (`cargo test --workspace --all-features`; 4 unit tests passed plus doc-tests)
- Build: Passed (`cargo build --workspace --all-features`; also via full PowerShell validation wrapper)
- Documentation generation: Passed (`cargo doc --workspace --all-features --no-deps`; also via full PowerShell validation wrapper)
- Full validation wrapper: Passed via `powershell -ExecutionPolicy Bypass -File scripts/Invoke-RustWorkspace.ps1 validate-project`
- User confirmation: Received approval to implement on 2026-06-19 (`Looks good to me, commit the feature planning docs first, then implement`).

### Notes
- Cargo package/executable naming does not use `PillarEditor` capitalization; `PillarEditor` is used as the user-facing window/product title and `pillar-editor` as the package/run target.

## Phase 2: Jackdaw Editor Host Integration
**Status:** Complete  
**Goal:** Replace the plain Bevy editor window with a Jackdaw-backed editor host.

### Tasks
- [x] Add Jackdaw dependencies to the editor crate/workspace.
  - Status: Complete
  - Notes: Added `jackdaw = { version = "0.4.1", default-features = false }` and `ctrlc = "3"` as workspace dependencies; `pillar-editor` depends on `bevy`, `ctrlc`, `jackdaw`, `pigame-engine`, and `pigame-game`.
- [x] Adjust Bevy feature configuration if needed for Jackdaw.
  - Status: Complete
  - Notes: Updated workspace Bevy dependency to enable `file_watcher`, `reflect_documentation`, `serialize`, and `experimental_bevy_feathers`, matching Jackdaw integration needs.
- [x] Add a minimal game plugin if needed for editor hosting.
  - Status: Complete
  - Notes: Added public `PiGamePlugin` in `crates/game/src/lib.rs`. It is currently empty but documented as the place for future gameplay systems/resources/reflected components. Ambient plugins remain in host binaries.
- [x] Implement PillarEditor app startup with Jackdaw plugins.
  - Status: Complete
  - Notes: `pillar-editor` builds an app with the shared PillarEditor window config, then adds `PhysicsPlugins::default()`, `EnhancedInputPlugin`, `EditorPlugins::default()`, and `PiGamePlugin`.
- [x] Preserve the game launcher.
  - Status: Complete
  - Notes: `cargo run -p pigame-game` still creates a `PiGame` Bevy window.
- [x] Add non-interactive tests where practical.
  - Status: Complete
  - Notes: Existing title/config tests were preserved and updated for `PillarEditor`; tests do not open a GPU window.

### Validation
- Format: Passed (`cargo fmt --all -- --check`; also via full PowerShell validation wrapper)
- Lint: Passed (`cargo clippy --workspace --all-targets --all-features -- -D warnings`)
- Tests: Passed (`cargo test --workspace --all-features`; 4 unit tests passed plus doc-tests)
- Build: Passed (`cargo build --workspace --all-features`)
- Documentation generation: Passed (`cargo doc --workspace --all-features --no-deps`)
- Manual game window check: Passed by running `cargo run -p pigame-game`; Bevy logged `Creating new window PiGame (0v0)`. Command was allowed to time out because the app remains open until the window closes.
- Manual PillarEditor Jackdaw launch check: Passed by running `cargo run -p pillar-editor`; Bevy logged `Creating new window PillarEditor (0v0)` and Jackdaw logged loading built-in extensions such as `jackdaw.asset_browser`, `jackdaw.core`, `jackdaw.inspector`, and `jackdaw.viewport_panel`. Command was allowed to time out because the editor remains open.
- Full validation wrapper: Passed via `powershell -ExecutionPolicy Bypass -File scripts/Invoke-RustWorkspace.ps1 validate-project`
- User confirmation: Not required until implementation review.

### Notes
- Added empty tracked `assets/.gitkeep` directories for `crates/game` and `crates/pillar-editor` after manual runs showed Bevy file-watcher warnings for missing package-local assets directories.
- PillarEditor uses static Jackdaw `EditorPlugins::default()` and does not enable Jackdaw dylib loading.
- The editor uses `ctrlc::set_handler`; Bevy logs that it skips installing its own handler because one already exists. This matches the Jackdaw migration advice to install Ctrl+C handling before wgpu/gilrs.

## Phase 3: Documentation, Validation, And Checkpoints
**Status:** Awaiting commit/push checkpoint  
**Goal:** Document the Jackdaw/PillarEditor workflow and complete validation/commit/push checkpoints.

### Tasks
- [x] Update README with PillarEditor and Jackdaw usage.
  - Status: Complete
  - Notes: README documents `cargo run -p pillar-editor` and states PillarEditor uses Jackdaw.
- [x] Run required validation wrappers.
  - Status: Complete
  - Notes: Full validation passed via the PowerShell wrapper.
- [x] Generate documentation.
  - Status: Complete
  - Notes: `cargo doc --workspace --all-features --no-deps` passed and generated documentation under `target/doc/`.
- [ ] Commit completed tasks/phases and push to `origin`.
  - Status: Awaiting commit/push
  - Notes: Implementation changes are ready to commit and push after this tracker update.

### Validation
- Format: Passed
- Lint: Passed
- Tests: Passed
- Build: Passed
- Documentation generation: Passed
- Full validation wrapper: Passed
- Push state: Implementation push pending
- User confirmation: Pending final user review after commit/push checkpoint.

### Notes
- Phase completion requires validation evidence or documented user-approved waivers.

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Active branch confirmed as `feature/jackdaw-editor-integration`.
- `dev` ancestry was verified before implementation edits.
- User-facing editor name: `PillarEditor`.
- Cargo package/run command: `pillar-editor` / `cargo run -p pillar-editor`.
- Jackdaw integration is static/minimal and does not enable Jackdaw `dylib`.
- Ready for optional `gpt-5.5` sanity review after implementation commit/push checkpoint.

## Postponed Work
- Dynamic Jackdaw extension/dylib loading is postponed unless the user explicitly asks for hot-reloadable editor extensions.
- Full game scene runtime integration and authored `assets/scene.jsn` loading are postponed; this feature integrates and opens the Jackdaw editor host without adding initial scene files.
- Advanced custom components, editor panels, and PillarEditor-specific branding assets/icons are postponed unless needed for the initial Jackdaw host.

## Progress Log
- `2026-06-19`: Read required feature planning, Rust workspace, and Gitflow skills.
- `2026-06-19`: Inspected root workspace manifest, current editor manifest/source, engine launcher code, game crate, README, Git branch state, and remotes.
- `2026-06-19`: Created branch `feature/jackdaw-editor-integration` from local `dev` for feature planning.
- `2026-06-19`: Researched Jackdaw 0.4.1 README, docs, migration guidance, custom component guidance, configuration docs, and source exports at commit `5e20671dbf6851180b9e032ee95039cdc1adab4d`.
- `2026-06-19`: Plan and tracker created; awaiting user review/approval before implementation.
- `2026-06-19`: User approved the plan, planning commit `ae3fcb3` was created and pushed to `origin/feature/jackdaw-editor-integration`, and implementation started with gpt-5.4.
- `2026-06-19`: Verified `dev` is an ancestor of `feature/jackdaw-editor-integration` before implementation edits.
- `2026-06-19`: Renamed editor crate to `crates/pillar-editor` / `pillar-editor`, added Jackdaw dependencies, added `PiGamePlugin`, implemented PillarEditor startup, and updated README.
- `2026-06-19`: Validation passed: format, clippy, tests, build, docs, and full PowerShell validation wrapper.
- `2026-06-19`: Manual launch checks confirmed `PiGame` and `PillarEditor` windows are created; PillarEditor also loaded Jackdaw built-in extensions.
