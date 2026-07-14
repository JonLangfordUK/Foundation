# Remove Jackdaw Editor And Adopt Bevy BSN Tracker

## Metadata
- Feature slug: `remove-jackdaw-editor`
- Feature area: `multi-area` (`engine`, `game`, and `editor removal`)
- Primary area: `engine`
- Branch: `feature/remove-jackdaw-editor`
- Overall status: `Planned - revised for Bevy 0.19, BSN, and retained Foundation editor library`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Waiting for user approval to begin gpt-5.4 implementation`
- Created: `2026-07-14`
- Last updated: `2026-07-14`

## Branch And Push State
- Active planning branch: `feature/remove-jackdaw-editor`
- Branch base: Created from local `dev` on 2026-07-14; `dev` was an ancestor at branch creation.
- Remote: `origin` is configured as `https://github.com/JonLangfordUK/Foundation.git`.
- Push status: Planning documents not yet committed or pushed.

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- This feature should not be considered complete while production manifests or source imports still reference `jackdaw`, `jackdaw_api`, `jackdaw_runtime`, or `jackdaw_jsn`, unless the user explicitly revises the request.
- This feature should not remove `crates/foundation-editor-library`; it should keep that crate as a cleared Bevy-only editor-time library.
- This feature should update Bevy to `0.19` and move required TemplateGame scenes to BSN Rust scene functions.

## Phase 1: Planning And Approval
**Status:** In progress  
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
- [ ] Receive user approval to begin implementation.
  - Status: Pending
  - Notes: Implementation must not start until the user approves the revised plan/tracker.

### Validation
- Format: Pending; no Rust/source formatting required for planning docs yet.
- Lint: Pending; not required for planning-only docs.
- Tests: Pending; not required for planning-only docs.
- Build: Pending; not required for planning-only docs.
- Documentation generation: Pending; required before implementation completion, not for planning approval.
- Full validation wrapper: Pending; required during implementation completion.
- User confirmation: Pending.

## Phase 2: Upgrade Bevy And Remove Jackdaw Crates/Dependencies
**Status:** Planned  
**Goal:** Move to Bevy `0.19`, remove Jackdaw, and keep both Foundation libraries.

### Tasks
- [ ] Update Bevy dependencies to `0.19`.
  - Status: Planned
  - Notes: Expect Bevy API migration work during compile checks.
- [ ] Remove `crates/jackdaw-editor` from workspace membership and delete the crate.
  - Status: Planned
  - Notes: This removes the standalone Jackdaw launcher.
- [ ] Keep `crates/foundation-editor-library` in workspace and clear it to Bevy-only.
  - Status: Planned
  - Notes: Remove asset picker/settings-window Jackdaw APIs; leave a minimal editor-time plugin/prelude.
- [ ] Remove Jackdaw dependencies from root and crate manifests.
  - Status: Planned
  - Notes: Remove `jackdaw`, `jackdaw_api`, `jackdaw_runtime`, `jackdaw_jsn`, and now-unused editor-only dependencies.
- [ ] Remove TemplateGame Jackdaw editor binary and configs.
  - Status: Planned
  - Notes: Delete `src/bin/editor.rs`, `.jsn/project.jsn`, `jackdaw.toml`, and Jackdaw Cargo aliases/config once no longer needed.
- [ ] Preserve TemplateGame `editor` feature for Bevy-only editor mode.
  - Status: Planned
  - Notes: Expected launch command is `cargo run -p template-game --features editor -- --editor` unless user confirms a different exact command.
- [ ] Regenerate/update lockfiles and verify no Jackdaw dependency remains.
  - Status: Planned
  - Notes: Use `cargo tree --workspace | rg "jackdaw|jackdaw_runtime|jackdaw_api|jackdaw_jsn"` as a removal check.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Not required unless scope changes.

## Phase 3: Convert FoundationRuntimeLibrary To Bevy-Only
**Status:** Planned  
**Goal:** Preserve Foundation scene stack/splash/menu/gameplay systems while removing Jackdaw runtime imports, metadata, and docs.

### Tasks
- [ ] Remove `jackdaw_runtime::prelude::*` imports and `EditorCategory` reflection metadata from Foundation modules.
  - Status: Planned
  - Notes: Use plain Bevy reflection/components unless a Foundation-owned editor metadata type is introduced later.
- [ ] Update scene-source terminology away from Jackdaw `.jsn` levels toward Bevy/BSN catalog scenes.
  - Status: Planned
  - Notes: Preserve scene stack lifecycle behavior.
- [ ] Update splash/menu/credits/settings documentation and field names where they imply Jackdaw editor ownership.
  - Status: Planned
  - Notes: Keep reusable runtime behavior intact.
- [ ] Add/update unit tests for Bevy-only scene stack/menu/splash APIs as needed.
  - Status: Planned
  - Notes: Tests should prove behavior without editor dependencies.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Not required unless public API choices need clarification.

## Phase 4: Rebuild TemplateGame Scene Flow With BSN Rust Scenes
**Status:** Planned  
**Goal:** Replace Jackdaw `.jsn` scene loading with Bevy `0.19` BSN Rust scene functions while preserving the requested game flow.

### Tasks
- [ ] Remove TemplateGame Jackdaw editor paths, viewport targeting, Jackdaw PlayState logic, and Jackdaw scene parsing/loading.
  - Status: Planned
  - Notes: Runtime should be a Bevy game path plus optional Bevy-only editor mode.
- [ ] Add BSN scene modules/functions for required scenes.
  - Status: Planned
  - Notes: Use `bsn!`, `bsn_list!`, and `impl Scene` where appropriate.
- [ ] Add a scene catalog/loader responding to `SceneLoadRequested`.
  - Status: Planned
  - Notes: Spawn scene roots/entities tagged with `SceneOwner`; use post-processing only where BSN cannot easily attach ownership or runtime wiring.
- [ ] Implement splash screen scene functions.
  - Status: Planned
  - Notes: Preserve startup splash sequence and transition to the menu flow.
- [ ] Implement main menu and options menu scene functions.
  - Status: Planned
  - Notes: Use existing Foundation menu systems where possible.
- [ ] Implement gameplay world and pause menu scene functions.
  - Status: Planned
  - Notes: Gameplay should pause when pause menu is open and resume/return through existing Foundation behavior.
- [ ] Remove or migrate Jackdaw `.jsn` assets.
  - Status: Planned
  - Notes: Delete `.jsn` assets after the required behavior is translated to BSN Rust scene functions.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Manual runtime smoke: Pending
- Full validation wrapper: Pending
- User confirmation: Recommended for visible scene flow.

## Phase 5: Editor Mode Shell, Documentation, And Final Validation
**Status:** Planned  
**Goal:** Preserve the Foundation editor-time boundary as Bevy-only, document launch behavior, and complete validation.

### Tasks
- [ ] Implement/keep a cleared Bevy-only `FoundationEditorPlugin` and prelude.
  - Status: Planned
  - Notes: No Jackdaw editor logic should remain.
- [ ] Parse and document TemplateGame `--editor` launch mode.
  - Status: Planned
  - Notes: Recommended command is `cargo run -p template-game --features editor -- --editor` unless revised.
- [ ] Rewrite `README.md` for Bevy `0.19`, BSN Rust scenes, both Foundation libraries, and commands.
  - Status: Planned
  - Notes: Remove Jackdaw Editor/static-game setup instructions.
- [ ] Rewrite `docs/scene-system.md` for Bevy-only scene stack and TemplateGame BSN scene catalog.
  - Status: Planned
  - Notes: Explain splash/main/options/gameplay/pause flow.
- [ ] Update `AGENTS.md` and relevant project-local skill guidance that still mandates Jackdaw crate boundaries, or record a user-approved postponement.
  - Status: Planned
  - Notes: Future Pi work should not be forced back into Jackdaw architecture.
- [ ] Run full validation and record results.
  - Status: Planned
  - Notes: Use project wrapper scripts and dependency tree check.
- [ ] Commit and push completed implementation checkpoints according to gitflow rules.
  - Status: Planned
  - Notes: Push to `origin` after each commit because `origin` is configured.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- Editor-mode smoke: Pending
- User confirmation: Pending final visible flow acceptance or recorded waiver.

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
**Status:** Planning revision pending approval  
**Goal:** Update the implementation target so Foundation is the engine/wrapper around Bevy and games are selected by launch arguments.

### User Clarification
The user approved the Bevy `0.19`, BSN, and retained Foundation editor-library plan, then clarified the expected launch architecture:

```cmd
cargo run -p foundation -- --game template-game --editor
```

Foundation should now be treated as the engine. The engine launcher should parse the requested game and runtime features, then run the selected game with Foundation runtime and optional editor-time systems.

### Additional / Revised Tasks
- [ ] Add a `foundation` engine executable package.
  - Status: Planned
  - Notes: Planned package name is `foundation`, likely under `crates/foundation`, so the command is `cargo run -p foundation -- --game template-game --editor`.
- [ ] Implement engine launch argument parsing for `--game <name>` and `--editor`.
  - Status: Planned
  - Notes: `--editor` should be a runtime engine mode, not a Jackdaw editor binary.
- [ ] Expose TemplateGame through a game registration/plugin surface consumed by the Foundation engine.
  - Status: Planned
  - Notes: The game still owns concrete BSN scenes and game-specific plugin glue; Foundation owns launch/app orchestration.
- [ ] Install the cleared Bevy-only Foundation editor-time plugin when `--editor` is present.
  - Status: Planned
  - Notes: Avoid requiring an additional Cargo feature for the initial cleared shell unless implementation discovers it is unavoidable.
- [ ] Update README and scene-system docs to use the Foundation engine launch command.
  - Status: Planned
  - Notes: Direct TemplateGame launch can remain as a development detail only if still useful, but primary examples should use `foundation`.

### Additional Validation
- `cargo check -p foundation`: Pending
- `cargo test -p foundation`: Pending
- `cargo run -p foundation -- --game template-game`: Pending smoke test
- `cargo run -p foundation -- --game template-game --editor`: Pending smoke test

### Progress Log
- `2026-07-14`: User clarified Foundation should become the engine wrapper around Bevy, launched with a selected game and runtime feature flags, e.g. `cargo run -p foundation -- --game template-game --editor`. Plan/tracker updated; waiting for confirmation before implementation.

## Revision: Game Linking Modes
**Status:** Planning revision pending approval  
**Goal:** Capture support for both loose game modules and bundled distributed game builds.

### User Clarification
The Foundation engine should eventually support two build/linking modes:
- Loose game module mode: launch/select a game such as `template-game` through `--game template-game`, potentially from a separately built DLL/module, useful for debugging and multi-game engine installs.
- Bundled distributed mode: compile the selected game directly into the Foundation executable so shipping produces one executable.

### Additional / Revised Tasks
- [ ] Design the Foundation game registration surface so it supports static bundled games now and future loose/dynamic modules later.
  - Status: Planned
  - Notes: Avoid hard-coding template-game except as a registered/default game.
- [ ] Decide implementation depth for loose dynamic game loading in this feature.
  - Status: Pending user confirmation
  - Notes: Recommended first pass is static bundled registration plus documented dynamic-module follow-up, because DLL/plugin loading adds ABI and build-system complexity.
- [ ] Document build modes in README/developer docs.
  - Status: Planned
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
