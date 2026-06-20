# Scene Stack Example Tracker

## Metadata
- Feature slug: `scene-stack-example`
- Feature area: `multi-area`
- Primary area: `game`
- Branch: `feature/scene-stack-example`
- Overall status: `Main menu stub implemented; validation passed`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Main menu stub implementation complete with gpt-5.4; ready for user verification`
- Created: `2026-06-20`
- Last updated: `2026-06-20`
- Branch creation: Created locally from `dev` on 2026-06-20; verified `dev` is an ancestor of the active branch before implementation on 2026-06-20.
- Push status: Planning, implementation, follow-up, tracker push-status, base background fill adjustment, detached background root adjustment, main menu stub, and tracker push-status commits pushed to `origin/feature/scene-stack-example`.

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- Use the standard project wrappers unless the user explicitly waives them:
  - `scripts/format-project.cmd`
  - `scripts/lint-project.cmd`
  - `scripts/test-project.cmd`
  - `scripts/compile-project.cmd`
  - `scripts/doc-project.cmd`
  - `scripts/validate-project.cmd`

## Phase 1: Foundation Reusable Splash Primitives
**Status:** Complete  
**Goal:** Add reusable, reflected FoundationLibrary splash-screen configuration and timing/fade logic that can be driven by Jackdaw `.jsn` scenes.

### Tasks
- [x] Add a Foundation splash module with reflected config components/resources for splash text, fade-in duration, hold duration, fade-out duration, and next-scene behavior.
  - Status: Complete
  - Notes: Added `FoundationSplashScreen`, `FoundationSplashTimings`, configurable next-scene path, reset-stack flag, and replace-current flag in `crates/foundation-library/src/splash_screen.rs`.
- [x] Add reusable systems for splash phase progression, alpha/fade updates, and final scene-stack command emission.
  - Status: Complete
  - Notes: Added shared systems that spawn centered UI text, drive fade/hold/fade-out alpha, and emit `SceneCommand` on completion.
- [x] Register and re-export public Foundation splash types from `FoundationPlugin` and `foundation_library::prelude`.
  - Status: Complete
  - Notes: Public types include Rustdoc comments and are registered through `FoundationSplashScreenPlugin`.
- [x] Add tests for timing phase behavior and next-scene command selection where practical.
  - Status: Complete
  - Notes: Added Foundation unit tests for default timings, phase alpha behavior, replace-current transition command, and reset-stack transition command.

### Validation
- Format: Passed via `scripts/format-project.cmd` on 2026-06-20
- Lint: Passed via `scripts/lint-project.cmd` and `scripts/validate-project.cmd` on 2026-06-20
- Tests: Passed via `scripts/test-project.cmd` and `scripts/validate-project.cmd` on 2026-06-20
- Build: Passed via `scripts/compile-project.cmd` and `scripts/validate-project.cmd` on 2026-06-20
- Documentation generation: Passed via `scripts/doc-project.cmd` and `scripts/validate-project.cmd` on 2026-06-20
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-06-20
- User confirmation: Not required for phase completion

### Notes
- Reusable logic belongs in `crates/foundation-library`.
- Concrete scene files and sequence choices belong in `games/template-game`.

## Phase 2: TemplateGame Jackdaw Scene Stack Bridge And Startup
**Status:** Complete  
**Goal:** Make TemplateGame load Jackdaw `.jsn` scene sources through the Foundation scene stack instead of spawning the initial scene directly.

### Tasks
- [x] Replace direct startup spawning of `scene.jsn` with opening the first splash scene through the scene stack.
  - Status: Complete
  - Notes: `TemplateGamePlugin` now emits an initial `SceneCommand` for `splash_pixel_perfect.jsn`; `src/main.rs` no longer spawns `scene.jsn` directly.
- [x] Add a TemplateGame bridge that consumes `SceneLoadRequested` and spawns `JackdawSceneRoot(asset_server.load(path))` for `.jsn` scene sources.
  - Status: Complete
  - Notes: `spawn_requested_jackdaw_scenes` spawns `JackdawSceneRoot` and tags roots with `SceneOwner`.
- [x] Verify scene stack replacement/clear behavior for splash transitions and final main menu transition.
  - Status: Complete
  - Notes: Pixel Perfect `.jsn` uses replace-current transition to Bevy; Bevy `.jsn` uses reset-stack transition to `main_menu.jsn`.
- [x] Add tests for scene path constants, startup command behavior, and bridge behavior where practical.
  - Status: Complete
  - Notes: Added TemplateGame tests for scene path constants, initial stack command, and reflected menu marker registration. Bridge behavior is covered indirectly by scene stack load-request tests and compile validation.

### Validation
- Format: Passed via `scripts/format-project.cmd` on 2026-06-20
- Lint: Passed via `scripts/lint-project.cmd` and `scripts/validate-project.cmd` on 2026-06-20
- Tests: Passed via `scripts/test-project.cmd` and `scripts/validate-project.cmd` on 2026-06-20
- Build: Passed via `scripts/compile-project.cmd` and `scripts/validate-project.cmd` on 2026-06-20
- Documentation generation: Passed via `scripts/doc-project.cmd` and `scripts/validate-project.cmd` on 2026-06-20
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-06-20
- User confirmation: Not required for phase completion

### Notes
- The implementation keeps the Jackdaw editor launcher generic and avoids game-specific sequence logic in `crates/jackdaw-editor`.

## Phase 3: Concrete `.jsn` Splash And Main Menu Scenes
**Status:** Complete  
**Goal:** Add three TemplateGame Jackdaw scenes and verify the requested visible flow/timings.

### Tasks
- [x] Add Pixel Perfect splash `.jsn` scene.
  - Status: Complete
  - Notes: Added `games/template-game/assets/splash_pixel_perfect.jsn` with centered `Pixel Perfect` text config and `1.5 / 2.0 / 1.5` timings.
- [x] Add Bevy splash `.jsn` scene.
  - Status: Complete
  - Notes: Added `games/template-game/assets/splash_bevy.jsn` with centered `Bevy` text config and shared Foundation splash logic.
- [x] Add main menu `.jsn` scene.
  - Status: Complete
  - Notes: Added `games/template-game/assets/main_menu.jsn` with `TemplateMainMenu` marker; game code generates the visible menu UI from that scene data.
- [x] Ensure final transition uses stack reset/clear-and-open so main menu is the only active stack entry.
  - Status: Complete
  - Notes: Bevy splash sets `reset_stack_for_next_scene = true`; Foundation splash logic emits `SceneCommand::ClearAndOpen` for `main_menu.jsn`.
- [x] Run/manual-check TemplateGame long enough to observe the splash-to-menu sequence if practical.
  - Status: Complete
  - Notes: `timeout 35s cargo run -p template-game` compiled, opened the `template-game` window, logged Jackdaw runtime startup, and was then intentionally terminated by timeout with exit code 143. No scene-load errors were logged before timeout.

### Validation
- Format: Passed via `scripts/format-project.cmd` on 2026-06-20
- Lint: Passed via `scripts/lint-project.cmd` and `scripts/validate-project.cmd` on 2026-06-20
- Tests: Passed via `scripts/test-project.cmd` and `scripts/validate-project.cmd` on 2026-06-20
- Build: Passed via `scripts/compile-project.cmd` and `scripts/validate-project.cmd` on 2026-06-20
- Documentation generation: Passed via `scripts/doc-project.cmd` and `scripts/validate-project.cmd` on 2026-06-20
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Manual launch check: Passed startup smoke check; process intentionally killed by timeout after window creation
- User confirmation: Pending final user acceptance

### Notes
- Desired runtime flow:
  ```text
  splash_pixel_perfect.jsn
    [fade in 1.5s][hold 2.0s][fade out 1.5s]
  splash_bevy.jsn
    [fade in 1.5s][hold 2.0s][fade out 1.5s]
  clear/reset stack
  main_menu.jsn
  ```
- Default total splash time before main menu is `10.0s`.

## Phase 4: Persistent Splash Background And Debug Clear Color
**Status:** Complete  
**Goal:** Remove flashes between splash screens by keeping a persistent background scene under non-covering splash UI overlays and make the game clear color visible for debugging.

### Tasks
- [x] Add a startup background `.jsn` scene that fills the screen before splash overlays.
  - Status: Complete
  - Notes: Added `games/template-game/assets/splash_background.jsn` with `TemplateFullscreenBackground` and startup now opens this fullscreen scene before the Pixel Perfect splash.
- [x] Change splash scene presentation so splash screens do not hide the background but still prevent interaction.
  - Status: Complete
  - Notes: Initial and splash-to-splash transitions now use `ScenePresentation::INPUT_BLOCKING_OVERLAY` for splash scenes.
- [x] Remove the generated splash UI background fill so splash scenes only contribute their own UI text.
  - Status: Complete
  - Notes: Foundation splash UI root no longer spawns `BackgroundColor`; the persistent background scene is responsible for the fill.
- [x] Set the game fallback clear color to debug blue for now.
  - Status: Complete
  - Notes: Standalone TemplateGame inserts `ClearColor(Color::srgb(0.0, 0.0, 0.0))` with a comment noting black is the intended normal fallback.

### Validation
- Format: Passed via `cargo fmt --all` and `scripts/validate-project.cmd` on 2026-06-20
- Lint: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Tests: Passed via `scripts/test-project.cmd` and `scripts/validate-project.cmd` on 2026-06-20
- Build: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Manual launch check: Passed startup smoke check; `timeout 30s cargo run -p template-game` opened the window and logged no scene-load errors before intentional timeout termination
- User confirmation: Pending final user acceptance

### Notes
- This is a follow-up to the initial implementation after the user observed a flash between splash screens.

## Phase 5: Template Game Prompt And Stub Main Menu
**Status:** Complete  
**Goal:** Replace the placeholder main menu with a Template Game prompt screen that advances on any keyboard, mouse, or gamepad button, then shows four hoverable stub buttons.

### Tasks
- [x] Update the main-menu `.jsn` data to show `Template Game` and `Press any button` prompt text.
  - Status: Complete
  - Notes: Updated `games/template-game/assets/main_menu.jsn` and `TemplateMainMenu::default()`.
- [x] Add main-menu prompt-to-buttons transition on any keyboard, mouse, or gamepad button press.
  - Status: Complete
  - Notes: Added prompt state transition using keyboard, mouse, and optional gamepad button input. Gamepad input resource is optional because manual launch showed it can be absent.
- [x] Add four main menu buttons with hover visual support: `New Game`, `Load Game`, `Options`, `Quit`.
  - Status: Complete
  - Notes: Added stub Bevy UI buttons with normal, hovered, and pressed colors. Buttons intentionally have no actions yet.

### Validation
- Format: Passed via `cargo fmt --all` and `scripts/validate-project.cmd` on 2026-06-20
- Lint: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Tests: Passed via `scripts/test-project.cmd` and `scripts/validate-project.cmd` on 2026-06-20
- Build: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Manual launch check: Passed startup smoke check; `timeout 30s cargo run -p template-game` opened the window and logged no system errors after making gamepad input optional
- User confirmation: Pending final user acceptance

### Notes
- This is a follow-up requested after the splash/background behavior was accepted.

## Implementation / Review Handoff Notes
- Implementation used `gpt-5.4`; never use Anthropic models.
- Active branch was confirmed as `feature/scene-stack-example` before implementation edits.
- Reusable splash behavior lives in FoundationLibrary and TemplateGame-specific scene assets/sequence implementation lives in `games/template-game`.
- Jackdaw `.jsn` is preserved as the data/source format for all three scenes. Direct Bevy UI/text serialization in `.jsn` was avoided; `.jsn` scenes hold reflected Foundation/TemplateGame config components and runtime systems spawn UI text from that data.
- Commit implementation and push to `origin` when available.

## Postponed Work
- Full interactive main menu navigation is postponed unless the user expands the example menu scope.
- Generic Foundation-owned Jackdaw `.jsn` load bridge is postponed because this feature kept the concrete bridge in TemplateGame as requested.
- Advanced transition effects beyond alpha fade are postponed.

## Open Issues / Questions
- Resolved: direct Bevy UI/text was not serialized in `.jsn`; `.jsn` scenes hold reflected Foundation/TemplateGame config components and runtime systems spawn UI. This keeps all three scenes as Jackdaw `.jsn` data sources while avoiding brittle UI serialization.
- Resolved: first splash replaces/closes itself when opening the second splash; final splash resets the stack before opening main menu.

## Progress Log
- `2026-06-20`: User approved the feature summary and clarified all three scenes should be Jackdaw `.jsn` scenes if possible; reusable logic should live in FoundationLibrary while concrete scene implementation should live in TemplateGame.
- `2026-06-20`: Created planning branch `feature/scene-stack-example` from `dev`.
- `2026-06-20`: Plan and tracker created.
- `2026-06-20`: Planning commit `30c1b6b` pushed to `origin/feature/scene-stack-example`.
- `2026-06-20`: User approved implementation. Confirmed active branch `feature/scene-stack-example` and verified `dev` is an ancestor; implementation started with `gpt-5.4`.
- `2026-06-20`: Implemented reusable Foundation splash logic, TemplateGame scene-stack startup/Jackdaw scene bridge, and three `.jsn` scene assets.
- `2026-06-20`: Validation passed: format, lint, tests, build, documentation generation, and full `scripts/validate-project.cmd`.
- `2026-06-20`: Manual startup smoke check opened the TemplateGame window and Jackdaw runtime without logged scene-load errors before intentional timeout termination.
- `2026-06-20`: Implementation commit `9275efd` pushed to `origin/feature/scene-stack-example`.
- `2026-06-20`: Tracker push-status commit `1508d61` pushed to `origin/feature/scene-stack-example`.
- `2026-06-20`: User observed a flash between splash screens and requested a debug-blue game fallback clear color plus a persistent background scene under non-covering splash UI overlays; follow-up implementation started with `gpt-5.4`.
- `2026-06-20`: Added persistent `splash_background.jsn`, `TemplateFullscreenBackground`, overlay splash presentations, transparent splash UI roots, and debug-blue standalone game clear color.
- `2026-06-20`: Follow-up validation passed via `scripts/test-project.cmd` and full `scripts/validate-project.cmd`; manual startup smoke check passed with intentional timeout termination.
- `2026-06-20`: Follow-up implementation commit `521ef2f` pushed to `origin/feature/scene-stack-example`.
- `2026-06-20`: Follow-up tracker push-status commit `ad38cef` pushed to `origin/feature/scene-stack-example`.
- `2026-06-20`: User confirmed the flow and clarified the base splash background should be black and fill the screen; adjusted the background UI node to be absolute full-screen black behind splash overlays.
- `2026-06-20`: Base background fill adjustment validation passed via `scripts/test-project.cmd` and full `scripts/validate-project.cmd`.
- `2026-06-20`: Base background fill adjustment commit `a45281f` pushed to `origin/feature/scene-stack-example`.
- `2026-06-20`: User still saw debug-blue clear color, indicating the black background UI was likely not stretching/rendering as intended while parented under the Jackdaw scene entity. Changed generated fullscreen background UI to be a top-level UI root with orphan cleanup tied to the source marker entity.
- `2026-06-20`: Detached background root adjustment validation passed via `scripts/test-project.cmd` and full `scripts/validate-project.cmd`; manual startup check opened the game window without scene-load errors.
- `2026-06-20`: Detached background root adjustment commit `02a4506` pushed to `origin/feature/scene-stack-example`.
- `2026-06-20`: User accepted splash/background behavior and requested a Template Game prompt screen that advances on any keyboard, mouse, or gamepad button to a stub main menu with hoverable New Game, Load Game, Options, and Quit buttons.
- `2026-06-20`: Implemented Template Game prompt, any-button prompt transition, and four hoverable stub main menu buttons.
- `2026-06-20`: First manual launch showed `ButtonInput<GamepadButton>` may be absent; changed gamepad input to `Option<Res<ButtonInput<GamepadButton>>>` and reran validation/manual launch successfully.
- `2026-06-20`: Main menu stub validation passed via `scripts/validate-project.cmd`; manual startup smoke check opened the game window without system errors.
- `2026-06-20`: Main menu stub commit `712ed94` pushed to `origin/feature/scene-stack-example`.
- `2026-06-20`: Main menu tracker push-status commit `55d0ddc` pushed to `origin/feature/scene-stack-example`.
