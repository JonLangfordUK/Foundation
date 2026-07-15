# Debug Console Tracker

## Metadata
- Feature slug: `debug-console`
- Feature area: `engine`
- Primary area: `engine`
- Branch: `feature/debug-console`
- Branch status: `Created from dev on 2026-07-15`
- Overall status: `In progress`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation in progress with gpt-5.4`
- Created: `2026-07-15`
- Last updated: `2026-07-15`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- Every completed task and phase must be committed on `feature/debug-console` and pushed to `origin` when available.

## Phase 1: Console Architecture And Command Core
**Status:** Complete  
**Goal:** Establish the runtime module, command metadata model, command parser/dispatcher, and automatic linked-crate command registry.

### Tasks
- [x] Add the console module and plugin skeleton to `foundation-runtime-library`.
  - Status: Complete
  - Notes: Added `console` module, `FoundationConsolePlugin`, console resources, and installation through `FoundationPlugin`.
- [x] Define command metadata, parameter metadata, command output/result, command context, history entry, and autocomplete candidate types.
  - Status: Complete
  - Notes: Added command descriptors, parameter metadata, result/error types, `ConsoleInputs<T>`, persisted history resource, registry resource, and autocomplete candidate type with Rustdoc.
- [x] Choose and integrate distributed registration dependency, preferring `linkme` if it supports the final descriptor design.
  - Status: Complete
  - Notes: Integrated `linkme` distributed slices. Only crates linked into the running binary contribute descriptors. Command-author crates currently need `linkme` available as a direct dependency because the `linkme` attribute macro expands with that crate name.
- [x] Add the macro crate or macro module needed for ergonomic command declaration.
  - Status: Complete
  - Notes: Added `foundation-console-macros` with `#[console_command]` and `#[derive(ConsoleCommandInput)]`. Command names default to the function name with optional override support.
- [x] Implement command parsing and dispatch for the initial named-parameter grammar.
  - Status: Complete
  - Notes: Implemented whitespace-separated `command name=value` parsing and dispatch through Bevy one-shot systems using `RunSystemOnce`.

### Validation
- Format: Passed via `scripts/format-project.cmd` on 2026-07-15
- Lint: Passed via `scripts/lint-project.cmd` on 2026-07-15
- Tests: Passed via `scripts/test-project.cmd` on 2026-07-15
- Build: Passed via `scripts/compile-project.cmd` on 2026-07-15
- Documentation generation: Passed via `scripts/doc-project.cmd` on 2026-07-15
- Full validation wrapper: Pending / Not required yet
- User confirmation: User approved implementation start on 2026-07-15

### Notes
- Commands from other games should not be runtime-filtered; they should be absent because those game crates are not compiled/linked into the running game binary.
- TemplateGame now has a macro-registered command and a test proving it is linked into the TemplateGame binary's registry.

## Phase 2: Feathers Console UI And Input Focus
**Status:** In progress  
**Goal:** Provide a UE5-style bottom console overlay using Bevy Feathers with robust keyboard and mouse focus while open.

### Tasks
- [ ] Verify Bevy 0.19 Feathers imports, required features, and text input APIs.
  - Status: Implemented; awaiting phase validation
  - Notes: Local Bevy 0.19 source confirms `bevy_feathers::FeathersPlugins`, `FeathersTextInputContainer`, and `FeathersTextInput` exist. Added direct `bevy_feathers` and `bevy_input_focus` dependencies for the runtime console UI.
- [ ] Implement backtick toggle behavior for opening/closing the console.
  - Status: Implemented; awaiting phase validation
  - Notes: Backquote now opens the console as a scene-stack runtime scene with `INPUT_BLOCKING_OVERLAY`, so gameplay continues updating while lower-scene input is blocked. Backquote closes the keyed console scene while open.
- [ ] Spawn/despawn or show/hide a full-width bottom console overlay with history/output above the input row.
  - Status: Implemented; awaiting phase validation
  - Notes: Added scene-load handling that spawns a full-width bottom overlay with high `GlobalZIndex`, history/output text, Feathers text input markers, and scene ownership for cleanup.
- [ ] Capture mouse and keyboard focus while the console is open.
  - Status: In progress
  - Notes: The generated input entity uses Bevy `EditableText`, `TabIndex`, `AutoFocus`, and explicit `InputFocus` assignment. More validation is still needed around gameplay input leakage and text input behavior.
- [ ] Implement Enter execution, Escape/backtick close behavior, Up/Down history navigation, and Tab autocomplete completion.
  - Status: Planned
  - Notes: Coordinate Tab with Bevy UI tab navigation.

### Validation
- Format: Passed focused check via `cargo fmt --all` and `scripts/format-project.cmd` on 2026-07-15
- Lint: Passed focused check via `cargo clippy -p foundation-runtime-library --all-targets --all-features -- -D warnings` on 2026-07-15
- Tests: Passed focused check via `cargo test -p foundation-runtime-library console --all-features` on 2026-07-15; rerun after reopen fix with 8 console tests passing; rerun after input/action fix with 8 console tests passing
- Build: Pending phase-level build
- Documentation generation: Pending phase-level documentation generation
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending / Not required yet

### Notes
- Manual runtime testing will be necessary because UI focus behavior is hard to prove with unit tests alone.
- User reported missing predictive text, Enter doing nothing, TemplateGame warning for `foundation/debug-console`, and Bevy scene-component errors from direct Feathers scene-component spawning.
- Addressed by adding focused keyboard action handling, suggestion text refresh, queued command execution, Escape close, Up/Down history navigation, Tab completion, ignoring Foundation runtime scenes in TemplateGame scene loading, and removing direct spawning of Feathers scene components that Bevy expects to be spawned through scene syntax.
- Remaining Phase 2 work: stronger runtime/focus validation and persisted history disk I/O.

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
- TemplateGame includes `say_hello` as an example command registered in-console as `example.say-hello`.

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
- User approved implementation after plan review. Implementation has started with `gpt-5.4`.
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
- `2026-07-15`: Committed and pushed planning documents in commit `4857290` on `feature/debug-console`.
- `2026-07-15`: Verified active branch `feature/debug-console`; `dev` is an ancestor of `HEAD` before implementation edits. Started implementation with `gpt-5.4`.
- `2026-07-15`: Implemented Phase 1 command core: console runtime module, plugin/resource registration, linked command registry, procedural macros, built-in Foundation command, TemplateGame example command, parser/dispatch tests, and linked-game registry test.
- `2026-07-15`: Phase 1 validation passed: `scripts/format-project.cmd`, `scripts/lint-project.cmd`, `scripts/test-project.cmd`, `scripts/compile-project.cmd`, and `scripts/doc-project.cmd`.
- `2026-07-15`: Phase 1 committed and pushed as `68e1218`.
- `2026-07-15`: Started Phase 2 Feathers console UI and scene-stack integration.
- `2026-07-15`: Implemented initial Phase 2 UI skeleton: Feathers dependencies/plugins, backquote scene-stack open/close, input-blocking non-pausing presentation, scene-load UI spawning, high-Z bottom overlay, `EditableText` input, and focus assignment.
- `2026-07-15`: User found the console could open once, close once, then fail to reopen. Fixed stale console-open state by storing the active console `SceneId` in `FoundationConsoleState` and clearing it directly from `SceneRemoved`, instead of relying on querying UI roots that may already have been despawned.
- `2026-07-15`: User reported missing predictive text, Enter doing nothing, and runtime errors/warnings. Added suggestion text, Tab completion, Enter command execution, Escape close, Up/Down history navigation, removed direct Feathers scene-component spawning that caused Bevy errors, and made TemplateGame ignore Foundation runtime scene requests.
- `2026-07-15`: Added TemplateGame `say_hello` example command registered as `example.say-hello`, demonstrating macro command-name override with named `name` input metadata.
