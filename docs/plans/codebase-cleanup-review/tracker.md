# Codebase Cleanup Review Tracker

## Metadata
- Feature slug: `codebase-cleanup-review`
- Feature area: `multi-area`
- Primary area: `game`
- Branch: `feature/codebase-cleanup-review`
- Overall status: `Complete; ready to merge`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `User accepted implementation; ready to merge into dev`
- Created: `2026-06-21`
- Last updated: `2026-06-21`

## Branch And Push State
- Planned branch: `feature/codebase-cleanup-review`
- Current branch during implementation: `feature/codebase-cleanup-review`
- Branch base: Created from `dev` during planning; `dev` is an ancestor of `HEAD`.
- Remote `origin`: Configured (`https://github.com/JonLangfordUK/Foundation.git`).
- Push status: Planning commit `31a0d9c`, cleanup checkpoint commit `43a5f34`, tracker commit `b02bbec`, scene guide commit `916138f`, and old-guide removal commit `474b56c` pushed.

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- Standard validation commands:
  - `scripts/format-project.cmd`
  - `scripts/lint-project.cmd`
  - `scripts/test-project.cmd`
  - `scripts/compile-project.cmd`
  - `scripts/doc-project.cmd`
  - `scripts/validate-project.cmd`

## Phase 0: Planning Approval
**Status:** Complete  
**Goal:** Confirm scope, feature classification, and implementation expectations before code edits.

### Tasks
- [x] Create dedicated feature branch.
  - Status: Complete
  - Notes: Created `feature/codebase-cleanup-review` from `dev` during planning.
- [x] Inspect workspace structure and relevant files.
  - Status: Complete
  - Notes: Inspected workspace manifests, `foundation-library` source modules, TemplateGame source modules, tests, launcher config, and `.jsn` asset summaries.
- [x] Create plan and tracker.
  - Status: Complete
  - Notes: Created `docs/plans/codebase-cleanup-review/plan.md` and this tracker.
- [x] Confirm feature area classification and scope with user.
  - Status: Complete
  - Notes: User approved the plan and classification with an added documentation requirement for a scene-system instruction manual.

### Validation
- Format: Not required for planning-only document creation
- Lint: Not required for planning-only document creation
- Tests: Not required for planning-only document creation
- Build: Not required for planning-only document creation
- Documentation generation: Not required for planning-only document creation
- Full validation wrapper: Not required for planning-only document creation
- User confirmation: Complete

### Notes
- Planning commit `31a0d9c` was pushed to `origin/feature/codebase-cleanup-review`.

## Phase 1: Baseline Audit And Validation
**Status:** Complete  
**Goal:** Capture the current validation state and identify concrete cleanup/robustness findings before making fixes.

### Tasks
- [x] Verify branch state and read required implementation skills.
  - Status: Complete
  - Notes: Read feature tracker/update, plan docs, Rust workspace, Rust coding standards, and Gitflow skills. Active branch is `feature/codebase-cleanup-review`; `dev` is an ancestor of `HEAD`.
- [x] Run baseline validation wrappers and record results.
  - Status: Complete
  - Notes: `scripts/format-project.cmd`, `scripts/lint-project.cmd`, `scripts/test-project.cmd`, `scripts/compile-project.cmd`, `scripts/doc-project.cmd`, and `scripts/validate-project.cmd` passed during this work session.
- [x] Audit `crates/foundation-library` source, tests, docs, and manifest.
  - Status: Complete
  - Notes: Reviewed scene stack, menu, splash screen, prelude, reflection registration, and public API documentation. Existing validation and tests passed; no FoundationLibrary code changes were required in this pass.
- [x] Audit `games/template-game` source, tests, docs, manifests, and launcher config.
  - Status: Complete
  - Notes: Found Jackdaw run configuration targeted `template_game` while Cargo metadata names the binary `template-game`; fixed `jackdaw.toml`. Removed an unnecessary `#[allow(dead_code)]` from a system that is registered by the plugin.
- [x] Audit `games/template-game/assets/*.jsn` consistency.
  - Status: Complete
  - Notes: Added tests proving TemplateGame scene constants point at existing asset files and authored scene assets reference expected next/menu/pause scene paths.
- [x] Record findings and proposed fix tasks in this tracker.
  - Status: Complete
  - Notes: Findings and fixes are recorded under Issues / Findings Log and Progress Log.
- [x] Add scene-system instruction manual requirement to implementation scope.
  - Status: Complete, gated
  - Notes: User requested a clear, concise, illustrative document explaining the scene system, standalone runtime, editor edit mode, editor play/game mode, Jackdaw integration, and recommended usage. User clarified that this manual must only be written after they confirm all features still work following the code review/refactor.

### Validation
- Format: Passed (`scripts/format-project.cmd`)
- Lint: Passed (`scripts/lint-project.cmd`)
- Tests: Passed (`scripts/test-project.cmd`)
- Build: Passed (`scripts/compile-project.cmd`)
- Documentation generation: Passed (`scripts/doc-project.cmd`)
- Full validation wrapper: Passed (`scripts/validate-project.cmd`)
- User confirmation: Not required for baseline audit

### Notes
- Baseline validation passed. No pre-existing validation blockers were found.

## Phase 2: FoundationLibrary Cleanup And Robustness Fixes
**Status:** Complete  
**Goal:** Apply focused fixes to `foundation-library` based on the Phase 1 audit.

### Tasks
- [x] Fix scene-stack issues or documentation gaps.
  - Status: Complete
  - Notes: Audit found existing scene-stack tests and docs sufficient for this pass; no behavior changes required.
- [x] Fix menu/pause/generated UI issues or documentation gaps.
  - Status: Complete
  - Notes: Audit found existing menu tests and docs sufficient for this pass; no behavior changes required.
- [x] Fix splash-screen issues or documentation gaps.
  - Status: Complete
  - Notes: Audit found existing splash-screen tests and docs sufficient for this pass; no behavior changes required.
- [x] Review public prelude and plugin registration consistency.
  - Status: Complete
  - Notes: Prelude and plugin registration remain consistent with current public APIs.

### Validation
- Format: Passed (`scripts/format-project.cmd`)
- Lint: Passed (`scripts/lint-project.cmd`)
- Tests: Passed (`scripts/test-project.cmd`)
- Build: Passed (`scripts/compile-project.cmd`)
- Documentation generation: Passed (`scripts/doc-project.cmd`)
- Full validation wrapper: Passed (`scripts/validate-project.cmd`)
- User confirmation: Not required

### Notes
- No `foundation-library` source changes were needed in this checkpoint.

## Phase 3: TemplateGame Cleanup And Robustness Fixes
**Status:** Complete  
**Goal:** Apply focused fixes to TemplateGame runtime/editor code, assets, tests, and config based on the Phase 1 audit.

### Tasks
- [x] Fix TemplateGame runtime/standalone issues or documentation gaps.
  - Status: Complete
  - Notes: Removed unnecessary `#[allow(dead_code)]` from the registered main-menu interaction system. Existing standalone flow validation passed.
- [x] Fix TemplateGame editor integration issues or documentation gaps.
  - Status: Complete
  - Notes: Fixed `games/template-game/jackdaw.toml` to target Cargo's `template-game` binary name. Existing editor-feature tests passed through `--all-features` validation.
- [x] Fix TemplateGame asset/config inconsistencies.
  - Status: Complete
  - Notes: Added regression tests covering scene constant asset existence and authored asset scene-path references.
- [x] Add or update TemplateGame tests for changed behavior.
  - Status: Complete
  - Notes: Added `scene_path_constants_match_existing_assets`, `jackdaw_run_config_targets_template_game_binary`, and `authored_scene_assets_reference_known_scene_paths` tests.

### Validation
- Format: Passed (`scripts/format-project.cmd`)
- Lint: Passed (`scripts/lint-project.cmd`)
- Tests: Passed (`scripts/test-project.cmd`)
- Build: Passed (`scripts/compile-project.cmd`)
- Documentation generation: Passed (`scripts/doc-project.cmd`)
- Full validation wrapper: Passed (`scripts/validate-project.cmd`)
- User confirmation: Not required

### Notes
- Manual editor smoke testing may still be useful because automated tests do not open the Jackdaw editor window.

## Phase 4: User Feature Confirmation And Scene-System Manual
**Status:** Complete  
**Goal:** Let the user confirm all reviewed/refactored features still work, then write the deferred scene-system instruction manual.

### Tasks
- [x] User confirms all features still work after the code review/refactor.
  - Status: Complete
  - Notes: User confirmed everything is working perfectly and all features are correct.
- [x] Write scene-system instruction manual under `docs/`.
  - Status: Complete
  - Notes: Added `docs/scene-system.md` explaining what the scene system is, how it works with Jackdaw, standalone game behavior, editor edit-mode behavior, editor play/game-mode behavior, and best utilization guidance for new developers.
- [x] Validate documentation after the manual is written.
  - Status: Complete
  - Notes: `scripts/doc-project.cmd` and `scripts/validate-project.cmd` passed after adding the manual.

### Validation
- Format: Passed via `scripts/validate-project.cmd`
- Lint: Passed via `scripts/validate-project.cmd`
- Tests: Passed via `scripts/validate-project.cmd`
- Build: Passed via `scripts/validate-project.cmd`
- Documentation generation: Passed (`scripts/doc-project.cmd` and full validation wrapper)
- Full validation wrapper: Passed (`scripts/validate-project.cmd`)
- User confirmation: Complete

### Notes
- The scene-system manual has been written at `docs/scene-system.md` after user feature confirmation.

## Phase 5: Final Validation, Commit, And Handoff
**Status:** Complete  
**Goal:** Prove the cleanup checkpoint is complete, record evidence, and prepare for the deferred documentation/manual checkpoint.

### Tasks
- [x] Run full standard validation.
  - Status: Complete
  - Notes: `scripts/validate-project.cmd` passed after cleanup changes.
- [x] Generate documentation and verify public API docs.
  - Status: Complete
  - Notes: `scripts/doc-project.cmd` passed after cleanup changes.
- [x] Record postponed work, waivers, and manual-verification gaps.
  - Status: Complete
  - Notes: Scene-system manual is intentionally gated until user confirms features still work after this review/refactor. Manual standalone/editor smoke checks are pending user confirmation.
- [x] Commit cleanup checkpoint and push.
  - Status: Complete
  - Notes: Planning commit `31a0d9c` and cleanup checkpoint commit `43a5f34` were pushed to `origin/feature/codebase-cleanup-review`.
- [x] Offer optional `gpt-5.5` final sanity review.
  - Status: Complete
  - Notes: Optional review was offered after the manual checkpoint; user accepted the implementation and requested commit/merge/delete-local workflow.

### Validation
- Format: Passed (`scripts/format-project.cmd`)
- Lint: Passed (`scripts/lint-project.cmd`)
- Tests: Passed (`scripts/test-project.cmd`)
- Build: Passed (`scripts/compile-project.cmd`)
- Documentation generation: Passed (`scripts/doc-project.cmd`)
- Full validation wrapper: Passed (`scripts/validate-project.cmd`)
- User confirmation: Complete; user confirmed features work and accepted final state

### Notes
- Overall feature is complete and ready to merge into `dev`.

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Read the plan and this tracker before implementation edits.
- Confirm active branch is `feature/codebase-cleanup-review`.
- Update this tracker before and after work sessions, including validation results and push status.
- Keep changes focused and evidence-driven; this is a cleanup/robustness feature, not an open-ended rewrite.
- Use `gpt-5.5` only for planning or optional final sanity review.

## Postponed Work
- None.

## Issues / Findings Log
- User-approved scope update: Add an instruction-manual style scene-system document under `docs/` covering how the scene system works, standalone mode, editor edit mode, editor play/game mode, Jackdaw integration, and best practices, but only after the user confirms all features still work after review/refactor.
- Finding: `games/template-game/jackdaw.toml` targeted `template_game`, while Cargo metadata names the binary `template-game`. Fixed the run config and added a regression test.
- Finding: `games/template-game/src/lib.rs` carried an unnecessary `#[allow(dead_code)]` on a system registered by `TemplateGamePlugin`. Removed it.
- Finding: TemplateGame scene constants and authored `.jsn` scene references were not directly tested. Added regression tests for asset existence and key scene-path references.
- Static scan note: Remaining `expect(...)` calls are in tests only.

## Progress Log
- `2026-06-21`: Created branch `feature/codebase-cleanup-review` from `dev` for planning and future implementation.
- `2026-06-21`: Inspected mandatory planning/Rust/Gitflow skills.
- `2026-06-21`: Inspected workspace manifests, relevant `foundation-library` and `template-game` source/config/test files, and summarized TemplateGame `.jsn` assets.
- `2026-06-21`: Created plan and tracker. Awaiting user review/approval before implementation.
- `2026-06-21`: User approved the plan and requested an additional scene-system instruction manual. Implementation started on `feature/codebase-cleanup-review`.
- `2026-06-21`: User clarified the scene-system manual must only be written after they confirm all features still work following the code review/refactor. Plan/tracker updated accordingly.
- `2026-06-21`: Committed and pushed planning documents as `31a0d9c`.
- `2026-06-21`: Completed cleanup checkpoint: fixed TemplateGame Jackdaw run config, removed unnecessary allow attribute, added TemplateGame asset/config regression tests, and passed full validation.
- `2026-06-21`: Committed and pushed cleanup checkpoint as `43a5f34`.
- `2026-06-21`: User confirmed all features are still working correctly after review/refactor.
- `2026-06-21`: Added `docs/scene-system.md` scene-system instruction manual after user confirmation.
- `2026-06-21`: `scripts/doc-project.cmd` and `scripts/validate-project.cmd` passed after adding the scene-system manual.
- `2026-06-21`: User chose to remove the older `docs/foundation-scene-stack.md` document so `docs/scene-system.md` is the single canonical scene-system guide.
- `2026-06-21`: User accepted the final state and requested commit, merge into `dev`, delete local feature branch, and keep the remote feature branch.
