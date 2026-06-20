# Scene Stack Example Tracker

## Metadata
- Feature slug: `scene-stack-example`
- Feature area: `multi-area`
- Primary area: `game`
- Branch: `feature/scene-stack-example`
- Overall status: `Planned`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for gpt-5.4 implementation after user approval`
- Created: `2026-06-20`
- Last updated: `2026-06-20`
- Branch creation: Created locally from `dev` on 2026-06-20.
- Push status: Planning branch is tracking `origin/feature/scene-stack-example`; planning and push-status update commits have been pushed.

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
**Status:** Planned  
**Goal:** Add reusable, reflected FoundationLibrary splash-screen configuration and timing/fade logic that can be driven by Jackdaw `.jsn` scenes.

### Tasks
- [ ] Add a Foundation splash module with reflected config components/resources for splash text, fade-in duration, hold duration, fade-out duration, and next-scene behavior.
  - Status: Planned
  - Notes: Defaults should match `1.5s / 2.0s / 1.5s`; values must be adjustable through serialized data or a documented config path.
- [ ] Add reusable systems for splash phase progression, alpha/fade updates, and final scene-stack command emission.
  - Status: Planned
  - Notes: Must avoid duplicating per-splash logic.
- [ ] Register and re-export public Foundation splash types from `FoundationPlugin` and `foundation_library::prelude`.
  - Status: Planned
  - Notes: Public types need Rustdoc comments.
- [ ] Add tests for timing phase behavior and next-scene command selection where practical.
  - Status: Planned
  - Notes: Prefer non-window tests.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

### Notes
- Reusable logic belongs in `crates/foundation-library`.
- Concrete scene files and sequence choices belong in `games/template-game`.

## Phase 2: TemplateGame Jackdaw Scene Stack Bridge And Startup
**Status:** Planned  
**Goal:** Make TemplateGame load Jackdaw `.jsn` scene sources through the Foundation scene stack instead of spawning the initial scene directly.

### Tasks
- [ ] Replace direct startup spawning of `scene.jsn` with opening the first splash scene through the scene stack.
  - Status: Planned
  - Notes: First scene should be a `SceneSource::JsnLevel` for the Pixel Perfect splash.
- [ ] Add a TemplateGame bridge that consumes `SceneLoadRequested` and spawns `JackdawSceneRoot(asset_server.load(path))` for `.jsn` scene sources.
  - Status: Planned
  - Notes: Spawned scene roots should be tagged with `SceneOwner` for cleanup.
- [ ] Verify scene stack replacement/clear behavior for splash transitions and final main menu transition.
  - Status: Planned
  - Notes: First splash should transition to second; final splash should clear/reset stack before main menu.
- [ ] Add tests for scene path constants, startup command behavior, and bridge behavior where practical.
  - Status: Planned
  - Notes: Tests should avoid opening a GPU window.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

### Notes
- The exact implementation should keep the Jackdaw editor launcher generic and avoid putting game-specific sequence logic in `crates/jackdaw-editor`.

## Phase 3: Concrete `.jsn` Splash And Main Menu Scenes
**Status:** Planned  
**Goal:** Add three TemplateGame Jackdaw scenes and verify the requested visible flow/timings.

### Tasks
- [ ] Add Pixel Perfect splash `.jsn` scene.
  - Status: Planned
  - Notes: Should display centered `Pixel Perfect` text and use default timing `1.5s fade-in / 2.0s hold / 1.5s fade-out` unless data overrides are documented.
- [ ] Add Bevy splash `.jsn` scene.
  - Status: Planned
  - Notes: Should display centered `Bevy` text and reuse the same Foundation splash logic.
- [ ] Add main menu `.jsn` scene.
  - Status: Planned
  - Notes: Minimal visible main menu is acceptable unless user expands scope to interactive buttons.
- [ ] Ensure final transition uses stack reset/clear-and-open so main menu is the only active stack entry.
  - Status: Planned
  - Notes: Record observed stack behavior in this tracker.
- [ ] Run/manual-check TemplateGame long enough to observe the splash-to-menu sequence if practical.
  - Status: Planned
  - Notes: If a window cannot be opened in the environment, record the blocker.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- Manual launch check: Pending
- User confirmation: Pending / Not required yet

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

## Implementation / Review Handoff Notes
- Implementation must use `gpt-5.4`; never use Anthropic models.
- Before implementation, read `.pi/skills/feature-tracker-update/SKILL.md`, this tracker, and `plan.md`.
- Confirm active branch is `feature/scene-stack-example` before implementation edits.
- Keep reusable splash behavior in FoundationLibrary and TemplateGame-specific scene assets/sequence implementation in `games/template-game`.
- Preserve Jackdaw `.jsn` as the data/source format for all three scenes. If direct Bevy text UI serialization in `.jsn` is brittle, use `.jsn` to hold reflected Foundation splash config and have reusable Foundation systems spawn the UI text at runtime; document that compromise here.
- Commit each completed task/phase and push to `origin` when available.

## Postponed Work
- Full interactive main menu navigation is postponed unless the user expands the example menu scope.
- Generic Foundation-owned Jackdaw `.jsn` load bridge is postponed unless implementation proves it should be reusable across games.
- Advanced transition effects beyond alpha fade are postponed.

## Open Issues / Questions
- Pending implementation verification: exact Jackdaw `.jsn` serialization shape for Bevy 0.18 UI/text or reflected Foundation splash config components.
- Pending decision: whether the first splash closes/replaces itself when opening the second splash. Proposed default is replacement/close-current behavior so old splash entities are cleaned up.

## Progress Log
- `2026-06-20`: User approved the feature summary and clarified all three scenes should be Jackdaw `.jsn` scenes if possible; reusable logic should live in FoundationLibrary while concrete scene implementation should live in TemplateGame.
- `2026-06-20`: Created planning branch `feature/scene-stack-example` from `dev`.
- `2026-06-20`: Plan and tracker created.
- `2026-06-20`: Planning commit `30c1b6b` pushed to `origin/feature/scene-stack-example`.
