# Remove Jackdaw Editor And Adopt Bevy BSN Plan

## Metadata
- Feature slug: `remove-jackdaw-editor`
- Feature area: `multi-area` (`engine`, `game`, and `editor removal`)
- Primary area: `engine`
- Branch: `feature/remove-jackdaw-editor`
- Status: `Planned - revised for Bevy 0.19 and BSN`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-07-14`
- Last updated: `2026-07-14`

## User Request
Remove the strict dependency on the Jackdaw editor completely. Keep the project as a Bevy-only game plus the Foundation systems already built together. Update Bevy to `0.19` and define scenes with BSN in Rust files for now because BSN is not currently a first-party asset file type. Preserve both Foundation libraries: runtime logic remains in `foundation-runtime-library`, and editor-time logic remains in `foundation-editor-library`, but the editor library should be cleared for now. Preserve a fully working scene system with splash screens, main menu, options menu, gameplay world, and pause menu. The game must be launched with `--editor` to include/use editor features.

## Feature Summary
This feature converts the repository from a Jackdaw-authored project into a Bevy `0.19` project with Foundation runtime and editor-time library boundaries intact.

The runtime scene flow will be rebuilt around Bevy Scene Notation (BSN) scene functions written in Rust. This replaces Jackdaw `.jsn` loading without waiting for `.bsn` asset support. TemplateGame will continue to prove the Foundation scene stack with:

1. splash screens,
2. main menu,
3. options menu,
4. gameplay world,
5. pause menu.

`foundation-editor-library` stays in the workspace as a Bevy-only, cleared editor-time crate. It should not depend on Jackdaw, and it should not contain the old Jackdaw asset picker/settings windows. TemplateGame should keep an editor feature/mode path for future editor-time functionality, but that path must be enabled explicitly by launching with editor mode.

Expected command shape unless implementation finds a safer simpler approach:

```cmd
cargo run -p template-game --features editor -- --editor
```

The `--features editor` part enables compile-time editor code; the `-- --editor` part is the runtime flag. If we want `cargo run -p template-game -- --editor` alone to work, editor code cannot rely on optional Cargo-feature dependencies.

## Feature Area Classification
- Area: `multi-area` (`engine`, `game`, and `editor removal`)
- Primary area: `engine`
- Rationale: The central work is removing Jackdaw and upgrading Foundation scene/runtime architecture to Bevy `0.19` BSN. TemplateGame proves the game flow. Editor code is retained only as an empty Bevy-only Foundation boundary.

## Codebase Research
- Root `Cargo.toml` currently includes `crates/foundation-editor-library`, `crates/foundation-runtime-library`, `crates/jackdaw-editor`, and `games/template-game`.
- Root workspace dependencies currently include Jackdaw crates and editor dependencies: `jackdaw`, `jackdaw_api`, `jackdaw_runtime`, `bevy_enhanced_input`, `rfd`, and `ctrlc`.
- `crates/jackdaw-editor` is a Jackdaw launcher and should be deleted and removed from the workspace.
- `crates/foundation-editor-library` currently depends on full Jackdaw editor APIs. It should remain in the workspace but be reduced to a minimal Bevy-only editor-time plugin/API shell.
- `crates/foundation-runtime-library` currently uses `jackdaw_runtime::prelude::*` mostly for `EditorCategory` reflection metadata. Runtime systems are otherwise Bevy ECS systems and should become Bevy-only.
- `foundation-runtime-library/src/scene_stack.rs` owns the useful ECS-first scene stack, lifecycle messages, `SceneOwner` cleanup, and presentation flags. Preserve this behavior.
- `foundation-runtime-library/src/splash_screen.rs`, `menu.rs`, and `credits.rs` use Jackdaw `.jsn` terminology and `EditorCategory` metadata. Replace that with Bevy/BSN language and plain Bevy reflection.
- `games/template-game/src/main.rs` currently adds `JackdawPlugin`. Replace this with Bevy `DefaultPlugins`, `FoundationPlugin`, `TemplateGamePlugin`, and optional Bevy-only editor mode setup.
- `games/template-game/src/lib.rs` has standalone and Jackdaw editor branches. Delete Jackdaw PlayState, viewport targeting, `jackdaw_jsn`, and `jackdaw::scene_io` code. Preserve or recreate editor feature/mode gating as Bevy-only.
- `games/template-game/src/bin/editor.rs`, `.jsn/project.jsn`, `jackdaw.toml`, old Jackdaw `.jsn` scene assets, and editor component-picker tests should be removed or replaced.

## External / API Research
Programming/API research found that Bevy `0.19` introduces the next-generation scene system and BSN. Relevant points for implementation:

- BSN is exposed through `bsn!` and `bsn_list!` macros.
- Scene functions can return `impl Scene` and compose child scenes with `Children [...]`.
- Scenes can be spawned through APIs such as `World::spawn_scene`, `Commands::spawn_scene`, and queued spawn variants.
- Bevy `0.19` focuses on code-driven BSN; first-party `.bsn` asset loading is not shipped yet. This matches the requested Rust-file BSN approach.
- BSN can use ordinary Rust expressions/templates, making it suitable for scene functions that still need runtime values or asset handles.

## Affected Files And Systems
- `Cargo.toml`: update Bevy to `0.19`; remove `crates/jackdaw-editor`; keep both Foundation libraries; remove Jackdaw workspace dependencies.
- `Cargo.lock`: regenerate after dependency changes.
- `crates/jackdaw-editor/`: delete.
- `crates/foundation-editor-library/`: keep, but clear Jackdaw implementation and expose a minimal Bevy-only `FoundationEditorPlugin`/prelude for future editor-time logic.
- `crates/foundation-runtime-library/Cargo.toml`: remove `jackdaw_runtime`; update Bevy dependency usage for `0.19`.
- `crates/foundation-runtime-library/src/*`: remove `EditorCategory`, Jackdaw imports, and Jackdaw-specific docs; preserve runtime systems.
- `games/template-game/Cargo.toml`: update Bevy to `0.19`; remove Jackdaw dependencies; keep an `editor` feature that enables the cleared `foundation-editor-library` if needed.
- `games/template-game/src/main.rs`: parse `--editor`, reject/warn if editor mode is requested without the editor feature, and initialize Bevy-only runtime/editor-mode plugins.
- `games/template-game/src/lib.rs` and likely new scene modules: replace Jackdaw scene loading with BSN Rust scene functions and a scene catalog/loader responding to `SceneLoadRequested`.
- `games/template-game/src/bin/editor.rs`: delete Jackdaw editor binary.
- Jackdaw assets/config: delete `.jsn/project.jsn`, `jackdaw.toml`, old `.jsn` assets once equivalent BSN scene functions exist.
- Tests/docs: replace Jackdaw/editor tests and rewrite README / scene-system docs.
- Project guidance: update `AGENTS.md` and relevant local skill guidance that still mandates Jackdaw crate boundaries.

## Proposed Implementation Approach
1. Confirm implementation starts on `feature/remove-jackdaw-editor`, verify `dev` ancestry, and update the tracker before code edits.
2. Update Bevy to `0.19` across workspace/game manifests.
3. Remove Jackdaw from the workspace:
   - delete `crates/jackdaw-editor`,
   - remove all `jackdaw*` dependencies,
   - remove Jackdaw editor binary/config/assets/tests.
4. Keep and clear `foundation-editor-library`:
   - remove Jackdaw-dependent modules,
   - leave a minimal Bevy-only plugin/resource/prelude,
   - keep it optional behind TemplateGame's `editor` feature for now.
5. Convert `foundation-runtime-library` to Bevy-only:
   - remove `jackdaw_runtime::prelude::*`,
   - remove `EditorCategory` reflect metadata,
   - update docs/comments from `.jsn`/Jackdaw to BSN/Bevy scene language,
   - preserve scene stack, splash, menu, pause, credits/settings behavior where still relevant.
6. Introduce TemplateGame BSN scene modules/functions:
   - use Bevy `0.19` BSN macros/functions for splash, main menu, options menu, gameplay world, and pause menu,
   - map scene constants/keys to scene functions,
   - spawn scene roots/entities with `SceneOwner`,
   - use imperative post-processing only where scene ownership, event wiring, or runtime resources make BSN awkward.
7. Replace `spawn_requested_jackdaw_scenes` with `spawn_requested_template_scenes` that loads from the BSN scene catalog.
8. Preserve menu and scene-stack behavior:
   - splash scenes transition through the stack,
   - main menu opens options and gameplay,
   - gameplay can open pause,
   - pause blocks gameplay updates and can resume/return as before.
9. Implement editor launch mode:
   - keep one game binary,
   - parse `--editor`,
   - when compiled with `--features editor`, install the cleared Bevy-only editor plugin/resource,
   - when not compiled with the feature, fail clearly or log a clear message if `--editor` is passed.
10. Rewrite documentation and project guidance for Bevy `0.19`, BSN Rust scenes, both Foundation libraries, and the editor-mode command.
11. Validate with focused and full checks, including Jackdaw dependency tree checks and runtime smoke tests.

## Alternatives Considered
- Keep Jackdaw `.jsn` as runtime data: rejected because the project should be Bevy-only and the user requested BSN.
- Remove `foundation-editor-library`: rejected after clarification. Keep it as the editor-time Foundation crate, cleared for now.
- Use Bevy legacy dynamic scenes instead of BSN: rejected because the requested scene definition format is BSN.
- Wait for `.bsn` asset files: rejected because Bevy does not currently ship first-party `.bsn` asset loading; use Rust BSN functions now.

## Risks, Constraints, And Assumptions
- Bevy `0.19` may require API updates beyond BSN (UI, events/messages, schedules, text, or asset APIs). Implementation should fix migration issues incrementally.
- Removing `jackdaw_runtime` removes `EditorCategory`; every affected reflected component must compile with plain Bevy reflection.
- Translating `.jsn` layout into BSN Rust functions may change exact visuals. Functional flow is the priority, then visual polish.
- Cargo features are compile-time. If editor-time logic has optional dependencies, the practical launch command is `cargo run -p template-game --features editor -- --editor`.
- Manual runtime validation is still needed for interactive menu flow unless tests are extended to simulate input.

## Open Questions
- Is `cargo run -p template-game --features editor -- --editor` acceptable, or must the command be exactly `cargo run -p template-game -- --editor`?
- Should credits and load-game scenes remain in the first Bevy/BSN conversion, or can they be deferred since the must-have list is splash/main/options/gameplay/pause?
- Should `foundation.settings.toml` keep an editor startup setting, or be simplified now that Jackdaw editor startup is gone?

## Documentation Expectations
- Public APIs changed by this feature must have Rustdoc comments, especially scene-source replacements, BSN scene catalog helpers exposed across modules, and editor-mode resources/plugins.
- `README.md` must document Bevy `0.19`, BSN Rust scene files, both Foundation libraries, and the editor-mode launch command.
- `docs/scene-system.md` must describe the Foundation scene stack plus TemplateGame BSN scene catalog.
- Project-local instructions/skills that hard-code Jackdaw boundaries should be updated or explicitly recorded as follow-up if not changed in this feature.
- Generated documentation must be produced before the feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Before editing, read this plan, the tracker, and required project skills: `feature-tracker-update`, `feature-plan-docs`, `rust-workspace-dev`, `rust-coding-standards`, and `gitflow-workflow`.
- Keep both Foundation libraries. Delete only the Jackdaw editor launcher crate.
- Do not leave `jackdaw`, `jackdaw_api`, `jackdaw_runtime`, or `jackdaw_jsn` in production manifests unless the user explicitly changes scope.
- Prefer small BSN scene functions/modules over one large scene-spawning function.

## Optional Review Focus Areas
- Verify no Jackdaw dependency/import/config remains.
- Verify Bevy `0.19` migration is complete and not pinned to old Bevy APIs.
- Verify `foundation-editor-library` remains but is Bevy-only and cleared.
- Verify TemplateGame scenes are BSN Rust functions and the requested flow works.
- Verify editor-mode launch behavior matches the chosen command.

## Success Criteria
- Workspace uses Bevy `0.19`.
- Root workspace builds without `jackdaw`, `jackdaw_api`, `jackdaw_runtime`, or `jackdaw_jsn`.
- `crates/jackdaw-editor` is removed.
- `crates/foundation-runtime-library` remains active and Bevy-only.
- `crates/foundation-editor-library` remains as a cleared Bevy-only editor-time library.
- TemplateGame uses BSN Rust scene functions for required scenes.
- `cargo run -p template-game` launches the normal Bevy-only game.
- Editor-time mode is explicit via `--editor` and documented.
- Splash screens transition into main menu.
- Main menu can open options and gameplay.
- Gameplay can open pause menu; pause blocks gameplay updates and can resume.
- Full validation passes or blockers are documented with user-approved waivers.

## Testing Methodology
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- `scripts/doc-project.cmd` when present, otherwise `cargo doc --workspace --all-features --no-deps`
- `scripts/validate-project.cmd` for full validation when present
- Focused checks during implementation:
  - `cargo check -p foundation-runtime-library`
  - `cargo test -p foundation-runtime-library`
  - `cargo check -p foundation-editor-library`
  - `cargo test -p foundation-editor-library`
  - `cargo check -p template-game`
  - `cargo check -p template-game --features editor`
  - `cargo test -p template-game`
  - `cargo test -p template-game --features editor`
  - `cargo tree --workspace | rg "jackdaw|jackdaw_runtime|jackdaw_api|jackdaw_jsn"` expecting no matches
  - timed/manual `cargo run -p template-game` smoke test
  - timed/manual `cargo run -p template-game --features editor -- --editor` smoke test, unless the launch command is revised

## Revision: Foundation Engine Launcher
The user approved the Bevy `0.19`, BSN, and retained Foundation editor-library direction, then clarified the intended launch architecture:

```cmd
cargo run -p foundation -- --game PiGame --editor
```

This revision supersedes earlier TemplateGame-direct launch examples in this plan.

Foundation should now be treated as the project engine: effectively a wrapper around Bevy plus the Foundation runtime/editor libraries. Implementation should add a `foundation` executable package, planned as `crates/foundation` with package name `foundation`, and make it the primary launch target.

Responsibilities for the `foundation` engine launcher:
1. Parse runtime arguments such as `--game PiGame` and `--editor`.
2. Select the requested registered game by name.
3. Build and run the Bevy app with Foundation runtime systems installed.
4. Include/enable the cleared Foundation editor-time logic when `--editor` is present.
5. Keep game-specific BSN scenes and plugins registered through a clear game integration surface rather than launching the game crate directly.

Implementation should treat `--editor` as a runtime engine mode. Because the user expects `cargo run -p foundation -- --game PiGame --editor`, the initial cleared editor-time shell should not require an additional Cargo feature flag unless a later dependency makes that unavoidable and the tracker records the reason.

Additional affected systems:
- `crates/foundation`: new engine executable package for launch argument parsing, game selection, and Bevy app construction.
- `games/template-game` / PiGame integration: expose the game plugin/registration/BSN scene catalog to the Foundation engine instead of relying on direct `cargo run -p template-game` as the primary launch path.
- README and scene-system docs: document Foundation as the engine wrapper around Bevy and use `cargo run -p foundation -- --game PiGame --editor` for editor-mode launch examples.

Additional success criteria:
- `cargo run -p foundation -- --game PiGame` launches the normal Bevy-only game.
- `cargo run -p foundation -- --game PiGame --editor` launches the selected game with Foundation editor-time mode included/enabled.
- `cargo check -p foundation` and `cargo test -p foundation` pass.

Additional validation:
- `cargo check -p foundation`
- `cargo test -p foundation`
- timed/manual `cargo run -p foundation -- --game PiGame` smoke test
- timed/manual `cargo run -p foundation -- --game PiGame --editor` smoke test

## Revision: Game Linking Modes
The user further clarified that the Foundation engine should support two game-linking/distribution modes:

1. **Loose game module mode** for development / multi-game engine installs.
   - The engine executable can load or select a game by `--game PiGame`.
   - The game may be built as a loose dynamic library/module or otherwise separately configured package.
   - This helps debugging and allows one Foundation engine build to work with many games.

2. **Bundled distributed mode** for shipping.
   - The selected game can be compiled directly into the Foundation executable depending on build configuration.
   - A distributed build should produce a single executable for the game/engine combination.

Implementation should design the first pass so both modes are architecturally possible, even if only the bundled/static mode is implemented initially to keep scope manageable. The game registration surface should not assume there will only ever be one statically linked game, and the `--game PiGame` argument should remain meaningful in both modes.

Planning implication:
- Add a build/configuration abstraction for game source selection, for example a `FoundationGameRegistry` with statically registered games now and a future dynamic-module registry later.
- Document which mode is implemented in this feature and which pieces are intentionally deferred.
- Avoid hard-coding PiGame in the engine launcher except as a registered/default game entry.

Additional open question:
- Should this feature implement both static bundled and loose dynamic game-module loading now, or implement static bundled registration first and leave loose DLL loading as a planned follow-up? Recommended first pass: static bundled registration with architecture and docs prepared for dynamic modules, because safe Rust dynamic plugin loading has extra ABI/build-system complexity.
