# Scene Stack Example Tracker

## Metadata
- Feature slug: `scene-stack-example`
- Feature area: `multi-area`
- Primary area: `game`
- Branch: `feature/scene-stack-example`
- Overall status: `Standalone and editor authored visual .jsn scene flow implemented; validation passed`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Standalone main-menu child-order fix implemented; validation passed with gpt-5.4; awaiting user verification/commit approval`
- Created: `2026-06-20`
- Last updated: `2026-06-20`
- Branch creation: Created locally from `dev` on 2026-06-20; verified `dev` is an ancestor of the active branch before implementation on 2026-06-20.
- Push status: Planning, implementation, follow-up, tracker push-status, base background fill adjustment, detached background root adjustment, main menu stub, tracker push-status, editor panic fix, editor play integration, editor current-scene support, editor viewport UI fix, editor viewport parenting fix, editor cargo alias, editor default project root fix, and viewport-centered/clipped UI fix commits pushed to `origin/feature/scene-stack-example`; final tracker push-status commit pending.

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

## Phase 6: Static Editor Scene Stack Startup Fix
**Status:** Complete  
**Goal:** Keep TemplateGame scene-stack and splash systems available during Jackdaw Play mode without running standalone startup in editor edit mode or requiring `JackdawPlugin` in the static editor.

### Tasks
- [x] Run runtime scene-stack startup/loading/menu systems only when the static editor enters Play mode.
  - Status: Complete
  - Notes: `TemplateGamePlugin` now opens the startup scene stack on `OnEnter(PlayState::Playing)`, clears it on `OnExit(PlayState::Playing)`, and runs runtime scene-stack/menu systems only while `play_gate::is_playing` is true.
- [x] Add an editor-compatible `.jsn` scene-stack bridge.
  - Status: Complete
  - Notes: Editor builds parse requested `.jsn` files through `jackdaw_jsn`/`serde_json` and spawn them with `jackdaw::scene_io::load_scene_from_jsn`, avoiding the `JackdawSceneRoot` asset path that requires `jackdaw_runtime::JackdawScene` asset initialization.
- [x] Validate the static TemplateGame editor opens without the `jackdaw_runtime::JackdawScene` asset-type panic.
  - Status: Complete
  - Notes: `cd games/template-game && timeout 30s cargo run --bin editor --features editor` opened the editor and loaded `assets/scene.jsn` without plugin duplication or `JackdawScene` asset initialization panic.

### Validation
- Format: Passed via `cargo fmt --all` and `scripts/validate-project.cmd` on 2026-06-20
- Lint: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Tests: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Build: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Manual editor launch check: Passed via `cd games/template-game && timeout 30s cargo run --bin editor --features editor`
- User confirmation: Pending final user acceptance

### Notes
- Root cause: `TemplateGamePlugin` runtime startup emitted scene-stack `.jsn` load requests in the static editor. The standalone game-side bridge spawned `JackdawSceneRoot(asset_server.load(...))`, but the static editor app does not initialize the `jackdaw_runtime::JackdawScene` asset type through `JackdawPlugin`.
- Adding `JackdawPlugin` to the static editor was attempted and rejected because it duplicated Jackdaw's `JsnPlugin`, causing a plugin-already-added panic. The final fix uses Jackdaw editor scene loading APIs for editor builds instead.

## Phase 7: Editor Play Current Scene Support
**Status:** Complete  
**Goal:** Let designers open any TemplateGame `.jsn` scene in the editor, click Play, and continue from that scene through the Foundation scene stack without the scene system removing editor UI.

### Tasks
- [x] Detect the currently open Jackdaw scene when Play starts.
  - Status: Complete
  - Notes: Editor Play startup now reads `jackdaw::scene_io::SceneFilePath`, extracts the asset file name, and starts the scene stack from that scene when it is a known TemplateGame scene.
- [x] Start from the opened scene rather than always restarting the full splash flow.
  - Status: Complete
  - Notes: Opening `splash_bevy.jsn` starts with the persistent background plus Bevy splash, allowing it to transition to `main_menu.jsn`. Opening `main_menu.jsn` starts directly at main menu. Unknown scenes fall back to the full startup flow.
- [x] Ensure editor UI is not owned or cleared by the scene stack.
  - Status: Complete
  - Notes: Scene stack cleanup only targets entities explicitly tagged with `SceneOwner`; editor UI is not tagged. The editor `.jsn` bridge only tags entities spawned from scene-stack loads, not Jackdaw editor chrome.

### Validation
- Format: Passed via `cargo fmt --all` and `scripts/validate-project.cmd` on 2026-06-20
- Lint: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Tests: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Build: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Manual editor launch check: Passed via `cd games/template-game && timeout 25s cargo run --bin editor --features editor`; editor opened without panic.
- User confirmation: Pending final user acceptance

### Notes
- The editor can still only infer a continuation point for known TemplateGame scenes: `splash_background.jsn`, `splash_pixel_perfect.jsn`, `splash_bevy.jsn`, and `main_menu.jsn`. Other scenes intentionally fall back to the default startup flow until a generic per-scene continuation contract exists.

## Phase 8: Editor Viewport-Confined Gameplay UI
**Status:** Complete  
**Goal:** Ensure gameplay UI spawned by the scene-stack flow is laid out inside Jackdaw's viewport during editor Play instead of covering/removing editor chrome.

### Tasks
- [x] Add reusable support for targeting and parenting generated Foundation splash UI to editor viewport context.
  - Status: Complete
  - Notes: Added `FoundationSplashUiTargetCamera` and `FoundationSplashUiParent`; splash UI roots insert `UiTargetCamera` when present and are parented under the viewport UI node when available.
- [x] Configure TemplateGame editor Play to target Jackdaw's viewport camera and viewport UI node.
  - Status: Complete
  - Notes: On Play enter, TemplateGame captures the active Jackdaw viewport camera/UI node, or falls back to the first `MainViewportCamera`/`SceneViewport`, and stores them as the gameplay UI target/parent.
- [x] Apply the same viewport target and parent to TemplateGame-generated background and main-menu UI roots.
  - Status: Complete
  - Notes: Persistent splash background, prompt menu, and button menu roots now insert `UiTargetCamera` and are parented under the viewport UI node in editor Play so percentage sizing is constrained to the viewport rather than the full editor window.

### Validation
- Format: Passed via `cargo fmt --all` and `scripts/validate-project.cmd` on 2026-06-20
- Lint: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Tests: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Build: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Manual editor launch check: Passed via `cd games/template-game && timeout 25s cargo run --bin editor --features editor`; editor opened without panic. Visual confirmation of Play viewport confinement is pending user verification.
- User confirmation: Pending final user acceptance

### Notes
- This addresses the issue where opening `main_menu.jsn` or splash scenes and pressing Play caused gameplay UI to cover the full editor window. `UiTargetCamera` alone was not enough for every generated root; generated UI roots are now also parented under the Jackdaw `SceneViewport` node so layout percentages resolve against the viewport.

## Phase 9: Editor Cargo Alias
**Status:** Complete  
**Goal:** Provide a short repository-local Cargo command for launching the TemplateGame editor.

### Tasks
- [x] Add a repository Cargo alias for the TemplateGame editor.
  - Status: Complete
  - Notes: Added `.cargo/config.toml` aliases: `cargo editor` and `cargo template-editor`, both expanding to `cargo run -p template-game --bin editor --features editor`.

### Validation
- Alias expansion: Passed via `cargo editor --help` on 2026-06-20.
- Full validation wrapper: Not rerun; alias-only config change does not affect Rust compilation.
- User confirmation: Pending final user acceptance

### Notes
- Cargo does not support adding a custom `--editor` flag to `cargo run`, so `cargo run -p template-game --editor` is not a valid Cargo shape. A Cargo alias is the closest idiomatic command.
- Follow-up fix: when launched from workspace root, the TemplateGame editor binary now defaults its Jackdaw project root and working directory to `CARGO_MANIFEST_DIR` (`games/template-game`) instead of the shell current directory. `JACKDAW_PROJECT` still overrides this for explicit project selection.

## Phase 10: Editor Runtime Scene Isolation And Viewport Surface
**Status:** Complete  
**Goal:** Replace viewport-confinement hacks with a stable runtime/editor separation: gameplay systems only act on scene-stack-owned runtime scene entities during editor Play, and generated gameplay UI targets the Jackdaw viewport render camera rather than the editor UI tree.

### Tasks
- [x] Gate Foundation splash runtime systems so opening `.jsn` files in editor edit mode cannot spawn gameplay UI over Jackdaw chrome.
  - Status: Complete
  - Notes: Added `FoundationSplashRuntimeSettings`; TemplateGame editor disables splash runtime while editing and enables it only on Play enter.
- [x] Require scene-stack ownership for TemplateGame runtime scene processors while in editor Play.
  - Status: Complete
  - Notes: Editor Play now requires `SceneOwner` for Foundation splash initialization and TemplateGame background/menu generation, so authoring-scene components are ignored while scene-stack runtime copies drive gameplay.
- [x] Use viewport camera targeting as the primary editor gameplay UI surface and remove parent/rectangle fallbacks from the accepted path.
  - Status: Complete
  - Notes: Generated gameplay UI roots now prefer `UiTargetCamera(viewport_camera)` and only parent to a viewport UI node if no target camera exists. Generated roots are also tagged with `SceneOwner` so scene-stack cleanup can remove them.

### Validation
- Format: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Lint: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Tests: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Build: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-06-20
- User confirmation: Passed on 2026-06-20; user reported viewport behavior is working perfectly.

### Notes
- Screenshots `ref/001.png` and `ref/002.png` demonstrate the failure: `Bevy` text is centered in the editor window and overlaps editor panels instead of being rendered inside the Jackdaw viewport.
- Jackdaw's viewport is a `SceneViewport` UI node containing a `ViewportNode` for a camera whose `RenderTarget` is an off-screen image. Bevy UI can target a camera render target with `UiTargetCamera`, but only when that component is on a root UI node.
- Previous attempts mixed `UiTargetCamera`, UI parenting, and window-space rectangles. That is fragile because it does not address edit-mode runtime systems and can invalidate `UiTargetCamera` by making gameplay roots children of editor UI.

## Phase 11: Separate Landing Page And Main Menu Scenes
**Status:** Awaiting full validation  
**Goal:** Split the `Press any button` landing page and the stub main menu into separate Jackdaw `.jsn` scenes.

### Tasks
- [x] Add a dedicated landing page `.jsn` scene.
  - Status: Complete
  - Notes: Added `games/template-game/assets/landing_page.jsn` with `TemplateLandingPage` data: `Template Game`, `Press any button`, and next scene `main_menu.jsn`.
- [x] Change the Bevy splash to transition to the landing page instead of directly to the main menu.
  - Status: Complete
  - Notes: Updated `splash_bevy.jsn` `next_scene_path` to `landing_page.jsn`.
- [x] Convert `main_menu.jsn` to represent only the main menu.
  - Status: Complete
  - Notes: `main_menu.jsn` now contains `TemplateMainMenu` with title `Main Menu`; runtime code spawns the stub buttons immediately when the main-menu scene loads.
- [x] Update editor Play known-scene routing and tests for the new landing page scene.
  - Status: Complete
  - Notes: Added `LANDING_PAGE_SCENE`, editor scene key/routing, reflected `TemplateLandingPage`, and updated TemplateGame unit tests.

### Validation
- Format: Passed via `cargo fmt --all` on 2026-06-20
- Lint: Passed during `scripts/validate-project.cmd` before the test phase blocked on locked `target/debug/editor.exe`
- Tests: Partial pass: `cargo test -p template-game --lib --features editor` and `cargo test -p foundation-library --lib` passed on 2026-06-20
- Build: `cargo check -p template-game --features editor` passed on 2026-06-20
- Documentation generation: Pending full validation rerun
- Full validation wrapper: Initially blocked because `target/debug/editor.exe` was locked/running; later validation passed after the editor lock cleared during Phase 12 work
- User confirmation: Pending

### Notes
- Close the running editor before rerunning `scripts/validate-project.cmd`.

## Phase 12: Authored Visual `.jsn` Scene Content
**Status:** Complete  
**Goal:** Move splash, background, landing page, and main-menu visual UI into the `.jsn` scene files so they can be edited visually, while keeping runtime behavior disabled in edit mode and enabled only during Play.

### Tasks
- [x] Add authorable Foundation splash UI markers.
  - Status: Complete
  - Notes: Added reflected `FoundationSplashUiRoot` and `FoundationSplashText`. Splash runtime now finds authored root/text entities and fades the authored text instead of spawning duplicate UI.
- [x] Add authorable TemplateGame gameplay UI markers.
  - Status: Complete
  - Notes: Added reflected `TemplateGameplayUiRoot` and reflected `TemplateMenuButton`. Editor builds retarget authored gameplay UI roots to the Jackdaw viewport camera in edit mode and Play mode.
- [x] Update all authored `.jsn` scene files to contain visible UI content.
  - Status: Complete
  - Notes: `splash_background.jsn`, `splash_pixel_perfect.jsn`, `splash_bevy.jsn`, `landing_page.jsn`, and `main_menu.jsn` now include Bevy UI `Node`/`Text`/`TextFont`/`TextColor`/`BackgroundColor`/`Button` content as appropriate.
- [x] Keep runtime behavior separate from edit-mode visuals.
  - Status: Complete
  - Notes: Foundation splash runtime remains disabled in editor edit mode. During Play, scene-stack-owned authored entities receive fades/transitions/input behavior. Standalone still uses the full game window.

### Validation
- Format: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Lint: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Tests: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Build: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-06-20
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-06-20
- User confirmation: Pending

### Notes
- `.jsn` visuals are now authored scene content. Rust systems provide behavior only: fade, transitions, input, hover, and editor viewport targeting.

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
- `2026-06-20`: User reported the Jackdaw launcher/static editor panicked in `template_game::spawn_requested_jackdaw_scenes` because `jackdaw_runtime::JackdawScene` asset type was not initialized.
- `2026-06-20`: Initially fixed the panic by gating TemplateGame runtime scene-stack startup/loading/menu systems out of `feature = "editor"` builds while preserving reflected component registration.
- `2026-06-20`: User clarified that Foundation runtime libraries must function while playing through the editor, including scene stack and splash screens.
- `2026-06-20`: Reworked editor integration so scene-stack startup runs on Jackdaw `PlayState::Playing`, runtime systems run only during play, and editor builds load scene-stack `.jsn` files through `jackdaw::scene_io::load_scene_from_jsn` instead of `JackdawSceneRoot`.
- `2026-06-20`: Validation passed via `scripts/validate-project.cmd`; manual static editor launch check passed with `cd games/template-game && timeout 30s cargo run --bin editor --features editor`.
- `2026-06-20`: Editor panic fix commit `bb09d96` pushed to `origin/feature/scene-stack-example`.
- `2026-06-20`: Editor play scene-stack integration commit `a0e0d13` pushed to `origin/feature/scene-stack-example`.
- `2026-06-20`: User clarified editor Play should start from whichever `.jsn` scene is open, so `splash_bevy.jsn` can continue to `main_menu.jsn`, and editor UI must never be removed by the scene stack.
- `2026-06-20`: Implemented editor current-scene detection from `SceneFilePath`; known TemplateGame scenes now start at the opened scene during Play while scene-stack ownership remains limited to runtime-spawned scene entities.
- `2026-06-20`: Current-scene editor support validation passed via `scripts/validate-project.cmd`; manual editor launch check passed without panic.
- `2026-06-20`: Editor current-scene support commit `dff4c76` pushed to `origin/feature/scene-stack-example`.
- `2026-06-20`: User reported that Play from `main_menu.jsn` and other scenes still renders gameplay UI over the whole editor window; gameplay UI should be confined to the Jackdaw viewport.
- `2026-06-20`: Implemented viewport camera targeting for generated Foundation splash UI and TemplateGame background/menu UI using `UiTargetCamera` with Jackdaw's active/first `MainViewportCamera` during editor Play.
- `2026-06-20`: Viewport-confined UI fix validation passed via `scripts/validate-project.cmd`; manual editor launch check passed without panic.
- `2026-06-20`: Editor viewport UI fix commit `ea07064` pushed to `origin/feature/scene-stack-example`.
- `2026-06-20`: User clarified that all scenes except the main menu still consumed the full editor window; implemented viewport-parenting for Foundation splash UI and TemplateGame background/menu UI roots using Jackdaw's `SceneViewport` in addition to `UiTargetCamera`.
- `2026-06-20`: Viewport-parenting validation passed via `scripts/validate-project.cmd`; manual editor launch check passed without panic.
- `2026-06-20`: Editor viewport parenting fix commit `b94fdd5` pushed to `origin/feature/scene-stack-example`.
- `2026-06-20`: Added repository Cargo aliases `cargo editor` and `cargo template-editor` for launching the TemplateGame editor; verified alias expansion with `cargo editor --help`.
- `2026-06-20`: Editor cargo alias commit `454778e` pushed to `origin/feature/scene-stack-example`.
- `2026-06-20`: User reported `cargo editor` opened the workspace root as a Jackdaw project; changed TemplateGame editor binary to default to `games/template-game` via `CARGO_MANIFEST_DIR`, set current directory to that project root, and kept `JACKDAW_PROJECT` as an override. Validation passed via `scripts/validate-project.cmd`; manual `timeout 30s cargo editor` loaded `games/template-game/assets/scene.jsn`.
- `2026-06-20`: Editor default project root fix commit `5888ec5` pushed to `origin/feature/scene-stack-example`.
- `2026-06-20`: User reported splash text was still centered on the editor window rather than the viewport and requested viewport clipping. Updated generated splash/menu/background UI roots to be absolute fill roots with `Overflow::clip()`, set the Jackdaw `SceneViewport` node to clip during Play setup, and avoided inserting `UiTargetCamera` when roots are parented under the editor viewport so they inherit editor UI layout/camera context. Validation passed via `scripts/validate-project.cmd`; manual `timeout 20s cargo editor` opened the editor and loaded `splash_bevy.jsn` without panic.
- `2026-06-20`: Viewport-centered/clipped UI fix commit `954c624` pushed to `origin/feature/scene-stack-example`.
- `2026-06-20`: User screenshots showed Bevy splash text still rendered over editor UI. Reassessed the architecture and identified two root causes: Foundation splash systems ran in editor edit mode on authoring-scene components, and previous viewport parenting made gameplay UI non-root so `UiTargetCamera` could be ignored. Implemented runtime scene isolation and root-node viewport camera targeting. Full validation passed via `scripts/validate-project.cmd`. No commit made pending user verification.
- `2026-06-20`: User confirmed viewport behavior is working perfectly, then requested the `Press any button` landing page and stub main menu become separate `.jsn` scenes. Added `landing_page.jsn`, changed Bevy splash to transition to landing page, changed landing page input to clear/open `main_menu.jsn`, and made `main_menu.jsn` spawn menu buttons directly. `cargo check -p template-game --features editor`, `cargo test -p template-game --lib --features editor`, and `cargo test -p foundation-library --lib` passed. Full validation blocked by a running/locked `target/debug/editor.exe`.
- `2026-06-20`: User clarified that all scenes should be visually editable in `.jsn`, with fades/behavior disabled in edit mode and active during Play. Refactored splash/background/landing/menu scenes to contain authored Bevy UI entities and changed runtime systems to target/mutate authored scene entities rather than constructing visible UI at runtime. Full validation passed via `scripts/validate-project.cmd`.
- `2026-06-20`: User reported standalone `cargo run -p template-game` showed only a black screen even though scene-stack transitions were occurring. Debug overlay/logging confirmed the scenes loaded, authored UI roots had valid full-window layout, and splash fade alpha advanced, but Jackdaw-runtime-reflected UI text had zero computed size because reflected insertion bypassed Bevy UI text required-component setup.
- `2026-06-20`: Fixed standalone authored UI text by completing reflected UI text components at runtime (`Node`, text layout/cache/measure components, line height, content size, font hinting) and forcing the reflected `ChildOf` relationship back through Bevy hierarchy commands so UI layout includes the text child. Removed temporary debug overlay/logging after user confirmed the splash/menu flow is visible in standalone.
- `2026-06-20`: Post-cleanup validation passed via `scripts/validate-project.cmd` after `cargo fmt --all`.
- `2026-06-20`: User reported the main-menu title was correct in edit mode but moved to the bottom during Play. Root cause was the standalone authored text completion using `add_child` per text entity, which appended the title to the end of the parent child list. Changed completion to rebuild affected parent `Children` collections in original entity/scene order so authored column order is preserved. Validation passed via `scripts/validate-project.cmd`.
