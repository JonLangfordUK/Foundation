# Gameplay Level And Pause Menu Plan

## Metadata
- Feature slug: `gameplay-level-pause-menu`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/gameplay-level-pause-menu`
- Branch status: `Created from dev; dev verified as an ancestor before planning edits`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-06-20`
- Last updated: `2026-06-20`

## User Request
Add a small gameplay level with a directional light and a cube in the center. Clicking `New Game` should load into that level as a fresh scene, clearing the scene stack. Add a pause menu with options to Resume, open Options, Quit to Menu, and Quit Game. Opening the pause menu should set a global resource that marks the game as paused so systems can react if they need to. As much logic as possible should live in FoundationLibrary; TemplateGame should mostly contain assets unless game-specific glue is unavoidable.

## Feature Summary
This feature adds a simple playable scene and reusable Foundation pause/menu primitives. The main menu's `New Game` action will clear the stack into a gameplay level. During gameplay, the player can open a pause menu overlay; the pause menu sets a global paused resource and supports resume, opening the existing options scene, returning to main menu, and quitting the game.

## Feature Area Classification
- Area: `multi-area` (`engine`, `game`, and `editor`)
- Primary area: `engine`
- Rationale: The reusable concepts are game-agnostic: pause state, pause-menu stack behavior, clear-stack scene navigation, and reusable menu actions. TemplateGame should mainly provide authored assets for the concrete sample level and pause menu.

## Codebase Research
- `Cargo.toml` is a workspace with `crates/foundation-library`, `crates/jackdaw-editor`, and `games/template-game`. Validation should use the wrapper scripts from `.pi/skills/rust-workspace-dev/SKILL.md`.
- `crates/foundation-library/src/scene_stack.rs` already supports `SceneCommand::ClearAndOpen`, `SceneCommand::CloseCurrent`, `ScenePresentation::PAUSE_OVERLAY`, `ScenePresentation::FULLSCREEN`, scene keys, and scene-owned cleanup via `SceneOwner`.
- `crates/foundation-library/src/menu.rs` currently contains reusable menu primitives: `FoundationMenuButton`, options menu generation, placeholder menu generation, `FoundationCloseOnEscape`, `FoundationExitRequested`, and generated UI markers. `FoundationMenuButton` currently supports `none`, `open_scene`, `close_current`, and `exit`; it does not yet support clear-stack navigation, pause overlays, resume/unpause semantics, or quit-to-menu semantics.
- `games/template-game/src/lib.rs` contains TemplateGame scene constants and minimal glue for editor/standalone exit handling. It loads Jackdaw `.jsn` scenes on `SceneLoadRequested`, targets UI roots to the editor viewport in editor Play mode, and opens startup/menu scenes.
- `games/template-game/assets/main_menu.jsn` now uses Foundation menu action components. The `New Game` button is currently a `none` action and should become a clear-stack open action to the new gameplay level.
- `games/template-game/assets/options_menu.jsn` already exists and is independent of the main menu, making it suitable for reuse from the pause menu.
- `games/template-game/assets/scene.jsn` contains an older Jackdaw scene with a brush; it can serve as a reference for serialized transform/scene format, but the requested level should be a fresh small gameplay asset with a directional light and centered cube.
- `games/template-game/.jsn/project.jsn` has an existing uncommitted local change from prior editor use. Planning and implementation should avoid committing or overwriting this machine/editor-local project state unless the user explicitly asks.

## External Research
No external online research was performed because the feature can be planned from existing Bevy, Jackdaw, and Foundation patterns already present in the repository.

## Affected Files And Systems
- `crates/foundation-library/src/menu.rs`: likely location for reusable pause resource, pause plugin behavior, extra menu actions, pause overlay actions, and generated pause menu primitives if runtime generation is needed.
- `crates/foundation-library/src/lib.rs`: export/register any new Foundation pause/menu types.
- `crates/foundation-library/src/scene_stack.rs`: possibly no changes needed, but may need helper APIs if Foundation menu actions need clear-stack/open-with-presentation behavior not expressible through current `FoundationMenuButton` fields.
- `games/template-game/src/lib.rs`: add scene path constants for gameplay level and pause menu; add minimal glue for opening pause menu from input if Foundation cannot fully own it; add editor play scene mappings for direct play from new assets.
- `games/template-game/assets/main_menu.jsn`: update `New Game` button to clear stack and open the gameplay level.
- `games/template-game/assets/gameplay_level.jsn`: new level asset containing a directional light and centered cube.
- `games/template-game/assets/pause_menu.jsn`: new pause menu asset, preferably using Foundation-owned pause/menu components and `TemplateGameplayUiRoot` for editor viewport targeting.
- `games/template-game/tests/template_components.rs`: update editor picker coverage if new authorable Foundation or TemplateGame components need visibility.
- `docs/plans/gameplay-level-pause-menu/*`: plan/tracker for this feature.

## Proposed Implementation Approach
1. Add Foundation pause state and reusable pause primitives.
   - Add a global resource such as `FoundationPauseState { paused: bool }` or `FoundationGamePaused(bool)` to FoundationLibrary and initialize it from `FoundationPlugin`.
   - Add public helper/run-condition APIs if useful, such as `foundation_not_paused` or `foundation_is_paused`, so gameplay systems can react without knowing the menu implementation.
   - Add reflected components/actions for pause-menu behavior where possible: open pause overlay, resume/close pause, clear-stack open scene, quit-to-menu, quit game.
2. Extend reusable menu actions.
   - Extend `FoundationMenuButton` in a backward-compatible way, likely using its existing string action model, to support actions such as `clear_and_open_scene`, `open_pause_scene`, `resume`, and possibly `set_paused`/`open_scene` with presentation.
   - Ensure `Resume` closes the pause scene and clears the pause resource.
   - Ensure `Quit to Menu` clears pause and clears the stack into `main_menu.jsn`.
   - Ensure `Quit Game` reuses existing `FoundationExitRequested`.
   - Ensure `Options` opens the existing options menu as a stack scene without hard-coding knowledge of the pause menu.
3. Add pause menu open behavior.
   - Prefer a Foundation-owned input component/resource/system that opens a configured pause menu on Escape during gameplay, sets the pause resource, and opens the pause menu with `ScenePresentation::PAUSE_OVERLAY` or an input-blocking/fullscreen presentation as appropriate.
   - Avoid opening pause while already paused or while non-gameplay menus are focused.
   - If a fully generic input solution is not practical, keep TemplateGame glue minimal: a scene marker or constant to configure the Foundation pause opener.
4. Add TemplateGame gameplay level asset.
   - Create `games/template-game/assets/gameplay_level.jsn` with a directional light and a centered cube.
   - Prefer authored Jackdaw/Bevy scene content over Rust code. If mesh/material handles are hard to author directly in `.jsn`, add minimal reusable Foundation/component-based runtime generation for a sample cube/light only if it is genuinely game-agnostic; otherwise document why the small TemplateGame glue is necessary.
5. Add TemplateGame pause menu asset.
   - Create `games/template-game/assets/pause_menu.jsn` with `Resume`, `Options`, `Quit to Menu`, and `Quit Game` buttons.
   - Use FoundationLibrary action components for button behavior.
   - Include `TemplateGameplayUiRoot` so editor viewport targeting still works.
6. Wire main menu New Game.
   - Update `main_menu.jsn` so `New Game` clears the stack and opens `gameplay_level.jsn` fresh.
   - Add `GAMEPLAY_LEVEL_SCENE` and `PAUSE_MENU_SCENE` constants in TemplateGame if needed for tests/editor direct play support.
7. Update editor direct-play handling.
   - Add the gameplay level and pause menu to `editor_play_scene_commands`/`editor_scene_key` if direct Play from these assets should be supported.
   - Ensure editor Play mode still clears runtime scenes and pause state on stop.
8. Tests and validation.
   - Add unit tests for pause resource defaults, action constructors/strings, and scene constants.
   - Run `scripts/validate-project.cmd` before completion.
   - Manual smoke testing should include New Game -> gameplay level, Escape -> pause, Resume, Options from pause and Back, Quit to Menu, and Quit Game.

## Alternatives Considered
- Put pause/menu logic in TemplateGame: rejected because the user explicitly wants game-agnostic logic in FoundationLibrary and TemplateGame mostly as assets.
- Make the pause menu know about the gameplay level/main menu directly: rejected because reusable menu actions should be generic and scene-path driven.
- Implement the gameplay level entirely in Rust: deferred unless Jackdaw `.jsn` cannot practically represent the requested light/cube; assets are preferred.

## Risks, Constraints, And Assumptions
- The exact Jackdaw `.jsn` serialization for Bevy mesh/material/directional-light entities may require inspection or iteration. If direct authored cube/light serialization is impractical, implementation should keep any runtime generation minimal and document why.
- `FoundationMenuButton` currently uses string actions. Extending this avoids enum serialization uncertainty but requires careful docs/tests for supported action strings.
- Pause state must be cleared on all resume/quit-to-menu/quit-game/scene stop paths to avoid stuck paused state.
- Escape currently closes any scene with `FoundationCloseOnEscape`; pause opening must avoid fighting with options menu Escape close.
- There is an uncommitted local editor/project layout change in `games/template-game/.jsn/project.jsn`; do not commit it unless explicitly requested.

## Open Questions
- What input should open the pause menu besides Escape, if any? Plan assumes Escape only for now.
- Should the gameplay level have a camera authored in the level, or use the existing default 2D camera plus 3D camera runtime support? Implementation should inspect whether a 3D camera is already present/needed and add the minimal required setup.

## Documentation Expectations
- New public Foundation pause/menu resources, components, and action strings must have Rustdoc comments.
- If Foundation exposes pause run conditions/helper functions, document how game systems should use them.
- Generated documentation must be produced before the feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Read this plan, `tracker.md`, `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, and `.pi/skills/gitflow-workflow/SKILL.md` before editing code.
- Confirm active branch is `feature/gameplay-level-pause-menu`; this branch was created from `dev` while carrying the user's uncommitted `games/template-game/.jsn/project.jsn` change. Do not include that file in commits unless explicitly requested.
- Keep game-agnostic logic in `crates/foundation-library`; TemplateGame should primarily receive `.jsn` assets and small path/editor glue.
- Update tracker before implementation edits and after validation/commits.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Verify pause state cannot get stuck true after resume, quit-to-menu, quit-game, or editor stop.
- Verify pause menu and options menu remain reusable and not hard-coded to each other beyond scene-path configuration.
- Verify New Game clears the stack rather than overlaying gameplay above menu scenes.
- Verify user-local `.jsn/project.jsn` was not accidentally committed.

## Success Criteria
- Clicking `New Game` clears the scene stack and opens a fresh gameplay level.
- Gameplay level contains a centered cube and directional light and is visible/playable in standalone runtime.
- Pressing Escape during gameplay opens a pause menu overlay and sets a global Foundation pause resource.
- Pause menu has Resume, Options, Quit to Menu, and Quit Game.
- Resume closes the pause menu and clears paused state.
- Options opens the existing options menu from pause and can return to pause.
- Quit to Menu clears paused state and clears/opens the main menu.
- Quit Game exits standalone runtime and stops Play mode in editor, reusing existing exit semantics.
- Game-agnostic logic lives in FoundationLibrary; TemplateGame is mostly assets and minimal glue.
- `scripts/validate-project.cmd` passes or any waiver is explicitly recorded.

## Testing Methodology
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- `scripts/doc-project.cmd`
- `scripts/validate-project.cmd` for the full validation sequence when practical
- Manual smoke test recommended:
  - standalone: main menu -> New Game -> gameplay level, Escape -> pause menu, Resume, Escape -> pause -> Options -> Back, Quit to Menu, Quit Game;
  - editor: direct Play from gameplay/pause assets if supported, verify Quit Game stops Play not editor.
