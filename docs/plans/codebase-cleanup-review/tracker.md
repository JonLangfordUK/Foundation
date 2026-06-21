# Codebase Cleanup Review Tracker

## Metadata
- Feature slug: `codebase-cleanup-review`
- Feature area: `multi-area`
- Primary area: `game`
- Branch: `feature/codebase-cleanup-review`
- Overall status: `In progress`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation in progress with gpt-5.4`
- Created: `2026-06-21`
- Last updated: `2026-06-21`

## Branch And Push State
- Planned branch: `feature/codebase-cleanup-review`
- Current branch during planning: `feature/codebase-cleanup-review`
- Branch base: Created from `dev` during planning.
- Remote `origin`: Configured (`https://github.com/JonLangfordUK/Foundation.git`).
- Push status: Pending; no implementation commit has been made yet.

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
**Status:** In progress  
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
- Full validation wrapper: Not required yet
- User confirmation: Pending

### Notes
- Implementation must not begin until user approves this plan/tracker.

## Phase 1: Baseline Audit And Validation
**Status:** In progress  
**Goal:** Capture the current validation state and identify concrete cleanup/robustness findings before making fixes.

### Tasks
- [x] Verify branch state and read required implementation skills.
  - Status: Complete
  - Notes: Read feature tracker/update, plan docs, Rust workspace, Rust coding standards, and Gitflow skills. Active branch is `feature/codebase-cleanup-review`; `dev` is an ancestor of `HEAD`.
- [ ] Run baseline validation wrappers and record results.
  - Status: Planned
  - Notes: Include format, lint, tests, build, docs, and full validation when practical.
- [ ] Audit `crates/foundation-library` source, tests, docs, and manifest.
  - Status: Planned
  - Notes: Review scene stack, menu, splash screen, prelude, reflection registration, and public API documentation.
- [ ] Audit `games/template-game` source, tests, docs, manifests, and launcher config.
  - Status: Planned
  - Notes: Review standalone/editor flows, cfg boundaries, UI targeting, asset paths, component registrations, and tests.
- [ ] Audit `games/template-game/assets/*.jsn` consistency.
  - Status: Planned
  - Notes: Check component paths, scene paths, parent/order metadata, UI roots, and flow consistency.
- [ ] Record findings and proposed fix tasks in this tracker.
  - Status: Planned
  - Notes: Categorize findings by correctness, robustness, maintainability, docs, tests, assets, and config.
- [ ] Add scene-system instruction manual requirement to implementation scope.
  - Status: Planned after user confirmation
  - Notes: User requested a clear, concise, illustrative document explaining the scene system, standalone runtime, editor edit mode, editor play/game mode, Jackdaw integration, and recommended usage. User clarified that this manual must only be written after they confirm all features still work following the code review/refactor.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Not required yet

### Notes
- Baseline failures should be recorded before fixes are made.

## Phase 2: FoundationLibrary Cleanup And Robustness Fixes
**Status:** Planned  
**Goal:** Apply focused fixes to `foundation-library` based on the Phase 1 audit.

### Tasks
- [ ] Fix scene-stack issues or documentation gaps.
  - Status: Planned
  - Notes: Scope depends on Phase 1 findings; preserve lifecycle semantics unless a defect is identified.
- [ ] Fix menu/pause/generated UI issues or documentation gaps.
  - Status: Planned
  - Notes: Include tests for action, ownership, or edge-case changes where practical.
- [ ] Fix splash-screen issues or documentation gaps.
  - Status: Planned
  - Notes: Include tests for timing, cleanup, or transition semantics where practical.
- [ ] Review public prelude and plugin registration consistency.
  - Status: Planned
  - Notes: Update Rustdoc for changed public APIs.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required until phase end unless broad changes are made
- User confirmation: Not required yet

### Notes
- Commit completed FoundationLibrary task/fix checkpoints separately when practical.

## Phase 3: TemplateGame Cleanup And Robustness Fixes
**Status:** Planned  
**Goal:** Apply focused fixes to TemplateGame runtime/editor code, assets, tests, and config based on the Phase 1 audit.

### Tasks
- [ ] Fix TemplateGame runtime/standalone issues or documentation gaps.
  - Status: Planned
  - Notes: Include startup scene flow, asset root handling, UI camera behavior, and game-specific systems.
- [ ] Fix TemplateGame editor integration issues or documentation gaps.
  - Status: Planned
  - Notes: Include play-mode setup/cleanup, scene loading bridge, UI viewport routing, and cfg-gated tests.
- [ ] Fix TemplateGame asset/config inconsistencies.
  - Status: Planned
  - Notes: Include `.jsn` scene path/component consistency, `.cargo/config.toml`, and `jackdaw.toml` if needed.
- [ ] Add or update TemplateGame tests for changed behavior.
  - Status: Planned
  - Notes: Consider default and `editor` feature coverage.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required until phase end unless broad changes are made
- User confirmation: Not required yet

### Notes
- Manual editor smoke testing may be needed if automated tests cannot cover play-mode behavior.

## Phase 4: Final Validation, Documentation, And Handoff
**Status:** Planned  
**Goal:** Prove the cleanup is complete, record evidence, and prepare for optional final review.

### Tasks
- [ ] Run full standard validation.
  - Status: Planned
  - Notes: Prefer `scripts/validate-project.cmd`; record individual command outcomes if run separately.
- [ ] Generate documentation and verify public API docs.
  - Status: Planned
  - Notes: Use `scripts/doc-project.cmd`.
- [ ] Record postponed work, waivers, and manual-verification gaps.
  - Status: Planned
  - Notes: Any accepted risk must be explicit.
- [ ] Commit final phase and push checkpoints.
  - Status: Planned
  - Notes: Use required Gitflow commit message format and push to `origin` when available.
- [ ] Offer optional `gpt-5.5` final sanity review.
  - Status: Planned
  - Notes: Review should focus on high-risk ECS/editor/asset interactions and validation evidence.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending final acceptance or optional review decision

### Notes
- Do not mark the feature complete until validation and documentation generation are recorded or a waiver is approved.

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
- Planning note: Static scan found only test `expect(...)` calls in `crates/foundation-library/src/scene_stack.rs` and one `#[allow(dead_code)]` in `games/template-game/src/lib.rs`; review during Phase 1.
- Planning note: TemplateGame assets contain component paths for Foundation and TemplateGame reflected components; audit consistency during Phase 1.

## Progress Log
- `2026-06-21`: Created branch `feature/codebase-cleanup-review` from `dev` for planning and future implementation.
- `2026-06-21`: Inspected mandatory planning/Rust/Gitflow skills.
- `2026-06-21`: Inspected workspace manifests, relevant `foundation-library` and `template-game` source/config/test files, and summarized TemplateGame `.jsn` assets.
- `2026-06-21`: Created plan and tracker. Awaiting user review/approval before implementation.
- `2026-06-21`: User approved the plan and requested an additional scene-system instruction manual. Implementation started on `feature/codebase-cleanup-review`.
- `2026-06-21`: User clarified the scene-system manual must only be written after they confirm all features still work following the code review/refactor. Plan/tracker updated accordingly.
