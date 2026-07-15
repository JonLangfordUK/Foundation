# Debug Console Plan

## Metadata
- Feature slug: `debug-console`
- Feature area: `engine`
- Primary area: `engine`
- Branch: `feature/debug-console`
- Branch status: `Created from dev on 2026-07-15`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-07-15`
- Last updated: `2026-07-15`

## User Request
Add a UE5-style debug console to the Foundation engine so every game using Foundation can press the backtick key `` ` `` during play to open a full-width console at the bottom of the screen. The console should show command history/output above an editable text box, give the user mouse and keyboard control while open, support autocomplete predictions, complete the current prediction with Tab, and show named command parameters as placeholder/dummy text so users know which properties are expected.

Console commands should be declared by placing a macro on a function in Foundation APIs or in the currently compiled game. Users should not manually add those functions to a Bevy system. Registration should be automatic behind the scenes. Commands from other games are not merely filtered out at runtime; those other game crates are not compiled into the selected game binary, so their commands are unavailable by construction. The UI should use Bevy Feathers.

## Feature Summary
The feature adds a reusable Foundation runtime debug-console subsystem: a toggleable in-game console UI, command registry, command invocation pipeline, history/output storage, autocomplete, and a macro-based command declaration API. The console should be treated as a Foundation scene-stack scene, but opening it should not pause gameplay. The runtime plugin will be available to all games through `FoundationPlugin`, while command definitions are collected only from crates linked into the running game binary.

## Feature Area Classification
- Area: `engine`
- Primary area: `engine`
- Rationale: The console is a Foundation runtime feature shared by all games, with optional game-authored command providers. It belongs in `foundation-runtime-library`, not in a concrete game crate. A companion macro crate may be needed to support ergonomic command declarations.

## Codebase Research
- Root `Cargo.toml` defines a workspace with `crates/foundation`, `crates/foundation-runtime-library`, `crates/foundation-editor-library`, and `games/template-game`.
- `crates/foundation` is a launcher only. It discovers game manifests and runs the selected game package with `cargo run -p <selected-package> --`; it intentionally does not depend on concrete game crates.
- `foundation-runtime-library` contains reusable systems and is already installed into games through `FoundationPlugin`.
- `games/template-game` calls `DefaultPlugins`, then `FoundationPlugin`, then `TemplateGamePlugin`; this is the correct integration point for a global console plugin.
- Current Foundation runtime modules include scene stack, splash screen, menu, credits, and game settings. A new `console` module can follow this pattern and be re-exported from the prelude.
- Current workspace has no procedural macro crate and no distributed registration dependency. Adding macro-based registration will require new dependency decisions and likely a new crate.
- The launcher currently compiles only the selected game package. This supports the requirement that commands in other game crates are not compiled into the active game binary.

## External Research
- Bevy 0.19 introduced upstream text entry through `EditableText`, including keyboard text input, cursor navigation, selection, backspace/delete, pointer placement, and drag selection. Source: Bevy 0.19 release notes and Bevy text input release content.
- Bevy Feathers in Bevy 0.19 includes styled editor-oriented widgets including text input, number input, dropdown/menu widgets, panes, groups, and list views. Source: Bevy 0.19 announcement and `bevy_feathers` docs/source.
- `linkme` provides distributed slices where elements can be defined anywhere in the final binary dependency graph and gathered by the linker. It does not use life-before-main runtime initialization, and only linked crates contribute elements. This matches the selected-game-only command requirement. Source: `linkme` docs and repository documentation.
- `inventory` also supports typed distributed registration from any linked source file, but it uses runtime initialization before main. It remains an alternative if `linkme` cannot satisfy metadata or platform requirements.

## Affected Files And Systems
- `Cargo.toml`: Add workspace dependencies and possibly a new workspace macro crate member.
- `crates/foundation-runtime-library/Cargo.toml`: Add Bevy feature/dependency support for Feathers/text input if not already exposed through the current `bevy` dependency.
- `crates/foundation-runtime-library/src/lib.rs`: Install the console plugin from `FoundationPlugin` and export console APIs in the prelude.
- `crates/foundation-runtime-library/src/console.rs` or `src/console/*`: Own console state, command registry view, input handling, autocomplete, UI spawning, history/output, and command dispatch.
- New macro crate, for example `crates/foundation-console-macros`: Provide the user-facing command attribute macro if a declarative macro cannot capture function signatures and parameter names sufficiently.
- `games/template-game/src/lib.rs` or a focused module: Add example/test commands that prove game-authored commands are compiled and registered when TemplateGame is the selected binary.
- `docs/`: Add usage documentation if Rustdoc alone is insufficient for command declaration and console UX.

## Proposed Implementation Approach
1. Confirm the exact Bevy 0.19 Feathers import path and required Cargo features before editing dependencies.
2. Add a Foundation console runtime module with a `FoundationConsolePlugin` installed by `FoundationPlugin`.
3. Define core command types: command metadata, parameter metadata, command result/output, command execution context, autocomplete candidate, and history entries.
4. Implement distributed command registration using `linkme` if practical, with a Foundation-owned distributed slice of command descriptors. Only crates linked into the selected game binary should contribute commands.
5. Add a user-facing macro API for full Bevy-system-style command functions. Bevy-filled parameters should remain normal system parameters, while user-provided command inputs should be grouped into a named input struct wrapped by a dedicated console input parameter such as `ConsoleInputs<T>`. Command names should default to the Rust function name, with an optional macro override. The macro should capture command name, function pointer, and named input field metadata so placeholder text and autocomplete can be generated automatically.
6. Open and close the debug console as a Foundation scene-stack scene when toggled by backtick, without pausing gameplay.
7. Spawn a Feathers-based bottom console overlay for that scene. The overlay should use a high UI ordering/layer, full width, bottom alignment, history/output region, and text input row.
8. Route keyboard/mouse focus to the console while open. Ensure gameplay systems can query a public console-open state if they need to ignore input, but do not pause simulation by default. Up/Down should cycle through saved command history; pressing Down after the newest history entry should return to a clear input.
9. Implement input behavior: command editing, Enter execution, Tab completion, Up/Down history navigation, Escape/backtick closing behavior, placeholder text for required named properties, and output logging.
10. Implement autocomplete over registered command names and parameter names. The initial version should use deterministic prefix matching and a stable candidate ordering.
11. Add tests for command registry metadata, parsing/dispatch, autocomplete, placeholder construction, and plugin resource initialization. Add TemplateGame integration tests where feasible.
12. Add Rustdoc and feature usage documentation showing how Foundation and game code declare console commands.
13. Run required validation wrappers and generated documentation before marking tasks/phases complete.

## Alternatives Considered
- Manual registration in each game plugin: Rejected because the user explicitly wants macro-declared commands without manually wiring functions into systems.
- Runtime filtering of all games' commands: Rejected because the corrected requirement is that other games' commands are not compiled into the selected game binary.
- `inventory` registration: Viable, but less preferred than `linkme` because it uses life-before-main runtime initialization. Keep as a fallback if `linkme` cannot support the desired command descriptor shape or target platforms.
- Building a custom unstyled Bevy UI: Rejected because the user specifically requested Bevy Feathers.

## Risks, Constraints, And Assumptions
- Feathers APIs in Bevy 0.19 may be experimental or feature-gated; implementation must verify exact imports and features before coding.
- Bevy text input/focus behavior may need careful scheduling so Tab autocomplete does not conflict with UI tab navigation.
- Rust attribute macros cannot make arbitrary function signatures callable without generated adapter code; command function signature constraints must be documented.
- Command functions should support full Bevy-system-style parameters where practical. User-provided command values should be separated from Bevy-filled system parameters through a named input struct wrapped in a dedicated console input parameter.
- Distributed registration relies on final binary linking behavior. The selected-game-only requirement is satisfied only if the launcher continues compiling/running the selected game package rather than linking all games into one binary.
- The first implementation may need a focused command argument grammar; complex shell-like parsing should be avoided unless explicitly needed.
- Console focus should not accidentally trigger gameplay hotkeys while the console is open.
- Opening the console should not pause gameplay, but the console should still be represented as a Foundation scene-stack scene.

## Open Questions
- None at this time.

## Documentation Expectations
- Public APIs added or changed by this feature must have Rustdoc comments, especially command metadata, command context, registration macros, and plugin/resources intended for game use.
- Add usage documentation under `docs/` if the macro syntax, supported signatures, or console UX need more detail than Rustdoc examples can provide.
- Generated documentation must be produced with `scripts/doc-project.cmd` before the feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Read this plan, `tracker.md`, `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/foundation-architecture/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, and `.pi/skills/gitflow-workflow/SKILL.md` before implementation edits.
- Confirm the active branch is `feature/debug-console` and record branch/base uncertainty in the tracker if it cannot be verified.
- Keep the feature in `foundation-runtime-library` unless a narrow procedural macro crate is required.
- Do not add dependencies on concrete game crates from `crates/foundation` or `foundation-runtime-library`.
- Preserve the selected-game-only compile model: commands from non-selected games should be absent because their crates are not linked into the current binary.
- Keep command APIs ergonomic but explicit enough that parameter names and placeholder text can be generated reliably.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Verify command registration truly includes Foundation plus the currently linked game only.
- Verify the macro API does not require manual Bevy system wiring.
- Verify console focus/input handling prevents gameplay input leakage while open.
- Verify Feathers usage is idiomatic for Bevy 0.19.
- Verify public API docs explain supported command signatures and limitations.

## Success Criteria
- Pressing `` ` `` in a Foundation game opens/closes a full-width bottom debug console as a Foundation scene-stack scene without pausing gameplay.
- The console displays command history/output above a text input.
- Mouse and keyboard focus work inside the console while it is open.
- Typing shows autocomplete predictions and pressing Tab completes the selected prediction.
- The input shows named parameter placeholder/dummy text for the active command, derived from a named input struct.
- Command history is preserved on disk under `saved/console/` and Up/Down history navigation works, including clearing the input after pressing Down past the newest entry.
- A command function can be annotated with a Foundation macro and becomes available without manually adding it to a system.
- Command names default to the Rust function name and can be overridden in the macro.
- Foundation-authored commands and commands from the currently compiled game are available.
- Commands from other games are unavailable because those game crates are not compiled into the active binary.
- Tests cover command metadata/registration, parsing/dispatch, autocomplete, and placeholder construction.
- Required formatting, linting, tests, build, and documentation generation pass or have user-approved waivers recorded.

## Testing Methodology
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- `scripts/doc-project.cmd`
- `scripts/validate-project.cmd`
- Focused tests should be added for command registry behavior, command parser behavior, autocomplete behavior, placeholder generation, and plugin initialization.
- Manual runtime smoke test: `cargo run -p foundation -- --game template-game`, open the console with `` ` ``, type a Foundation command, use Tab autocomplete, execute it, and confirm history/output updates.
