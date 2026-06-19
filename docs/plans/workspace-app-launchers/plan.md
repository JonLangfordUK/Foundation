# Workspace App Launchers Plan

## Metadata
- Feature slug: `workspace-app-launchers`
- Feature area: `multi-area` (`engine`, `game`, `editor`)
- Primary area: `engine`
- Branch: `feature/workspace-app-launchers`
- Branch status: Created from local `dev` on 2026-06-19; remote `origin` is configured.
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-06-19`
- Last updated: `2026-06-19`

## User Request
"Lets start setting up the new sub projects, and linking. At the end, I would like to be able to open a bevy window for the game, and be able to open the editor. Lets treat this as a feature"

## Feature Summary
Convert the current blank Rust template into a small Bevy-oriented workspace with separate subprojects for shared/engine code, the game executable, and the editor executable. The first deliverable is intentionally minimal: `cargo run` targets should open a Bevy window for the game and a separate Bevy-based editor window, with shared code linked through workspace crates.

## Feature Area Classification
- Area: `multi-area` (`engine`, `game`, `editor`)
- Primary area: `engine`
- Rationale: The main work is foundational workspace/subproject structure and crate linking. The visible outputs are game and editor launchers, but their implementation depends on the shared engine/workspace layout.

## Codebase Research
- `Cargo.toml` is currently a single package named `pi-rust-template` with no dependencies.
- `src/lib.rs` contains placeholder template code and a placeholder test. It should be replaced or moved as part of the workspace conversion.
- `README.md` explicitly allows converting this template into a Cargo workspace by converting the root `Cargo.toml` to `[workspace]` and moving crates under `crates/`.
- Validation wrappers exist under `scripts/`: format, lint, test, compile, doc, and full validation.
- Existing plan/tracker workflow requires feature work on a dedicated `feature/*` branch and tracker updates during implementation.
- There is an untracked `NUL` file in the working tree before this feature's planning changes. Implementation should avoid touching it unless the user explicitly asks.

## External Research
- Bevy `DefaultPlugins` include the typical plugins needed for a Bevy app with a window and presentation components; Bevy docs note that `DefaultPlugins` obey Cargo feature flags and can be customized through plugin settings.
- Bevy `WindowPlugin` controls the primary window and supports `primary_window: Some(Window { ... })` for title/resolution customization; by default, closing all windows exits the app.
- Bevy's window settings example shows `App::new().add_plugins(DefaultPlugins.set(WindowPlugin { primary_window: Some(Window { title, resolution, ..default() }), ..default() })).run();` as the idiomatic pattern for a configured window.
- A public Bevy editor-structure example (`recatek/demo_bevy_editor_structure`) separates game logic as a library/plugin from game and editor executables and runs them with commands like `cargo run -p exe_game` and `cargo run -p exe_editor`. This supports the planned separation, while this feature will keep dynamic reloading out of scope for now.

## Affected Files And Systems
- `Cargo.toml`: Convert from a single package manifest into a workspace manifest and define workspace dependency/version settings.
- `Cargo.lock`: Update for Bevy and any workspace crates.
- `src/lib.rs`: Remove or replace the root placeholder once the root becomes a virtual workspace, unless a root package is intentionally retained.
- `crates/engine` or similarly named shared crate: Host common Bevy plugin/app setup shared by game and editor.
- `crates/game` or similarly named game crate: Provide a runnable game binary that opens a Bevy game window.
- `crates/editor` or similarly named editor crate: Provide a runnable editor binary that opens a Bevy editor window.
- `README.md`: Update run commands and workspace description if implementation changes the project layout.
- `docs/plans/workspace-app-launchers/tracker.md`: Keep progress, validation, push state, and handoff notes current during implementation.

## Proposed Implementation Approach
1. Convert the root package into a Cargo workspace with resolver `2` and workspace package metadata inherited by member crates where useful.
2. Add minimal member crates for shared engine code, game executable, and editor executable under `crates/`.
3. Add Bevy as a workspace dependency with the smallest practical configuration that still opens native windows reliably. Prefer Bevy default features initially unless compile cost becomes a blocker.
4. Put common window-launch/app setup in a shared crate so the game and editor launchers prove the subprojects are linked.
5. Implement the game launcher binary to open a titled Bevy window, e.g. `PiGame` or `PiGame - Game`.
6. Implement the editor launcher binary to open a distinct titled Bevy window, e.g. `PiGame Editor`.
7. Add focused tests for non-window logic, such as exported app/window configuration builders or title constants. Do not rely on automated tests that must open GPU windows in CI-like validation.
8. Update documentation with the run commands for both launchers.
9. Run formatting, linting, tests, build, and documentation generation. If native Bevy window opening cannot be exercised in automated validation, record manual run commands/results in the tracker.
10. Commit completed tasks/phases and push to `origin` when configured, following the project Gitflow skill.

## Alternatives Considered
- Keep a single package with multiple binaries: rejected for now because the user specifically asked for new subprojects and linking.
- Implement hot-reload/dynamic editor linking immediately: deferred because the requested outcome is opening the game and editor, not runtime plugin reloading.
- Add a full editor UI framework immediately: deferred; the first editor can be a minimal Bevy window that proves executable separation and linking.
- Use no Bevy default features: deferred until there is a concrete compile/runtime need, because `DefaultPlugins` is the simplest path to a visible window.

## Risks, Constraints, And Assumptions
- Bevy can significantly increase compile time and dependency count.
- The exact latest Bevy version may require Rust toolchain compatibility checks during implementation.
- Automated validation should not require an interactive window; manual launcher checks may be recorded separately.
- The editor's first version is assumed to be a minimal window, not a complete scene/game editor.
- Workspace conversion will remove or obsolete the root package placeholder; implementation should avoid leaving dead template code.
- The remote `origin` currently points to `https://github.com/JonLangfordUK/Foundation.git`; push attempts should be recorded and failures handled per Gitflow rules.

## Open Questions
- Preferred crate names: proposed names are `pigame_engine`, `pigame_game`, and `pigame_editor`, with package names like `pigame-engine`, `pigame-game`, and `pigame-editor`.
- Preferred window titles and default resolution are not specified; implementation should choose sensible defaults unless the user provides names.
- Should the editor share all game plugins immediately, or only link shared engine/config code for this first feature?

## Documentation Expectations
- Public APIs added or changed by this feature must have Rustdoc comments, or this plan must explicitly justify why they are internal/undocumented.
- Feature-level documentation should be updated in `README.md` with workspace layout and commands:
  - `cargo run -p pigame-game`
  - `cargo run -p pigame-editor`
- Generated documentation must be produced before the feature is considered complete using `scripts/doc-project.cmd`.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Before editing, read `.pi/skills/feature-tracker-update/SKILL.md`, this plan, and `tracker.md`.
- Confirm the active branch is `feature/workspace-app-launchers` and record any branch-base uncertainty in the tracker.
- Keep implementation minimal and idiomatic; first prove workspace linking and launcher windows before adding editor-specific systems.
- Keep the tracker updated before and after substantive implementation work.
- Do not mark tasks/phases complete until required validation and documentation generation pass or an approved waiver is recorded.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Confirm workspace/package naming is consistent and no template placeholder package remains accidentally active.
- Confirm shared code is genuinely linked by both game and editor crates.
- Confirm validation avoids requiring interactive windows while still documenting manual launcher checks.

## Success Criteria
- The repository is a Cargo workspace with separate game and editor subprojects plus shared code.
- The game executable can be launched and opens a Bevy game window.
- The editor executable can be launched and opens a Bevy editor window.
- Game and editor crates depend on shared workspace code rather than duplicating all setup.
- README or equivalent documentation lists the run commands.
- Format, lint, tests, build, and documentation generation pass, or any waiver is explicitly approved and recorded.

## Testing Methodology
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- `scripts/doc-project.cmd`
- `scripts/validate-project.cmd` for the full validation sequence when practical
- Manual launcher checks recorded in the tracker:
  - `cargo run -p pigame-game`
  - `cargo run -p pigame-editor`
