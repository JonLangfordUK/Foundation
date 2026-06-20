# Gameplay Level And Pause Menu Tracker

## Metadata
- Feature slug: `gameplay-level-pause-menu`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/gameplay-level-pause-menu`
- Branch status: `Created from dev; dev verified as an ancestor before planning edits`
- Overall status: `Planned`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for user review before gpt-5.4 implementation`
- Created: `2026-06-20`
- Last updated: `2026-06-20`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Use project wrappers by default: `scripts/format-project.cmd`, `scripts/lint-project.cmd`, `scripts/test-project.cmd`, `scripts/compile-project.cmd`, `scripts/doc-project.cmd`, and `scripts/validate-project.cmd` when a full validation sequence is practical.

## Phase 1: Foundation Pause And Menu Actions
**Status:** Planned  
**Goal:** Add reusable Foundation pause state and scene-stack menu actions needed by gameplay and pause menus.

### Tasks
- [ ] Add a global Foundation pause resource and helper/run-condition APIs.
  - Status: Planned
  - Validation: Pending
  - Notes: Resource should be game-agnostic and cleared on resume/menu transitions.
- [ ] Extend reusable Foundation menu actions for clear-stack scene opening and pause/resume behavior.
  - Status: Planned
  - Validation: Pending
  - Notes: Existing `FoundationMenuButton` supports `none`, `open_scene`, `close_current`, and `exit`; new behavior should remain asset-authorable.
- [ ] Add reusable pause-menu open behavior.
  - Status: Planned
  - Validation: Pending
  - Notes: Prefer Foundation-owned Escape-to-pause support configured by scene path; avoid conflicts with options menu Escape close.
- [ ] Add tests for pause defaults and new Foundation action behavior.
  - Status: Planned
  - Validation: Pending
  - Notes: Include docs for new action strings/components.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

## Phase 2: Gameplay Level Asset And New Game Flow
**Status:** Planned  
**Goal:** Add the small gameplay level and make New Game clear the stack into it.

### Tasks
- [ ] Add `gameplay_level.jsn` with centered cube and directional light.
  - Status: Planned
  - Validation: Pending
  - Notes: Prefer authored asset content; add minimal runtime generation only if direct asset representation is not practical.
- [ ] Wire `New Game` in `main_menu.jsn` to clear stack and open the gameplay level.
  - Status: Planned
  - Validation: Pending
  - Notes: Use Foundation action data, not TemplateGame-specific button systems.
- [ ] Add TemplateGame scene path constants/tests and editor direct-play mapping for gameplay level if needed.
  - Status: Planned
  - Validation: Pending
  - Notes: Keep TemplateGame Rust changes minimal.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

## Phase 3: Pause Menu Asset And Pause Flow
**Status:** Planned  
**Goal:** Add pause menu with Resume, Options, Quit to Menu, and Quit Game, backed by Foundation pause state.

### Tasks
- [ ] Add `pause_menu.jsn` using Foundation reusable action components.
  - Status: Planned
  - Validation: Pending
  - Notes: Include `TemplateGameplayUiRoot` for editor viewport targeting.
- [ ] Implement Resume to close pause menu and clear pause state.
  - Status: Planned
  - Validation: Pending
  - Notes: Should not reload gameplay.
- [ ] Implement Options from pause using existing `options_menu.jsn` as another stack scene.
  - Status: Planned
  - Validation: Pending
  - Notes: Existing options scene should remain independent and reusable.
- [ ] Implement Quit to Menu to clear pause and clear/open main menu.
  - Status: Planned
  - Validation: Pending
  - Notes: Must clear the gameplay stack.
- [ ] Implement Quit Game via existing Foundation exit request path.
  - Status: Planned
  - Validation: Pending
  - Notes: Standalone exits; editor stops Play.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

## Phase 4: Integration, Validation, And Handoff
**Status:** Planned  
**Goal:** Ensure standalone/editor flows work, validate, commit, and prepare handoff.

### Tasks
- [ ] Ensure pause state is cleared on editor stop and scene transitions.
  - Status: Planned
  - Validation: Pending
  - Notes: Avoid stuck paused state.
- [ ] Ensure user-local `games/template-game/.jsn/project.jsn` change is not committed unless requested.
  - Status: Planned
  - Validation: Pending
  - Notes: This file was dirty before planning and appears to be editor-local layout/project state.
- [ ] Run full validation and record results.
  - Status: Planned
  - Validation: Pending
  - Notes: Use `scripts/validate-project.cmd` unless blocked.
- [ ] Perform recommended manual smoke tests or record why skipped.
  - Status: Planned
  - Validation: Pending
  - Notes: New Game, pause, resume, options, quit-to-menu, quit-game.
- [ ] Commit completed work and push to `origin` if available.
  - Status: Planned
  - Validation: Pending
  - Notes: Use gitflow commit message format and include plan/tracker updates.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending

## Implementation / Review Handoff Notes
- Implementation must use `gpt-5.4`; never use Anthropic models.
- Review must use `gpt-5.5`; never use Anthropic models.
- Before implementation edits, read this tracker, `plan.md`, `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/feature-plan-docs/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, and `.pi/skills/gitflow-workflow/SKILL.md`.
- Confirm active branch is `feature/gameplay-level-pause-menu` and record implementation start before code edits.
- Keep game-agnostic logic in `crates/foundation-library`; `games/template-game` should mostly contain `.jsn` assets and minimal constants/editor glue.
- Do not commit `games/template-game/.jsn/project.jsn` unless the user explicitly asks.

## Notes / Issues / Oversights
- `games/template-game/.jsn/project.jsn` was already modified before this planning branch was created. It appears to contain editor/project layout changes and should remain uncommitted unless requested.
- The exact asset serialization for a centered cube and directional light may require iteration; prefer assets over game-specific Rust code.
- Pause opener should avoid interfering with `FoundationCloseOnEscape` on options/load-game/pause scenes.

## Postponed Work
- Real gameplay mechanics beyond a simple lit cube level are postponed.
- Real settings persistence remains postponed; options are still dummy placeholders.
- Save/load functionality remains postponed.

## Progress Log
- `2026-06-20`: User requested gameplay level, New Game flow, pause menu, and global pause resource, with reusable logic in FoundationLibrary.
- `2026-06-20`: Read mandatory feature planning, Rust workspace, and gitflow skills.
- `2026-06-20`: Inspected workspace manifest, Foundation scene-stack/menu code, TemplateGame runtime glue, existing assets, and current git state.
- `2026-06-20`: Created branch `feature/gameplay-level-pause-menu` from `dev` and verified `dev` is an ancestor. Existing uncommitted `games/template-game/.jsn/project.jsn` change carried over and must not be committed unless requested.
- `2026-06-20`: Created `docs/plans/gameplay-level-pause-menu/plan.md` and `docs/plans/gameplay-level-pause-menu/tracker.md`.
