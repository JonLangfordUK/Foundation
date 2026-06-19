# Workspace App Launchers Tracker

## Metadata
- Feature slug: `workspace-app-launchers`
- Feature area: `multi-area` (`engine`, `game`, `editor`)
- Primary area: `engine`
- Branch: `feature/workspace-app-launchers`
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
- Active planning branch: `feature/workspace-app-launchers`
- Branch creation: Created locally from `dev` on 2026-06-19 during planning.
- Remote: `origin` is configured as `https://github.com/JonLangfordUK/Foundation.git`.
- Push status: Pending; no feature commits have been made yet.
- Pre-existing working tree note: an untracked `NUL` file existed before planning and is not part of this feature unless the user later approves touching it.

## Phase 1: Workspace Structure And Shared Linking
**Status:** Planned  
**Goal:** Convert the blank template into a Cargo workspace with shared code linked by separate game and editor subprojects.

### Tasks
- [ ] Convert root `Cargo.toml` from single package to workspace manifest.
  - Status: Planned
  - Notes: Preserve useful package metadata as workspace metadata where appropriate.
- [ ] Add shared engine crate under `crates/`.
  - Status: Planned
  - Notes: Should host common app/window setup or configuration used by both launchers.
- [ ] Add game crate under `crates/` linked to the shared crate.
  - Status: Planned
  - Notes: Should be runnable with a clear package name, proposed `pigame-game`.
- [ ] Add editor crate under `crates/` linked to the shared crate.
  - Status: Planned
  - Notes: Should be runnable with a clear package name, proposed `pigame-editor`.
- [ ] Remove or obsolete root placeholder `src/lib.rs` after workspace conversion.
  - Status: Planned
  - Notes: Avoid leaving active template-only code.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending before implementation starts

### Notes
- This phase proves project structure and crate linking before visual launcher work is considered complete.

## Phase 2: Bevy Game And Editor Launchers
**Status:** Planned  
**Goal:** Add minimal Bevy launchers so the game and editor each open a distinct native window.

### Tasks
- [ ] Add Bevy dependency configuration to the workspace.
  - Status: Planned
  - Notes: Prefer defaults initially for reliable native window creation; revisit features only if compile/runtime issues arise.
- [ ] Implement shared window/app configuration helpers with Rustdoc for public APIs.
  - Status: Planned
  - Notes: Keep window-opening behavior in binaries; keep testable pure configuration in library code.
- [ ] Implement game launcher binary.
  - Status: Planned
  - Notes: Should open a Bevy game window with a distinct title.
- [ ] Implement editor launcher binary.
  - Status: Planned
  - Notes: Should open a Bevy editor window with a distinct title.
- [ ] Add non-interactive tests for shared configuration.
  - Status: Planned
  - Notes: Avoid tests that require a GPU/window.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Manual game window check: Pending
- Manual editor window check: Pending
- Full validation wrapper: Pending
- User confirmation: Pending / Not required until phase review

### Notes
- Manual checks should record commands and observed results because automated validation should not require an interactive window.

## Phase 3: Documentation, Validation, And Checkpoints
**Status:** Planned  
**Goal:** Document the new workspace and complete required validation/commit/push checkpoints.

### Tasks
- [ ] Update `README.md` with workspace layout and run commands.
  - Status: Planned
  - Notes: Include `cargo run -p pigame-game` and `cargo run -p pigame-editor` unless implementation chooses different package names and records why.
- [ ] Run required validation wrappers.
  - Status: Planned
  - Notes: Use project scripts unless a script fails for an environment reason that is recorded and approved.
- [ ] Generate documentation.
  - Status: Planned
  - Notes: Use `scripts/doc-project.cmd`.
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
- Confirm active branch is `feature/workspace-app-launchers` before editing.
- Record implementation start/resume in this tracker before code changes.
- Proposed package names: `pigame-engine`, `pigame-game`, `pigame-editor`.
- Proposed run commands: `cargo run -p pigame-game` and `cargo run -p pigame-editor`.

## Postponed Work
- Full editor UI, scene editing, asset browser, inspector, and hot-reload/dynamic game library loading are postponed. This feature only needs a minimal editor window and project linking.
- Advanced Bevy feature-flag optimization is postponed unless default Bevy setup causes a concrete issue.

## Progress Log
- `2026-06-19`: Read required feature planning, Rust workspace, and Gitflow skills.
- `2026-06-19`: Inspected `Cargo.toml`, `src/lib.rs`, `README.md`, plan templates, existing plan directories, Git branch state, and remotes.
- `2026-06-19`: Created branch `feature/workspace-app-launchers` from local `dev` for feature planning.
- `2026-06-19`: Researched Bevy `DefaultPlugins`, `WindowPlugin`, window settings, and a Bevy editor/game workspace separation example.
- `2026-06-19`: Plan and tracker created; awaiting user review/approval before implementation.
