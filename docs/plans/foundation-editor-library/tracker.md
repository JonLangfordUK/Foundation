# Foundation Editor Library Tracker

## Metadata
- Feature slug: `foundation-editor-library`
- Feature area: `multi-area`
- Primary area: `editor`
- Branch: `feature/foundation-editor-library`
- Overall status: `Implementation in progress`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation in progress with gpt-5.4`
- Created: `2026-06-21`
- Last updated: `2026-06-21`

## Branch And Push State
- Active planning branch: `feature/foundation-editor-library`
- Branch base: created locally from `dev` on `2026-06-21`; `dev` verified as an ancestor of `HEAD`.
- Remote: `origin` configured at `https://github.com/JonLangfordUK/Foundation.git`.
- Push status: Planning docs commit `e12d73b` pushed to `origin/feature/foundation-editor-library`; implementation commits pending.
- Pre-existing local changes: `games/template-game/.jsn/project.jsn` was already modified by local editor use before this feature. Do not commit it unless explicitly requested.

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
**Status:** In progress  
**Goal:** Rename the existing reusable runtime crate from `foundation-library` to `foundation-runtime-library` without changing behavior.

### Tasks
- [x] Verify implementation branch and protect pre-existing editor-local project state.
  - Status: Complete
  - Notes: Confirmed active branch `feature/foundation-editor-library`; verified `dev` is an ancestor of `HEAD`; `games/template-game/.jsn/project.jsn` remains a pre-existing local modification and must stay out of commits.
- [x] Move `crates/foundation-library` to `crates/foundation-runtime-library`.
  - Status: Awaiting full validation
  - Notes: Moved the crate directory, updated root workspace membership, changed the package name to `foundation-runtime-library`, and regenerated the workspace lock entry through `cargo check`.
- [x] Rename Rust references from `foundation_library` to `foundation_runtime_library` where a real crate rename is used.
  - Status: Awaiting full validation
  - Notes: Updated TemplateGame Rust imports and active runtime docs; focused `cargo check -p foundation-runtime-library -p template-game --all-features` passed.
- [x] Migrate `.jsn` serialized component type paths from `foundation_library::...` to `foundation_runtime_library::...` if the implementation changes the Rust crate name.
  - Status: Awaiting full validation
  - Notes: Updated TemplateGame `.jsn` assets so reflected Foundation runtime components use the new crate path.
- [x] Update active documentation references for the runtime crate rename.
  - Status: Awaiting full validation
  - Notes: Updated README and `docs/scene-system.md` for the runtime crate name. Broader runtime/editor split docs and skill updates remain planned for Phase 6.

### Validation
- Format: Passed (`scripts/format-project.cmd`, 2026-06-21)
- Lint: Pending full feature validation
- Tests: Focused pass (`cargo test -p foundation-runtime-library`; `cargo test -p template-game --lib --all-features`, 2026-06-21)
- Build: Focused pass (`cargo check -p foundation-runtime-library -p template-game --all-features`, 2026-06-21)
- Documentation generation: Focused pass (`cargo doc -p foundation-runtime-library --no-deps`, 2026-06-21)
- Full validation wrapper: Pending full feature validation
- User confirmation: Not required after implementation approval

## Phase 2: Foundation Editor Library Crate
**Status:** Planned  
**Goal:** Add a dedicated editor library crate that owns Jackdaw editor extension code and depends on the runtime library.

### Tasks
- [ ] Add `crates/foundation-editor-library` as a workspace member.
  - Status: Planned
  - Notes: Package name should be `foundation-editor-library`; Rust import path should be `foundation_editor_library`.
- [ ] Create documented editor library baseline API.
  - Status: Planned
  - Notes: Expose a plugin/extension entry point suitable for game-specific Jackdaw editor binaries.
- [ ] Add dependencies without leaking full `jackdaw` into the runtime crate.
  - Status: Planned
  - Notes: Editor crate may depend on `bevy`, `jackdaw`, and `foundation-runtime-library`.
- [ ] Add non-window tests for editor extension metadata or API shape where practical.
  - Status: Planned
  - Notes: Avoid GPU/window launch in automated tests.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Not required after implementation approval

## Phase 3: Shared Game Settings Model And Persistence
**Status:** Planned  
**Goal:** Introduce reusable settings data for startup map and editor startup map, with persistence if feasible.

### Tasks
- [ ] Add a runtime/shared game settings type.
  - Status: Planned
  - Notes: Preferred owner is `foundation-runtime-library`; expected fields include `startup_map` and `editor_startup_map`.
- [ ] Define default settings and fallback behavior.
  - Status: Planned
  - Notes: Defaults should preserve TemplateGame's current startup behavior until the user changes settings.
- [ ] Implement project-local settings persistence or record a user-approved deferral.
  - Status: Planned
  - Notes: Proposed file name is `foundation.settings.toml` in the game project root.
- [ ] Add parsing/default/fallback tests.
  - Status: Planned
  - Notes: Tests should not require launching Jackdaw or Bevy windows.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Not required after implementation approval

## Phase 4: Game Settings Jackdaw Window
**Status:** Planned  
**Goal:** Add a reusable dockable Jackdaw Game Settings window in the editor library.

### Tasks
- [ ] Implement a Jackdaw extension for Foundation game settings.
  - Status: Planned
  - Notes: Use public `JackdawExtension`, `ExtensionContext`, and `WindowDescriptor` APIs.
- [ ] Register a dockable `Game Settings` window.
  - Status: Planned
  - Notes: Window should show startup map and editor startup map values.
- [ ] Add minimal update actions/operators.
  - Status: Planned
  - Notes: Prefer robust baseline actions such as using the currently open scene for startup/editor startup map, plus save/reload if persistence exists.
- [ ] Keep the UI simple and documented.
  - Status: Planned
  - Notes: Avoid private Jackdaw internals; defer rich text editing if public API friction is high.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Not required after implementation approval

## Phase 5: TemplateGame Integration
**Status:** Planned  
**Goal:** Wire the renamed runtime library and new editor library into TemplateGame without regressing standalone or editor Play behavior.

### Tasks
- [ ] Update TemplateGame dependencies and imports.
  - Status: Planned
  - Notes: Runtime dependency should use `foundation-runtime-library`; editor dependency should be behind the `editor` feature.
- [ ] Register Foundation editor extension/plugin from `games/template-game/src/bin/editor.rs`.
  - Status: Planned
  - Notes: Use Jackdaw `EditorPlugins` extension registration pattern.
- [ ] Use configured startup map in standalone startup where practical.
  - Status: Planned
  - Notes: Preserve current fallback flow when settings are missing or invalid.
- [ ] Use configured editor startup map according to documented precedence.
  - Status: Planned
  - Notes: Current open known scene should likely remain highest precedence for designer workflow; setting is fallback/default.
- [ ] Add/update tests for startup map resolution and editor feature composition.
  - Status: Planned
  - Notes: Avoid tests that require opening the editor window.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Not required after implementation approval

## Phase 6: Documentation, Skills, Validation, And Handoff
**Status:** Planned  
**Goal:** Complete docs and Pi workflow guidance, validate the workspace, commit/push checkpoints, and prepare for optional review.

### Tasks
- [ ] Update README and architecture docs for the runtime/editor Foundation split.
  - Status: Planned
  - Notes: Include crate names, commands, dependency guidance, and settings window usage.
- [ ] Update project instructions and skills for the new Foundation editor area.
  - Status: Planned
  - Notes: Update `AGENTS.md` and relevant `.pi/skills/*` guidance so future work distinguishes `foundation-runtime-library` runtime/game systems from `foundation-editor-library` Jackdaw editor windows/extensions.
- [ ] Run full project validation.
  - Status: Planned
  - Notes: Use `scripts/validate-project.cmd` unless a documented blocker requires focused validation plus waiver.
- [ ] Manually smoke-test the TemplateGame editor settings window if practical.
  - Status: Planned
  - Notes: Launch editor and verify the window appears without closing the editor.
- [ ] Commit and push completed work following gitflow rules.
  - Status: Planned
  - Notes: Each completed task/phase should be committed and pushed to `origin` when available.
- [ ] Update tracker with validation evidence, push state, and handoff notes.
  - Status: Planned
  - Notes: Do not mark phases complete without validation or a recorded waiver.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending final feature acceptance

## Implementation / Review Handoff Notes
- Implementation model: `gpt-5.4`.
- Review model: `gpt-5.5`.
- Never use Anthropic models.
- Read `.pi/skills/feature-tracker-update/SKILL.md` before implementation starts.
- Read `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, and `.pi/skills/gitflow-workflow/SKILL.md` before editing.
- Update `AGENTS.md` and relevant project skills during implementation because this feature creates a new Foundation editor area.
- Keep `foundation-runtime-library` free of full `jackdaw` editor dependency.
- Put Jackdaw extension/window code in `foundation-editor-library`.
- Keep runtime/shared settings data available to standalone games through `foundation-runtime-library`.
- Watch for `.jsn` serialized component type paths during the runtime crate rename.
- Do not commit `games/template-game/.jsn/project.jsn` unless explicitly requested.

## Postponed Work
- Native modal settings dialog: postponed in favor of a dockable Jackdaw extension window.
- Rich text-entry settings UI: may be postponed if public Jackdaw text-edit APIs are not straightforward; button-driven baseline is acceptable.
- Generic `crates/jackdaw-editor` launcher integration: postponed unless the user wants generic projects to load the Foundation editor library. Initial integration should target game-specific editor binaries.
- Broad settings categories beyond startup map and editor startup map: postponed until the baseline settings window exists.

## Issues / Oversights Discovered
- None yet.

## Progress Log
- `2026-06-21`: Read feature planning, gitflow, Rust workspace, and Rust coding standards skills.
- `2026-06-21`: Inspected workspace manifests, runtime Foundation crate, TemplateGame standalone/editor entry points, Jackdaw extension examples/API, current git branch, remotes, and dirty local state.
- `2026-06-21`: Created branch `feature/foundation-editor-library` from `dev` and verified `dev` is an ancestor.
- `2026-06-21`: Created plan and tracker for the Foundation runtime/editor library split and Game Settings window feature.
- `2026-06-21`: User approved the plan direction and asked to also update project skills because this creates a new area; plan and tracker updated to include `AGENTS.md` and relevant `.pi/skills/*` updates.
- `2026-06-21`: Planning docs committed as `e12d73b` and pushed to `origin/feature/foundation-editor-library`; implementation started with gpt-5.4.
- `2026-06-21`: Completed the runtime crate rename from `foundation-library`/`foundation_library` to `foundation-runtime-library`/`foundation_runtime_library`, updated TemplateGame Rust imports and `.jsn` type paths, and recorded focused validation passes.
