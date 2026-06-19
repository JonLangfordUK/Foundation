# Jackdaw Editor Integration Tracker

## Metadata
- Feature slug: `jackdaw-editor-integration`
- Feature area: `editor`
- Primary area: `editor`
- Branch: `feature/jackdaw-editor-integration`
- Overall status: `Planned`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for user review before gpt-5.4 implementation`
- Created: `2026-06-19`
- Last updated: `2026-06-19`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- Push after every commit and merge checkpoint when `origin` is configured. If push fails, record the failure and do not treat the checkpoint as complete until remediated.

## Branch And Push State
- Active planning branch: `feature/jackdaw-editor-integration`
- Branch creation: Created locally from `dev` on 2026-06-19 during planning.
- Branch-base verification: Pending; implementation must verify `dev` is an ancestor before code edits.
- Remote: `origin` is configured as `https://github.com/JonLangfordUK/Foundation.git`.
- Push status: Pending; no feature commits have been made yet.

## Phase 1: Rename Editor Subproject To PillarEditor
**Status:** Planned  
**Goal:** Rename the current editor subproject and user-facing editor identity to PillarEditor while preserving Cargo compatibility.

### Tasks
- [ ] Rename the editor workspace member path from `crates/editor` to a PillarEditor path.
  - Status: Planned
  - Notes: Proposed path is `crates/pillar-editor` unless implementation records a reason to prefer `crates/pillar_editor`.
- [ ] Rename the editor package from `pigame-editor` to `pillar-editor`.
  - Status: Planned
  - Notes: Use lowercase hyphenated package naming for Cargo compatibility; product/window title should be `PillarEditor` exactly.
- [ ] Update workspace membership and dependencies for the renamed editor package.
  - Status: Planned
  - Notes: Root `Cargo.toml` must refer to the new path/package.
- [ ] Update README and run commands for the new editor name.
  - Status: Planned
  - Notes: Proposed command is `cargo run -p pillar-editor`.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending before implementation starts

### Notes
- Cargo package/executable naming may not match the exact `PillarEditor` capitalization. The plan treats `PillarEditor` as the user-facing product/window title and `pillar-editor` as the Cargo package name unless the user asks for an explicitly capitalized binary target.

## Phase 2: Jackdaw Editor Host Integration
**Status:** Planned  
**Goal:** Replace the plain Bevy editor window with a Jackdaw-backed editor host.

### Tasks
- [ ] Add Jackdaw dependencies to the editor crate/workspace.
  - Status: Planned
  - Notes: Prefer `jackdaw = { version = "0.4", default-features = false }` initially; add `ctrlc` and/or `jackdaw_runtime` if implementation needs them.
- [ ] Adjust Bevy feature configuration if needed for Jackdaw.
  - Status: Planned
  - Notes: Jackdaw docs reference Bevy 0.18 plus `file_watcher` and `reflect_documentation`; record any feature additions.
- [ ] Add a minimal game plugin if needed for editor hosting.
  - Status: Planned
  - Notes: Proposed `PiGamePlugin` in `crates/game/src/lib.rs`; avoid moving ambient plugins into the game plugin.
- [ ] Implement PillarEditor app startup with Jackdaw plugins.
  - Status: Planned
  - Notes: Add `DefaultPlugins`, `PhysicsPlugins::default()`, `EnhancedInputPlugin`, `EditorPlugins::default()`, and the game plugin in an order consistent with Jackdaw docs.
- [ ] Preserve the game launcher.
  - Status: Planned
  - Notes: `cargo run -p pigame-game` should continue to build/run separately.
- [ ] Add non-interactive tests where practical.
  - Status: Planned
  - Notes: Prefer tests for naming/configuration; do not require opening the editor window in automated tests.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Manual game window check: Pending
- Manual PillarEditor Jackdaw launch check: Pending
- Full validation wrapper: Pending
- User confirmation: Pending / Not required until phase review

### Notes
- Jackdaw is pre-1.0 and can add significant dependency/build complexity. Any compile/runtime issue should be recorded here with exact error text and remediation.

## Phase 3: Documentation, Validation, And Checkpoints
**Status:** Planned  
**Goal:** Document the Jackdaw/PillarEditor workflow and complete validation/commit/push checkpoints.

### Tasks
- [ ] Update README with PillarEditor and Jackdaw usage.
  - Status: Planned
  - Notes: Include `cargo run -p pillar-editor` and clarify that PillarEditor uses Jackdaw.
- [ ] Run required validation wrappers.
  - Status: Planned
  - Notes: Use project scripts where practical; known wrapper quirk may require direct PowerShell invocation.
- [ ] Generate documentation.
  - Status: Planned
  - Notes: Use `scripts/doc-project.cmd` or direct cargo doc equivalent.
- [ ] Commit completed tasks/phases and push to `origin`.
  - Status: Planned
  - Notes: Follow `.pi/skills/gitflow-workflow/SKILL.md` commit format and remote backup rules.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- Push state: Pending
- User confirmation: Pending / Not required until implementation review

### Notes
- Phase completion requires validation evidence or documented user-approved waivers.

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Read `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/feature-plan-docs/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/gitflow-workflow/SKILL.md`, `plan.md`, and this tracker before implementation edits.
- Confirm active branch is `feature/jackdaw-editor-integration` before editing.
- Verify `dev` is an ancestor of the feature branch before code edits and record the result.
- Treat `PillarEditor` as the user-facing editor name.
- Proposed Cargo package/run command: `pillar-editor` / `cargo run -p pillar-editor`.
- Keep Jackdaw integration static/minimal first; do not enable Jackdaw `dylib` unless explicitly justified.

## Postponed Work
- Dynamic Jackdaw extension/dylib loading is postponed unless the user explicitly asks for hot-reloadable editor extensions.
- Full game scene runtime integration and authored `assets/scene.jsn` loading may be postponed if not required to open and validate PillarEditor with Jackdaw.
- Advanced custom components, editor panels, and PillarEditor-specific branding assets/icons are postponed unless needed for the initial Jackdaw host.

## Progress Log
- `2026-06-19`: Read required feature planning, Rust workspace, and Gitflow skills.
- `2026-06-19`: Inspected root workspace manifest, current editor manifest/source, engine launcher code, game crate, README, Git branch state, and remotes.
- `2026-06-19`: Created branch `feature/jackdaw-editor-integration` from local `dev` for feature planning.
- `2026-06-19`: Researched Jackdaw 0.4.1 README, docs, migration guidance, custom component guidance, configuration docs, and source exports at commit `5e20671dbf6851180b9e032ee95039cdc1adab4d`.
- `2026-06-19`: Plan and tracker created; awaiting user review/approval before implementation.
