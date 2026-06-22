# Credits Scene Tracker

## Metadata
- Feature slug: `credits-scene`
- Feature area: `multi-area`
- Primary area: `game`
- Branch: `feature/credits-scene`
- Overall status: `Planned`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for user review after nested-group refinement before gpt-5.4 implementation`
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
- Branch base: Created from `dev` during planning on 2026-06-22.
- Push status: Pending; no implementation commits have been made yet.
- Pre-existing working tree note: `games/template-game/.jsn/project.jsn` was modified before feature planning began and should be treated as unrelated unless the user says otherwise.

## Phase 1: Credits Data Model And Runtime Ownership
**Status:** Planned  
**Goal:** Establish where credits behavior lives and add tested JSON schema support.

### Tasks
- [ ] Confirm implementation ownership split: reusable `foundation-runtime-library` component/system versus TemplateGame-local component/system.
  - Status: Planned
  - Notes: Plan recommendation is reusable Foundation runtime behavior plus TemplateGame assets.
- [ ] Add credits JSON data model types for documents, groups, and people.
  - Status: Planned
  - Notes: Must support an unbounded recursive group schema where every child group has the same `name`/`people`/`groups` shape as the root groups in the user's example.
- [ ] Add JSON loading and recursive flattening/render-row preparation.
  - Status: Planned
  - Notes: Include deterministic pre-order traversal tests and arbitrary-depth nested group coverage.
- [ ] Add or update dependency declarations for JSON parsing.
  - Status: Planned
  - Notes: `serde_json` is likely needed outside the current editor-only dependency path.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

### Notes
- Avoid adding a full `jackdaw` dependency to `foundation-runtime-library`.
- Public types must have Rustdoc comments.

## Phase 2: Credits Scene Runtime UI
**Status:** Planned  
**Goal:** Generate and animate a credits roll with black background and scene-stack cleanup behavior.

### Tasks
- [ ] Add reflected credits marker component with configurable JSON path and scroll settings.
  - Status: Planned
  - Notes: Suggested fields include `credits_path`, `scroll_speed_pixels_per_second`, `start_offset_pixels`, and `end_padding_pixels`.
- [ ] Register the marker component and systems in the appropriate plugin.
  - Status: Planned
  - Notes: If in Foundation, register from `FoundationPlugin`; if game-local, register from `TemplateGamePlugin`.
- [ ] Spawn generated UI text rows from parsed credits data.
  - Status: Planned
  - Notes: Generated entities must receive `SceneOwner` when initialized from a scene-stack-owned marker.
- [ ] Apply depth-based group header sizing and indentation.
  - Status: Planned
  - Notes: Top-level group headers must be largest, nested group headers must get smaller by depth, and very deep group headers must clamp to a readable minimum size.
- [ ] Animate credits text from bottom to top over time.
  - Status: Planned
  - Notes: Default behavior should be one-way scrolling; no looping unless explicitly configured.
- [ ] Support closing the credits scene with Escape and a Back button or equivalent authored UI.
  - Status: Planned
  - Notes: Use existing `FoundationCloseOnEscape` and `FoundationMenuButton` patterns where possible.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

### Notes
- Runtime file path handling must work in standalone and editor Play paths or document any user-approved limitation.

## Phase 3: TemplateGame Asset And Menu Integration
**Status:** Planned  
**Goal:** Add the concrete credits assets and hook them into the main menu.

### Tasks
- [ ] Add `games/template-game/assets/credits.json` using the requested nested groups schema.
  - Status: Planned
  - Notes: Initial content can use the user's sample unless they provide final credits copy.
- [ ] Add `games/template-game/assets/credits.jsn` with black background, credits marker, UI root, Escape close marker, and Back button if appropriate.
  - Status: Planned
  - Notes: Serialized reflected component crate paths must match implementation ownership.
- [ ] Add `CREDITS_SCENE` constant in `games/template-game/src/lib.rs`.
  - Status: Planned
  - Notes: Include it in scene path tests.
- [ ] Update `games/template-game/assets/main_menu.jsn` to add Credits button under Options and above Exit.
  - Status: Planned
  - Notes: Use `FoundationUiOrder` to preserve deterministic order.
- [ ] Update asset/reference tests for `main_menu.jsn`, scene constants, and file existence.
  - Status: Planned
  - Notes: Keep test coverage aligned with existing patterns.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

### Notes
- Preserve pre-existing unrelated changes in `games/template-game/.jsn/project.jsn` unless confirmed as part of this feature.

## Phase 4: Validation, Documentation, And Commit Checkpoints
**Status:** Planned  
**Goal:** Prove the feature is complete, documented, and ready for optional review.

### Tasks
- [ ] Add or update Rustdoc and any feature-level documentation needed for the credits JSON schema.
  - Status: Planned
  - Notes: A short docs page is recommended if Rustdoc is not enough for content authors.
- [ ] Run `scripts/format-project.cmd`.
  - Status: Pending
  - Notes: Required before completion.
- [ ] Run `scripts/lint-project.cmd`.
  - Status: Pending
  - Notes: Required before completion.
- [ ] Run `scripts/test-project.cmd`.
  - Status: Pending
  - Notes: Required before completion.
- [ ] Run `scripts/compile-project.cmd`.
  - Status: Pending
  - Notes: Required before completion.
- [ ] Run `scripts/doc-project.cmd`.
  - Status: Pending
  - Notes: Required before completion.
- [ ] Run `scripts/validate-project.cmd`.
  - Status: Pending
  - Notes: Required final validation wrapper unless waived.
- [ ] Commit completed tasks/phases and push to `origin` when available.
  - Status: Pending
  - Notes: Commit messages must follow `.pi/skills/gitflow-workflow/SKILL.md`.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending

### Notes
- Do not mark this phase complete until validation results and documentation generation are recorded.

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation.
- Use `gpt-5.5` for optional final review.
- Never use Anthropic models.
- Mandatory implementation pre-read: `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/feature-plan-docs/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, `.pi/skills/gitflow-workflow/SKILL.md`, `.pi/skills/foundation-architecture/SKILL.md`, this tracker, and `plan.md`.
- Before implementation edits, confirm active branch `feature/credits-scene`, record implementation start in this tracker, and verify branch base from `dev` when possible.
- Keep tracker updates in regular commits with code/assets.
- Treat `games/template-game/.jsn/project.jsn` as pre-existing unrelated working tree state until clarified.

## Postponed Work
- None.

## Open Issues And Questions
- Ownership split still needs final confirmation before implementation. Recommendation: reusable `foundation-runtime-library` credits component with TemplateGame-owned JSON and scene assets.
- Final credits copy is not provided. Initial implementation can use the user's sample JSON as placeholder content unless the user supplies final names/roles.
- End-of-roll behavior is unspecified. Recommendation: no automatic scene transition; remain closable by Back/Escape.

## Progress Log
- `2026-06-22`: Read mandatory planning, Rust workspace, Gitflow, and Foundation architecture skills.
- `2026-06-22`: Inspected workspace `Cargo.toml`, Foundation runtime menu/scene-stack systems, TemplateGame plugin/startup systems, TemplateGame Cargo manifest, and `main_menu.jsn`.
- `2026-06-22`: Created branch `feature/credits-scene` from `dev` for planning.
- `2026-06-22`: Plan and tracker created. Awaiting user review before implementation.
- `2026-06-22`: Updated plan and tracker to explicitly require unbounded recursive nested groups and depth-based shrinking group headers with minimum-size clamping.
