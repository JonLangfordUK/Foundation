# Workspace App Launchers Tracker

## Metadata
- Feature slug: `workspace-app-launchers`
- Feature area: `multi-area` (`engine`, `game`, `editor`)
- Primary area: `engine`
- Branch: `feature/workspace-app-launchers`
- Overall status: `Implemented`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for final user review or optional gpt-5.5 sanity review`
- Created: `2026-06-19`
- Last updated: `2026-06-19`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- Push after every commit and merge checkpoint when `origin` is configured. If push fails, record the failure and do not treat the checkpoint as complete until remediated.

## Branch And Push State
- Active planning branch: `feature/workspace-app-launchers`
- Branch creation: Created locally from `dev` on 2026-06-19 during planning.
- Branch-base verification: `git merge-base --is-ancestor dev HEAD` passed before implementation edits.
- Remote: `origin` is configured as `https://github.com/JonLangfordUK/Foundation.git`.
- Push status: Plan commit `2edb36a` and implementation commit `28fd9f8` pushed to `origin/feature/workspace-app-launchers`; tracker checkpoint update in progress.
- Pre-existing working tree note: an untracked `NUL` file existed before planning and is not part of this feature unless the user later approves touching it.

## Phase 1: Workspace Structure And Shared Linking
**Status:** Complete  
**Goal:** Convert the blank template into a Cargo workspace with shared code linked by separate game and editor subprojects.

### Tasks
- [x] Convert root `Cargo.toml` from single package to workspace manifest.
  - Status: Complete
  - Notes: Root manifest now defines a virtual workspace with members `crates/engine`, `crates/game`, and `crates/editor`; shared package metadata and workspace dependencies are centralized.
- [x] Add shared engine crate under `crates/`.
  - Status: Complete
  - Notes: Added `pigame-engine` with shared Bevy launcher/window configuration.
- [x] Add game crate under `crates/` linked to the shared crate.
  - Status: Complete
  - Notes: Added `pigame-game`; package exposes game metadata/configuration and a runnable binary using `pigame-engine`.
- [x] Add editor crate under `crates/` linked to the shared crate.
  - Status: Complete
  - Notes: Added `pigame-editor`; package uses `pigame-engine` and links to `pigame-game` for shared game metadata.
- [x] Remove or obsolete root placeholder `src/lib.rs` after workspace conversion.
  - Status: Complete
  - Notes: Removed the root placeholder source because the root is now a virtual workspace manifest.

### Validation
- Format: Passed (`cargo fmt --all -- --check`; also via `powershell -ExecutionPolicy Bypass -File scripts/Invoke-RustWorkspace.ps1 validate-project`)
- Lint: Passed (`cargo clippy --workspace --all-targets --all-features -- -D warnings`; also via full validation wrapper)
- Tests: Passed (`cargo test --workspace --all-features`; 4 unit tests passed plus doc-tests)
- Build: Passed (`cargo build --workspace --all-features`; also via full validation wrapper)
- Documentation generation: Passed (`cargo doc --workspace --all-features --no-deps`; also via full validation wrapper)
- Full validation wrapper: Passed via `powershell -ExecutionPolicy Bypass -File scripts/Invoke-RustWorkspace.ps1 validate-project`
- User confirmation: Received approval to implement on 2026-06-19 (`looks good. Commit the plan, then start implementation`).

### Notes
- This phase proves project structure and crate linking before visual launcher work is considered complete.

## Phase 2: Bevy Game And Editor Launchers
**Status:** Complete  
**Goal:** Add minimal Bevy launchers so the game and editor each open a distinct native window.

### Tasks
- [x] Add Bevy dependency configuration to the workspace.
  - Status: Complete
  - Notes: Added Bevy `0.16.1` as a workspace dependency. Cargo noted `0.18.1` is available, but `0.16.1` was selected to match the researched API and compile cleanly with this implementation.
- [x] Implement shared window/app configuration helpers with Rustdoc for public APIs.
  - Status: Complete
  - Notes: `pigame-engine` exposes `LauncherWindowConfig`, default resolution constants, `add_launcher_plugins`, and `run_launcher` with Rustdoc comments.
- [x] Implement game launcher binary.
  - Status: Complete
  - Notes: `cargo run -p pigame-game` starts a Bevy app and creates a window titled `PiGame`.
- [x] Implement editor launcher binary.
  - Status: Complete
  - Notes: `cargo run -p pigame-editor` starts a Bevy app and creates a window titled `PiGame Editor`.
- [x] Add non-interactive tests for shared configuration.
  - Status: Complete
  - Notes: Added tests for engine defaults, custom resolution data, game title configuration, and editor title configuration. Tests do not require opening GPU windows.

### Validation
- Format: Passed (`cargo fmt --all -- --check`; also via full validation wrapper)
- Lint: Passed after fixing a dead-code warning in the editor launcher (`cargo clippy --workspace --all-targets --all-features -- -D warnings`)
- Tests: Passed (`cargo test --workspace --all-features`; 4 unit tests passed plus doc-tests)
- Build: Passed (`cargo build --workspace --all-features`)
- Documentation generation: Passed (`cargo doc --workspace --all-features --no-deps`)
- Manual game window check: Passed by running `cargo run -p pigame-game`; Bevy logged `Creating new window PiGame (0v1)`. Command was allowed to time out after 30 seconds because the app remains open until the window closes.
- Manual editor window check: Passed by running `cargo run -p pigame-editor`; Bevy logged `Creating new window PiGame Editor (0v1)`. Command was allowed to time out after 30 seconds because the app remains open until the window closes.
- Full validation wrapper: Passed via `powershell -ExecutionPolicy Bypass -File scripts/Invoke-RustWorkspace.ps1 validate-project`
- User confirmation: Not required for this internal implementation phase; final user review is pending.

### Notes
- Initial `Start-Process` manual-launch attempt exited early without captured stdout; direct `cargo run` commands confirmed both windows are created.
- Bevy logs reported Vulkan on `NVIDIA GeForce RTX 2080` and a connected gamepad warning; these did not block launch.

## Phase 3: Documentation, Validation, And Checkpoints
**Status:** Complete  
**Goal:** Document the new workspace and complete required validation/commit/push checkpoints.

### Tasks
- [x] Update `README.md` with workspace layout and run commands.
  - Status: Complete
  - Notes: README now documents `cargo run -p pigame-game` and `cargo run -p pigame-editor`.
- [x] Run required validation wrappers.
  - Status: Complete
  - Notes: Full validation passed via the PowerShell wrapper. Direct `.cmd` invocation from bash printed a Windows prompt without useful validation output, so the underlying PowerShell script was used directly.
- [x] Generate documentation.
  - Status: Complete
  - Notes: `cargo doc --workspace --all-features --no-deps` passed and generated documentation under `target/doc/`.
- [x] Commit completed tasks/phases and push to `origin`.
  - Status: Complete
  - Notes: Implementation commit `28fd9f8` was pushed to `origin/feature/workspace-app-launchers`; this tracker update records the checkpoint.

### Validation
- Format: Passed
- Lint: Passed
- Tests: Passed
- Build: Passed
- Documentation generation: Passed
- Full validation wrapper: Passed
- Push state: Implementation commit `28fd9f8` pushed; tracker checkpoint update in progress.
- User confirmation: Pending final user review.

### Notes
- Phase completion requires validation evidence or documented user-approved waivers.

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Active branch confirmed as `feature/workspace-app-launchers`.
- Proposed package names were used: `pigame-engine`, `pigame-game`, `pigame-editor`.
- Proposed run commands were used: `cargo run -p pigame-game` and `cargo run -p pigame-editor`.
- Ready for optional `gpt-5.5` sanity review or final user review.

## Postponed Work
- Full editor UI, scene editing, asset browser, inspector, and hot-reload/dynamic game library loading are postponed. This feature only needs a minimal editor window and project linking.
- Advanced Bevy feature-flag optimization is postponed unless default Bevy setup causes a concrete issue.

## Progress Log
- `2026-06-19`: Read required feature planning, Rust workspace, and Gitflow skills.
- `2026-06-19`: Inspected `Cargo.toml`, `src/lib.rs`, `README.md`, plan templates, existing plan directories, Git branch state, and remotes.
- `2026-06-19`: Created branch `feature/workspace-app-launchers` from local `dev` for feature planning.
- `2026-06-19`: Researched Bevy `DefaultPlugins`, `WindowPlugin`, window settings, and a Bevy editor/game workspace separation example.
- `2026-06-19`: Plan and tracker created; awaiting user review/approval before implementation.
- `2026-06-19`: User approved the plan, plan commit `2edb36a` was created and pushed to `origin/feature/workspace-app-launchers`, and implementation started with gpt-5.4.
- `2026-06-19`: Verified `dev` is an ancestor of `feature/workspace-app-launchers` before implementation edits.
- `2026-06-19`: Converted root manifest to a workspace, added `pigame-engine`, `pigame-game`, and `pigame-editor`, removed root placeholder source, and updated README.
- `2026-06-19`: Validation passed: format, clippy, tests, build, docs, and full PowerShell validation wrapper.
- `2026-06-19`: Manual launch checks confirmed Bevy creates `PiGame` and `PiGame Editor` windows; both direct `cargo run` commands were stopped by timeout after successful window creation logs.
- `2026-06-19`: Implementation commit `28fd9f8` was created and pushed to `origin/feature/workspace-app-launchers`.
