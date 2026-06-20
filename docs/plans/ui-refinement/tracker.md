# UI Refinement Tracker

## Metadata
- Feature slug: `ui-refinement`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/ui-refinement`
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

## Phase 1: Splash Contract Cleanup
**Status:** Planned  
**Goal:** Remove `FoundationSplashScreen.text` and make splash copy come from authored text components.

### Tasks
- [ ] Remove `text` from `FoundationSplashScreen` and update constructor/defaults/Rustdoc.
  - Status: Planned
  - Validation: Pending
  - Notes: Preserve timing and next-scene behavior.
- [ ] Update splash runtime fallback/generation path so it no longer depends on component-owned text.
  - Status: Planned
  - Validation: Pending
  - Notes: Prefer authored `FoundationSplashText`; if fallback remains, it should not reintroduce a reflected text property.
- [ ] Remove serialized `text` fields from `splash_pixel_perfect.jsn` and `splash_bevy.jsn`.
  - Status: Planned
  - Validation: Pending
  - Notes: Keep existing authored `Text` components for visible copy.
- [ ] Update splash tests and any affected public API expectations.
  - Status: Planned
  - Validation: Pending
  - Notes: Existing tests construct `FoundationSplashScreen::new("...")` and must change.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

## Phase 2: Main Menu Actions And Exit Behavior
**Status:** Planned  
**Goal:** Turn main-menu buttons into action-bearing controls and implement standalone/editor exit semantics.

### Tasks
- [ ] Add reflected main-menu action data for `New Game`, `Load Game`, `Options`, and `Exit`.
  - Status: Planned
  - Validation: Pending
  - Notes: Existing `TemplateMenuButton` is only a marker; prefer FoundationLibrary-owned reusable action/menu components, with TemplateGame-specific glue only if needed.
- [ ] Update `main_menu.jsn` buttons with actions and rename/treat `Quit` as `Exit`.
  - Status: Planned
  - Validation: Pending
  - Notes: Open question in plan asks whether to rename; default implementation should rename for clarity.
- [ ] Implement button pressed handling for `Options`, `Load Game`, and `Exit` while preserving hover/pressed visuals.
  - Status: Planned
  - Validation: Pending
  - Notes: `New Game` may remain a no-op unless implementation chooses a harmless placeholder.
- [ ] Implement standalone exit by writing Bevy app-exit through the Bevy 0.18-compatible API.
  - Status: Planned
  - Validation: Pending
  - Notes: Do not use `std::process::exit` from gameplay UI systems.
- [ ] Implement editor exit as stop Play mode only, leaving the editor process open.
  - Status: Planned
  - Validation: Pending
  - Notes: Use existing Jackdaw `PlayState` pattern or a more specific Jackdaw stop-play mechanism if found during implementation.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

## Phase 3: Options Menu Scene
**Status:** Planned  
**Goal:** Add a reusable stack-based options menu scene with tabs, dummy settings, Escape close, and Back button close.

### Tasks
- [ ] Add `OPTIONS_MENU_SCENE` constant and new reflected options menu scene components/markers.
  - Status: Planned
  - Validation: Pending
  - Notes: Game-agnostic options menu components/behavior should live in FoundationLibrary; TemplateGame should mostly provide assets/paths.
- [ ] Add `games/template-game/assets/options_menu.jsn` as a Jackdaw-authored marker/UI scene.
  - Status: Planned
  - Validation: Pending
  - Notes: Include `TemplateGameplayUiRoot` so editor viewport targeting works.
- [ ] Build or initialize the options UI with horizontal tabs: `Gameplay`, `Display`, `Graphics`, `Accessibility`.
  - Status: Planned
  - Validation: Pending
  - Notes: Runtime generation is acceptable if the `.jsn` owns the scene marker/root.
- [ ] Show five dummy setting rows for each selected tab with label on left and value setter/control on right.
  - Status: Planned
  - Validation: Pending
  - Notes: Dummy controls should not imply persisted settings.
- [ ] Implement tab selection and visual feedback for selected/hovered/pressed states.
  - Status: Planned
  - Validation: Pending
  - Notes: Keep state scene-owned so closing the scene cleans it up.
- [ ] Implement Escape and Back button close using `SceneCommand::CloseCurrent`.
  - Status: Planned
  - Validation: Pending
  - Notes: Must not explicitly reopen or reference the main menu.
- [ ] Add tests for constants/defaults and close/action behavior where practical.
  - Status: Planned
  - Validation: Pending
  - Notes: UI interaction may require focused unit coverage plus manual smoke testing.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

## Phase 4: Dummy Load Game Scene
**Status:** Planned  
**Goal:** Add a placeholder load-game scene opened from the main menu.

### Tasks
- [ ] Add `LOAD_GAME_SCENE` constant and any needed dummy load-game marker/component.
  - Status: Planned
  - Validation: Pending
  - Notes: Prefer reusable FoundationLibrary close/back/action primitives; keep TemplateGame code asset-focused.
- [ ] Add `games/template-game/assets/load_game.jsn` as a stack scene with placeholder content.
  - Status: Planned
  - Validation: Pending
  - Notes: Include `TemplateGameplayUiRoot` and a Back button if implemented.
- [ ] Wire `Load Game` main-menu action to open the dummy load-game scene on the stack.
  - Status: Planned
  - Validation: Pending
  - Notes: Use scene-stack open command rather than hard-coded main-menu replacement.
- [ ] Implement Back close and optionally Escape close for consistency.
  - Status: Planned
  - Validation: Pending
  - Notes: Escape close is not explicitly required for load-game, but may be useful.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

## Phase 5: Editor Integration, Validation, And Handoff
**Status:** Planned  
**Goal:** Ensure new scenes work in standalone and editor Play mode, complete validation, and prepare review/handoff.

### Tasks
- [ ] Add options/load-game scenes to editor play scene selection helpers.
  - Status: Planned
  - Validation: Pending
  - Notes: Update `editor_play_scene_commands` and `editor_scene_key` for direct Play from these scenes.
- [ ] Ensure new UI roots are targeted/parented through existing `TemplateGameplayUiRoot` and viewport targeting patterns.
  - Status: Planned
  - Validation: Pending
  - Notes: Required for editor Play mode.
- [ ] Update docs or Rustdoc for changed public APIs and generated UI assumptions.
  - Status: Planned
  - Validation: Pending
  - Notes: At minimum, update `FoundationSplashScreen` documentation.
- [ ] Run full validation and record results.
  - Status: Planned
  - Validation: Pending
  - Notes: Use wrapper scripts unless blocked.
- [ ] Perform recommended manual smoke tests or record why they were skipped.
  - Status: Planned
  - Validation: Pending
  - Notes: Standalone and editor Play-mode checks are recommended.
- [ ] Commit completed work and push to `origin` if available.
  - Status: Planned
  - Validation: Pending
  - Notes: Use gitflow commit message format with files changed.

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
- Follow the project architecture convention: game-agnostic code belongs in `crates/foundation-library`; `games/template-game` should mostly contain assets and only contain game-specific glue when FoundationLibrary cannot reasonably own the behavior.
- Confirm the active branch is `feature/ui-refinement` and record implementation start in this tracker before code edits.
- Keep tracker progress, validation state, issues, postponements, and push status updated throughout implementation.
- Do not mark phases complete until required validation and documentation generation pass or a documented waiver exists.

## Notes / Issues / Oversights
- Existing main-menu asset uses `Quit Button`; user called it `exit button`. Plan assumes renaming to `Exit` is acceptable unless the user says otherwise.
- Existing `New Game` button behavior remains unspecified and can stay placeholder/no-op.
- User clarified that game-agnostic code should go in FoundationLibrary, and the game should mostly receive assets unless that is not possible.
- Load-game Escape close is not required by user but may be implemented for consistency.

## Postponed Work
- Real gameplay settings persistence is postponed; options values are intentionally dummy placeholders.
- Real load-game/save-slot behavior is postponed; scene is intentionally a dummy placeholder.
- Pause-menu integration is postponed; options menu should be independent so it can be reused later.

## Progress Log
- `2026-06-20`: User requested UI refinement feature and confirmed summary.
- `2026-06-20`: Read mandatory feature planning, Rust workspace, and gitflow skills.
- `2026-06-20`: Inspected workspace manifest, relevant FoundationLibrary and TemplateGame source files, scene-stack docs, and existing Jackdaw `.jsn` UI scenes.
- `2026-06-20`: Created branch `feature/ui-refinement` from `dev` and verified `dev` is an ancestor.
- `2026-06-20`: Created `docs/plans/ui-refinement/plan.md` and `docs/plans/ui-refinement/tracker.md`.
- `2026-06-20`: Updated plan/tracker after user clarified that game-agnostic code belongs in FoundationLibrary and TemplateGame should mostly contain assets.
