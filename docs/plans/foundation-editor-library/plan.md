# Foundation Editor Library Plan

## Metadata
- Feature slug: `foundation-editor-library`
- Feature area: `multi-area`
- Primary area: `editor`
- Branch: `feature/foundation-editor-library`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-06-21`
- Last updated: `2026-06-21`

## User Request
"I like option 2, going with a deadicated editor library. Lets start a new feature. Rename the foundation-library to foundation-runtime-library. Then add a new foundation-editor-library. Then in the new library, introduce a new game settings window. This will be for any settings we want to set for our game. Like the start up map, or the editor startup map"

## Feature Summary
Split Foundation into runtime and editor-focused crates. The existing `foundation-library` crate becomes `foundation-runtime-library`, keeping reusable game/runtime systems separate from full Jackdaw editor APIs. A new `foundation-editor-library` crate provides reusable Jackdaw editor extensions, starting with a dockable game settings window for project-level settings such as the standalone startup map and editor startup map.

## Feature Area Classification
- Area: `multi-area` (`editor`, shared runtime library, and game integration)
- Primary area: `editor`
- Rationale: The main new capability is a Jackdaw editor extension/window. The runtime crate rename and TemplateGame integration are required to keep crate boundaries clean and prove the editor library works in a real game editor binary.

## Branch Status
- Planned branch: `feature/foundation-editor-library`
- Current status: branch created locally from `dev` on `2026-06-21`.
- Base verification: `dev` is an ancestor of `HEAD`.
- Remote: `origin` is configured. Push status is pending until planning/implementation commits are created.
- Existing local state: `games/template-game/.jsn/project.jsn` was already modified by local editor use before this feature. Do not overwrite or commit it unless the user explicitly asks.

## Codebase Research
- Root `Cargo.toml` currently lists workspace members `crates/foundation-library`, `crates/jackdaw-editor`, and `games/template-game`.
- `crates/foundation-library` is package `foundation-library` and imports as `foundation_library`. It depends on workspace `bevy` and `jackdaw_runtime` and exposes `FoundationPlugin`, `FoundationSettings`, `FoundationActor`, `menu`, `scene_stack`, `splash_screen`, and a public prelude.
- `games/template-game` currently depends on `foundation-library = { path = "../../crates/foundation-library" }` and imports `foundation_library::prelude::*` from `src/main.rs`, `src/lib.rs`, and `src/bin/editor.rs`.
- TemplateGame `.jsn` scene assets serialize runtime component type paths such as `foundation_library::splash_screen::FoundationSplashScreen`. A full crate rename likely requires updating serialized type paths to `foundation_runtime_library::...` unless implementation intentionally preserves the old Rust crate name via `[lib] name = "foundation_library"` for compatibility. The preferred plan is a real runtime crate rename and asset migration.
- `games/template-game/src/lib.rs` currently hard-codes startup scene constants such as `PIXEL_PERFECT_SPLASH_SCENE`, `LANDING_PAGE_SCENE`, and `MAIN_MENU_SCENE`, and `open_initial_scene` drives standalone/editor Play startup. This is the likely integration point for startup map settings.
- `games/template-game/src/bin/editor.rs` currently adds `EditorPlugins::default()`, then `FoundationPlugin`, then `TemplateGamePlugin`. It should become the first consumer of `foundation-editor-library` by registering the editor extension/plugin while keeping runtime plugin order intact.
- Jackdaw `0.4.1` exposes a public extension API through `JackdawExtension`, `ExtensionContext`, `WindowDescriptor`, `TopLevelMenu`, and operator registration. Examples in the local Cargo registry show `EditorPlugins::default().set(ExtensionPlugin::default().with_extension::<...>())` and registering a dockable window with `WindowDescriptor::new(...).with_name(...).with_default_area(...).with_build(...)`.
- Jackdaw public extension windows are dockable editor windows rather than native OS settings dialogs. The settings UI should start as a dockable Jackdaw window available from the Window menu.

## External Research
No external online research was performed because the required Jackdaw extension examples and APIs are already available in the local Cargo registry, and the feature can be planned from existing workspace structure and local Jackdaw `0.4.1` sources.

## Affected Files And Systems
- `Cargo.toml`: Rename the runtime workspace member path and add `crates/foundation-editor-library`; consider adding workspace dependencies such as `serde`/`toml` only if persistent settings need them.
- `crates/foundation-library/`: Rename/move to `crates/foundation-runtime-library` and update package metadata.
- `crates/foundation-runtime-library/Cargo.toml`: Package should become `foundation-runtime-library`; Rust import path should become `foundation_runtime_library` unless a compatibility alias is deliberately chosen.
- `crates/foundation-runtime-library/src/lib.rs`: Update crate-level docs from FoundationLibrary to FoundationRuntimeLibrary and consider replacing baseline `FoundationSettings` with a clearer runtime/editor-shared game settings resource.
- `crates/foundation-runtime-library/src/menu.rs`, `scene_stack.rs`, `splash_screen.rs`: Update crate docs and any test strings that mention FoundationLibrary; preserve runtime behavior.
- `crates/foundation-editor-library/Cargo.toml`: New library package depending on `bevy`, `jackdaw`, and `foundation-runtime-library`; add `serde`/`toml` only if persistence is implemented in this feature.
- `crates/foundation-editor-library/src/lib.rs`: New documented editor library API and plugin/extension exports.
- `crates/foundation-editor-library/src/game_settings.rs` or similar: Own the game settings window extension, operators, UI state, and persistence helpers if implemented.
- `games/template-game/Cargo.toml`: Update runtime dependency and add an editor-only dependency on `foundation-editor-library` behind the `editor` feature.
- `games/template-game/src/main.rs`: Import `foundation_runtime_library::prelude::*`; add/read runtime game settings for standalone startup map if in scope.
- `games/template-game/src/bin/editor.rs`: Register the Foundation editor extension/plugin with Jackdaw editor plugins; import runtime/editor crates under their new names.
- `games/template-game/src/lib.rs`: Update runtime imports and use configured startup/editor startup scene paths where practical.
- `games/template-game/assets/*.jsn`: Migrate serialized component type paths from `foundation_library::...` to `foundation_runtime_library::...` if the Rust crate name changes.
- `games/template-game/tests/template_components.rs`: Update imports and reflected type path expectations if affected.
- `README.md` and `docs/scene-system.md`: Update architecture and usage docs from FoundationLibrary to FoundationRuntimeLibrary plus FoundationEditorLibrary.
- `.cargo/config.toml` and scripts/docs references: Update any package names or commands that still reference `foundation-library`.
- `AGENTS.md` and `.pi/skills/*`: Update project workflow guidance and reusable skills so future runtime-library and editor-library work follows the new Foundation Runtime / Foundation Editor split.

## Proposed Implementation Approach
1. Confirm the active branch is `feature/foundation-editor-library`, verify `dev` ancestry, and record existing dirty editor-local `.jsn` state before implementation edits.
2. Rename the existing runtime crate.
   - Move `crates/foundation-library` to `crates/foundation-runtime-library`.
   - Change package name to `foundation-runtime-library`.
   - Prefer changing the Rust crate import path to `foundation_runtime_library` for consistency with the package rename.
   - Update docs, tests, workspace members, dependencies, and references.
3. Update TemplateGame and assets for the runtime rename.
   - Replace Rust imports from `foundation_library` to `foundation_runtime_library`.
   - Update `games/template-game/Cargo.toml` dependency path/name.
   - Update serialized `.jsn` type paths that reference the old Rust crate name.
   - Run focused checks for `foundation-runtime-library` and `template-game` before continuing if practical.
4. Add the `foundation-editor-library` crate.
   - Add it as a workspace member under `crates/foundation-editor-library`.
   - Depend on `bevy.workspace = true`, `jackdaw.workspace = true`, and the path dependency to `foundation-runtime-library`.
   - Keep editor-only Jackdaw APIs out of the runtime crate.
5. Add a shared game settings model.
   - Preferred ownership: `foundation-runtime-library` owns plain runtime/shared settings data, such as `FoundationGameSettings` with `startup_map` and `editor_startup_map` paths.
   - `foundation-editor-library` owns editing UI and Jackdaw extension plumbing.
   - Use descriptive defaults, likely matching TemplateGame's current startup flow initially.
6. Implement a first dockable Jackdaw game settings window.
   - Register a `FoundationGameSettingsExtension` or similar with a unique extension ID.
   - Register a `WindowDescriptor` named something like `Game Settings` with default area in a sidebar.
   - Show current startup map and editor startup map values.
   - Provide minimal actions/operators, for example setting either value from the currently open Jackdaw scene and saving/reloading settings.
   - Prefer a simple baseline UI over a complex text-edit UI unless Jackdaw's public text editing API is straightforward and stable enough.
7. Decide and implement persistence for settings.
   - Preferred baseline: a project-local config file such as `foundation.settings.toml` under the game project root, with documented fields for `startup_map` and `editor_startup_map`.
   - If persistence is too large for the first pass, record a user-approved deferral and keep the window resource-backed only; however, persistence is strongly preferred because settings should survive editor restarts.
8. Wire TemplateGame editor to the new editor library.
   - Enable `foundation-editor-library` only for the `editor` feature.
   - Register the editor extension through `EditorPlugins::default().set(ExtensionPlugin::default().with_extension::<...>())` or an equivalent helper plugin exported by `foundation-editor-library`.
   - Keep `FoundationPlugin` from the runtime crate available in both standalone and editor builds.
9. Wire startup map usage.
   - TemplateGame standalone startup should prefer `startup_map` when a valid setting exists, falling back to current hard-coded defaults.
   - Jackdaw editor Play startup should prefer `editor_startup_map` where appropriate, while preserving current behavior of continuing from a known currently open scene when that is the better editor workflow.
   - Clearly document precedence rules.
10. Update README/docs and Pi workflow guidance to explain the new crate split and settings file/window behavior.
    - Update `AGENTS.md` so future feature planning/implementation knows there are separate Foundation runtime and editor library areas.
    - Update or add project skills when the existing skills still refer to `foundation-library` as the only shared Foundation area.
    - Ensure guidance says runtime/game systems belong in `foundation-runtime-library`, while Jackdaw editor windows/extensions belong in `foundation-editor-library`.
11. Add tests that avoid opening windows.
   - Runtime crate tests for settings defaults and parsing/serialization if persistence exists.
   - Editor crate tests for extension/window metadata where possible without launching a GPU window.
   - TemplateGame tests for dependency/import compatibility and startup map resolution logic.
12. Validate using project wrapper scripts and commit/push each completed task/phase according to gitflow rules.

## Alternatives Considered
- Keep one `foundation-library` crate with an optional `editor` feature: rejected by user preference for a dedicated editor library and by architectural clarity. It would also risk pulling editor concepts into runtime APIs.
- Preserve the Rust crate name `foundation_library` with `[lib] name = "foundation_library"` while renaming only the package/directory: possible compatibility shortcut, but it would make the rename less explicit and keep old serialized type paths. Prefer a real rename unless migration proves too disruptive.
- Put the settings window in `crates/jackdaw-editor`: rejected because the window should be reusable for game-specific editor binaries, not just the generic launcher shell.
- Put game-specific settings entirely in TemplateGame: rejected because the user wants a reusable Foundation editor library for settings we want to set for any game.
- Build a native modal settings dialog: deferred. Jackdaw's public extension surface is designed around dockable windows and menu/operator integration.

## Risks, Constraints, And Assumptions
- Renaming the Rust crate import path may break existing `.jsn` serialized type names until all assets are migrated.
- Existing plan/docs references to `foundation-library` are historical; implementation should update active docs/README/skills, but old plan records should usually remain historical unless they need current-command references fixed.
- Jackdaw extension APIs are public but relatively young; the settings window should use the documented/public extension surface and avoid private Jackdaw internals.
- Persisting settings requires selecting a file format and project-root path. `toml` is likely appropriate but adds a dependency if not already available.
- Text entry fields may require additional Jackdaw/Feathers APIs. A button-driven baseline using the currently open scene may be more reliable for the first pass.
- TemplateGame currently has nuanced editor Play routing from the currently open scene. Startup/editor startup map settings must not regress that workflow.
- The existing dirty `games/template-game/.jsn/project.jsn` file should not be committed accidentally.

## Open Questions
- Confirm feature classification: this plan treats the feature as `multi-area` with primary area `editor`.
- Should settings persistence be required in the first implementation pass? Proposed answer: yes, use a project-local settings file so startup map choices survive editor restarts.
- What should the exact config file name be? Proposed answer: `foundation.settings.toml` in the game project root.
- Should editor Play always use `editor_startup_map`, or should currently open known scenes continue to take precedence? Proposed answer: currently open scene should keep precedence for designer workflow, with `editor_startup_map` as a fallback/default when launching editor or playing without a recognized open scene.
- Should the generic `crates/jackdaw-editor` launcher also include `foundation-editor-library`, or only game-specific editor binaries? Proposed answer: only game-specific editor binaries initially, because settings belong to a selected game project.

## Documentation Expectations
- Public APIs added or changed by this feature must have Rustdoc comments.
- The runtime/editor crate split must be documented in `README.md`.
- Project guidance in `AGENTS.md` and relevant `.pi/skills/*` files must be updated so future work recognizes `foundation-runtime-library` and `foundation-editor-library` as distinct areas.
- Settings window usage and settings-file fields should be documented in README or a focused doc under `docs/` if README becomes too long.
- Generated documentation must be produced before the feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Start by reading this `plan.md`, `tracker.md`, `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, and `.pi/skills/gitflow-workflow/SKILL.md`.
- Confirm the active branch is `feature/foundation-editor-library` and `dev` is an ancestor before editing.
- Do not commit `games/template-game/.jsn/project.jsn` unless explicitly requested.
- Update `AGENTS.md` and relevant project skills as part of implementation because this feature creates a new Foundation editor area.
- Keep Jackdaw editor APIs in `foundation-editor-library`, not `foundation-runtime-library`.
- Keep runtime/shared settings data in `foundation-runtime-library` so standalone games can read startup map settings without depending on Jackdaw editor APIs.
- Watch for `.jsn` serialized type path migration after the runtime crate rename.
- Commit and push after each completed task/phase when `origin` is available.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Verify crate boundary cleanliness: full `jackdaw` dependency should not leak into `foundation-runtime-library`.
- Verify `.jsn` migration from `foundation_library` to `foundation_runtime_library` is complete.
- Verify startup/editor startup map precedence is documented and tested.
- Verify settings persistence handles missing/invalid config gracefully.

## Success Criteria
- Workspace member `crates/foundation-library` is renamed to `crates/foundation-runtime-library` with package/import references updated.
- New workspace member `crates/foundation-editor-library` exists and builds.
- Standalone TemplateGame still runs with the runtime Foundation plugin after the rename.
- TemplateGame editor binary includes the Foundation editor library and exposes a dockable Game Settings window.
- Game Settings window can display and update at least startup map and editor startup map settings.
- Settings are persisted or a documented/user-approved deferral exists.
- Startup map settings are used with documented fallback behavior.
- README/docs and project skills explain the new runtime/editor split and settings workflow.
- Required validation passes or any waiver is documented and approved.

## Testing Methodology
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- `scripts/doc-project.cmd` when present, otherwise `cargo doc --workspace --all-features --no-deps`
- `scripts/validate-project.cmd` for the full validation sequence when present
- Focused checks during implementation may include:
  - `cargo check -p foundation-runtime-library`
  - `cargo test -p foundation-runtime-library`
  - `cargo check -p foundation-editor-library`
  - `cargo test -p foundation-editor-library`
  - `cargo check -p template-game --features editor`
  - `cargo test -p template-game --lib --features editor`
- Manual smoke testing after automated validation should launch the TemplateGame editor and verify the Game Settings window appears without closing the editor.
