# Foundation Scene Stack Tracker

## Metadata
- Feature slug: `foundation-scene-stack`
- Feature area: `multi-area`
- Primary area: `engine`
- Foundation area: `Foundation Library`
- Branch: `feature/foundation-scene-stack`
- Overall status: `Planned`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for user review before gpt-5.4 implementation`
- Created: `2026-06-20`
- Last updated: `2026-06-20`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- Use `scripts/format-project.cmd`, `scripts/lint-project.cmd`, `scripts/test-project.cmd`, `scripts/compile-project.cmd`, `scripts/doc-project.cmd`, and `scripts/validate-project.cmd` unless a documented waiver exists.

## Phase 1: ECS scene stack API foundation
**Status:** Planned  
**Goal:** Add core FoundationLibrary scene stack types, resources, messages, plugin registration, prelude exports, and unit tests for command API shape.

### Tasks
- [ ] Add a scene stack module under `crates/foundation-library/src/` and register it from `FoundationPlugin`.
  - Status: Planned
  - Notes: Must preserve existing `FoundationSettings` and `FoundationActor` behavior.
- [ ] Define public core types with Rustdoc: scene id, optional key/name, scene source, scene presentation, stack entry, stack resource, and open options.
  - Status: Planned
  - Notes: Include `.jsn` level source support without creating a competing level format.
- [ ] Define buffered scene command messages and ergonomic game-facing helpers for open, close current, close target, clear, and clear-and-open.
  - Status: Planned
  - Notes: Multiple commands per frame are required.
- [ ] Add unit tests for public defaults and command construction/helpers.
  - Status: Planned
  - Notes: Prefer tests inside `crates/foundation-library/src/lib.rs` or module-specific tests.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending before implementation starts

### Notes
- Architecture is ECS-first by user preference; do not introduce trait-object scene controllers.

## Phase 2: Command processing, focus, and lifecycle messages
**Status:** Planned  
**Goal:** Implement deterministic stack mutation from queued commands, lifecycle message emission, and derived runtime flags.

### Tasks
- [ ] Implement ordered scene command processing at a safe Bevy schedule point.
  - Status: Planned
  - Notes: Capture old focus, process all commands in queue order, then recalculate derived state.
- [ ] Emit lifecycle messages for `SceneAdded`, `SceneRemoved`, `SceneFocused`, and `SceneUnfocused`.
  - Status: Planned
  - Notes: Use Bevy 0.18 `Message` APIs where buffered predictable processing is appropriate.
- [ ] Implement focus restoration and runtime flag derivation for visible, interactive, and updating scenes using `ScenePresentation`.
  - Status: Planned
  - Notes: Stack and presentation policy remain the source of truth.
- [ ] Add tests for push, close current, close target, clear, clear-and-open, buried-scene removal, and focus/visibility/input/update behavior.
  - Status: Planned
  - Notes: Include multi-command same-frame scenarios.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

### Notes
- Lifecycle ordering must be deterministic and documented.

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
