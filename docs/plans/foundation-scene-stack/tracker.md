# Foundation Scene Stack Tracker

## Metadata
- Feature slug: `foundation-scene-stack`
- Feature area: `multi-area`
- Primary area: `engine`
- Foundation area: `Foundation Library`
- Branch: `feature/foundation-scene-stack`
- Overall status: `In progress`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation in progress with gpt-5.4`
- Created: `2026-06-20`
- Last updated: `2026-06-20`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- Use `scripts/format-project.cmd`, `scripts/lint-project.cmd`, `scripts/test-project.cmd`, `scripts/compile-project.cmd`, `scripts/doc-project.cmd`, and `scripts/validate-project.cmd` unless a documented waiver exists.

## Phase 1: ECS scene stack API foundation
**Status:** Complete  
**Goal:** Add core FoundationLibrary scene stack types, resources, messages, plugin registration, prelude exports, and unit tests for command API shape.

### Tasks
- [x] Add a scene stack module under `crates/foundation-library/src/` and register it from `FoundationPlugin`.
  - Status: Complete
  - Notes: Added `scene_stack` module and `FoundationSceneStackPlugin`; `FoundationPlugin` now installs it while preserving existing `FoundationSettings` and `FoundationActor` behavior.
- [x] Define public core types with Rustdoc: scene id, optional key/name, scene source, scene presentation, stack entry, stack resource, and open options.
  - Status: Complete
  - Notes: Added `.jsn` level source support through `SceneSource::JsnLevel` without creating a competing level format.
- [x] Define buffered scene command messages and ergonomic game-facing helpers for open, close current, close target, clear, and clear-and-open.
  - Status: Complete
  - Notes: Added `SceneCommand` message and `SceneCommandsExt` for queuing multiple commands through Bevy `Commands`.
- [x] Add unit tests for public defaults and command construction/helpers.
  - Status: Complete
  - Notes: Added module tests for presentation defaults, open options, scene sources, command helpers, and empty stack defaults.

### Validation
- Format: Passed (`scripts/format-project.cmd`, 2026-06-20)
- Lint: Passed (`scripts/lint-project.cmd`, 2026-06-20; fixed initial `clippy::derivable_impls` warning for `OpenSceneOptions`)
- Tests: Passed (`scripts/test-project.cmd`, 2026-06-20; 6 FoundationLibrary tests passed plus workspace tests)
- Build: Passed (`scripts/compile-project.cmd`, 2026-06-20)
- Documentation generation: Passed (`scripts/doc-project.cmd`, 2026-06-20; generated FoundationLibrary docs)
- Full validation wrapper: Pending / Not required yet
- User confirmation: Received (`I'm happy. Commit, and then start work`)

### Notes
- Architecture is ECS-first by user preference; no trait-object scene controllers were introduced.
- Phase 1 implementation is ready to commit and push.

## Phase 2: Command processing, focus, and lifecycle messages
**Status:** Complete  
**Goal:** Implement deterministic stack mutation from queued commands, lifecycle message emission, and derived runtime flags.

### Tasks
- [x] Implement ordered scene command processing at a safe Bevy schedule point.
  - Status: Complete
  - Notes: `process_scene_commands` runs in `PostUpdate`, processes queued `SceneCommand` messages in read order, and recalculates derived state after each command.
- [x] Emit lifecycle messages for `SceneAdded`, `SceneRemoved`, `SceneFocused`, and `SceneUnfocused`.
  - Status: Complete
  - Notes: Uses Bevy 0.18 `Message` APIs for buffered predictable processing.
- [x] Implement focus restoration and runtime flag derivation for visible, interactive, and updating scenes using `ScenePresentation`.
  - Status: Complete
  - Notes: Stack and presentation policy remain the source of truth; focus is assigned to the topmost interactive scene after runtime flags are derived.
- [x] Add tests for push, close current, close target, clear, clear-and-open, buried-scene removal, and focus/visibility/input/update behavior.
  - Status: Complete
  - Notes: Added tests for open processing, pause overlay flag derivation, close current focus restoration, close by key, clear-and-open, and lifecycle messages.

### Validation
- Format: Passed (`scripts/format-project.cmd`, 2026-06-20; applied `cargo fmt --all` first)
- Lint: Passed (`scripts/lint-project.cmd`, 2026-06-20; fixed initial bool assert comparison warnings)
- Tests: Passed (`scripts/test-project.cmd`, 2026-06-20; 12 FoundationLibrary tests passed plus workspace tests)
- Build: Passed (`scripts/compile-project.cmd`, 2026-06-20)
- Documentation generation: Passed (`scripts/doc-project.cmd`, 2026-06-20; generated FoundationLibrary docs)
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

### Notes
- Lifecycle message types are separate Bevy message streams; systems that need one combined global ordering should consume stack state changes via a dedicated future aggregate message if required.
- Phase 2 implementation is ready to commit and push.

## Phase 3: Scene ownership cleanup and Jackdaw `.jsn` integration bridge
**Status:** Planned  
**Goal:** Ensure scene memory lifetime semantics are enforced and Jackdaw `.jsn` scenes have a clear ECS integration path.

### Tasks
- [ ] Add `SceneOwner` component and cleanup system for removed scenes.
  - Status: Planned
  - Notes: Removing a scene must clear scene-owned entities from memory; stacked scenes remain resident.
- [ ] Add tests proving cleanup only affects entities tagged with the removed scene id.
  - Status: Planned
  - Notes: Non-scene-owned entities must survive cleanup.
- [ ] Define the `.jsn` level scene source bridge and document how Jackdaw runtime loading systems should consume scene stack lifecycle/source data.
  - Status: Planned
  - Notes: Verify exact Jackdaw runtime API during implementation before deciding how much loading FoundationLibrary should perform directly.
- [ ] Add or update example/template usage only if it clarifies the public API without coupling TemplateGame to test-only behavior.
  - Status: Planned
  - Notes: Keep FoundationLibrary reusable.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

### Notes
- Ownership tagging is the key memory-lifetime rule.

## Phase 4: Documentation, full validation, and handoff
**Status:** Planned  
**Goal:** Finalize docs, run full validation, update tracker, and prepare for optional sanity review.

### Tasks
- [ ] Complete Rustdoc for all public scene stack APIs.
  - Status: Planned
  - Notes: Include schedule/lifecycle caveats.
- [ ] Add feature-level documentation if Rustdoc alone is insufficient.
  - Status: Planned
  - Notes: Document command ordering, presentation policy, focus model, cleanup, and `.jsn` bridge.
- [ ] Run full project validation and record results.
  - Status: Planned
  - Notes: Use `scripts/validate-project.cmd` plus individual command results if needed.
- [ ] Commit completed phase work and record push status.
  - Status: Planned
  - Notes: Repository currently has an `origin`; push will be required after commits unless remote state changes.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending

### Notes
- Final optional review should use `gpt-5.5` and record findings here.

## Implementation / Review Handoff Notes
- Implementation model: `gpt-5.4`.
- Review model: `gpt-5.5`.
- Never use Anthropic models.
- User approved ECS-first direction and rejected hybrid/trait-object scene architecture.
- Scene stack presence controls memory lifetime: on-stack scenes stay resident; removed scenes are cleaned from memory.
- Multiple scene commands per frame are required.
- The feature must work with Jackdaw scene concepts and `.jsn` level files.

## Postponed Work
- None yet.

## Progress Log
- `2026-06-20`: Discussed and selected ECS-first scene stack architecture for FoundationLibrary.
- `2026-06-20`: Created `feature/foundation-scene-stack` from `dev` for feature planning.
- `2026-06-20`: Plan and tracker created for user review.
- `2026-06-20`: Planning commit `8e5b672` created and pushed to `origin/feature/foundation-scene-stack`.
- `2026-06-20`: User approved implementation start; branch matches tracker and `dev` is an ancestor of `HEAD`.
- `2026-06-20`: Completed Phase 1 ECS scene stack API foundation and recorded validation results.
- `2026-06-20`: Phase 1 commit `d73c8f9` pushed to `origin/feature/foundation-scene-stack`; started Phase 2 command processing work.
- `2026-06-20`: Completed Phase 2 command processing, lifecycle messages, focus restoration, runtime flags, and validation.
