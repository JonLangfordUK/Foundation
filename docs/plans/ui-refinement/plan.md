# UI Refinement Plan

## Metadata
- Feature slug: `ui-refinement`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/ui-refinement`
- Branch status: `Created from dev; dev verified as an ancestor before planning edits`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-06-20`
- Last updated: `2026-06-20`

## User Request
Refine the TemplateGame UI as one feature:
1. Remove the `text` property from `FoundationSplashScreen`; splash text should come from the authored Bevy `Text` component.
2. Make the main-menu exit button close the standalone game, while in editor Play mode it should stop Play without closing the editor.
3. Add an options menu as an independent scene on the scene stack. It needs horizontal tabs `Gameplay`, `Display`, `Graphics`, and `Accessibility`, each showing 5 dummy setting rows with the property name on the left and a value setter on the right. It closes with Escape or a Back button and returns to the previous scene.
4. Add a dummy load-game scene opened by clicking `Load Game`.

## Feature Summary
This feature turns the current static main-menu button visuals into real navigation/actions, adds reusable stack-based menu scenes for options and load-game placeholders, and cleans up splash data so authored text entities own the displayed copy.

## Feature Area Classification
- Area: `multi-area` (`engine`, `game`, and `editor`)
- Primary area: `engine`
- Rationale: The visible deliverables are TemplateGame UI scenes/assets and main-menu behavior, but reusable/game-agnostic menu/navigation code should live in FoundationLibrary. The feature also changes FoundationLibrary splash-screen API/data and editor Play-mode behavior for the exit action.

## Codebase Research
- `Cargo.toml` is a workspace with `crates/foundation-library`, `crates/jackdaw-editor`, and `games/template-game` members. Validation should use the project wrapper scripts.
- `crates/foundation-library/src/splash_screen.rs` defines `FoundationSplashScreen` with `text`, `timings`, `font_size`, `next_scene_path`, `reset_stack_for_next_scene`, and `replace_current_scene` fields. The module already supports authored `FoundationSplashUiRoot` and `FoundationSplashText` entities; if both exist, generated UI is skipped and the authored `Text` value can be faded directly.
- `spawn_generated_splash_ui` currently uses `splash.text.clone()` to generate fallback text. Removing `text` means generated fallback UI either needs a safe placeholder-free strategy or should only create generic empty/default text where no authored text exists. Because current splash assets contain authored text entities, implementation should prefer authored text and update docs/tests accordingly.
- `games/template-game/assets/splash_pixel_perfect.jsn` and `games/template-game/assets/splash_bevy.jsn` serialize `foundation_library::splash_screen::FoundationSplashScreen` with a `text` property. These assets must be updated to remove that serialized field while preserving authored `bevy_ui::widget::text::Text` child components.
- `games/template-game/src/lib.rs` currently registers `TemplateMainMenu` and `TemplateMenuButton`, initializes/attaches main menu UI roots, and only updates button hover/pressed colors. Menu buttons have no action data yet.
- `games/template-game/assets/main_menu.jsn` has buttons labeled `New Game`, `Load Game`, `Options`, and `Quit`. The user specifically requested exit behavior; implementation should reconcile the existing `Quit` label with the requested `Exit` behavior, either by treating it as the exit button or renaming it to `Exit` for clarity.
- `foundation_library::scene_stack` already supports `SceneCommand::Open`, `CloseCurrent`, `ClearAndOpen`, `SceneSource::JsnLevel`, `SceneSource::Runtime`, scene keys, and presentation options. Options/load-game can be opened as stack scenes and closed generically without hard-coding knowledge of the main menu.
- Editor builds of `TemplateGamePlugin` run scene-stack and UI systems only during `jackdaw::prelude::PlayState::Playing`, enable splash runtime on enter, and clear the stack on exit. A play-mode exit action should transition PlayState out of `Playing` instead of writing `AppExit`.
- Standalone `games/template-game/src/main.rs` returns `AppExit` from `App::run()`. Runtime exit behavior should use Bevy's exit message/resource path from a gameplay system rather than `std::process::exit`.

## External Research
No external online research was performed because the feature can be planned from existing Bevy/Jackdaw/Foundation patterns already present in the repository.

## Affected Files And Systems
- `crates/foundation-library/src/splash_screen.rs`: remove `FoundationSplashScreen::text`, adjust constructor/default/tests/docs, and ensure splash fading uses authored `FoundationSplashText`/`Text` data.
- `crates/foundation-library/src/lib.rs`: expose any new reusable menu/navigation/settings-scene primitives through the prelude when appropriate.
- `crates/foundation-library/src/*`: place game-agnostic scene-stack menu code here, including reusable button actions, stack-close/back behavior, options-tab scaffolding, dummy setting-row scaffolding, and app/editor exit abstractions where practical.
- `games/template-game/src/lib.rs`: keep changes minimal; primarily wire TemplateGame-specific scene constants/assets and only add code here when the behavior cannot reasonably be owned by FoundationLibrary.
- `games/template-game/assets/main_menu.jsn`: assign actions to menu buttons and possibly rename `Quit` to `Exit`.
- `games/template-game/assets/options_menu.jsn`: new authored stack scene or marker scene for options UI, using FoundationLibrary reusable components where possible.
- `games/template-game/assets/load_game.jsn`: new authored stack scene or marker scene for dummy load-game UI, using FoundationLibrary reusable components where possible.
- `games/template-game/assets/splash_pixel_perfect.jsn` and `games/template-game/assets/splash_bevy.jsn`: remove serialized `text` property from `FoundationSplashScreen` components.
- `games/template-game/tests/template_components.rs`: expand editor pickability smoke test if new reflected components/actions should appear in Jackdaw.
- `docs/foundation-scene-stack.md` or new docs if needed: update only if scene-stack usage documentation must mention reusable menu scenes.

## Proposed Implementation Approach
1. Update the splash component contract.
   - Remove `text: String` from `FoundationSplashScreen` and update Rustdoc to state the visible copy comes from an authored `FoundationSplashText` entity's `Text` component.
   - Change `FoundationSplashScreen::new` to no longer require text, or replace it with a no-argument constructor such as `FoundationSplashScreen::new()`.
   - Update default/tests and the generated UI fallback path so it no longer depends on component-owned text. Prefer failing gracefully or generating empty fallback text while warning that authored text is expected.
   - Remove `text` from both splash `.jsn` files.
2. Add explicit main-menu actions using Foundation-owned reusable primitives where possible.
   - Prefer a FoundationLibrary reflected menu/action component over a TemplateGame-only action component for game-agnostic behaviors such as open scene, close current scene/back, and exit/stop-play.
   - Keep TemplateGame-specific code limited to asset paths/constants or adapters that cannot reasonably be made reusable.
   - Update `main_menu.jsn` button components to serialize the correct action values.
   - Extend interaction handling so `Interaction::Pressed` performs the action once on changed interaction.
3. Implement exit behavior.
   - In standalone builds, write Bevy's app-exit message when the exit action is pressed.
   - In editor builds, set Jackdaw PlayState away from `Playing` (or use the editor's existing supported stop-play mechanism if exposed) so the editor remains open and `OnExit(PlayState::Playing)` clears runtime scenes.
4. Add independent options menu scene.
   - Prefer FoundationLibrary-owned reflected marker/component(s) for reusable options menu behavior, such as an options menu root, back button, tab buttons, and setting rows. Add TemplateGame-specific wrappers only if required by Jackdaw asset paths or serialization constraints.
   - Add `OPTIONS_MENU_SCENE` constant in TemplateGame only if needed to point the main-menu asset/action at the game asset.
   - Open it from the main menu using `SceneCommand::open_with_options(SceneSource::jsn_level(OPTIONS_MENU_SCENE), OpenSceneOptions::default().with_key("options-menu").with_presentation(ScenePresentation::FULLSCREEN or INPUT_BLOCKING_OVERLAY))` depending on desired visual coverage. Since it is a standalone menu scene, do not reference the main menu from its close logic; closing should write `SceneCommand::CloseCurrent`.
   - Build/render four horizontal tabs: `Gameplay`, `Display`, `Graphics`, `Accessibility`.
   - Maintain selected tab state in a scene-owned runtime component/resource; changing tabs updates the displayed dummy properties.
   - For each tab, show 5 rows with property label left and dummy value setter/control right. The setter can be placeholder text/buttons such as `< Value 1 >` so no real settings persistence is implied.
   - Close on Escape or Back button by issuing `SceneCommand::CloseCurrent`.
5. Add dummy load-game scene.
   - Prefer FoundationLibrary reusable back/close/action components for the placeholder scene.
   - Add `LOAD_GAME_SCENE` constant and a TemplateGame marker only if the game asset needs it and FoundationLibrary primitives are insufficient.
   - Open it from `Load Game` using the scene stack.
   - Show placeholder text and a Back button; close via Back and optionally Escape for consistency unless intentionally scoped otherwise.
6. Keep editor viewport behavior consistent.
   - Ensure new scene UI roots use `TemplateGameplayUiRoot`, `attach_gameplay_ui_root`, and `should_process_runtime_scene_entity` patterns so they work in standalone and editor Play mode.
   - Add new scene names to `editor_play_scene_commands` and `editor_scene_key` so opening those assets directly in editor Play mode is sensible.
7. Update tests.
   - Adjust splash tests for no text field.
   - Add tests for menu action defaults/constants and initial scene path constants.
   - Add editor component-picker smoke coverage for new reflected components if they are user-authorable.

## Alternatives Considered
- Generate all options/load-game UI entirely from Rust with `SceneSource::Runtime`: rejected for now because the current TemplateGame scene-stack flow is built around Jackdaw `.jsn` marker scenes, and the request describes scenes on the stack.
- Let the options menu know it was opened from the main menu and explicitly reopen main menu on close: rejected because the user explicitly wants no main-menu concept so it can be reused later from pause.
- Keep `FoundationSplashScreen.text` as backward-compatible but ignore it: rejected because the user specifically requested the property no longer exist.

## Risks, Constraints, And Assumptions
- Removing a reflected field from `FoundationSplashScreen` is a serialized asset/API breaking change. All repository `.jsn` assets that contain the old field must be updated, and external scenes would need migration.
- Jackdaw reflected enum serialization shape for a new `TemplateMenuAction` must match existing reflection/serialization expectations; implementation should verify by compiling and, if possible, running editor-feature tests.
- Bevy app-exit APIs can differ by version. Use the API compatible with workspace Bevy `0.18.1` and validate with `cargo check/build`.
- Editor stop-play behavior should use Jackdaw's state mechanism already present in this codebase. If there is a more specific Jackdaw stop-play API, prefer that after inspecting docs/source.
- The options menu value setters are dummy controls only; no settings resource or persistence is planned.
- Existing `New Game` behavior was not requested. It can remain a no-op unless implementing it is necessary to avoid regressions.

## Open Questions
- Should the existing `Quit` label in `main_menu.jsn` be renamed to `Exit`, or is treating it as the requested exit button sufficient? Implementation should prefer renaming to `Exit` unless the user objects.
- Should the dummy load-game scene also close with Escape? The request only requires this for options, but adding Escape to the dummy load scene would be consistent.

## Documentation Expectations
- Public APIs added or changed by this feature must have Rustdoc comments, or the plan must explicitly justify why they are internal runtime-only markers.
- `FoundationSplashScreen` Rustdoc must clearly document that text content belongs to the authored `Text` component/`FoundationSplashText` entity.
- Feature-level docs are optional unless implementation introduces non-obvious reusable menu APIs.
- Generated documentation must be produced before the feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Begin by reading this plan, `tracker.md`, `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, and `.pi/skills/gitflow-workflow/SKILL.md`.
- Follow the project architecture convention: game-agnostic code belongs in `crates/foundation-library`; `games/template-game` should mostly contain assets and only contain game-specific glue when FoundationLibrary cannot reasonably own the behavior.
- Confirm branch is `feature/ui-refinement` before implementation edits.
- Update `tracker.md` before edits to record implementation start/resume.
- Keep options/load-game close logic stack-relative (`SceneCommand::CloseCurrent`) and not main-menu-specific.
- Commit each completed task/phase with the project gitflow commit-message format and push to `origin` if available.

## Optional Review Focus Areas
- Use `gpt-5.5` for final review.
- Verify `FoundationSplashScreen` no longer serializes/depends on component-owned text.
- Verify options menu has no dependency on main-menu-specific state.
- Verify exit behavior differs correctly between standalone game and editor Play mode.
- Verify scene-owned UI entities are cleaned up when their stack scene closes.

## Success Criteria
- Splash `.jsn` files and Rust code no longer require or serialize `FoundationSplashScreen.text`; splash text still displays/fades through authored `Text` + `FoundationSplashText` entities.
- Main-menu `Exit`/former `Quit` button exits standalone runtime.
- Main-menu exit action in editor Play mode stops Play and leaves the editor running.
- Main-menu `Options` opens an options scene on the stack.
- Options scene displays four horizontal tabs and five dummy setting rows per selected tab.
- Options scene closes via Escape and Back, returning to the previous stack scene.
- Options scene logic has no direct main-menu dependency.
- Main-menu `Load Game` opens a dummy load-game scene.
- Full validation passes or any waiver is explicitly recorded in the tracker.

## Testing Methodology
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- `scripts/doc-project.cmd`
- `scripts/validate-project.cmd` for the full validation sequence when practical
- Manual runtime smoke test recommended:
  - standalone: launch game, navigate to main menu, press Options, switch tabs, Escape/Back, press Load Game, press Back, press Exit and confirm process closes;
  - editor: launch editor, enter Play, press main-menu exit and confirm Play stops while editor remains open.
