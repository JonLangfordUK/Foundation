# Debug Console Tracker

## Metadata
- Feature slug: `debug-console`
- Feature area: `engine`
- Primary area: `engine`
- Branch: `feature/debug-console`
- Branch status: `Created from dev on 2026-07-15`
- Overall status: `Planned`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for user review before gpt-5.4 implementation`
- Created: `2026-07-15`
- Last updated: `2026-07-15`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- Every completed task and phase must be committed on `feature/debug-console` and pushed to `origin` when available.

## Phase 1: Console Architecture And Command Core
**Status:** Planned  
**Goal:** Establish the runtime module, command metadata model, command parser/dispatcher, and automatic linked-crate command registry.

### Tasks
- [ ] Add the console module and plugin skeleton to `foundation-runtime-library`.
  - Status: Planned
  - Notes: Install through `FoundationPlugin` after confirming scheduling requirements.
- [ ] Define command metadata, parameter metadata, command output/result, command context, history entry, and autocomplete candidate types.
  - Status: Planned
  - Notes: Public game-facing types require Rustdoc.
- [ ] Choose and integrate distributed registration dependency, preferring `linkme` if it supports the final descriptor design.
  - Status: Planned
  - Notes: Preserve the requirement that only linked Foundation/current-game crates contribute commands.
- [ ] Add the macro crate or macro module needed for ergonomic command declaration.
  - Status: Planned
  - Notes: Attribute macro likely needs a new proc-macro crate.
- [ ] Implement command parsing and dispatch for the initial named-parameter grammar.
  - Status: Planned
  - Notes: Avoid overbuilding shell-like parsing unless required.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending before implementation starts

### Notes
- Commands from other games should not be runtime-filtered; they should be absent because those game crates are not compiled/linked into the running game binary.

## Phase 2: Feathers Console UI And Input Focus
**Status:** Planned  
**Goal:** Provide a UE5-style bottom console overlay using Bevy Feathers with robust keyboard and mouse focus while open.

### Tasks
- [ ] Verify Bevy 0.19 Feathers imports, required features, and text input APIs.
  - Status: Planned
  - Notes: Research indicates Feathers has text input widgets and Bevy 0.19 includes `EditableText`.
- [ ] Implement backtick toggle behavior for opening/closing the console.
  - Status: Planned
  - Notes: Ensure backtick while focused in console does not produce unwanted text.
- [ ] Spawn/despawn or show/hide a full-width bottom console overlay with history/output above the input row.
  - Status: Planned
  - Notes: Use a high UI order/layer suitable for in-game overlays.
- [ ] Capture mouse and keyboard focus while the console is open.
  - Status: Planned
  - Notes: Gameplay input leakage is a key review risk.
- [ ] Implement Enter execution, Escape/backtick close behavior, Up/Down history navigation, and Tab autocomplete completion.
  - Status: Planned
  - Notes: Coordinate Tab with Bevy UI tab navigation.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

### Notes
- Manual runtime testing will be necessary because UI focus behavior is hard to prove with unit tests alone.

## Phase 3: Autocomplete, Placeholder Text, And Example Commands
**Status:** Planned  
**Goal:** Complete the user-facing console behavior with predictions, parameter hints, and Foundation/TemplateGame command examples.

### Tasks
- [ ] Implement deterministic prefix-based command and parameter autocomplete.
  - Status: Planned
  - Notes: Stable ordering should make tests reliable.
- [ ] Generate placeholder/dummy text from named command parameter metadata.
  - Status: Planned
  - Notes: Placeholders should update as the active command/prediction changes.
- [ ] Add at least one Foundation-authored command and one TemplateGame-authored command using the macro.
  - Status: Planned
  - Notes: Examples should prove Foundation plus current game command availability.
- [ ] Add tests for autocomplete, placeholder generation, and example command registration.
  - Status: Planned
  - Notes: Tests should avoid depending on nondeterministic linker ordering.
- [ ] Add user-facing docs or examples for declaring commands.
  - Status: Planned
  - Notes: Include supported signatures and limitations.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

### Notes
- TemplateGame commands are compiled when running TemplateGame. Other game crates should not be included unless their package is selected/linked.

## Phase 4: Full Validation, Commit Checkpoints, And Handoff
**Status:** Planned  
**Goal:** Complete validation, documentation generation, commit/push checkpoints, and prepare optional review.

### Tasks
- [ ] Run `scripts/format-project.cmd`.
  - Status: Planned
  - Notes: Required before completion.
- [ ] Run `scripts/lint-project.cmd`.
  - Status: Planned
  - Notes: Required before completion.
- [ ] Run `scripts/test-project.cmd`.
  - Status: Planned
  - Notes: Required before completion.
- [ ] Run `scripts/compile-project.cmd`.
  - Status: Planned
  - Notes: Required before completion.
- [ ] Run `scripts/doc-project.cmd`.
  - Status: Planned
  - Notes: Required before completion because public APIs are expected.
- [ ] Run `scripts/validate-project.cmd` for full validation.
  - Status: Planned
  - Notes: Preferred final validation wrapper.
- [ ] Commit completed tasks/phases with required file lists and push to `origin`.
  - Status: Planned
  - Notes: `origin` is configured as `https://github.com/JonLangfordUK/Foundation.git`.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending / Not required yet

### Notes
- Do not mark the feature complete until validation and documentation generation pass or a user-approved waiver is recorded.

## Implementation / Review Handoff Notes
- Planning is complete and waiting for user approval to begin implementation.
- Implementation must use `gpt-5.4`; review must use `gpt-5.5`; never use Anthropic models.
- Before implementation edits, read the plan and tracker, confirm branch `feature/debug-console`, and update this tracker to record implementation start/resume.
- Required skills before implementation: `feature-tracker-update`, `feature-plan-docs`, `foundation-architecture`, `rust-workspace-dev`, `rust-coding-standards`, and `gitflow-workflow`.
- Preserve the corrected requirement: commands from other games are unavailable because those game crates are not compiled into the running game binary, not because all games' commands are loaded then filtered.

## Postponed Work
- None.

## Issues And Open Questions
- Answered: Opening the console should not pause gameplay. The debug console should be treated as a Foundation scene-stack scene.
- Answered: The first macro version should aim for full Bevy-system-style command functions, including access to resources, `World`, and entity queries where appropriate.
- Answered: Macro-declared command functions should separate Bevy-filled parameters from user-provided command inputs by using a named input struct wrapped in a dedicated input parameter such as `ConsoleInputs<T>`.
- Answered: Console command history should persist to disk under `saved/console/`. Up/Down should cycle through history, and pressing Down after the newest history entry should clear the input.
- Answered: Command names should default to the Rust function name, with an option to override the command name in the macro.

## Progress Log
- `2026-07-15`: User approved the feature summary and clarified that other games' commands are unavailable because those games are not compiled into the running binary.
- `2026-07-15`: Read required planning, gitflow, Foundation architecture, Rust workspace, and Rust coding standards skills.
- `2026-07-15`: Inspected workspace manifests, Foundation launcher, Foundation runtime plugin, and TemplateGame integration.
- `2026-07-15`: Created branch `feature/debug-console` from `dev`.
- `2026-07-15`: Created `plan.md` and `tracker.md` for user review.
- `2026-07-15`: User clarified that the console should not pause gameplay and should be represented as a Foundation scene-stack scene.
- `2026-07-15`: User preferred full Bevy-system-style command functions and raised the need to distinguish Bevy-filled parameters from user-provided command inputs.
- `2026-07-15`: User approved named input structs for command inputs and requested persisted command history under `saved/console/` with Up/Down navigation and clear-input behavior after the newest entry.
- `2026-07-15`: User requested command names default to the Rust function name, with an optional macro override.
