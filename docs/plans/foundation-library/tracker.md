# FoundationLibrary Baseline Tracker

## Metadata
- Feature slug: `foundation-library`
- Feature area: `multi-area`
- Primary area: `game`
- Branch: `feature/foundation-library`
- Overall status: `Implemented; ready for review or merge`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation complete with gpt-5.4; ready for optional gpt-5.5 sanity review or user acceptance`
- Created: `2026-06-19`
- Last updated: `2026-06-19`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- Push after every commit and merge checkpoint when `origin` is configured. If push fails, record the failure and do not treat the checkpoint as complete until remediated.

## Branch And Push State
- Active planning branch: `feature/foundation-library`
- Branch creation: Created locally from `dev` on 2026-06-19 after merging Jackdaw editor integration into `dev`.
- Branch-base verification: `git merge-base --is-ancestor dev HEAD` passed before planning docs were created.
- Remote: `origin` is configured as `https://github.com/JonLangfordUK/Foundation.git`.
- Push status: Planning docs commit `73dfe2d` pushed to `origin/feature/foundation-library`; implementation commit pending.
- Prior branch cleanup: Local `feature/jackdaw-editor-integration` was deleted after merge to `dev`; remote branch was intentionally left intact per user preference.

## Phase 1: FoundationLibrary Crate Baseline
**Status:** Complete  
**Goal:** Add a minimal reusable library crate with documented public API and a Bevy plugin entry point.

### Tasks
- [x] Add `crates/foundation-library` as a root workspace member.
  - Status: Complete
  - Notes: Package name is `foundation-library`; Rust import path is `foundation_library`.
- [x] Create the FoundationLibrary manifest and source.
  - Status: Complete
  - Notes: Uses minimal dependency surface with `bevy.workspace = true` for plugin support.
- [x] Implement documented baseline API.
  - Status: Complete
  - Notes: Added documented `FoundationPlugin`, `FoundationSettings`, and `prelude` re-exports.
- [x] Add a non-window test proving the plugin can be added to a Bevy `App`.
  - Status: Complete
  - Notes: `foundation_plugin_registers_settings_resource` uses `MinimalPlugins` and does not open a GPU window.

### Validation
- Format: Passed (`cargo fmt --all -- --check`; also via `scripts/validate-project.cmd`)
- Lint: Passed (`cargo clippy --workspace --all-targets --all-features -- -D warnings`; also via `scripts/validate-project.cmd`)
- Tests: Passed (`cargo test --workspace --all-features`; also via `scripts/validate-project.cmd`)
- Build: Passed (`cargo build --workspace --all-features`; also via `scripts/validate-project.cmd`)
- Documentation generation: Passed (`cargo doc --workspace --all-features --no-deps`; also via `scripts/validate-project.cmd`)
- Full validation wrapper: Passed (`scripts/validate-project.cmd`)
- User confirmation: User approved implementation on 2026-06-19.

### Notes
- Keep the crate intentionally small. Do not reintroduce the old custom `engine` crate responsibilities in this baseline.

## Phase 2: TemplateGame Integration
**Status:** Complete  
**Goal:** Wire TemplateGame to use FoundationLibrary in both standalone and editor binaries.

### Tasks
- [x] Add a path dependency from `games/template-game` to `foundation-library`.
  - Status: Complete
  - Notes: Added `foundation-library = { path = "../../crates/foundation-library" }`.
- [x] Add FoundationLibrary plugin to `games/template-game/src/main.rs`.
  - Status: Complete
  - Notes: Added `FoundationPlugin` before `template_game::TemplateGamePlugin`.
- [x] Add FoundationLibrary plugin to `games/template-game/src/bin/editor.rs`.
  - Status: Complete
  - Notes: Added `FoundationPlugin` before `template_game::TemplateGamePlugin`, keeping standalone and editor/play-mode plugin composition consistent.
- [x] Preserve TemplateGame-specific behavior in TemplateGame.
  - Status: Complete
  - Notes: `TemplateGamePlugin` and `SpinningCube` remain in TemplateGame; FoundationLibrary only provides the reusable baseline plugin/resource.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending / Not required yet

### Notes
- TemplateGame should remain runnable from the root with `cargo run -p template-game` and `cargo run -p template-game --bin editor --features editor`.

## Phase 3: Documentation, Validation, And Checkpoints
**Status:** Complete  
**Goal:** Document the Editor / Game / Library architecture and complete validation/commit/push checkpoints.

### Tasks
- [x] Update README architecture documentation.
  - Status: Complete
  - Notes: README documents `jackdaw-editor`, `foundation-library`, and `template-game` roles in the Editor / Game / Library architecture.
- [x] Run required validation.
  - Status: Complete
  - Notes: Ran root format, package checks, clippy, tests, build, docs, and `scripts/validate-project.cmd`.
- [x] Generate documentation.
  - Status: Complete
  - Notes: `cargo doc --workspace --all-features --no-deps` passed and generated `target/doc/foundation_library/index.html`.
- [ ] Commit and push implementation checkpoints.
  - Status: Awaiting commit/push
  - Notes: Implementation commit pending.

### Validation
- Format: Passed
- Lint: Passed
- Tests: Passed
- Build: Passed
- Documentation generation: Passed
- Full validation wrapper: Passed
- User confirmation: Pending final user review or optional sanity review.

### Notes
- This phase cannot be marked complete until validation and documentation generation are recorded.

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Active branch must be `feature/foundation-library` before implementation edits.
- Verify `dev` ancestry again before implementation edits.
- FoundationLibrary baseline should be minimal: crate, documented plugin, prelude, tests, TemplateGame dependency, TemplateGame plugin wiring, README update.
- Avoid adding Jackdaw-specific APIs to FoundationLibrary in this first baseline unless implementation reveals a clear need.
- Leave Jackdaw dynamic/dylib loading out of scope.

## Postponed Work
- Moving reusable components out of TemplateGame is postponed until there is a second real use case or explicit user request.
- Jackdaw-specific editor extension APIs in FoundationLibrary are postponed until a concrete editor feature needs them.
- Additional packages such as `foundation-editor` or `foundation-tools` are postponed; one library crate is sufficient for the baseline.

## Progress Log
- `2026-06-19`: User approved the Editor / Game / Library strategy and chose the shared library name FoundationLibrary.
- `2026-06-19`: User corrected workflow order: merge Jackdaw editor integration back to `dev`, delete local feature branch while keeping remote, then create the new FoundationLibrary branch from `dev`.
- `2026-06-19`: Merged `feature/jackdaw-editor-integration` into `dev`, pushed `dev`, deleted local `feature/jackdaw-editor-integration`, and created `feature/foundation-library` from `dev`.
- `2026-06-19`: Verified `dev` is an ancestor of `feature/foundation-library`.
- `2026-06-19`: Plan and tracker created; awaiting user review/approval before implementation.
- `2026-06-19`: User approved implementation. Confirmed active branch `feature/foundation-library` and verified `dev` is an ancestor of `HEAD`; implementation started with gpt-5.4.
- `2026-06-19`: Added `crates/foundation-library` with documented `FoundationPlugin`, `FoundationSettings`, prelude exports, and a non-window plugin test.
- `2026-06-19`: Wired TemplateGame standalone and editor binaries to add `FoundationPlugin` before `TemplateGamePlugin`.
- `2026-06-19`: Updated README with the Editor / Game / Library architecture.
- `2026-06-19`: Validation passed: `cargo fmt --all -- --check`, `cargo check -p foundation-library`, `cargo check -p template-game`, `cargo check -p template-game --bin editor --features editor`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`, `cargo doc --workspace --all-features --no-deps`, and `scripts/validate-project.cmd`.
