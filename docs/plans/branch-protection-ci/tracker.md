# Branch Protection CI Tracker

## Metadata
- Feature slug: `branch-protection-ci`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/branch-protection-ci`
- Overall status: `Planned; awaiting user approval to implement`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for gpt-5.4 implementation after user approval`
- Created: `2026-07-15`
- Last updated: `2026-07-15`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- Pull request validation must not create Git tags, prereleases, or releases.

## Phase 1: Pull Request Workflow Support
**Status:** Planned  
**Goal:** Ensure pull requests targeting protected branches run the same runner-backed validation and package checks without releasing.

### Tasks
- [ ] Add `pull_request` triggers for `dev` and `main` to `.github/workflows/foundation-build.yml`.
  - Status: Planned
  - Notes: Release jobs already have push-only `if` conditions and should remain skipped for PR events.
- [ ] Confirm package matrix behavior is appropriate for PRs.
  - Status: Planned
  - Notes: Default plan keeps both `test` and `shipping` package jobs so branch protection proves packaged outputs before merge.
- [ ] Validate workflow behavior with inspection and, ideally, a test PR.
  - Status: Planned
  - Notes: GitHub branch protection check names should be copied from the first successful PR run.

### Validation
- Format: Not run
- Lint: Not run
- Tests: Not run
- Build: Not run
- Documentation generation: Not run
- Pull request workflow run: Not run
- User confirmation: Pending

## Phase 2: Documentation And GitHub Setup Guidance
**Status:** Planned  
**Goal:** Provide clear setup steps for protecting `dev` and `main` in GitHub.

### Tasks
- [ ] Document branch protection or ruleset settings.
  - Status: Planned
  - Notes: Include pull request requirement, required status checks, stale review dismissal recommendation, and admin bypass recommendation.
- [ ] Document release behavior distinction between PR checks and protected-branch push releases.
  - Status: Planned
  - Notes: PR checks should validate/package only; merged pushes to `dev`/`main` should continue release tagging.

### Validation
- Format: Not run
- Lint: Not run
- Tests: Not run
- Build: Not run
- Documentation generation: Not run
- User confirmation: Pending

## Postponed Work
- None.

## Progress Log
- `2026-07-15`: Created planning documents for `feature/branch-protection-ci`. Identified that project changes are needed because the existing workflow only triggers on pushes to `dev`/`main` and manual dispatch, not pull requests.

## Git And Push State
- Branch created from: `origin/dev`
- Branch: `feature/branch-protection-ci`
- Plan/tracker commit: Pending
- Push state: Pending
