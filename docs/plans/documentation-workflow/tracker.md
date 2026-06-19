# Documentation Workflow Tracker

## Metadata
- Feature slug: `documentation-workflow`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/documentation-workflow`
- Overall status: `Complete`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for gpt-5.5 sanity review`
- Created: `2026-06-19`
- Last updated: `2026-06-19`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.

## Phase 1: Documentation workflow enforcement
**Status:** Complete  
**Goal:** Add documentation generation to the mandatory development workflow and require feature area classification during planning.

### Tasks
- [x] Update skills to require feature area classification and documentation generation.
  - Status: Complete
  - Notes: Updated Rust, planning, tracker, and review skills.
- [x] Add documentation generation script support.
  - Status: Complete
  - Notes: Added `scripts/doc-project.cmd` and `doc-project` handling in `scripts/Invoke-RustWorkspace.ps1`.
- [x] Commit and push initial workflow changes.
  - Status: Complete
  - Notes: Commit `cfaff90 Add documentation workflow` pushed to `origin/feature/documentation-workflow`.

### Validation
- Format: Passed via `scripts/format-project.cmd` on 2026-06-19
- Lint: Passed via `scripts/lint-project.cmd` on 2026-06-19
- Tests: Passed via `scripts/test-project.cmd` on 2026-06-19
- Build: Passed via `scripts/compile-project.cmd` on 2026-06-19
- Documentation generation: Passed via `scripts/doc-project.cmd` on 2026-06-19
- User confirmation: Received approval to continue workflow hardening on 2026-06-19

### Notes
- This phase was started before plan/tracker documents existed. The process issue is corrected by this retrospective plan/tracker and the follow-up consistency pass.

## Phase 2: Pipeline consistency hardening
**Status:** Complete  
**Goal:** Ensure every workflow entry point matches the updated rules and provide a one-command full validation wrapper.

### Tasks
- [x] Update `AGENTS.md` with documentation validation requirements.
  - Status: Complete
  - Notes: Added `scripts/doc-project.cmd` and `scripts/validate-project.cmd` to standard workflow guidance.
- [x] Update prompt files to mention feature area and documentation validation.
  - Status: Complete
  - Notes: Updated planning, implementation, review, and scaffold prompts.
- [x] Update planning templates to include feature area and documentation validation.
  - Status: Complete
  - Notes: Updated plan and tracker templates used by scaffold script.
- [x] Extend `scripts/Scaffold-FeaturePlan.ps1` for feature area placeholders.
  - Status: Complete
  - Notes: Added `FeatureArea` and `PrimaryArea` parameters and template replacements.
- [x] Add full validation wrapper.
  - Status: Complete
  - Notes: Added `scripts/validate-project.cmd` and `validate-project` handling in `scripts/Invoke-RustWorkspace.ps1`.
- [x] Run full validation and record results.
  - Status: Complete
  - Notes: `scripts/validate-project.cmd` passed on 2026-06-19.
- [x] Commit and push consistency hardening changes.
  - Status: Complete
  - Notes: Commit `b4e8fe7 Harden documentation workflow` pushed to `origin/feature/documentation-workflow`.

### Validation
- Format: Passed via `scripts/validate-project.cmd` on 2026-06-19
- Lint: Passed via `scripts/validate-project.cmd` on 2026-06-19
- Tests: Passed via `scripts/validate-project.cmd` on 2026-06-19
- Build: Passed via `scripts/validate-project.cmd` on 2026-06-19
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-06-19
- Full validation wrapper: Passed on 2026-06-19
- User confirmation: Received approval to make fixes on 2026-06-19

### Notes
- The workflow now treats generated documentation as a completion gate, not just an optional artifact.

## Implementation / Review Handoff Notes
- Keep `AGENTS.md`, `.pi/skills`, `.pi/prompts`, `docs/plans/_templates`, and scripts synchronized.
- Use `scripts/validate-project.cmd` as the preferred final validation command.
- Optional final review should focus on contradictions or stale workflow entry points.

## Postponed Work
- Public Rust API missing-doc enforcement using crate attributes such as `#![warn(missing_docs)]` is postponed until the Bevy workspace crates are created.

## Progress Log
- `2026-06-19`: Initial documentation workflow changes committed and pushed as `cfaff90`.
- `2026-06-19`: User approved full workflow consistency review and hardening.
- `2026-06-19`: Retrospective plan/tracker created to bring the branch into compliance with the feature workflow.
- `2026-06-19`: Full validation passed with `scripts/validate-project.cmd`.
- `2026-06-19`: Consistency hardening committed as `b4e8fe7` and pushed to `origin/feature/documentation-workflow`.
- `2026-06-19`: Final full validation passed with `scripts/validate-project.cmd` after tracker/plan completion updates.
