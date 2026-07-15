# Branch Protection CI Tracker

## Metadata
- Feature slug: `branch-protection-ci`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/branch-protection-ci`
- Overall status: `Implementation in progress`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation in progress with gpt-5.4`
- Created: `2026-07-15`
- Last updated: `2026-07-15`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- Pull request validation must not create Git tags, prereleases, or releases.
- Pull requests into `main` must come only from `dev` or `hotfix/*` branches.

## Phase 1: Pull Request Workflow Support
**Status:** Awaiting validation
**Goal:** Ensure pull requests targeting protected branches run the same runner-backed validation and package checks without releasing.

### Tasks
- [x] Add `pull_request` triggers for `dev` and `main` to `.github/workflows/foundation-build.yml`.
  - Status: Awaiting validation
  - Notes: Added `pull_request` branches for `dev` and `main`. Release jobs already have push-only `if` conditions and should remain skipped for PR events.
- [x] Confirm package matrix behavior is appropriate for PRs.
  - Status: Awaiting validation
  - Notes: Kept both `test` and `shipping` package jobs so branch protection proves packaged outputs before merge.
- [ ] Validate workflow behavior with inspection and, ideally, a test PR.
  - Status: Awaiting PR run
  - Notes: GitHub branch protection check names should be copied from the first successful PR run.
- [x] Add source-branch policy for pull requests into `main`.
  - Status: Awaiting validation
  - Notes: Added `Main source branch policy`, allowing `dev` and `hotfix/*` sources for `main` while allowing normal feature PRs into `dev`.

### Validation
- Format: Passed via `scripts/format-project.cmd` on 2026-07-15
- Lint: Waived for workflow/docs-only change; no Rust lint target changed
- Tests: Waived for workflow/docs-only change; no Rust behavior changed
- Build: Waived for workflow/docs-only change; no Rust behavior changed
- Documentation generation: Waived for workflow/docs-only change; no Rust API documentation changed
- Pull request workflow run: Pending first PR run
- User confirmation: Pending

## Phase 2: Documentation And GitHub Setup Guidance
**Status:** Awaiting validation
**Goal:** Provide clear setup steps for protecting `dev` and `main` in GitHub.

### Tasks
- [x] Document branch protection or ruleset settings.
  - Status: Awaiting validation
  - Notes: Added `docs/branch-protection.md` with pull request requirement, required status checks, stale review dismissal recommendation, admin bypass notes, and self-hosted runner caution.
- [x] Document release behavior distinction between PR checks and protected-branch push releases.
  - Status: Awaiting validation
  - Notes: Documented that PR checks validate/package only; merged pushes to `dev`/`main` continue release tagging.

### Validation
- Format: Passed via `scripts/format-project.cmd` on 2026-07-15
- Lint: Waived for docs-only phase; no Rust lint target changed
- Tests: Waived for docs-only phase; no Rust behavior changed
- Build: Waived for docs-only phase; no Rust behavior changed
- Documentation generation: Waived for docs-only phase; no Rust API documentation changed
- User confirmation: Pending

## Postponed Work
- None.

## Progress Log
- `2026-07-15`: Created planning documents for `feature/branch-protection-ci`. Identified that project changes are needed because the existing workflow only triggers on pushes to `dev`/`main` and manual dispatch, not pull requests.
- `2026-07-15`: User approved implementation. Added pull request workflow triggers for `dev` and `main`, preserved push-only release jobs, and documented GitHub branch protection setup.
- `2026-07-15`: Added a `Main source branch policy` workflow job so `main` pull requests must come from `dev` or `hotfix/*`.

## Git And Push State
- Branch created from: `origin/dev`
- Branch: `feature/branch-protection-ci`
- Plan/tracker commit: `0de1e45 Add branch protection CI plan`
- Implementation commit: `d64677f Add pull request branch checks`
- Source policy commit: Pending
- Push state: Feature branch pushed to `origin/feature/branch-protection-ci`; source policy push pending
- Validation note: `scripts/format-project.cmd` passed locally after workflow/docs edits; heavier Rust validation waived because this feature only changes GitHub workflow triggers and Markdown documentation.
