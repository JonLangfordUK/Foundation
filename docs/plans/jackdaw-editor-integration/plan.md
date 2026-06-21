# Jackdaw Editor Integration Plan

## Metadata
- Feature slug: `jackdaw-editor-integration`
- Feature area: `editor`
- Primary area: `editor`
- Branch: `feature/jackdaw-editor-integration`
- Branch status: Created locally from `dev` on 2026-06-19; remote `origin` is configured.
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-06-19`
- Last updated: `2026-06-19`

## User Request
"Now lets plan the jackdaw work for the editor. Part of this, the editor should be renamed to PillarEditor"

## Feature Summary
Integrate the Jackdaw Bevy editor into the current editor subproject and rename the editor product/subproject to PillarEditor. The current editor is only a minimal Bevy window launcher. This feature should turn it into a Jackdaw-backed editor host while preserving the game launcher and shared workspace structure.

## Feature Area Classification
- Area: `editor`
- Primary area: `editor`
- Rationale: The work primarily affects the editor crate, editor dependencies, editor launch command, editor window/product naming, and editor documentation. Some small game/shared changes may be needed so the editor can add a game plugin, but those support the editor integration.

## Codebase Research
- The root `Cargo.toml` is a Cargo workspace with members `crates/engine`, `crates/game`, and `crates/editor`.
- The workspace already uses `bevy = "0.18.1"`, matching Jackdaw's Bevy 0.18 requirement.
- The current editor package is `pigame-editor` in `crates/editor`, with description `PiGame editor launcher`.
- `crates/editor/src/main.rs` currently only builds `LauncherWindowConfig::new(format!("{GAME_NAME} Editor"))` and calls `pigame_engine::run_launcher`; it does not use Jackdaw.
- `crates/engine/src/lib.rs` owns shared `DefaultPlugins.set(WindowPlugin { primary_window: ... })` launcher setup. Jackdaw integration may bypass `run_launcher` for the editor because Jackdaw needs additional plugins in a specific order.
- `crates/game/src/lib.rs` exposes `GAME_NAME`, `GAME_WINDOW_TITLE`, `game_window_config`, and `run_game`, but it does not yet expose a Bevy `Plugin` containing gameplay systems. Jackdaw's static-editor pattern expects user gameplay to be available as a plugin.
- `README.md` currently documents `cargo run -p pigame-editor`; implementation should update this to the new PillarEditor command.
- The prior workspace launcher tracker records Jackdaw as postponed/future work and confirms the current editor does not use Jackdaw.

## External Research
- Jackdaw `0.4.1` is published as a Bevy 0.18 scene/level editor with hierarchy, inspector, 3D viewport, scene serialization, transform tools, undo/redo, and extension support.
- Jackdaw's README shows embedding by adding `jackdaw::EditorPlugin`/editor plugin support to a Bevy app; current source exports `jackdaw::prelude::*`, including `EditorPlugins`, `PhysicsPlugins`, and `EnhancedInputPlugin`.
- Jackdaw's current source at commit `5e20671dbf6851180b9e032ee95039cdc1adab4d` defines `EditorPlugins` as the primary editor plugin group in `src/lib.rs` and documents `EditorPlugins::default()` usage.
- Jackdaw source comments state `DylibLoaderPlugin` is intentionally not included in `EditorPlugins`; per-project static editor binaries should usually use `EditorPlugins` and avoid scanning user config dylib directories.
- Jackdaw's `EditorCorePlugin` debug-asserts that `EnhancedInputPlugin` is added before `EditorPlugins`.
- Jackdaw migration docs for existing Bevy 0.18 projects recommend adding `jackdaw`, adding `jackdaw_runtime` for scene loading, moving gameplay into a `Plugin`, and using an editor binary with `DefaultPlugins`, `PhysicsPlugins::default()`, `EnhancedInputPlugin`, `EditorPlugins::default()`, and the game plugin.
- Jackdaw migration docs also note `bevy/file_watcher` powers hot-reload of `assets/scene.jsn`, and `ctrlc` helps terminal Ctrl+C handling before wgpu/gilrs swallow signals.
- Jackdaw custom component docs say Bevy 0.18 reflect auto-registration plus the `reflect_documentation` feature allow reflected game components to appear in the editor component picker.

## Affected Files And Systems
- `Cargo.toml`: Rename workspace member from `crates/editor` to the chosen PillarEditor path, add Jackdaw-related workspace dependencies, and possibly adjust Bevy feature configuration.
- `Cargo.lock`: Update for Jackdaw and its transitive dependencies.
- `crates/editor/` or replacement path: Rename/move editor subproject to a PillarEditor-specific path.
- `crates/editor/Cargo.toml`: Rename package from `pigame-editor` to a valid Cargo package name, proposed `pillar-editor`, and add `jackdaw`/supporting dependencies.
- `crates/editor/src/main.rs`: Replace minimal launcher with a Jackdaw editor host that opens as PillarEditor.
- `crates/game/src/lib.rs`: Add a minimal `PiGamePlugin` or similarly named plugin if needed so Jackdaw can host current/future game functionality.
- `crates/engine/src/lib.rs`: Potentially add reusable app/window setup helpers that can configure `DefaultPlugins` without immediately running, if useful for both game and editor.
- `README.md`: Update workspace layout, editor name, and launch command.
- `docs/plans/jackdaw-editor-integration/tracker.md`: Keep implementation, validation, launch-check, and push state current.

## Proposed Implementation Approach
1. Keep the feature on `feature/jackdaw-editor-integration` and verify the branch base from `dev` before implementation edits.
2. Rename the editor subproject to PillarEditor:
   - Proposed directory: `crates/pillar-editor` or `crates/pillar_editor`; implementation should choose one and keep README/workspace paths consistent.
   - Proposed Cargo package name: `pillar-editor` because package names should be lowercase and hyphenated.
   - Proposed user-facing window/product title: `PillarEditor` exactly as requested.
   - Proposed run command: `cargo run -p pillar-editor` unless implementation finds a better Cargo target arrangement.
3. Add Jackdaw dependencies. Prefer a conservative initial dependency shape based on Jackdaw migration docs:
   - `jackdaw = { version = "0.4", default-features = false }` for the editor crate.
   - Consider `ctrlc = "3"` for editor process termination.
   - Consider `jackdaw_runtime = "0.4"` if this feature also wires scene runtime or game components for Jackdaw-authored scenes.
4. Adjust Bevy feature configuration if Jackdaw needs explicit `file_watcher`, `reflect_documentation`, `serialize`, or `experimental_bevy_feathers` features in this workspace. Keep feature changes minimal and justified in the tracker.
5. Refactor current game code to expose a minimal Bevy plugin, for example `PiGamePlugin`, without changing game behavior beyond what is needed for the editor to host it.
6. Replace the current editor `run_launcher(editor_window_config())` flow with a Jackdaw host app:
   - create `App::new()`;
   - add configured `DefaultPlugins` / window setup so the primary window title is `PillarEditor`;
   - add `PhysicsPlugins::default()` and `EnhancedInputPlugin` before `EditorPlugins::default()`;
   - add the game plugin if created;
   - run the app.
7. Decide whether this first Jackdaw integration should auto-open the current project directory using Jackdaw's `PendingAutoOpen`. If used, record why; if deferred, record it as future polish.
8. Add/update non-interactive tests for naming/configuration and game plugin availability where practical. Avoid tests that must open the Jackdaw window.
9. Update `README.md` with the new editor name and run command.
10. Run required validation: format, lint, tests, build, docs, and full validation wrapper.
11. Manually launch `cargo run -p pillar-editor` long enough to confirm a Jackdaw/PillarEditor window opens. Record observed output. If Jackdaw launch requires assets/project files or user interaction, document the result clearly in the tracker.
12. Commit completed tasks/phases and push to `origin`, following the project Gitflow skill.

## Alternatives Considered
- Keep package name `pigame-editor` and only change the window title: rejected because the user specifically said the editor sub-project should be renamed to PillarEditor.
- Use Jackdaw's standalone installed binary instead of embedding Jackdaw in this workspace: rejected because the request is for editor subproject work.
- Enable Jackdaw `dylib`/hot-reload flow immediately: deferred because Jackdaw docs mark dylib support experimental and the current project first needs a static editor host.
- Add full game scene runtime loading in the same step: potentially useful, but may be deferred if it expands scope beyond proving PillarEditor can open with Jackdaw.

## Risks, Constraints, And Assumptions
- Jackdaw is pre-1.0 and its docs warn to expect bugs and changes.
- Jackdaw `0.4.1` uses edition 2024 internally, but this workspace currently uses edition 2021. Dependencies can use different editions, but implementation should not change this workspace edition unless needed.
- Jackdaw may require additional Bevy features that increase build time and dependency count.
- Jackdaw integration may surface duplicate-plugin panics if `DefaultPlugins`, `PhysicsPlugins`, `EnhancedInputPlugin`, or future game plugins are added in the wrong place.
- The exact desired PillarEditor naming surface is assumed to include package/subproject name, executable/window title, and docs. Cargo package names cannot practically be `PillarEditor` with that capitalization, so the plan proposes package `pillar-editor` and product/window title `PillarEditor`.
- Manual editor-window validation may require stopping a long-running process by timeout or by closing the window.

## Open Questions
- Should the directory be `crates/pillar-editor` or `crates/pillar_editor`? Proposed default: `crates/pillar-editor` to match package naming.
- Should the executable name be lowercase/default (`pillar-editor.exe`) or explicitly `PillarEditor.exe`? Proposed default: package/binary `pillar-editor` with window title `PillarEditor`; if the executable itself must be `PillarEditor`, add an explicit `[[bin]]` target.
- Should this feature create initial Jackdaw project files such as `project.jsn` and `assets/scene.jsn`, or only integrate/open the editor host? Proposed default: only integrate/open the editor host unless Jackdaw requires seed files for a useful launch.

## Documentation Expectations
- Public APIs added or changed by this feature must have Rustdoc comments, especially any game plugin or editor configuration helpers.
- README must document the renamed editor subproject and new run command.
- If Jackdaw project/scene files are created, README should explain their purpose.
- Generated documentation must be produced before the feature is considered complete using `scripts/doc-project.cmd` or the underlying `cargo doc --workspace --all-features --no-deps` command.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Before editing, read `.pi/skills/feature-tracker-update/SKILL.md`, this plan, and `tracker.md`.
- Confirm active branch is `feature/jackdaw-editor-integration` and verify `dev` ancestry before implementation edits.
- Keep Jackdaw integration static and minimal first; do not enable Jackdaw `dylib` unless a specific compile/runtime issue requires it and the tracker explains the decision.
- Preserve the game launcher command `cargo run -p pigame-game` unless a deliberate game-package rename is separately approved.
- Treat `PillarEditor` as the user-facing editor name.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Verify `PillarEditor` naming is applied consistently in package/docs/window title without violating Cargo naming conventions.
- Verify Jackdaw plugin ordering matches Jackdaw docs and avoids duplicate plugin additions.
- Verify the editor crate does not accidentally add Jackdaw dependencies to the standalone game path.
- Verify validation/manual launch results are recorded in the tracker.

## Success Criteria
- The editor subproject is renamed from `pigame-editor` to PillarEditor naming, with a valid Cargo package command such as `cargo run -p pillar-editor`.
- The editor app opens as a Jackdaw-backed editor host rather than a plain Bevy window.
- The editor window/product name is `PillarEditor`.
- The game launcher still builds and runs separately.
- README documents the new editor command and name.
- Format, lint, tests, build, docs, and full validation pass, or any waiver is explicitly approved and recorded.
- A manual editor launch check confirms PillarEditor/Jackdaw opens, or any launch blocker is recorded with next steps.

## Testing Methodology
- `scripts/format-project.cmd` or `cargo fmt --all -- --check`
- `scripts/lint-project.cmd` or `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `scripts/test-project.cmd` or `cargo test --workspace --all-features`
- `scripts/compile-project.cmd` or `cargo build --workspace --all-features`
- `scripts/doc-project.cmd` or `cargo doc --workspace --all-features --no-deps`
- `scripts/validate-project.cmd` or `powershell -ExecutionPolicy Bypass -File scripts/Invoke-RustWorkspace.ps1 validate-project`
- Manual launch checks:
  - `cargo run -p pigame-game`
  - `cargo run -p pillar-editor`
