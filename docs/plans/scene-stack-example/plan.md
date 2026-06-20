# Scene Stack Example Plan

## Metadata
- Feature slug: `scene-stack-example`
- Feature area: `multi-area`
- Primary area: `game`
- Branch: `feature/scene-stack-example`
- Branch status: Created locally from `dev` on 2026-06-20; push status pending.
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-06-20`
- Last updated: `2026-06-20`

## User Request
Create an example scene stack flow with three scenes:
1. A splash screen showing centered `Pixel Perfect` text.
2. A splash screen showing centered `Bevy` text.
3. An example main menu.

Both splash screens should fade in over 1.5 seconds, stay on screen for 2 seconds, then fade out over 1.5 seconds. These timings should be adjustable. After the final splash screen, the game should load the main menu and reset the scene stack. All three scenes should use the existing scene system and, if possible, be Jackdaw scenes authored as `.jsn` files. Shared/reusable splash logic should live in `foundation-library`; the concrete scene sequence/assets should live in `games/template-game`.

## Feature Summary
This feature will turn the current placeholder TemplateGame startup into a data-driven example of the Foundation scene stack. The game will start by opening a Jackdaw `.jsn` splash scene through the stack, transition to the second `.jsn` splash scene, then clear/reset the stack and open the `.jsn` main menu scene. FoundationLibrary will provide reusable splash-screen primitives and runtime systems so future games can add additional timed splash scenes by authoring new `.jsn` scene data rather than duplicating timing/fade logic.

## Feature Area Classification
- Area: `multi-area` (`game` + shared `foundation-library` support)
- Primary area: `game`
- Rationale: The visible feature is a TemplateGame example and scene asset flow. Shared splash primitives belong in `foundation-library`, but the concrete scenes, text labels, and sequence belong to `games/template-game`.

## Codebase Research
- Root `Cargo.toml` is a workspace with `crates/foundation-library`, `crates/jackdaw-editor`, and `games/template-game` members. It uses Bevy `0.18.1`, Jackdaw `0.4.1`, and `jackdaw_runtime` `0.4.1`.
- `crates/foundation-library/src/lib.rs` installs `FoundationSceneStackPlugin`, registers shared reflected types, and re-exports scene stack APIs from its prelude.
- `crates/foundation-library/src/scene_stack.rs` already supports `SceneSource::JsnLevel { path }`, `SceneLoadRequested`, `SceneOwner`, `SceneCommand::ClearAndOpen`, `OpenSceneOptions::clear_stack`, and `SceneCommandsExt::clear_and_open_scene`.
- The current scene stack intentionally only emits load requests for `.jsn` scenes; a game-side bridge system must consume `SceneLoadRequested`, spawn `JackdawSceneRoot(asset_server.load(path))`, and tag spawned scene roots with `SceneOwner`.
- `games/template-game/src/main.rs` currently bypasses the scene stack for startup by spawning `JackdawSceneRoot(asset_server.load("scene.jsn"))` directly and spawning a default 3D camera.
- `games/template-game/assets/scene.jsn` is a Jackdaw scene containing serialized components in a top-level `scene` array. New splash/menu scene assets should follow Jackdaw `.jsn` format rather than inventing a separate scene format.
- `games/template-game/src/lib.rs` currently contains only `TemplateGamePlugin`, `SpinningCube`, and play-gate logic. It is the right place for game-specific startup sequence wiring and any TemplateGame-specific bridge systems.
- `crates/jackdaw-editor/src/main.rs` is a generic Jackdaw editor launcher, not the place for game-specific scene sequence implementation.

## External Research
No external online research was performed because the current repository already contains the relevant Foundation scene stack, Jackdaw runtime usage, and existing `.jsn` examples needed for planning. Implementation should verify exact Bevy 0.18 UI/text component serialization details locally before hand-authoring `.jsn` UI/text content.

## Affected Files And Systems
- `crates/foundation-library/src/lib.rs`: register and re-export reusable splash-screen types/systems.
- `crates/foundation-library/src/splash_screen.rs` or similar new module: reusable splash configuration component/resource, fade timer/state component, and systems that drive fade/hold/fade-out and scene transitions.
- `crates/foundation-library/src/scene_stack.rs`: likely unchanged unless implementation discovers a small stack API ergonomic gap.
- `games/template-game/src/lib.rs`: add TemplateGame scene sequence/plugin wiring, game-side Jackdaw `.jsn` load bridge, startup stack command, and tests where practical.
- `games/template-game/src/main.rs`: replace direct `scene.jsn` startup with scene-stack startup; adjust camera/UI setup as needed.
- `games/template-game/assets/*.jsn`: add Jackdaw scenes for `splash_pixel_perfect.jsn`, `splash_bevy.jsn`, and `main_menu.jsn` (names can vary if implementation documents them consistently).
- `games/template-game/tests/*`: add or update tests for scene sequence constants/configuration and bridge behavior where practical.
- `docs/plans/scene-stack-example/*`: maintain plan/tracker updates during implementation.

## Proposed Implementation Approach
1. Add reusable Foundation splash primitives.
   - Define a reflected component such as `FoundationSplashScreen` that can be serialized in Jackdaw `.jsn` and contains text, fade-in seconds, hold seconds, fade-out seconds, and next-scene behavior.
   - Define next-scene behavior data that supports opening the next `.jsn` scene normally or clearing/resetting the stack before opening it.
   - Register these reflected types in `FoundationPlugin` and export them from `foundation_library::prelude`.
2. Add reusable Foundation splash systems.
   - On splash component detection/scene load, create or update the centered UI/text presentation and initialize timing/fade state.
   - Drive alpha from 0.0 to 1.0 during fade-in, remain at 1.0 during hold, then return to 0.0 during fade-out.
   - At completion, emit the appropriate scene command: first splash opens/replaces the second splash; second splash clears stack and opens the main menu.
   - Ensure any spawned UI/text entities are tagged with `SceneOwner` when possible so scene cleanup removes them.
3. Add TemplateGame concrete scene definitions.
   - Author three Jackdaw `.jsn` scene files for Pixel Perfect splash, Bevy splash, and main menu.
   - Put scene-specific data in those `.jsn` files: splash text/timings/next target for splash scenes and a marker/config for the main menu scene.
   - Keep the splash values adjustable by changing serialized data or a clearly documented default config.
4. Wire TemplateGame to the scene stack.
   - Replace direct startup scene spawning with a scene-stack open of the first splash `.jsn` scene.
   - Add a game-side bridge that consumes `SceneLoadRequested` for `SceneSource::JsnLevel` and spawns `JackdawSceneRoot` for the requested asset path with `SceneOwner`.
   - Keep bridge implementation in the game crate unless reusable enough to promote later; the user specifically requested concrete implementation in the game.
5. Build an example main menu.
   - The main menu should also be a Jackdaw `.jsn` scene and participate in the scene stack.
   - Minimal acceptable menu content is a centered title/label and/or basic options demonstrating that the splash sequence reached the menu.
6. Add tests and validation.
   - Unit-test reusable timing phase math and/or scene command selection without opening a window.
   - Test TemplateGame constants/paths and bridge behavior where possible in Bevy `MinimalPlugins`.
   - Run the project validation wrappers before marking work complete.

## Flow And Timing Illustration
```text
Start TemplateGame
  ↓
Scene stack opens Jackdaw .jsn: splash_pixel_perfect.jsn
  ↓
Pixel Perfect splash
[ fade in 1.5s ][ hold 2.0s ][ fade out 1.5s ] = 5.0s
  ↓
Scene stack transitions to Jackdaw .jsn: splash_bevy.jsn
  ↓
Bevy splash
[ fade in 1.5s ][ hold 2.0s ][ fade out 1.5s ] = 5.0s
  ↓
Scene stack reset / clear-and-open
  ↓
Jackdaw .jsn: main_menu.jsn
  ↓
Main menu active as the only scene on the stack
```

Default total splash time before main menu: `10.0s`.

## Alternatives Considered
- Hard-code both splash screens directly in `games/template-game`: rejected because the user wants shared splash logic in FoundationLibrary and wants adding more splash screens later to be easy.
- Implement splash screens as non-`.jsn` runtime-only scenes: rejected for the planned path because the user prefers all three scenes to be Jackdaw `.jsn` scenes if possible.
- Put concrete TemplateGame scene sequence data in FoundationLibrary: rejected because FoundationLibrary should own reusable logic, while the game crate owns concrete game scenes and sequence choices.
- Add a generic Jackdaw `.jsn` bridge directly to FoundationLibrary immediately: deferred unless implementation proves it should be reusable. The current scene stack was designed for game systems to consume `SceneLoadRequested`, and the user asked for scene implementation in the game.

## Risks, Constraints, And Assumptions
- Jackdaw `.jsn` UI/text serialization may require exact reflected Bevy type names/field formats. Implementation should prefer generating or validating scene files through Jackdaw when possible; hand-authored `.jsn` must be tested.
- If UI/text cannot be cleanly authored directly in `.jsn`, a fallback is to put Foundation splash config components in `.jsn` and let reusable Foundation systems spawn the runtime UI/text. This still keeps each splash as a Jackdaw scene because the scene data/config comes from `.jsn`.
- Scene-owned cleanup depends on all scene-spawned roots or spawned child entities receiving `SceneOwner`.
- The current `cleanup_removed_scene_entities` despawns tagged entities; implementation should verify whether this removes Jackdaw scene descendants correctly in Bevy 0.18/Jackdaw runtime.
- Splash timing should use Bevy time delta and be deterministic enough for tests via isolated phase/timer math.
- The main menu scope is example-level, not a full interactive menu system, unless the user expands the request.

## Open Questions
- Should transitioning from the first splash to the second splash close/replace the first splash immediately, or briefly stack the second over the first? Proposed default: replace/close the completed splash so only the active splash remains.
- Should the centered text be authored as Bevy UI/text components in the `.jsn` files, or should the `.jsn` hold a Foundation splash config and Foundation systems spawn the text at runtime? Proposed default: use `.jsn` for scene/config and reusable Foundation systems for runtime UI/text if direct text serialization is brittle.
- Should the main menu include interactive buttons now, or only an example centered title/options? Proposed default: minimal visible main menu scene.

## Documentation Expectations
- Public reusable Foundation splash types and systems must have Rustdoc comments.
- Any scene sequence constants in TemplateGame should be documented enough that future splash scenes can be added by copying/changing `.jsn` data.
- If hand-authored `.jsn` files are added, include comments in Rust/docs explaining their role because JSON itself cannot contain comments.
- Generated documentation must be produced before the feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Before editing implementation files, read this plan, `tracker.md`, `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, and `.pi/skills/gitflow-workflow/SKILL.md`.
- Confirm the active branch is `feature/scene-stack-example` and record any branch-base uncertainty in the tracker.
- Keep reusable splash logic in `crates/foundation-library`; keep concrete scene assets and TemplateGame sequence setup in `games/template-game`.
- Preserve Jackdaw `.jsn` as the scene/data source for all three scenes.
- Keep timings adjustable through serialized component data or a clearly exposed config, with defaults of fade-in `1.5`, hold `2.0`, fade-out `1.5` seconds.
- Update the tracker before and during implementation, including validation results and any `.jsn` serialization compromises.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Verify all three scenes are opened through the Foundation scene stack and use Jackdaw `.jsn` sources.
- Verify the last splash resets the stack before opening the main menu.
- Verify splash logic is reusable and not duplicated per splash.
- Verify ownership cleanup prevents old splash entities from remaining after transitions.

## Success Criteria
- Starting TemplateGame opens the Pixel Perfect splash via the scene stack from a Jackdaw `.jsn` source.
- Pixel Perfect splash fades in for 1.5 seconds, holds for 2 seconds, fades out for 1.5 seconds, with timings adjustable.
- The Bevy splash uses the same reusable logic and follows the same default timings.
- After the Bevy splash completes, the scene stack is reset and the main menu `.jsn` scene is opened as the only stack entry.
- Splash logic lives in FoundationLibrary; concrete scene assets/sequence wiring live in TemplateGame.
- Rust tests and validation wrappers pass or any blocker/waiver is recorded in the tracker.

## Testing Methodology
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- `scripts/doc-project.cmd`
- `scripts/validate-project.cmd`
- Manual launch check for `cargo run -p template-game` long enough to observe splash-to-menu flow, if practical in the environment.
