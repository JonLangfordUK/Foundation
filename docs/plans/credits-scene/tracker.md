# Credits Scene Tracker

## Metadata
- Feature slug: `credits-scene`
- Feature area: `multi-area`
- Primary area: `game`
- Branch: `feature/credits-scene`
- Overall status: `Implementation complete; validation passed`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for gpt-5.5 sanity review or user acceptance`
- Created: `2026-06-22`
- Last updated: `2026-06-22`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- Use the standard wrappers unless the user explicitly waives them:
  - `scripts/format-project.cmd`
  - `scripts/lint-project.cmd`
  - `scripts/test-project.cmd`
  - `scripts/compile-project.cmd`
  - `scripts/doc-project.cmd`
  - `scripts/validate-project.cmd`

## Branch And Working Tree State
- Branch: `feature/credits-scene`
- Branch base: Created from `dev` during planning on 2026-06-22; verified with `git merge-base --is-ancestor dev HEAD` before implementation.
- Push status: Planning commit `3b64972` pushed to `origin/feature/credits-scene`; latest implementation commit pending push.
- Pre-existing working tree note: `games/template-game/.jsn/project.jsn` was modified before feature planning began and remains unrelated/uncommitted.

## Phase 1: Credits Data Model And Runtime Ownership
**Status:** Complete  
**Goal:** Establish where credits behavior lives and add tested JSON schema support.

### Tasks
- [x] Confirm implementation ownership split: reusable `foundation-runtime-library` component/system versus TemplateGame-local component/system.
  - Status: Complete
  - Notes: Implemented reusable Foundation runtime behavior plus TemplateGame-owned assets, following plan recommendation and crate-boundary rules.
- [x] Add credits JSON data model types for documents, groups, and people.
  - Status: Complete
  - Notes: `CreditsDocument`, `CreditsGroup`, and `CreditPerson` support the recursive `name`/`people`/`groups` schema.
- [x] Add JSON loading and recursive flattening/render-row preparation.
  - Status: Complete
  - Notes: Added deterministic pre-order traversal tests and arbitrary-depth nested group coverage.
- [x] Add or update dependency declarations for JSON parsing.
  - Status: Complete
  - Notes: Added workspace `serde_json` dependency and wired it into `foundation-runtime-library`.

### Validation
- Format: Passed via `scripts/format-project.cmd` and `scripts/validate-project.cmd` on 2026-06-22
- Lint: Passed via `scripts/validate-project.cmd` on 2026-06-22
- Tests: Passed via focused `cargo test -p foundation-runtime-library --all-features`, focused `cargo test -p template-game --all-features`, and `scripts/validate-project.cmd` on 2026-06-22
- Build: Passed via `scripts/validate-project.cmd` on 2026-06-22
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-06-22
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-06-22
- User confirmation: Received approval to proceed on 2026-06-22

### Notes
- Avoided adding a full `jackdaw` dependency to `foundation-runtime-library`.
- Public credits types and helpers have Rustdoc comments.

## Phase 2: Credits Scene Runtime UI
**Status:** Complete  
**Goal:** Generate and animate a credits roll with black background and scene-stack cleanup behavior.

### Tasks
- [x] Add reflected credits marker component with configurable JSON path and scroll settings.
  - Status: Complete
  - Notes: Added `FoundationCreditsRoll` with JSON path, scroll speed, start offset, depth-based font sizing, indentation, and row gap fields.
- [x] Register the marker component and systems in the appropriate plugin.
  - Status: Complete
  - Notes: Added `FoundationCreditsPlugin` and registered it from `FoundationPlugin`.
- [x] Spawn generated UI text rows from parsed credits data.
  - Status: Complete
  - Notes: Generated content and rows receive `SceneOwner` when initialized from a scene-stack-owned marker.
- [x] Apply depth-based group header sizing and indentation.
  - Status: Complete
  - Notes: Top-level group headers are largest, nested group headers get smaller by depth, and very deep group headers clamp to a readable minimum size.
- [x] Animate credits text from bottom to top over time.
  - Status: Complete
  - Notes: Generated content node moves upward by `scroll_speed_pixels_per_second` from the configured start offset.
- [x] Support closing the credits scene with Escape and a Back button or equivalent authored UI.
  - Status: Complete
  - Notes: `credits.jsn` uses `FoundationCloseOnEscape` and a Back button with `FoundationMenuButton` `close_current`.

### Validation
- Format: Passed via `scripts/format-project.cmd` and `scripts/validate-project.cmd` on 2026-06-22
- Lint: Passed via `scripts/validate-project.cmd` on 2026-06-22
- Tests: Passed via focused `cargo test -p foundation-runtime-library --all-features`, focused `cargo test -p template-game --all-features`, and `scripts/validate-project.cmd` on 2026-06-22
- Build: Passed via `scripts/validate-project.cmd` on 2026-06-22
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-06-22
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-06-22
- User confirmation: Received approval to proceed on 2026-06-22

### Notes
- Added `FoundationCreditsRuntimeSettings` so editor Play can require `SceneOwner` and avoid generating runtime credits UI for the open editor-authored scene.

## Phase 3: TemplateGame Asset And Menu Integration
**Status:** Complete  
**Goal:** Add the concrete credits assets and hook them into the main menu.

### Tasks
- [x] Add `games/template-game/assets/credits.json` using the requested nested groups schema.
  - Status: Complete
  - Notes: Added placeholder content based on the user's sample schema.
- [x] Add `games/template-game/assets/credits.jsn` with black background, credits marker, UI root, Escape close marker, and Back button if appropriate.
  - Status: Complete
  - Notes: Serialized reflected component path uses `foundation_runtime_library::credits::FoundationCreditsRoll`.
- [x] Add `CREDITS_SCENE` constant in `games/template-game/src/lib.rs`.
  - Status: Complete
  - Notes: Included in scene path tests.
- [x] Update `games/template-game/assets/main_menu.jsn` to add Credits button under Options and above Exit.
  - Status: Complete
  - Notes: Added Credits button and adjusted `FoundationUiOrder` so Exit remains after Credits.
- [x] Update asset/reference tests for `main_menu.jsn`, scene constants, and file existence.
  - Status: Complete
  - Notes: Existing asset tests now include `CREDITS_SCENE`, `credits.jsn`, and `credits.json`.

### Validation
- Format: Passed via `scripts/format-project.cmd` and `scripts/validate-project.cmd` on 2026-06-22
- Lint: Passed via `scripts/validate-project.cmd` on 2026-06-22
- Tests: Passed via focused `cargo test -p foundation-runtime-library --all-features`, focused `cargo test -p template-game --all-features`, and `scripts/validate-project.cmd` on 2026-06-22
- Build: Passed via `scripts/validate-project.cmd` on 2026-06-22
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-06-22
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-06-22
- User confirmation: Received approval to proceed on 2026-06-22

### Notes
- Preserved pre-existing unrelated changes in `games/template-game/.jsn/project.jsn` by not staging them.

## Phase 4: Validation, Documentation, And Commit Checkpoints
**Status:** Complete pending implementation push  
**Goal:** Prove the feature is complete, documented, and ready for optional review.

### Tasks
- [x] Add or update Rustdoc and any feature-level documentation needed for the credits JSON schema.
  - Status: Complete
  - Notes: Added Rustdoc for public credits types, plugin, marker component, runtime settings, and helpers.
- [x] Run `scripts/format-project.cmd`.
  - Status: Complete
  - Notes: Passed on 2026-06-22.
- [x] Run `scripts/lint-project.cmd`.
  - Status: Complete
  - Notes: Passed as part of `scripts/validate-project.cmd` on 2026-06-22.
- [x] Run `scripts/test-project.cmd`.
  - Status: Complete
  - Notes: Passed as part of `scripts/validate-project.cmd` on 2026-06-22.
- [x] Run `scripts/compile-project.cmd`.
  - Status: Complete
  - Notes: Passed as part of `scripts/validate-project.cmd` on 2026-06-22.
- [x] Run `scripts/doc-project.cmd`.
  - Status: Complete
  - Notes: Passed as part of `scripts/validate-project.cmd` on 2026-06-22.
- [x] Run `scripts/validate-project.cmd`.
  - Status: Complete
  - Notes: Passed on 2026-06-22.
- [x] Commit completed tasks/phases and push to `origin` when available.
  - Status: Complete pending push verification
  - Notes: Implementation committed; push verification pending.

### Validation
- Format: Passed via `scripts/format-project.cmd` and `scripts/validate-project.cmd` on 2026-06-22
- Lint: Passed via `scripts/validate-project.cmd` on 2026-06-22
- Tests: Passed via `scripts/validate-project.cmd` on 2026-06-22
- Build: Passed via `scripts/validate-project.cmd` on 2026-06-22
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-06-22
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-06-22
- User confirmation: Received approval to proceed on 2026-06-22

### Notes
- Implementation commit created; push verification pending.

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation.
- Use `gpt-5.5` for optional final review.
- Never use Anthropic models.
- Mandatory implementation pre-read completed: `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/feature-plan-docs/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, `.pi/skills/gitflow-workflow/SKILL.md`, `.pi/skills/foundation-architecture/SKILL.md`, this tracker, and `plan.md`.
- Active branch confirmed as `feature/credits-scene`.
- Keep tracker updates in regular commits with code/assets.
- Treat `games/template-game/.jsn/project.jsn` as pre-existing unrelated working tree state until clarified.

## Postponed Work
- Final production credits copy is postponed until the user provides real names/roles; current `credits.json` uses the user's sample placeholder content.

## Open Issues And Questions
- Ownership split resolved: reusable `foundation-runtime-library` credits component with TemplateGame-owned JSON and scene assets.
- Final credits copy is not provided. Current `credits.json` uses the user's sample JSON as placeholder content.
- End-of-roll behavior resolved as no automatic scene transition; credits remain closable by Back/Escape.

## Progress Log
- `2026-06-22`: Read mandatory planning, Rust workspace, Gitflow, and Foundation architecture skills.
- `2026-06-22`: Inspected workspace `Cargo.toml`, Foundation runtime menu/scene-stack systems, TemplateGame plugin/startup systems, TemplateGame Cargo manifest, and `main_menu.jsn`.
- `2026-06-22`: Created branch `feature/credits-scene` from `dev` for planning.
- `2026-06-22`: Plan and tracker created. Awaiting user review before implementation.
- `2026-06-22`: Updated plan and tracker to explicitly require unbounded recursive nested groups and depth-based shrinking group headers with minimum-size clamping.
- `2026-06-22`: User approved implementation and requested committing first; planning docs committed as `3b64972` and pushed to `origin/feature/credits-scene`.
- `2026-06-22`: Implementation started with gpt-5.4 on `feature/credits-scene`; branch base verified with `git merge-base --is-ancestor dev HEAD`.
- `2026-06-22`: Implemented reusable Foundation credits runtime, recursive JSON model, depth-based header sizing, TemplateGame credits assets, and main-menu Credits button.
- `2026-06-22`: Focused validation passed: `cargo test -p foundation-runtime-library --all-features` and `cargo test -p template-game --all-features`.
- `2026-06-22`: Full validation passed with `scripts/validate-project.cmd`; `scripts/format-project.cmd` also passed separately.
- `2026-06-22`: Implementation committed; final tracker commit metadata update prepared before push.
