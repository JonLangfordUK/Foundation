# UI Refinement Tracker

## Metadata
- Feature slug: `ui-refinement`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/ui-refinement`
- Branch status: `Created from dev; dev verified as an ancestor before planning edits and again before implementation edits`
- Overall status: `Implemented; validation passed`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for gpt-5.5 sanity review or user acceptance`
- Created: `2026-06-20`
- Last updated: `2026-06-20`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Use project wrappers by default: `scripts/format-project.cmd`, `scripts/lint-project.cmd`, `scripts/test-project.cmd`, `scripts/compile-project.cmd`, `scripts/doc-project.cmd`, and `scripts/validate-project.cmd` when a full validation sequence is practical.

## Phase 1: Splash Contract Cleanup
**Status:** Complete  
**Goal:** Remove `FoundationSplashScreen.text` and make splash copy come from authored text components.

### Tasks
- [x] Remove `text` from `FoundationSplashScreen` and update constructor/defaults/Rustdoc.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: `FoundationSplashScreen::new()` no longer accepts text; visible copy is documented as owned by authored `Text` + `FoundationSplashText`.
- [x] Update splash runtime fallback/generation path so it no longer depends on component-owned text.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: Generated fallback text is empty and no reflected text property was reintroduced.
- [x] Remove serialized `text` fields from `splash_pixel_perfect.jsn` and `splash_bevy.jsn`.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: Existing authored `Text` components remain the visible splash copy.
- [x] Update splash tests and any affected public API expectations.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: Splash tests now use `FoundationSplashScreen::new()`.

### Validation
- Format: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Lint: Passed (`scripts/validate-project.cmd`, 2026-06-20; fixed initial `clippy::type_complexity` failure in `menu.rs` before final pass)
- Tests: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Build: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Documentation generation: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Full validation wrapper: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- User confirmation: Not required for this phase

## Phase 2: Main Menu Actions And Exit Behavior
**Status:** Complete  
**Goal:** Turn main-menu buttons into action-bearing controls and implement standalone/editor exit semantics.

### Tasks
- [x] Add reflected main-menu action data for `New Game`, `Load Game`, `Options`, and `Exit`.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: Added reusable `FoundationMenuButton` in FoundationLibrary; TemplateGame uses it through assets.
- [x] Update `main_menu.jsn` buttons with actions and rename/treat `Quit` as `Exit`.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: `Quit` button/label renamed to `Exit`; buttons now carry Foundation action data.
- [x] Implement button pressed handling for `Options`, `Load Game`, and `Exit` while preserving hover/pressed visuals.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: `New Game` remains a no-op placeholder.
- [x] Implement standalone exit by writing Bevy app-exit through the Bevy 0.18-compatible API.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: Standalone TemplateGame translates `FoundationExitRequested` to `AppExit::Success`.
- [x] Implement editor exit as stop Play mode only, leaving the editor process open.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: Editor TemplateGame translates `FoundationExitRequested` to `NextState<PlayState>::set(Stopped)`.

### Validation
- Format: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Lint: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Tests: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Build: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Documentation generation: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Full validation wrapper: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- User confirmation: Not required for this phase

## Phase 3: Options Menu Scene
**Status:** Complete  
**Goal:** Add a reusable stack-based options menu scene with tabs, dummy settings, Escape close, and Back button close.

### Tasks
- [x] Add `OPTIONS_MENU_SCENE` constant and new reflected options menu scene components/markers.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: Options behavior lives in FoundationLibrary via `FoundationOptionsMenu`; TemplateGame adds the asset path constant.
- [x] Add `games/template-game/assets/options_menu.jsn` as a Jackdaw-authored marker/UI scene.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: Asset includes `TemplateGameplayUiRoot`, `FoundationOptionsMenu`, and `FoundationCloseOnEscape`.
- [x] Build or initialize the options UI with horizontal tabs: `Gameplay`, `Display`, `Graphics`, `Accessibility`.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: FoundationLibrary generates the tab UI from the scene marker/root.
- [x] Show five dummy setting rows for each selected tab with label on left and value setter/control on right.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: Rows display `Property N` labels and `< Value N >` dummy setter text.
- [x] Implement tab selection and visual feedback for selected/hovered/pressed states.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: Tab state is scene-owned in `FoundationOptionsRuntime`; generated UI inherits `SceneOwner` for cleanup.
- [x] Implement Escape and Back button close using `SceneCommand::CloseCurrent`.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: Close behavior is stack-relative and has no main-menu dependency.
- [x] Add tests for constants/defaults and close/action behavior where practical.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: Added Foundation menu constructor/tab-order tests and TemplateGame path constant coverage.

### Validation
- Format: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Lint: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Tests: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Build: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Documentation generation: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Full validation wrapper: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- User confirmation: Not required for this phase

## Phase 4: Dummy Load Game Scene
**Status:** Complete  
**Goal:** Add a placeholder load-game scene opened from the main menu.

### Tasks
- [x] Add `LOAD_GAME_SCENE` constant and any needed dummy load-game marker/component.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: Placeholder behavior uses reusable FoundationLibrary `FoundationPlaceholderMenu`; TemplateGame adds the asset path constant.
- [x] Add `games/template-game/assets/load_game.jsn` as a stack scene with placeholder content.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: Asset includes `TemplateGameplayUiRoot`, `FoundationPlaceholderMenu`, and `FoundationCloseOnEscape`; Back button is generated by FoundationLibrary.
- [x] Wire `Load Game` main-menu action to open the dummy load-game scene on the stack.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: `main_menu.jsn` opens `load_game.jsn` through `FoundationMenuButton`.
- [x] Implement Back close and optionally Escape close for consistency.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: Back and Escape both close the current scene.

### Validation
- Format: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Lint: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Tests: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Build: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Documentation generation: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Full validation wrapper: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- User confirmation: Not required for this phase

## Phase 5: Editor Integration, Validation, And Handoff
**Status:** Complete  
**Goal:** Ensure new scenes work in standalone and editor Play mode, complete validation, and prepare review/handoff.

### Tasks
- [x] Add options/load-game scenes to editor play scene selection helpers.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: `editor_play_scene_commands` and `editor_scene_key` now include options/load-game scenes.
- [x] Ensure new UI roots are targeted/parented through existing `TemplateGameplayUiRoot` and viewport targeting patterns.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: New assets include `TemplateGameplayUiRoot`; generated menu UI inherits scene ownership for cleanup.
- [x] Update docs or Rustdoc for changed public APIs and generated UI assumptions.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: Added Rustdoc for reusable Foundation menu primitives and updated splash Rustdoc.
- [x] Run full validation and record results.
  - Status: Complete
  - Validation: Passed via `scripts/validate-project.cmd` on 2026-06-20.
  - Notes: Initial full validation found a clippy `type_complexity` warning; fixed with a query type alias and reran successfully.
- [x] Perform recommended manual smoke tests or record why they were skipped.
  - Status: Complete
  - Validation: Not run; non-interactive harness.
  - Notes: Manual standalone/editor UI smoke testing is recommended for the user after launch.
- [x] Commit completed work and push to `origin` if available.
  - Status: Complete
  - Validation: Committed and pushed to `origin/feature/ui-refinement` on 2026-06-20.
  - Notes: Implementation commit `7d4ec8b` pushed successfully; this tracker follow-up records final commit/push state.

### Validation
- Format: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Lint: Passed (`scripts/validate-project.cmd`, 2026-06-20; fixed initial `clippy::type_complexity` failure in `menu.rs` before final pass)
- Tests: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Build: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Documentation generation: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- Full validation wrapper: Passed (`scripts/validate-project.cmd`, 2026-06-20)
- User confirmation: Not required for this phase

## Implementation / Review Handoff Notes
- Implementation used `gpt-5.4`; never use Anthropic models.
- Review must use `gpt-5.5`; never use Anthropic models.
- The active branch is `feature/ui-refinement`; `dev` was verified as an ancestor before implementation edits.
- Game-agnostic code was placed in `crates/foundation-library`; `games/template-game` changes are primarily assets plus small scene-path/editor-exit glue.
- Manual UI smoke tests were not run in the non-interactive harness; recommended before accepting without final review.

## Notes / Issues / Oversights
- Existing main-menu asset used `Quit Button`; implementation renamed it to `Exit Button` and label `Exit`.
- Existing `New Game` button behavior remains placeholder/no-op.
- User clarified that game-agnostic code should go in FoundationLibrary, and the game should mostly receive assets unless that is not possible.
- Load-game Escape close was implemented for consistency.
- Initial validation issue: `scripts/validate-project.cmd` failed on `clippy::type_complexity` for `GeneratedMenuUiWithoutOwnerQuery`; fixed by adding the type alias and rerunning successfully.

## Postponed Work
- Real gameplay settings persistence is postponed; options values are intentionally dummy placeholders.
- Real load-game/save-slot behavior is postponed; scene is intentionally a dummy placeholder.
- Pause-menu integration is postponed; options menu is independent so it can be reused later.

## Progress Log
- `2026-06-20`: User requested UI refinement feature and confirmed summary.
- `2026-06-20`: Read mandatory feature planning, Rust workspace, and gitflow skills.
- `2026-06-20`: Inspected workspace manifest, relevant FoundationLibrary and TemplateGame source files, scene-stack docs, and existing Jackdaw `.jsn` UI scenes.
- `2026-06-20`: Created branch `feature/ui-refinement` from `dev` and verified `dev` is an ancestor.
- `2026-06-20`: Created `docs/plans/ui-refinement/plan.md` and `docs/plans/ui-refinement/tracker.md`.
- `2026-06-20`: Updated plan/tracker after user clarified that game-agnostic code belongs in FoundationLibrary and TemplateGame should mostly contain assets.
- `2026-06-20`: User approved implementation; planning docs committed and pushed (`3354c38`, `origin/feature/ui-refinement`). Implementation started with `gpt-5.4`; active branch confirmed as `feature/ui-refinement` and `dev` verified as an ancestor.
- `2026-06-20`: Implemented FoundationLibrary menu primitives, splash contract cleanup, options/load-game scenes, and standalone/editor exit handling.
- `2026-06-20`: First full validation failed on `clippy::type_complexity` in `crates/foundation-library/src/menu.rs`; added `GeneratedMenuUiWithoutOwnerQuery` alias.
- `2026-06-20`: Full validation passed with `scripts/validate-project.cmd`. Manual UI smoke tests were not run in the non-interactive harness.
- `2026-06-20`: Implementation committed and pushed to `origin/feature/ui-refinement` as `7d4ec8b`.
