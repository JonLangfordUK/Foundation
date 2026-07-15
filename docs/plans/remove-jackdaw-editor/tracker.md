# Remove Jackdaw Editor And Adopt Bevy BSN Tracker

## Metadata
- Feature slug: `remove-jackdaw-editor`
- Feature area: `multi-area` (`engine`, `game`, and `editor removal`)
- Primary area: `engine`
- Branch: `feature/remove-jackdaw-editor`
- Overall status: `Implementation complete; review follow-up fixes in progress`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Review feedback sent to gpt-5.4 for fixes`
- Created: `2026-07-14`
- Last updated: `2026-07-14`

## Branch And Push State
- Active planning branch: `feature/remove-jackdaw-editor`
- Branch base: Created from local `dev` on 2026-07-14; `dev` was an ancestor at branch creation.
- Remote: `origin` is configured as `https://github.com/JonLangfordUK/Foundation.git`.
- Push status: Feature commits are pushed to `origin/feature/remove-jackdaw-editor` after each checkpoint.

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- This feature should not be considered complete while production manifests or source imports still reference `jackdaw`, `jackdaw_api`, `jackdaw_runtime`, or `jackdaw_jsn`, unless the user explicitly revises the request.
- This feature should not remove `crates/foundation-editor-library`; it should keep that crate as a cleared Bevy-only editor-time library.
- This feature should update Bevy to `0.19` and move required TemplateGame scenes to BSN Rust scene functions.

## Phase 1: Planning And Approval
**Status:** Complete  
**Goal:** Capture the Jackdaw removal, Bevy `0.19`, BSN scene, and retained Foundation editor-library plan before implementation starts.

### Tasks
- [x] Inspect current workspace members and dependency structure.
  - Status: Complete
  - Notes: Root workspace contains `crates/foundation-editor-library`, `crates/foundation-runtime-library`, `crates/jackdaw-editor`, and `games/template-game`; Jackdaw dependencies appear in root, FoundationRuntimeLibrary, FoundationEditorLibrary, and TemplateGame manifests.
- [x] Inspect relevant runtime/editor source files and documentation.
  - Status: Complete
  - Notes: Reviewed TemplateGame runtime/editor entry points, Foundation runtime modules, README, docs/scene-system.md, and repository Jackdaw references.
- [x] Research Bevy `0.19` BSN shape.
  - Status: Complete
  - Notes: Bevy `0.19` supports code-driven BSN through `bsn!`, `bsn_list!`, `impl Scene`, and spawn scene APIs; first-party `.bsn` asset loading is not shipped yet.
- [x] Create `docs/plans/remove-jackdaw-editor/plan.md`.
  - Status: Complete
  - Notes: Plan revised for Bevy `0.19`, BSN Rust scenes, retained but cleared `foundation-editor-library`, and explicit `--editor` mode.
- [x] Create `docs/plans/remove-jackdaw-editor/tracker.md`.
  - Status: Complete
  - Notes: Tracker revised for the updated scope.
- [x] Receive user approval to begin implementation.
  - Status: Complete
  - Notes: User approved implementation and later review follow-up fixes.

### Validation
- Format: Passed / not required for planning-only steps; no Rust/source formatting required for planning docs yet.
- Lint: Passed / not required for planning-only steps; not required for planning-only docs.
- Tests: Passed / not required for planning-only steps; not required for planning-only docs.
- Build: Passed / not required for planning-only steps; not required for planning-only docs.
- Documentation generation: Passed / not required for planning-only steps; required before implementation completion, not for planning approval.
- Full validation wrapper: Full cargo validation passed; wrapper not separately required; required during implementation completion.
- User confirmation: User approved implementation.

## Phase 2: Upgrade Bevy And Remove Jackdaw Crates/Dependencies
**Status:** Complete  
**Goal:** Move to Bevy `0.19`, remove Jackdaw, and keep both Foundation libraries.

### Tasks
- [x] Update Bevy dependencies to `0.19`.
  - Status: Complete
  - Notes: Expect Bevy API migration work during compile checks.
- [x] Remove `crates/jackdaw-editor` from workspace membership and delete the crate.
  - Status: Complete
  - Notes: This removes the standalone Jackdaw launcher.
- [x] Keep `crates/foundation-editor-library` in workspace and clear it to Bevy-only.
  - Status: Complete
  - Notes: Remove asset picker/settings-window Jackdaw APIs; leave a minimal editor-time plugin/prelude.
- [x] Remove Jackdaw dependencies from root and crate manifests.
  - Status: Complete
  - Notes: Remove `jackdaw`, `jackdaw_api`, `jackdaw_runtime`, `jackdaw_jsn`, and now-unused editor-only dependencies.
- [x] Remove TemplateGame Jackdaw editor binary and configs.
  - Status: Complete
  - Notes: Delete `src/bin/editor.rs`, `.jsn/project.jsn`, `jackdaw.toml`, and Jackdaw Cargo aliases/config once no longer needed.
- [x] Preserve TemplateGame `editor` feature for Bevy-only editor mode.
  - Status: Complete
  - Notes: Expected launch command is `cargo run -p template-game --features editor -- --editor` unless user confirms a different exact command.
- [x] Regenerate/update lockfiles and verify no Jackdaw dependency remains.
  - Status: Complete
  - Notes: Use `cargo tree --workspace | rg "jackdaw|jackdaw_runtime|jackdaw_api|jackdaw_jsn"` as a removal check.

### Validation
- Format: Passed / not required for planning-only steps
- Lint: Passed / not required for planning-only steps
- Tests: Passed / not required for planning-only steps
- Build: Passed / not required for planning-only steps
- Documentation generation: Passed / not required for planning-only steps
- Full validation wrapper: Full cargo validation passed; wrapper not separately required
- User confirmation: Not required unless scope changes.

## Phase 3: Convert FoundationRuntimeLibrary To Bevy-Only
**Status:** Complete  
**Goal:** Preserve Foundation scene stack/splash/menu/gameplay systems while removing Jackdaw runtime imports, metadata, and docs.

### Tasks
- [x] Remove `jackdaw_runtime::prelude::*` imports and `EditorCategory` reflection metadata from Foundation modules.
  - Status: Complete
  - Notes: Use plain Bevy reflection/components unless a Foundation-owned editor metadata type is introduced later.
- [x] Update scene-source terminology away from Jackdaw `.jsn` levels toward Bevy/BSN catalog scenes.
  - Status: Complete
  - Notes: Preserve scene stack lifecycle behavior.
- [x] Update splash/menu/credits/settings documentation and field names where they imply Jackdaw editor ownership.
  - Status: Complete
  - Notes: Keep reusable runtime behavior intact.
- [x] Add/update unit tests for Bevy-only scene stack/menu/splash APIs as needed.
  - Status: Complete
  - Notes: Tests should prove behavior without editor dependencies.

### Validation
- Format: Passed / not required for planning-only steps
- Lint: Passed / not required for planning-only steps
- Tests: Passed / not required for planning-only steps
- Build: Passed / not required for planning-only steps
- Documentation generation: Passed / not required for planning-only steps
- Full validation wrapper: Full cargo validation passed; wrapper not separately required
- User confirmation: Not required unless public API choices need clarification.

## Phase 4: Rebuild TemplateGame Scene Flow With BSN Rust Scenes
**Status:** Complete  
**Goal:** Replace Jackdaw `.jsn` scene loading with Bevy `0.19` BSN Rust scene functions while preserving the requested game flow.

### Tasks
- [x] Remove TemplateGame Jackdaw editor paths, viewport targeting, Jackdaw PlayState logic, and Jackdaw scene parsing/loading.
  - Status: Complete
  - Notes: Runtime should be a Bevy game path plus optional Bevy-only editor mode.
- [x] Add BSN scene modules/functions for required scenes.
  - Status: Complete
  - Notes: Use `bsn!`, `bsn_list!`, and `impl Scene` where appropriate.
- [x] Add a scene catalog/loader responding to `SceneLoadRequested`.
  - Status: Complete
  - Notes: Spawn scene roots/entities tagged with `SceneOwner`; use post-processing only where BSN cannot easily attach ownership or runtime wiring.
- [x] Implement splash screen scene functions.
  - Status: Complete
  - Notes: Preserve startup splash sequence and transition to the menu flow.
- [x] Implement main menu and options menu scene functions.
  - Status: Complete
  - Notes: Use existing Foundation menu systems where possible.
- [x] Implement gameplay world and pause menu scene functions.
  - Status: Complete
  - Notes: Gameplay should pause when pause menu is open and resume/return through existing Foundation behavior.
- [x] Remove or migrate Jackdaw `.jsn` assets.
  - Status: Complete
  - Notes: Delete `.jsn` assets after the required behavior is translated to BSN Rust scene functions.

### Validation
- Format: Passed / not required for planning-only steps
- Lint: Passed / not required for planning-only steps
- Tests: Passed / not required for planning-only steps
- Build: Passed / not required for planning-only steps
- Documentation generation: Passed / not required for planning-only steps
- Manual runtime smoke: Passed with timeout-based window launch
- Full validation wrapper: Full cargo validation passed; wrapper not separately required
- User confirmation: Recommended for visible scene flow.

## Phase 5: Editor Mode Shell, Documentation, And Final Validation
**Status:** Complete  
**Goal:** Preserve the Foundation editor-time boundary as Bevy-only, document launch behavior, and complete validation.

### Tasks
- [x] Implement/keep a cleared Bevy-only `FoundationEditorPlugin` and prelude.
  - Status: Complete
  - Notes: No Jackdaw editor logic should remain.
- [x] Parse and document TemplateGame `--editor` launch mode.
  - Status: Complete
  - Notes: Recommended command is `cargo run -p template-game --features editor -- --editor` unless revised.
- [x] Rewrite `README.md` for Bevy `0.19`, BSN Rust scenes, both Foundation libraries, and commands.
  - Status: Complete
  - Notes: Remove Jackdaw Editor/static-game setup instructions.
- [x] Rewrite `docs/scene-system.md` for Bevy-only scene stack and TemplateGame BSN scene catalog.
  - Status: Complete
  - Notes: Explain splash/main/options/gameplay/pause flow.
- [x] Update `AGENTS.md` and relevant project-local skill guidance that still mandates Jackdaw crate boundaries, or record a user-approved postponement.
  - Status: Complete
  - Notes: Future Pi work should not be forced back into Jackdaw architecture.
- [x] Run full validation and record results.
  - Status: Complete
  - Notes: Use project wrapper scripts and dependency tree check.
- [x] Commit and push completed implementation checkpoints according to gitflow rules.
  - Status: Complete
  - Notes: Push to `origin` after each commit because `origin` is configured.

### Validation
- Format: Passed / not required for planning-only steps
- Lint: Passed / not required for planning-only steps
- Tests: Passed / not required for planning-only steps
- Build: Passed / not required for planning-only steps
- Documentation generation: Passed / not required for planning-only steps
- Full validation wrapper: Full cargo validation passed; wrapper not separately required
- Editor-mode smoke: Passed with timeout-based window launch
- User confirmation: User confirmed runtime flow working perfectly; final merge decision pending.

## Implementation / Review Handoff Notes
- Implementation model: `gpt-5.4`.
- Review model: `gpt-5.5`.
- Do not begin implementation until the user approves this revised plan/tracker.
- Mandatory implementation setup: read `feature-tracker-update`, `feature-plan-docs`, `rust-workspace-dev`, `rust-coding-standards`, and `gitflow-workflow` skills; confirm branch and branch base; update this tracker before editing code.
- Keep the requested functional flow front and center: splash screens, main menu, options menu, gameplay world, and pause menu.
- Treat any lingering Jackdaw dependency/import as a blocker unless the user explicitly changes scope.
- Keep both Foundation libraries; only the Jackdaw editor launcher is removed.
- Use BSN Rust scene functions for scenes until `.bsn` asset support exists.

## Postponed Work
- A replacement Bevy scene editor/tooling app is not part of this feature.
- First-party `.bsn` asset loading is not available now; file-backed BSN scenes are postponed until Bevy/project support exists.
- Credits and load-game scenes are optional to preserve; they may be deferred if needed because the user did not list them as must-have scenes.

## Notes / Issues / Oversights
- Project instructions and existing Foundation architecture skill currently describe Jackdaw boundaries. This feature should update them or explicitly record a reason if they cannot be changed in the same implementation.
- Existing `.jsn` assets contain layout/content that will be translated into BSN Rust scene functions. Functional Bevy-only parity is the priority; exact visual parity may need follow-up polish.
- Clarify whether `--editor` means `cargo run -p template-game --features editor -- --editor` or exactly `cargo run -p template-game -- --editor`.

## Progress Log
- `2026-07-14`: User requested removing Jackdaw editor entirely and keeping a Bevy-only project with existing systems plus splash/main/options/gameplay/pause scene flow.
- `2026-07-14`: Read mandatory feature planning, Foundation architecture, Gitflow, and Rust workspace skills.
- `2026-07-14`: Searched prior project memory for scene/Jackdaw context.
- `2026-07-14`: Inspected branch/remotes; repository was on `dev` tracking `origin/dev` with `origin` configured.
- `2026-07-14`: Created planning branch `feature/remove-jackdaw-editor` from `dev`.
- `2026-07-14`: Inspected workspace manifests, TemplateGame runtime/editor source, Foundation runtime modules, README, scene-system docs, and Jackdaw references.
- `2026-07-14`: Created initial plan and tracker; waiting for user approval before implementation.
- `2026-07-14`: User clarified the plan should update Bevy to `0.19`, use BSN scenes written as Rust files, keep both Foundation libraries with editor-time logic cleared, and require explicit `--editor` launch for editor features.
- `2026-07-14`: Researched Bevy `0.19` BSN API shape and revised plan/tracker for the clarified scope.

## Revision: Foundation Engine Launcher
**Status:** Complete  
**Goal:** Update the implementation target so Foundation is the engine/wrapper around Bevy and games are selected by launch arguments.

### User Clarification
The user approved the Bevy `0.19`, BSN, and retained Foundation editor-library plan, then clarified the expected launch architecture:

```cmd
cargo run -p foundation -- --game template-game --editor
```

Foundation should now be treated as the engine. The engine launcher should parse the requested game and runtime features, then run the selected game with Foundation runtime and optional editor-time systems.

### Additional / Revised Tasks
- [x] Add a `foundation` engine executable package.
  - Status: Complete
  - Notes: Planned package name is `foundation`, likely under `crates/foundation`, so the command is `cargo run -p foundation -- --game template-game --editor`.
- [x] Implement engine launch argument parsing for `--game <name>` and `--editor`.
  - Status: Complete
  - Notes: `--editor` should be a runtime engine mode, not a Jackdaw editor binary.
- [x] Expose TemplateGame through a game registration/plugin surface consumed by the Foundation engine.
  - Status: Complete
  - Notes: The game still owns concrete BSN scenes and game-specific plugin glue; Foundation owns launch/app orchestration.
- [x] Install the cleared Bevy-only Foundation editor-time plugin when `--editor` is present.
  - Status: Complete
  - Notes: Avoid requiring an additional Cargo feature for the initial cleared shell unless implementation discovers it is unavoidable.
- [x] Update README and scene-system docs to use the Foundation engine launch command.
  - Status: Complete
  - Notes: Direct TemplateGame launch can remain as a development detail only if still useful, but primary examples should use `foundation`.

### Additional Validation
- `cargo check -p foundation`: Passed.
- `cargo test -p foundation`: Passed.
- `cargo run -p foundation -- --game template-game`: Passed with timeout-based smoke launch.
- `cargo run -p foundation -- --game template-game --editor`: Passed with timeout-based smoke launch.

### Progress Log
- `2026-07-14`: User clarified Foundation should become the engine wrapper around Bevy, launched with a selected game and runtime feature flags, e.g. `cargo run -p foundation -- --game template-game --editor`. Plan/tracker updated; waiting for confirmation before implementation.

## Revision: Game Linking Modes
**Status:** Complete  
**Goal:** Capture support for both loose game modules and bundled distributed game builds.

### User Clarification
The Foundation engine should eventually support two build/linking modes:
- Loose game module mode: launch/select a game such as `template-game` through `--game template-game`, potentially from a separately built DLL/module, useful for debugging and multi-game engine installs.
- Bundled distributed mode: compile the selected game directly into the Foundation executable so shipping produces one executable.

### Additional / Revised Tasks
- [x] Design the Foundation game registration surface so it supports static bundled games now and future loose/dynamic modules later.
  - Status: Complete
  - Notes: Avoid hard-coding template-game except as a registered/default game.
- [x] Decide implementation depth for loose dynamic game loading in this feature.
  - Status: Complete
  - Notes: Recommended first pass is static bundled registration plus documented dynamic-module follow-up, because DLL/plugin loading adds ABI and build-system complexity.
- [x] Document build modes in README/developer docs.
  - Status: Complete
  - Notes: Explain development/multi-game loose mode versus single-exe distributed mode, including what is implemented now.

### Progress Log
- `2026-07-14`: User clarified Foundation should support loose game modules for development/multi-game use and bundled single-exe distributed builds. Plan/tracker updated; awaiting confirmation on how much dynamic loading to implement in this feature.

## Implementation Start
- `2026-07-14`: User confirmed the plan and clarified priority: removing Jackdaw and upgrading Bevy are higher priority than implementing the full Foundation engine launcher/dynamic game-module model. Implementation is starting with that priority order: first remove Jackdaw dependencies/configs and upgrade Bevy to `0.19`, then restore BSN scene flow, then add the Foundation engine launcher and document static/dynamic game-linking modes.

## Implementation Checkpoint: Jackdaw Removal / Bevy Upgrade Started
- `2026-07-14`: Began implementation with user-approved priority: remove Jackdaw and upgrade Bevy before deeper engine-linking work.
- `2026-07-14`: Updated root workspace dependencies to Bevy `0.19.0`, added a new `crates/foundation` engine launcher package, removed `crates/jackdaw-editor`, removed TemplateGame Jackdaw editor/config files, cleared `foundation-editor-library` to a Bevy-only shell, and removed Jackdaw manifest dependencies from active crates.
- `2026-07-14`: First validation attempt `cargo check -p foundation-runtime-library -p foundation-editor-library -p template-game -p foundation` is blocked before compilation because Bevy `0.19.0` requires `rustc 1.95.0`; current active toolchain is `rustc 1.92.0`, and installed nightly is `rustc 1.94.0-nightly`. Per project rules, machine-local toolchain changes were not made automatically.

## Implementation Checkpoint: Bevy 0.19 / Jackdaw Removal Validated
- `2026-07-14`: After Rust was upgraded to `rustc 1.97.0`, focused compile passed: `cargo check -p foundation-runtime-library -p foundation-editor-library -p template-game -p foundation`.
- `2026-07-14`: Removed remaining active source/manifest Jackdaw references, renamed scene-stack source API from `SceneSource::JsnLevel` / `jsn_level` to `SceneSource::BsnScene` / `bsn_scene`, added a BSN-authored splash scene function in template-game, and rewrote README plus `docs/scene-system.md` for the Bevy/Foundation architecture.
- `2026-07-14`: Dependency check passed: `cargo tree --workspace | rg "jackdaw|jackdaw_runtime|jackdaw_api|jackdaw_jsn"` returned no matches.
- `2026-07-14`: Validation passed: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`, and `cargo doc --workspace --all-features --no-deps`.
- `2026-07-14`: Smoke launch `timeout 6s cargo run -p foundation -- --game template-game` and `timeout 12s cargo run -p foundation -- --game template-game --editor` opened a Bevy window and were intentionally terminated by timeout. Bevy/wgpu Vulkan validation messages were logged during smoke runs on this machine; compile/test/build/doc validation remains passing.
- `2026-07-14`: Committed checkpoint `a65453c` (`Remove Jackdaw and upgrade Bevy`) and pushed it to `origin/feature/remove-jackdaw-editor`.

## Implementation Checkpoint: BSN Scene Organization Starting
- `2026-07-14`: User approved the next step to make Foundation cleaner/robust and TemplateGame a rock-solid example. Starting a focused pass to move TemplateGame scene catalog code into dedicated scene modules, convert remaining required scenes to BSN functions, and improve the Foundation engine game registry so game-specific asset/plugin setup is not hard-coded in the runner.

## Implementation Checkpoint: template-game BSN Scene Modules
- `2026-07-14`: Moved TemplateGame scene catalog into dedicated modules under `games/template-game/src/scenes/`: `mod.rs`, `splash.rs`, `menu.rs`, and `gameplay.rs`.
- `2026-07-14`: Converted the required TemplateGame scenes to BSN scene functions: splash screens, main menu, options menu marker, gameplay level marker, and pause menu.
- `2026-07-14`: Simplified `games/template-game/src/lib.rs` into the game plugin/engine integration surface and added `template_game::asset_root()` so the Foundation engine registry no longer hard-codes the game asset path.
- `2026-07-14`: Validation passed after scene split: `cargo check -p foundation -p template-game`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`, and `cargo doc --workspace --all-features --no-deps`.

## Implementation Checkpoint: TemplateGame Registration Name
- `2026-07-14`: Renamed the Foundation engine game registration from `PiGame` to `template-game` so the launch command matches the Cargo package/template identity: `cargo run -p foundation -- --game template-game`.
- `2026-07-14`: Renamed the Foundation engine game registration from `PiGame` to `template-game`, updated scene keys from `pigame/*` to `template-game/*`, renamed the plugin to `TemplateGamePlugin`, and updated current docs/guidance to use `cargo run -p foundation -- --game template-game`.
- `2026-07-14`: Validation passed after registration rename: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`, and `cargo doc --workspace --all-features --no-deps`.

## Implementation Checkpoint: Decouple Foundation From Concrete Games
- `2026-07-14`: User clarified the Foundation engine must not know about concrete games. Reworked `crates/foundation` to remove the direct `template-game` dependency and discover game extensions from `games/*/foundation.game.toml` manifests instead.
- `2026-07-14`: Added `games/template-game/foundation.game.toml` with game name `template-game` and launch package `template-game`. The Foundation launcher now forwards `--editor` to the selected game package rather than installing game plugins itself.
- `2026-07-14`: Updated TemplateGame's direct launcher to accept `--editor` and install the cleared `FoundationEditorPlugin` in that mode.
- `2026-07-14`: Validation passed after decoupling: `cargo check -p foundation -p template-game`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`, and `cargo doc --workspace --all-features --no-deps`.
- `2026-07-14`: Smoke launch `timeout 20s cargo run -p foundation -- --game template-game --editor` discovered the manifest, launched `target\debug\template-game.exe --editor`, opened a Bevy window, and was intentionally terminated by timeout. The same machine-specific wgpu Vulkan validation logs appeared during the smoke run.
- `2026-07-14`: Removed all `template-game` knowledge from `crates/foundation/src/main.rs`. The Foundation engine now requires `--game <game-name>` and uses generic test fixtures (`example-game`) instead of the template game name. Focused validation passed: `cargo check -p foundation`, `cargo test -p foundation`, `cargo clippy -p foundation --all-targets -- -D warnings`, and `cargo test --workspace --all-features`.

## Implementation Checkpoint: Scene Transition Bug Fix Starting
- `2026-07-14`: User reported two core scene-flow bugs: opening options from main menu or pause freezes the previous scene until Escape, and resuming from pause leaves the gameplay cube frozen. Resuming implementation with `gpt-5.4` on `feature/remove-jackdaw-editor`; branch is still a descendant of `dev` (`git merge-base --is-ancestor dev HEAD` returned 0). These are blocking Foundation scene-stack/runtime bugs and should be fixed before continuing planned architecture work.
- `2026-07-14`: Fixed options menu visibility by giving the TemplateGame options scene the same UI root Node/background style as other menus before `FoundationOptionsMenu` generates its runtime children.
- `2026-07-14`: Fixed Escape-based unpause by adding `FoundationResumeOnEscape`, exporting/registering it, and attaching it to the pause menu Escape handler so closing pause via Escape also clears `FoundationPauseState`.
- `2026-07-14`: Added a regression test proving an Escape close marker with `FoundationResumeOnEscape` closes the pause scene and unpauses gameplay. Validation passed: `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`, and `cargo doc --workspace --all-features --no-deps`.

## Implementation Checkpoint: Scene File Organization Starting
- `2026-07-14`: User confirmed scene transition fixes are working and requested each splash screen and each menu scene be split into its own Rust file with its own BSN setup, instead of grouping scenes by type. Resuming implementation with `gpt-5.4`; this is a game-scene organization/refactor within the approved BSN scene work.
- `2026-07-14`: Split grouped TemplateGame splash/menu scene modules into scene-specific Rust files: `pixel_perfect_splash.rs`, `bevy_splash.rs`, `main_menu.rs`, `options_menu.rs`, and `pause_menu.rs`. Removed the grouped `splash.rs` and `menu.rs` files; each split file now owns its scene-specific BSN setup.
- `2026-07-14`: Updated README and scene-system docs for the per-scene module layout. Validation passed: `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`, and `cargo doc --workspace --all-features --no-deps`.

## Implementation Checkpoint: Runtime Log Cleanup
- `2026-07-14`: Cleaned up runtime logs reported by the user. Removed TemplateGame's custom Ctrl+C handler so Bevy can install its own graceful-exit handler without warning, disabled the unused Gilrs gamepad plugin to avoid unmapped-controller warnings, and configured TemplateGame to prefer DirectX 12 on Windows to avoid local Vulkan validation-layer errors.
- `2026-07-14`: Demoted routine Foundation menu transition and starter-level logs from `info!` to `debug!` so normal play logs stay quieter.
- `2026-07-14`: Validation passed: `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`, and `cargo doc --workspace --all-features --no-deps`. Smoke launch `timeout 10s cargo run -p foundation -- --game template-game` used backend `Dx12` and no longer showed the previous Ctrl+C, Gilrs mapping, or Vulkan validation warnings/errors before timeout termination.
- `2026-07-14`: User reported the DirectX 12 cleanup introduced a black-screen delay before the gameplay cube appeared. Reverted the Windows renderer preference back to Vulkan for the fast path, but kept WGPU validation layers disabled by default through `InstanceFlags::empty().with_env()`. This preserves `WGPU_BACKEND` and validation env overrides while avoiding the previous Vulkan validation log flood.
- `2026-07-14`: Validation passed after restoring the fast renderer path: `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`, and `cargo doc --workspace --all-features --no-deps`. Smoke launch `timeout 8s cargo run -p foundation -- --game template-game` used backend `Vulkan` without the prior validation-layer errors before timeout termination.

## Implementation Checkpoint: Foundation Launcher Refactor Starting
- `2026-07-14`: User approved continuing with the planned Foundation launcher cleanup. Resuming implementation with `gpt-5.4` to split `crates/foundation/src/main.rs` into focused launch/manifest modules while keeping the engine game-agnostic.
- `2026-07-14`: Refactored `crates/foundation/src/main.rs` into focused launcher modules: `launch.rs` for argument parsing/game process spawning and `manifest.rs` for game-manifest discovery/parsing. `main.rs` now only wires Ctrl+C handling, argument parsing, and top-level error reporting.
- `2026-07-14`: Validation passed after Foundation launcher refactor: `cargo test -p foundation`, `cargo clippy -p foundation --all-targets -- -D warnings`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`, and `cargo doc --workspace --all-features --no-deps`.

## Implementation Checkpoint: Engine Docs And Scene-Key Cleanup Starting
- `2026-07-14`: User approved bundling Foundation engine documentation, scene-key terminology cleanup, regression tests, and full validation. Resuming implementation with `gpt-5.4` on `feature/remove-jackdaw-editor`.
- `2026-07-14`: Added `docs/foundation-engine.md` documenting Foundation's game-agnostic launcher, `--game`, `--editor`, `foundation.game.toml`, current loose Cargo-package mode, and future bundled/module loading modes. Linked it from README.
- `2026-07-14`: Renamed remaining runtime scene-target terminology from `scene_path` / `next_scene_path` to scene-key language in scene stack, menu buttons, pause opener, splash transition config, and TemplateGame scene setup. `SceneSource::BsnScene` now stores `key` instead of `path`.
- `2026-07-14`: Added a scene-stack regression test proving closing an input-blocking options overlay restores the underlying scene's visibility/input/update/focus flags.
- `2026-07-14`: Validation passed for the bundled cleanup: `cargo check -p foundation-runtime-library -p template-game`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`, and `cargo doc --workspace --all-features --no-deps`.

## Review Findings
- `2026-07-14` - gpt-5.5 sanity review
  - Overall result: Pass with optional follow-up items. No must-fix correctness or architecture blockers were found for the approved Jackdaw removal / Bevy 0.19 / BSN / Foundation launcher scope.
  - Must-fix:
    - None.
  - Optional improvements:
    - Tracker phase/task checkboxes and metadata still reflect early planning states even though later progress-log checkpoints record the implemented and validated work. Before merging, the tracker could be normalized for readability, but this is documentation hygiene rather than a runtime blocker.
    - The Foundation launcher's current loose mode intentionally shells out to `cargo run -p <package>`. This matches the documented current development mode, but packaged/non-Cargo launching and bundled single-executable builds remain follow-up architecture work.
    - Manifest discovery currently assumes the process current directory is the workspace root. This is fine for the documented `cargo run -p foundation -- --game <game-name>` workflow, but future installed-engine usage should add explicit search paths or configuration.
  - Validation reviewed:
    - `cargo tree --workspace | rg "jackdaw|jackdaw_runtime|jackdaw_api|jackdaw_jsn"` returned no matches.
    - Latest recorded full validation passed: `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`, and `cargo doc --workspace --all-features --no-deps`.
  - User decision: `Send to gpt-5.4 for fixes`.

## Implementation Checkpoint: Review Follow-Up Fixes Starting
- `2026-07-14`: User chose review option 3: send optional findings back for fixes before merge. Resuming implementation with `gpt-5.4` to normalize tracker state, make manifest discovery less dependent on process CWD, and explicitly record larger packaged-launch work as deferred follow-up.
- `2026-07-14`: Review follow-up fixes complete. Normalized tracker status/checklists to reflect implemented and validated work, updated review decision to `Send to gpt-5.4 for fixes`, documented packaged non-Cargo launching as deferred distribution work, and made Foundation workspace discovery walk up from the current directory with a compile-time fallback.
- `2026-07-14`: Added a Foundation launcher regression test proving workspace-root discovery works from nested directories. Validation passed after review follow-up fixes: `cargo test -p foundation`, `cargo clippy -p foundation --all-targets -- -D warnings`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`, and `cargo doc --workspace --all-features --no-deps`.

## Implementation Checkpoint: TemplateGame Thin Binary Wrapper Starting
- `2026-07-14`: User approved keeping both `lib.rs` and `main.rs`, but moving app construction into the library so the binary is only a thin wrapper. Resuming implementation with `gpt-5.4`.
- `2026-07-14`: Moved TemplateGame app construction, renderer setup, editor flag handling, default camera setup, and shared `run()` entry point into `games/template-game/src/lib.rs`. `src/main.rs` is now a thin wrapper that only calls `template_game::run()`.
- `2026-07-14`: Documented that the primary path remains Foundation-first while the thin wrapper preserves `cargo run -p template-game` and future `template-game.exe` packaging paths. Validation passed: `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`, and `cargo doc --workspace --all-features --no-deps`.
