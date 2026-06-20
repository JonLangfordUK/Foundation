# Foundation Scene Stack Plan

## Metadata
- Feature slug: `foundation-scene-stack`
- Feature area: `multi-area`
- Primary area: `engine`
- Foundation area: `Foundation Library`
- Branch: `feature/foundation-scene-stack`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-06-20`
- Last updated: `2026-06-20`

## User Request
Build a robust scene management system in FoundationLibrary. The user wants a stack of scenes usable for UI and gameplay levels. Opening a scene pushes it onto the stack; closing a scene removes it and restores focus/visibility/interaction to the previous relevant scene. Each scene controls how earlier scenes react, such as hiding them, keeping them visible but non-interactive, or keeping them visible and interactive. Scenes need lifecycle notifications for added, removed, focused, and unfocused. The public game API should be simple: open scenes, optionally remove previous scenes, close the current scene, or close a target scene. The design must lean into Jackdaw's ECS concepts of scenes and support `.jsn` level files.

## Feature Summary
FoundationLibrary will provide an ECS-first scene stack system for Jackdaw-style games. The stack will be represented by Bevy resources, components, messages, and systems rather than trait-object scene controllers. Scene entries remain resident while they are on the stack; removing a scene cleans up its owned entities/resources. Scene commands are buffered so multiple commands can be issued per frame and applied at a safe schedule point.

## Feature Area Classification
- Area: `multi-area`
- Primary area: `engine`
- Rationale: The feature lives in `crates/foundation-library` and supplies reusable runtime architecture for games. It also touches game integration and Jackdaw editor compatibility, so it is cross-cutting under the current game/engine/editor taxonomy. The user's intended owner is FoundationLibrary.

## Codebase Research
- The workspace contains `crates/foundation-library`, `crates/jackdaw-editor`, and `games/template-game`.
- `crates/foundation-library/src/lib.rs` currently exposes `FoundationPlugin`, `FoundationSettings`, `FoundationActor`, and a `prelude`. `FoundationPlugin` registers reflected Foundation types and initializes resources.
- `crates/foundation-library/Cargo.toml` depends on workspace `bevy` and `jackdaw_runtime`; this is the right crate for reusable scene stack APIs.
- `games/template-game/src/lib.rs` uses Bevy and Jackdaw runtime concepts and gates gameplay systems through `play_gate::is_playing`.
- `games/template-game/assets/scene.jsn` is a Jackdaw `.jsn` scene containing serialized components under a `scene` array. The scene stack must not invent a competing level format.
- Root `Cargo.toml` uses Bevy `0.18.1` with `serialize`, `reflect_documentation`, `file_watcher`, and Jackdaw runtime `0.4.1`.
- Current branch at planning start was `dev`; feature planning moved to `feature/foundation-scene-stack` from `dev`.

## External Research
- Bevy 0.18 distinguishes pull-based `Message`/`Messages` from observer-triggered `Event`s. `MessageWriter`/`MessageReader` support buffered, predictable, batch processing and are suitable for deferred scene command queues and lifecycle notifications.
- Bevy 0.18 `Event`s trigger observers immediately via `World::trigger`/`Commands::trigger`. That is useful for immediate observer patterns, but scene stack changes should be deterministic and processed at known schedule points, so messages are the better default for command input and lifecycle output.

## Affected Files And Systems
- `crates/foundation-library/src/lib.rs`: register the scene stack plugin/module and re-export public scene stack APIs.
- `crates/foundation-library/src/scene_stack.rs` or `crates/foundation-library/src/scenes/`: likely home for scene IDs, keys, stack resource, command messages, lifecycle messages, components, policies, and systems.
- `games/template-game/src/lib.rs`: may receive a small demonstration or smoke-test integration only if useful; avoid coupling FoundationLibrary to TemplateGame specifics.
- `games/template-game/assets/scene.jsn`: used as an example of the Jackdaw scene format to support; do not modify unless a later phase adds explicit sample assets.
- Documentation under `docs/`: add feature-level API/architecture notes if Rustdoc alone is insufficient.

## Proposed Implementation Approach
1. Add a FoundationLibrary scene stack module and plugin registration.
2. Define core data types:
   - `FoundationSceneId` generated internally.
   - `FoundationSceneKey` or label/name for optional target lookup and debugging.
   - `SceneSource` / `FoundationSceneSource` with at least `.jsn` level path support and a generic/runtime key variant if needed.
   - `ScenePresentation` with explicit `covers_previous`, `blocks_previous_input`, and `blocks_previous_update` flags.
   - `SceneStack` resource as the authoritative ordered stack.
   - `SceneStackEntry` records containing id, optional key, source, presentation, and runtime flags.
3. Define buffered scene commands supporting multiple commands per frame:
   - open scene,
   - open with options,
   - close current,
   - close by id,
   - close by key/name,
   - clear stack,
   - clear and open.
4. Define lifecycle messages:
   - `SceneAdded`,
   - `SceneRemoved`,
   - `SceneFocused`,
   - `SceneUnfocused`.
5. Define ownership and runtime state components/resources:
   - `SceneOwner { scene_id }` for entities spawned/owned by a scene.
   - Runtime flags for focused, visible, interactive, and updating status, recalculated from the stack and presentation policy.
6. Implement scene command processing as an ordered system at a safe point in the Bevy schedule. It should capture old focus, apply all queued commands deterministically, emit lifecycle messages, and recalculate runtime flags.
7. Implement cleanup behavior so removing a scene despawns or marks for despawn all entities tagged with that scene's `SceneOwner`.
8. Integrate Jackdaw `.jsn` level support as an ECS-level source concept. The Foundation stack should issue messages/state that a Jackdaw-compatible loading system can consume, without duplicating Jackdaw's serialized scene format.
9. Add ergonomic game-facing APIs via extension traits/helpers, for example `commands.open_scene(...)`, `commands.close_current_scene()`, and `commands.clear_and_open_scene(...)`.
10. Add unit tests for stack command behavior, lifecycle message ordering, focus restoration, visibility/interaction/update flag calculation, key/id targeting, multi-command processing, and cleanup ownership rules.

## Alternatives Considered
- Trait-object scene controllers: rejected by the user because they would create a parallel scene concept and may conflict with Jackdaw's ECS/data-driven scene model.
- Hybrid trait-object plus ECS model: rejected by the user because it risks becoming confusing.
- A single enum such as `Hidden`, `VisibleNonInteractive`, `VisibleInteractive`: deferred/rejected because explicit policy flags offer better combinations for gameplay, pause menus, inventory overlays, debug overlays, and future systems.

## Risks, Constraints, And Assumptions
- The exact Jackdaw runtime API for loading `.jsn` scenes must be verified during implementation; FoundationLibrary should avoid depending on private Jackdaw internals.
- Bevy 0.18 uses `Message` APIs for buffered message processing; implementation should use the correct Bevy 0.18 names and scheduling patterns.
- Scene cleanup must be safe and deterministic; every entity spawned for a scene must receive `SceneOwner` or cleanup will leak scene-owned entities.
- Focus should be recalculated rather than stored as independent authoritative state.
- Runtime flags should be derived from stack order and presentation policy to avoid state drift.
- Scene stack presence controls memory lifetime: scenes on the stack remain loaded/resident; removed scenes are cleared from memory.

## Open Questions
- Should `SceneSource` initially include only `.jsn` level paths and named runtime scenes, or should UI scenes have a first-class source variant from day one?
- Should close-by-target support both `FoundationSceneId` and `FoundationSceneKey` in the first implementation? Current preference is yes.
- Which Bevy schedule labels should FoundationLibrary expose for scene command processing and post-lifecycle reactions?
- How much direct `.jsn` loading should FoundationLibrary perform versus delegating to `jackdaw_runtime` systems?

## Documentation Expectations
- Public APIs added by this feature must have Rustdoc comments.
- Add feature-level documentation if the API has scheduling/lifecycle ordering rules that are too large for Rustdoc alone.
- Document ownership rules: scene-spawned entities must be tagged with `SceneOwner`.
- Document command processing order and focus/visibility/input/update derivation.
- Generated documentation must be produced before the feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Keep the architecture ECS-first; do not introduce trait-object scene controllers.
- Preserve Jackdaw `.jsn` as the source level format rather than creating a competing Foundation scene serialization format.
- Prefer minimal dependencies; current Bevy and Jackdaw runtime dependencies should be enough for the first implementation.
- Use generated scene IDs internally and optional keys/names for game-friendly targeting.
- Support multiple commands per frame and process them in deterministic queue order.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Verify no trait-object or hybrid scene-controller architecture slipped in.
- Verify lifecycle message ordering around clear-and-open, close-current, and close-buried-scene cases.
- Verify scene cleanup cannot accidentally despawn non-scene-owned entities.
- Verify `.jsn`/Jackdaw integration is a clean bridge rather than a duplicate scene format.

## Success Criteria
- FoundationLibrary exposes an ECS-first scene stack plugin/API.
- Games can open a `.jsn` scene source, open overlays, close current scenes, close target scenes by id/key, clear the stack, and clear-and-open.
- Multiple scene commands can be issued in one frame and are processed deterministically.
- Lifecycle messages are emitted for added, removed, focused, and unfocused transitions.
- Scene presentation policy correctly drives visibility, input, and update eligibility for scenes below the top.
- Scene-owned entities are retained while their scene is stacked and cleaned up when the scene is removed.
- Unit tests cover core stack behavior and pass with workspace validation.

## Testing Methodology
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- `scripts/doc-project.cmd`
- `scripts/validate-project.cmd`
