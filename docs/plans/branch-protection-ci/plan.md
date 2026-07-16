# Branch Protection CI Plan

## Metadata
- Feature slug: `branch-protection-ci`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/branch-protection-ci`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-07-15`
- Last updated: `2026-07-15`

## User Request
The repository is public and needs protected `dev` and `main` branches. Changes into both branches should go through pull requests, should be validated by the self-hosted runner before merge, and should not create release tags or GitHub Releases while testing pull requests.

## Feature Summary
Add pull-request validation coverage to the existing Foundation Build workflow so GitHub branch protection can require runner-backed checks before merging to `dev` or `main`. Keep release creation limited to direct push events on protected branches after merges complete.

## Feature Area Classification
- Area: `multi-area`
- Primary area: `engine`
- Rationale: This is workflow/tooling infrastructure for the Foundation engine repository. It does not change runtime game or editor behavior, but the existing taxonomy has no standalone CI/tooling area.

## Codebase Research
- `.github/workflows/foundation-build.yml` currently runs on `push` to `dev` and `main`, plus manual `workflow_dispatch`.
- The workflow already separates release jobs with `if: github.event_name == 'push' && github.ref == 'refs/heads/dev'` and `if: github.event_name == 'push' && github.ref == 'refs/heads/main'`, so release jobs will be skipped for pull request events.
- The workflow validates the Rust workspace, packages Windows `test` and `shipping` builds, uploads artifacts, and uses a workflow-only `CARGO_TARGET_DIR` for self-hosted runner build reuse.
- `Cargo.toml` is a workspace with Foundation crates and `games/template-game`; no Rust code changes are expected for pull-request branch protection support.

## External Research
No external online research was performed because the required behavior follows existing GitHub Actions trigger and job-condition semantics already used by this workflow.

## Affected Files And Systems
- `.github/workflows/foundation-build.yml`: add pull request triggers for `dev` and `main` so branch protection can require the workflow checks before merge.
- `docs/`: document the branch protection setup and required checks if useful.
- GitHub repository settings: configure branch protection/rulesets for `dev` and `main` to require pull requests and successful workflow checks.

## Proposed Implementation Approach
1. Add a `pull_request` trigger for branches `dev` and `main` to `.github/workflows/foundation-build.yml`.
2. Keep release jobs guarded by push-only conditions so pull requests validate/package but do not tag or publish releases.
3. Add a pull request source-branch policy check so `main` accepts pull requests only from `dev` or `hotfix/*` branches.
4. Consider whether package artifacts should still upload for PR runs. The default plan keeps uploads because they prove packaging output exists and help inspection.
5. Update docs with the exact GitHub branch protection/ruleset settings and expected required check names.
6. Validate the workflow syntax and run the standard project validation wrapper if practical.

## Alternatives Considered
- Separate PR validation workflow: More explicit but duplicates the existing validation/package matrix and can drift from release packaging behavior.
- Disable package jobs on PRs and only validate Rust: Faster, but weaker; it would not prove packaging still works before merge.
- Use hard-coded release blocking elsewhere: Not needed because release jobs already require `github.event_name == 'push'`.

## Risks, Constraints, And Assumptions
- Branch protection required check names must match GitHub's reported job names after the pull request workflow runs at least once.
- Self-hosted runner availability will gate pull request merges when checks are required.
- Public repositories with self-hosted runners should be careful with untrusted pull requests. If outside contributors are expected, GitHub approval policies for self-hosted runner use should remain enabled.
- Package jobs on pull requests may take time, but they provide the strongest pre-merge signal.

## Open Questions
- Should PR checks require both packaging matrix entries (`test` and `shipping`) or only validation plus one package configuration? Default recommendation: require both for now.
- Should the repository use classic branch protection rules or GitHub rulesets? Default recommendation: use rulesets if available, otherwise classic branch protection is acceptable.

## Documentation Expectations
- No public Rust APIs are expected to change.
- Add or update docs for branch protection setup if implementation proceeds.
- Generated Rust documentation should be produced before the feature is considered complete, unless waived because only workflow/docs changed.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Do not alter release versioning or publishing conditions except to preserve push-only release behavior.
- Do not implement direct commits to `dev` or `main`; commit on `feature/branch-protection-ci` and merge through `dev` when approved.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Confirm pull request events cannot create tags/releases.
- Confirm `main` pull requests from branches other than `dev` or `hotfix/*` fail the source-branch policy check.
- Confirm branch protection setup instructions match the actual workflow job names.

## Success Criteria
- Pull requests targeting `dev` or `main` trigger the Foundation Build workflow.
- Pull request checks validate and package via the Windows self-hosted runner.
- Pull request checks do not create tags, prereleases, or releases.
- Pull requests into `main` are allowed only from `dev` or `hotfix/*` branches.
- Pushes to `dev` still create `0.0.#` prereleases after successful packaging.
- Pushes to `main` still create `0.#.0` releases after successful packaging.
- GitHub branch protection can require pull requests and successful runner checks before merging to `dev` or `main`.

## Testing Methodology
- Inspect workflow trigger and job conditions.
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- `scripts/doc-project.cmd`
- `scripts/validate-project.cmd` for the full validation sequence when practical
- Open a test pull request and confirm only validation/package jobs run; release jobs should be skipped.
