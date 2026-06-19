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
- Current handoff state: `Accepted without further review changes`
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

## Review Findings
- `2026-06-19` - gpt-5.5 sanity review
  - Overall result: Pass with concerns; the workflow is usable, but not yet rock-solid.
  - Validation evidence: `scripts/validate-project.cmd` passed during review on 2026-06-19.
  - Must-fix before calling the pipeline rock-solid:
    - `feature-plan-docs` still has weaker completion wording than `feature-tracker-update`: its validation rules say tasks/phases complete after required validation or waiver, but do not explicitly repeat that documentation generation is part of task/phase completion. This is likely intended through the validation list, but the language is less explicit than the tracker skill and template.
    - `scripts/Scaffold-FeaturePlan.ps1` allows default placeholder feature areas (`<Feature Area>` / `<Primary Area>`), so manual use can generate non-compliant planning documents even though the workflow says the area must be confirmed.
    - `feature-plan-docs` documents the scaffold helper as `scripts/scaffold-feature-plan.cmd <feature-slug> [feature-name] [branch-name]`, but the script now accepts feature area and primary area arguments too. The usage docs are stale.
    - The current branch tracker marks phase completion based on approval to make fixes, not a distinct post-validation user confirmation that the phase is suitable. That technically violates the stated phase completion rule.
  - Optional improvements:
    - Add `scripts/validate-project.cmd` to template testing methodology so new plans prefer the one-command full gate as well as individual wrappers.
    - Add validation in `Scaffold-FeaturePlan.ps1` for feature slug format, branch name format, and feature area/primary area consistency.
    - Consider a `workflow` or `infrastructure` feature area, or explicitly document that workflow-only changes use `multi-area` with a primary area rationale.
    - Add CI/branch protection later; skills guide the agent but do not enforce the pipeline outside Pi.
  - User decision: `Send to gpt-5.4 for fixes` on 2026-06-19

## Phase 3: Review fix pass
**Status:** Complete  
**Goal:** Fix review findings that prevented the workflow from being considered rock-solid.

### Tasks
- [x] Strengthen `feature-plan-docs` completion wording for documentation generation.
  - Status: Complete
  - Notes: Task and phase completion now explicitly mention documentation generation.
- [x] Require concrete feature area metadata in the scaffold script.
  - Status: Complete
  - Notes: The scaffold script now rejects placeholder feature areas and validates slug, branch, and primary-area consistency.
- [x] Update scaffold usage documentation.
  - Status: Complete
  - Notes: The helper signature now documents feature area and primary area arguments.
- [x] Add full validation wrapper expectations to templates.
  - Status: Complete
  - Notes: Plan/tracker templates now reference `scripts/validate-project.cmd` where appropriate.

### Validation
- Format: Passed via `scripts/validate-project.cmd` on 2026-06-19
- Lint: Passed via `scripts/validate-project.cmd` on 2026-06-19
- Tests: Passed via `scripts/validate-project.cmd` on 2026-06-19
- Build: Passed via `scripts/validate-project.cmd` on 2026-06-19
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-06-19
- Full validation wrapper: Passed on 2026-06-19
- User confirmation: Confirmed suitable by user on 2026-06-19.

### Notes
- Scaffold script smoke checks were run for invalid slug rejection and valid scaffold creation/removal.

## Review Findings
- `2026-06-19` - gpt-5.5 sanity review
  - Overall result: Pass with concerns; improved substantially, but still not rock-solid.
  - Validation evidence: `scripts/validate-project.cmd` passed during review on 2026-06-19.
  - Must-fix before calling the pipeline rock-solid:
    - `feature-plan-docs` suggested plan metadata still omits a separate `Primary area` field even though the actual template and tracker use one. This is a documentation/template drift risk.
    - `feature-plan-docs` scaffold helper usage is confusing: it shows required `<feature-area> <primary-area>` after optional `[feature-name] [branch-name]`. Positional command usage cannot cleanly require later arguments after optional earlier ones without making users provide all earlier arguments.
    - `scripts/Scaffold-FeaturePlan.ps1` accepts `hotfix/*` branch names, but `feature-plan-docs` says every feature must use `feature/*`. This is a direct policy/script mismatch.
    - `AGENTS.md` has an ambiguous implementation entry point: the “plan a new feature or begin implementing one” section says read `feature-plan-docs` first, while the later implementation section says read `feature-tracker-update` first. The enforcement rule says both are mandatory, but the workflow order is muddy.
  - Optional improvements:
    - Add `scripts/validate-project.cmd` to `feature-plan-docs` default validation list, not only the suggested testing methodology.
    - Add explicit branch base verification guidance before implementation, e.g. verify branch was created from current `dev` or record if that cannot be proven.
    - Add CI/branch protection later; current enforcement is agent-process-only.
  - User decision: `Send to gpt-5.4 for fixes` on 2026-06-19

## Phase 4: Follow-up review fix pass
**Status:** Complete  
**Goal:** Fix the remaining policy/script/documentation mismatches found in the follow-up review.

### Tasks
- [x] Align suggested plan metadata with the real templates.
  - Status: Complete
  - Notes: `feature-plan-docs` suggested plan metadata now includes a separate primary area field.
- [x] Clarify scaffold helper usage and make positional arguments concrete.
  - Status: Complete
  - Notes: The helper documentation now requires all positional arguments, and the PowerShell script marks feature name, branch name, feature area, and primary area as mandatory.
- [x] Remove `hotfix/*` acceptance from feature planning scaffold validation.
  - Status: Complete
  - Notes: `scripts/Scaffold-FeaturePlan.ps1` now requires `feature/*` branches for feature planning docs.
- [x] Clarify implementation entry point ordering.
  - Status: Complete
  - Notes: `AGENTS.md` now separates planning from implementation and states what to do when approved planning docs do or do not exist.
- [x] Add optional improvement fixes.
  - Status: Complete
  - Notes: Added full validation wrapper guidance to `feature-plan-docs`, branch-base verification guidance to implementation workflow, and explicit workflow/tooling area guidance to the planning prompt.
- [x] Smoke test scaffold validation.
  - Status: Complete
  - Notes: Valid feature scaffold succeeded and was removed; hotfix branch and missing-argument invocations were rejected.

### Validation
- Format: Passed via `scripts/validate-project.cmd` on 2026-06-19
- Lint: Passed via `scripts/validate-project.cmd` on 2026-06-19
- Tests: Passed via `scripts/validate-project.cmd` on 2026-06-19
- Build: Passed via `scripts/validate-project.cmd` on 2026-06-19
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-06-19
- Full validation wrapper: Passed on 2026-06-19
- User confirmation: Confirmed suitable by user on 2026-06-19.

### Notes
- CI/branch protection remains postponed because this branch is focused on Pi workflow files and local validation scripts.

## Review Findings
- `2026-06-19` - gpt-5.5 sanity review
  - Overall result: Pass; no must-fix workflow inconsistencies found in this pass.
  - Validation evidence: `scripts/validate-project.cmd` passed during review on 2026-06-19.
  - Scaffold evidence: valid `multi-area` scaffold with primary `editor` succeeded and was removed.
  - Consistency evidence: searched workflow files for stale optional scaffold usage, stale hotfix feature-planning usage, compressed feature-area metadata, weak validation wording, and old format/lint/test/build-only validation wording; no hits found.
  - Must-fix: None found.
  - Optional improvements:
    - Add CI/branch protection later so the workflow is enforced outside Pi and local scripts.
    - Add public Rust API missing-doc linting when real Bevy workspace crates are created.
  - User decision: Pending

## Postponed Work
- Public Rust API missing-doc enforcement using crate attributes such as `#![warn(missing_docs)]` is postponed until the Bevy workspace crates are created.
- CI/branch protection is postponed until the repository has the first real workspace structure and a chosen CI provider configuration.

## Progress Log
- `2026-06-19`: Initial documentation workflow changes committed and pushed as `cfaff90`.
- `2026-06-19`: User approved full workflow consistency review and hardening.
- `2026-06-19`: Retrospective plan/tracker created to bring the branch into compliance with the feature workflow.
- `2026-06-19`: Full validation passed with `scripts/validate-project.cmd`.
- `2026-06-19`: Consistency hardening committed as `b4e8fe7` and pushed to `origin/feature/documentation-workflow`.
- `2026-06-19`: Final full validation passed with `scripts/validate-project.cmd` after tracker/plan completion updates.
- `2026-06-19`: User accepted review findings and requested fixes.
- `2026-06-19`: Review fix pass validation passed with `scripts/validate-project.cmd`; awaiting user confirmation that the phase is suitable.
- `2026-06-19`: User confirmed the review fix phase is suitable; workflow marked complete.
- `2026-06-19`: Follow-up review findings accepted and fixed; validation and scaffold smoke checks passed. A post-fix pass found and corrected one remaining suggested-template wording drift.
- `2026-06-19`: User confirmed the follow-up review fix phase is suitable; workflow marked complete.
- `2026-06-19`: Additional full workflow review passed with no must-fix findings; validation and scaffold checks passed.
