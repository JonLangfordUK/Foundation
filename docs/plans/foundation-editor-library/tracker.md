# Foundation Editor Library Tracker

## Metadata
- Feature slug: `foundation-editor-library`
- Feature area: `multi-area`
- Primary area: `editor`
- Branch: `feature/foundation-editor-library`
- Overall status: `Implemented; awaiting user acceptance`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation complete with gpt-5.4; ready for user acceptance or optional gpt-5.5 review`
- Created: `2026-06-21`
- Last updated: `2026-06-21`

## Branch And Push State
- Active planning branch: `feature/foundation-editor-library`
- Branch base: created locally from `dev` on `2026-06-21`; `dev` verified as an ancestor of `HEAD`.
- Remote: `origin` configured at `https://github.com/JonLangfordUK/Foundation.git`.
- Push status: Planning docs commit `e12d73b`, runtime rename commit `501ffd7`, implementation commit `e04ef99`, and final tracker push-status commit pushed to `origin/feature/foundation-editor-library`.
- Pre-existing local changes: `games/template-game/.jsn/project.jsn` was already modified by local editor use before this feature. It was not committed.

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Use project wrappers unless a narrower focused command is documented for an intermediate checkpoint:
  - `scripts/format-project.cmd`
  - `scripts/lint-project.cmd`
  - `scripts/test-project.cmd`
  - `scripts/compile-project.cmd`
  - `scripts/doc-project.cmd`
  - `scripts/validate-project.cmd`

## Phase 1: Runtime Library Rename
**Status:** Complete  
**Goal:** Rename the existing reusable runtime crate from `foundation-library` to `foundation-runtime-library` without changing behavior.

### Tasks
- [x] Verify implementation branch and protect pre-existing editor-local project state.
  - Status: Complete
  - Notes: Confirmed active branch `feature/foundation-editor-library`; verified `dev` is an ancestor of `HEAD`; `games/template-game/.jsn/project.jsn` remains a pre-existing local modification and stayed out of commits.
- [x] Move `crates/foundation-library` to `crates/foundation-runtime-library`.
  - Status: Complete
  - Notes: Moved the crate directory, updated root workspace membership, changed the package name to `foundation-runtime-library`, and regenerated the workspace lock entry.
- [x] Rename Rust references from `foundation_library` to `foundation_runtime_library` where a real crate rename is used.
  - Status: Complete
  - Notes: Updated TemplateGame Rust imports and active runtime docs.
- [x] Migrate `.jsn` serialized component type paths from `foundation_library::...` to `foundation_runtime_library::...`.
  - Status: Complete
  - Notes: Updated TemplateGame `.jsn` assets so reflected Foundation runtime components use the new crate path.
- [x] Update active documentation references for the runtime crate rename.
  - Status: Complete
  - Notes: Updated README and `docs/scene-system.md` for the runtime crate name.

### Validation
- Format: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Lint: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Tests: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Build: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Documentation generation: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Full validation wrapper: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- User confirmation: Not required after implementation approval

## Phase 2: Foundation Editor Library Crate
**Status:** Complete  
**Goal:** Add a dedicated editor library crate that owns Jackdaw editor extension code and depends on the runtime library.

### Tasks
- [x] Add `crates/foundation-editor-library` as a workspace member.
  - Status: Complete
  - Notes: Added package `foundation-editor-library`; Rust import path is `foundation_editor_library`.
- [x] Create documented editor library baseline API.
  - Status: Complete
  - Notes: Added documented `FoundationEditorPlugin`, `FoundationGameSettingsExtension`, stable extension/window IDs, and prelude exports.
- [x] Add dependencies without leaking full `jackdaw` into the runtime crate.
  - Status: Complete
  - Notes: Editor crate depends on Bevy, `jackdaw`, `jackdaw_api`, `bevy_enhanced_input`, and `foundation-runtime-library`; runtime crate still avoids full `jackdaw`.
- [x] Add non-window tests for editor extension metadata or API shape where practical.
  - Status: Complete
  - Notes: Added tests for extension metadata and scene path normalization without launching a GPU window.

### Validation
- Format: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Lint: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Tests: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Build: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Documentation generation: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Full validation wrapper: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- User confirmation: Not required after implementation approval

## Phase 3: Shared Game Settings Model And Persistence
**Status:** Complete  
**Goal:** Introduce reusable settings data for startup map and editor startup map, with persistence.

### Tasks
- [x] Add a runtime/shared game settings type.
  - Status: Complete
  - Notes: Added `FoundationGameSettings` in `foundation-runtime-library` with `startup_map` and `editor_startup_map`.
- [x] Define default settings and fallback behavior.
  - Status: Complete
  - Notes: Empty settings mean the game uses its built-in default flow; TemplateGame preserves current splash flow unless a setting is configured.
- [x] Implement project-local settings persistence.
  - Status: Complete
  - Notes: Added TOML persistence using `foundation.settings.toml` in the game project root.
- [x] Add parsing/default/fallback tests.
  - Status: Complete
  - Notes: Added runtime tests for defaults, blank values, missing-file defaults, and TOML round trip.

### Validation
- Format: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Lint: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Tests: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Build: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Documentation generation: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Full validation wrapper: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- User confirmation: Not required after implementation approval

## Phase 4: Game Settings Jackdaw Window
**Status:** Complete  
**Goal:** Add a reusable dockable Jackdaw Game Settings window in the editor library.

### Tasks
- [x] Implement a Jackdaw extension for Foundation game settings.
  - Status: Complete
  - Notes: Added `FoundationGameSettingsExtension` using public Jackdaw extension APIs.
- [x] Register a dockable `Game Settings` window.
  - Status: Complete
  - Notes: Window shows startup map, editor startup map, and status labels.
- [x] Add minimal update actions/operators.
  - Status: Complete
  - Notes: Added operators to set startup/editor startup maps from the open scene and to save/reload settings.
- [x] Keep the UI simple and documented.
  - Status: Complete
  - Notes: Used public Jackdaw button/operator APIs and simple Bevy UI labels; rich text editing remains out of scope.

### Validation
- Format: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Lint: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Tests: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Build: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Documentation generation: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Full validation wrapper: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- User confirmation: Not required after implementation approval

## Phase 5: TemplateGame Integration
**Status:** Complete  
**Goal:** Wire the renamed runtime library and new editor library into TemplateGame without regressing standalone or editor Play behavior.

### Tasks
- [x] Update TemplateGame dependencies and imports.
  - Status: Complete
  - Notes: TemplateGame depends on `foundation-runtime-library` and enables `foundation-editor-library` only through the `editor` feature.
- [x] Register Foundation editor extension/plugin from `games/template-game/src/bin/editor.rs`.
  - Status: Complete
  - Notes: TemplateGame editor registers `FoundationGameSettingsExtension` through Jackdaw `ExtensionPlugin` and adds `FoundationEditorPlugin`.
- [x] Use configured startup map in standalone startup where practical.
  - Status: Complete
  - Notes: Standalone loads `FoundationGameSettings` from the project root and falls back to the existing splash flow when unset or invalid.
- [x] Use configured editor startup map according to documented precedence.
  - Status: Complete
  - Notes: Current open scenes still take precedence; `editor_startup_map` is used when no useful open scene exists.
- [x] Add/update tests for startup map resolution and editor feature composition.
  - Status: Complete
  - Notes: Added TemplateGame test for `editor_startup_map` fallback; editor extension composition is covered by compile/check and smoke launch.

### Validation
- Format: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Lint: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Tests: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Build: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Documentation generation: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Full validation wrapper: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- User confirmation: Not required after implementation approval

## Phase 6: Documentation, Skills, Validation, And Handoff
**Status:** Complete  
**Goal:** Complete docs and Pi workflow guidance, validate the workspace, commit/push checkpoints, and prepare for optional review.

### Tasks
- [x] Update README and architecture docs for the runtime/editor Foundation split.
  - Status: Complete
  - Notes: README documents the runtime/editor split, commands, settings file, and Game Settings window; `docs/scene-system.md` references the runtime crate.
- [x] Update project instructions and skills for the new Foundation editor area.
  - Status: Complete
  - Notes: Updated `AGENTS.md` and added `.pi/skills/foundation-architecture/SKILL.md` to distinguish runtime systems from editor windows/extensions.
- [x] Run full project validation.
  - Status: Complete
  - Notes: `scripts/validate-project.cmd` passed on 2026-06-21 after fixing a clippy `type_complexity` finding in the editor crate.
- [x] Manually smoke-test the TemplateGame editor settings window if practical.
  - Status: Complete
  - Notes: `timeout 25s cargo editor` launched the editor, loaded `assets/scene.jsn`, and logged `Loading extension: foundation.game_settings`; timeout exit code 143 was expected.
- [x] Commit and push completed work following gitflow rules.
  - Status: Complete
  - Notes: Runtime rename commit `501ffd7`, implementation/docs/skills commit `e04ef99`, and final tracker push-status commit were pushed to `origin/feature/foundation-editor-library`.
- [x] Update tracker with validation evidence, push state, and handoff notes.
  - Status: Complete
  - Notes: Tracker updated with implementation, validation, and smoke-test evidence.

### Validation
- Format: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Lint: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Tests: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Build: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Documentation generation: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- Full validation wrapper: Passed (`scripts/validate-project.cmd`, 2026-06-21)
- User confirmation: Pending final feature acceptance

## Implementation / Review Handoff Notes
- Implementation model: `gpt-5.4`.
- Review model: `gpt-5.5`.
- Never use Anthropic models.
- `foundation-runtime-library` remains free of the full `jackdaw` editor dependency.
- Jackdaw extension/window code lives in `foundation-editor-library`.
- Runtime/shared settings data and TOML persistence live in `foundation-runtime-library`.
- `foundation.settings.toml` stores `startup_map` and `editor_startup_map`; blank values mean use game defaults.
- `games/template-game/.jsn/project.jsn` remains a pre-existing local editor modification and was not committed.

## Postponed Work
- Native modal settings dialog: postponed in favor of a dockable Jackdaw extension window.
- Rich text-entry settings UI: postponed; the baseline uses buttons to set startup maps from the currently open scene, plus save/reload.
- Generic `crates/jackdaw-editor` launcher integration: postponed unless the user wants generic projects to load the Foundation editor library. Initial integration targets game-specific editor binaries.
- Broad settings categories beyond startup map and editor startup map: postponed until future settings needs are identified.

## Issues / Oversights Discovered
- `2026-06-21`: First `scripts/validate-project.cmd` run failed on `clippy::type_complexity` in `crates/foundation-editor-library/src/lib.rs`; fixed by adding query type aliases and reran validation successfully.

## Progress Log
- `2026-06-21`: Read feature planning, gitflow, Rust workspace, and Rust coding standards skills.
- `2026-06-21`: Inspected workspace manifests, runtime Foundation crate, TemplateGame standalone/editor entry points, Jackdaw extension examples/API, current git branch, remotes, and dirty local state.
- `2026-06-21`: Created branch `feature/foundation-editor-library` from `dev` and verified `dev` is an ancestor.
- `2026-06-21`: Created plan and tracker for the Foundation runtime/editor library split and Game Settings window feature.
- `2026-06-21`: User approved the plan direction and asked to also update project skills because this creates a new area; plan and tracker updated to include `AGENTS.md` and relevant `.pi/skills/*` updates.
- `2026-06-21`: Planning docs committed as `e12d73b` and pushed to `origin/feature/foundation-editor-library`; implementation started with gpt-5.4.
- `2026-06-21`: Completed the runtime crate rename from `foundation-library`/`foundation_library` to `foundation-runtime-library`/`foundation_runtime_library`, updated TemplateGame Rust imports and `.jsn` type paths, and recorded focused validation passes.
- `2026-06-21`: Runtime rename committed as `501ffd7` and pushed to `origin/feature/foundation-editor-library`.
- `2026-06-21`: Added `foundation-editor-library`, `FoundationGameSettings`, TOML settings persistence, Game Settings Jackdaw extension/window, TemplateGame editor wiring, settings startup-map integration, README updates, AGENTS guidance, and a new `foundation-architecture` skill.
- `2026-06-21`: Full validation passed via `scripts/validate-project.cmd`; manual smoke launch via `timeout 25s cargo editor` loaded `foundation.game_settings` and was terminated by expected timeout.
- `2026-06-21`: Implementation/docs/skills commit `e04ef99` pushed to `origin/feature/foundation-editor-library`; final tracker push-status commit pushed afterward.
