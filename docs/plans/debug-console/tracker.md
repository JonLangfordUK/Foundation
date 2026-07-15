# Debug Console Tracker

## Metadata
- Feature slug: `debug-console`
- Feature area: `engine`
- Primary area: `engine`
- Branch: `feature/debug-console`
- Branch status: `Created from dev on 2026-07-15`
- Overall status: `Implementation complete; optional review available`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for optional gpt-5.5 sanity review`
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
**Status:** Complete  
**Goal:** Provide a UE5-style bottom console overlay using Bevy Feathers with robust keyboard and mouse focus while open.

### Tasks
- [x] Verify Bevy 0.19 Feathers imports, required features, and text input APIs.
  - Status: Complete
  - Notes: Local Bevy 0.19 source confirmed `bevy_feathers::FeathersPlugins`, `FeathersTextInputContainer`, and `FeathersTextInput` exist. Added direct `bevy_feathers` and `bevy_input_focus` dependencies for the runtime console UI.
- [x] Implement backtick toggle behavior for opening/closing the console.
  - Status: Complete
  - Notes: Backquote opens the console as a scene-stack runtime scene with `INPUT_BLOCKING_OVERLAY`, so gameplay continues updating while lower-scene input is blocked. Backquote closes the keyed console scene while open.
- [x] Spawn/despawn or show/hide a full-width bottom console overlay with history/output above the input row.
  - Status: Complete
  - Notes: Added scene-load handling that spawns a full-width bottom overlay with high `GlobalZIndex`, scrollback output text, suggestion text, input text, and scene ownership for cleanup.
- [x] Capture mouse and keyboard focus while the console is open.
  - Status: Complete
  - Notes: The generated input entity uses Bevy `EditableText`, `TabIndex`, `AutoFocus`, explicit `InputFocus` assignment, and runtime user testing confirmed open/close, typing, Tab, Enter, history, and scrollback behavior.
- [x] Implement Enter execution, Escape/backtick close behavior, Up/Down history navigation, and Tab autocomplete completion.
  - Status: Complete
  - Notes: Implemented focused keyboard action handling for Enter, Escape, Backquote, Up/Down history traversal, Tab completion, output scrolling, and persisted command history.

### Validation
- Format: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Lint: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Tests: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Build: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-07-15
- User confirmation: User confirmed console behavior is working perfectly on 2026-07-15

### Notes
- Manual runtime testing was performed by the user throughout Phase 2 and drove fixes for reopen state, input actions, Bevy query conflicts, Tab completion crash, output ordering, scroll containment, scrollback range, history navigation, output area size, and executable-relative persisted history.
- TemplateGame ignores Foundation runtime scene requests so the game scene catalog no longer warns about `foundation/debug-console`.
- Feathers plugin installation is guarded so MinimalPlugins tests that lack asset resources do not panic while normal DefaultPlugins game launches still receive Feathers support.

## Phase 3: Autocomplete, Placeholder Text, And Example Commands
**Status:** Complete  
**Goal:** Complete the user-facing console behavior with predictions, parameter hints, and Foundation/TemplateGame command examples.

### Tasks
- [x] Implement deterministic prefix-based command and parameter autocomplete.
  - Status: Complete
  - Notes: Command-name autocomplete uses deterministic sorted registry order and Tab completion inserts the command with generated parameter placeholders.
- [x] Generate placeholder/dummy text from named command parameter metadata.
  - Status: Complete
  - Notes: `ConsoleCommandInput` derives named field metadata and placeholders render as `name=<Type>` style parameter hints.
- [x] Add at least one Foundation-authored command and one TemplateGame-authored command using the macro.
  - Status: Complete
  - Notes: Added `foundation_console_history_size`; TemplateGame includes `template_game_greeting` and `say_hello` registered as `example.say-hello`.
- [x] Add tests for autocomplete, placeholder generation, and example command registration.
  - Status: Complete
  - Notes: Console tests cover parser, metadata, command dispatch, scrollback, history persistence, and history navigation. TemplateGame tests verify command registration and overridden command name metadata.
- [x] Add user-facing docs or examples for declaring commands.
  - Status: Complete
  - Notes: Added `docs/debug-console.md` and linked it from `docs/foundation-engine.md`.

### Validation
- Format: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Lint: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Tests: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Build: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-07-15
- User confirmation: User confirmed command UX is working on 2026-07-15

### Notes
- TemplateGame commands are compiled when running TemplateGame. Other game crates should not be included unless their package is selected/linked.
- TemplateGame includes `say_hello` as an example command registered in-console as `example.say-hello`.
- Command history persists under `<executable-dir>/saved/console/history.json`.

## Phase 4: Full Validation, Commit Checkpoints, And Handoff
**Status:** Complete  
**Goal:** Complete validation, documentation generation, commit/push checkpoints, and prepare optional review.

### Tasks
- [x] Run `scripts/format-project.cmd`.
  - Status: Complete
  - Notes: Passed as part of `scripts/validate-project.cmd` on 2026-07-15.
- [x] Run `scripts/lint-project.cmd`.
  - Status: Complete
  - Notes: Passed as part of `scripts/validate-project.cmd` on 2026-07-15.
- [x] Run `scripts/test-project.cmd`.
  - Status: Complete
  - Notes: Passed as part of `scripts/validate-project.cmd` on 2026-07-15.
- [x] Run `scripts/compile-project.cmd`.
  - Status: Complete
  - Notes: Passed as part of `scripts/validate-project.cmd` on 2026-07-15.
- [x] Run `scripts/doc-project.cmd`.
  - Status: Complete
  - Notes: Passed as part of `scripts/validate-project.cmd` on 2026-07-15.
- [x] Run `scripts/validate-project.cmd` for full validation.
  - Status: Complete
  - Notes: Passed on 2026-07-15 after fixing Feathers plugin initialization for MinimalPlugins tests.
- [x] Commit completed tasks/phases with required file lists and push to `origin`.
  - Status: Complete
  - Notes: Final documentation/tracker update is included in the feature finish-up commit and pushed to `origin`. `origin` is configured as `https://github.com/JonLangfordUK/Foundation.git`.

### Validation
- Format: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Lint: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Tests: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Build: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-07-15
- User confirmation: User requested finish-up on 2026-07-15

### Notes
- Full validation initially failed because `FoundationConsolePlugin` always installed `FeathersPlugins`, and MinimalPlugins tests lacked required asset resources. The console plugin now installs Feathers only when the app already has `AssetServer`, allowing lightweight tests to pass while DefaultPlugins game launches keep Feathers support.
- Optional final sanity review remains available through the project review workflow.

## Implementation / Review Handoff Notes
- User approved implementation after plan review. Implementation is complete with `gpt-5.4`; optional final sanity review can use `gpt-5.5`.
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
- Answered: Console command history persists to disk under `<executable-dir>/saved/console/history.json`. Up/Down cycles through history, and pressing Down after the newest history entry clears the input.
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
- `2026-07-15`: User reported a Bevy B0001 panic after the console input/action change. Fixed conflicting mutable `Text` queries in the console text refresh system by using disjoint `Without` filters for output and suggestion text queries.
- `2026-07-15`: User reported a Tab crash with a Bevy tab-navigation warning and a Parley char-boundary assertion. Added a `TabGroup` to the console root and changed Tab/autocomplete replacement to set the `EditableText` buffer directly before moving the cursor to the end, instead of queueing an insert after clearing.
- `2026-07-15`: User reported output/history ordering looked wrong after running a successful command followed by a failing command. Fixed the output panel so it displays the execution log only, without appending command-navigation history entries again.
- `2026-07-15`: User reported long console output/history text overflowed past the output box and overlapped the input. Constrained the output area to a fixed scrollable region and added mouse-wheel scrolling while the console is open.
- `2026-07-15`: User clarified output should grow upward, with the newest command just above the input box. Split the output area into a scrollable viewport and a child text node, bottom-aligning the output text inside the viewport while preserving mouse-wheel scrolling.
- `2026-07-15`: User reported scrolling only reached the newest few commands and blank space instead of older output. Changed output rendering to a line-window model with an explicit scrollback offset, so the newest lines show by default and mouse-wheel up pages through older output lines.
- `2026-07-15`: User reported Up/Down history navigation only cycled the most recent command. Fixed `EditableText` change synchronization so programmatic history replacements do not immediately clear the history cursor, and added a test covering full history traversal and clear-input behavior.
- `2026-07-15`: User confirmed history navigation works and requested a larger history/output box that fills more of the console vertically. Increased console height, output viewport height, and visible output line count.
- `2026-07-15`: User confirmed the larger history area looks good and requested two additional visible output lines. Increased visible output line count from 14 to 16.
- `2026-07-15`: User reported command history was not available between game runs. Implemented loading history from `saved/console/history.json` when the console plugin starts and saving history after each submitted command, including failed command attempts, so Up/Down navigation persists across runs.
- `2026-07-15`: User clarified the saved history path should be relative to the built executable, not the project/current working directory. Updated history path resolution to use `std::env::current_exe()` and store under `<executable-dir>/saved/console/history.json`.
- `2026-07-15`: Added user-facing documentation in `docs/debug-console.md` and linked it from `docs/foundation-engine.md`.
- `2026-07-15`: Ran `scripts/validate-project.cmd`; initial run failed because Feathers plugin installation required asset resources in MinimalPlugins tests. Guarded Feathers plugin installation behind existing `AssetServer` availability.
- `2026-07-15`: Re-ran `scripts/validate-project.cmd`; full validation passed including format, lint, tests, build, and docs.
- `2026-07-15`: Added TemplateGame `say_hello` example command registered as `example.say-hello`, demonstrating macro command-name override with named `name` input metadata.
