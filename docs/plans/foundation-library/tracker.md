# FoundationLibrary Baseline Tracker

## Metadata
- Feature slug: `foundation-library`
- Feature area: `multi-area`
- Primary area: `game`
- Branch: `feature/foundation-library`
- Overall status: `Planned`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for user review before gpt-5.4 implementation`
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
- Push status: Planning docs commit pending.
- Prior branch cleanup: Local `feature/jackdaw-editor-integration` was deleted after merge to `dev`; remote branch was intentionally left intact per user preference.

## Phase 1: FoundationLibrary Crate Baseline
**Status:** Planned  
**Goal:** Add a minimal reusable library crate with documented public API and a Bevy plugin entry point.

### Tasks
- [ ] Add `crates/foundation-library` as a root workspace member.
  - Status: Planned
  - Notes: Package name should be `foundation-library`; Rust import path will be `foundation_library`.
- [ ] Create the FoundationLibrary manifest and source.
  - Status: Planned
  - Notes: Prefer minimal dependency surface; use `bevy.workspace = true` for plugin support.
- [ ] Implement documented baseline API.
  - Status: Planned
  - Notes: Proposed items are `FoundationPlugin` and `prelude` re-exports.
- [ ] Add a non-window test proving the plugin can be added to a Bevy `App`.
  - Status: Planned
  - Notes: Avoid tests that open windows or require GPU access.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending approval to implement.

### Notes
- Keep the crate intentionally small. Do not reintroduce the old custom `engine` crate responsibilities in this baseline.

## Phase 2: TemplateGame Integration
**Status:** Planned  
**Goal:** Wire TemplateGame to use FoundationLibrary in both standalone and editor binaries.

### Tasks
- [ ] Add a path dependency from `games/template-game` to `foundation-library`.
  - Status: Planned
  - Notes: Use `foundation-library = { path = "../../crates/foundation-library" }`.
- [ ] Add FoundationLibrary plugin to `games/template-game/src/main.rs`.
  - Status: Planned
  - Notes: Add before `template_game::TemplateGamePlugin`.
- [ ] Add FoundationLibrary plugin to `games/template-game/src/bin/editor.rs`.
  - Status: Planned
  - Notes: Keep standalone and editor/play-mode plugin composition consistent.
- [ ] Preserve TemplateGame-specific behavior in TemplateGame.
  - Status: Planned
  - Notes: Do not move `TemplateGamePlugin` or `SpinningCube` in this baseline unless the user changes scope.

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
**Status:** Planned  
**Goal:** Document the Editor / Game / Library architecture and complete validation/commit/push checkpoints.

### Tasks
- [ ] Update README architecture documentation.
  - Status: Planned
  - Notes: Document `jackdaw-editor`, `foundation-library`, and `template-game` roles.
- [ ] Run required validation.
  - Status: Planned
  - Notes: Use root validation wrappers where possible.
- [ ] Generate documentation.
  - Status: Planned
  - Notes: `cargo doc --workspace --all-features --no-deps` or project wrapper.
- [ ] Commit and push implementation checkpoints.
  - Status: Planned
  - Notes: Push to `origin/feature/foundation-library` after each commit.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending final user review.

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
