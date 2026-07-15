# Branch Protection

This repository protects `dev` and `main` by requiring pull requests and runner-backed checks before merge.

## Workflow behavior

The `Foundation Build` workflow runs for:

- Pull requests targeting `dev` or `main`.
- Pushes to `dev` or `main`.
- Manual workflow dispatch.

Pull request runs validate and package the project, but they do not publish GitHub releases or create version tags. Release jobs are guarded so they only run for push events on protected branches:

- Pushes to `dev` create `0.0.#` prereleases.
- Pushes to `main` create `0.#.0` releases.

## Recommended GitHub branch protection

Configure protection for both `dev` and `main` in GitHub.

Use either GitHub Rulesets or classic Branch protection rules. Rulesets are preferred when available.

### Required settings

For each branch pattern, `dev` and `main`:

1. Require a pull request before merging.
2. Require status checks to pass before merging.
3. Require branches to be up to date before merging.
4. Block force pushes.
5. Block deletions.
6. Require conversation resolution before merging.

### Recommended pull request settings

For a solo-maintained public repository, use these as a practical baseline:

- Required approving reviews: `0` or `1`, depending on whether external review is desired.
- Dismiss stale pull request approvals when new commits are pushed: enabled if reviews are required.
- Require review from Code Owners: disabled unless CODEOWNERS is added later.
- Allow bypass by repository administrators: optional. Disable bypass for strict enforcement; enable it only if emergency maintenance requires it.

### Required status checks

After the first pull request run, GitHub will expose the exact check names. Require the checks from the `Foundation Build` workflow, including:

- `Main source branch policy`
- `Validate workspace on Windows`
- `Package windows-x64 test on Windows`
- `Package windows-x64 shipping on Windows`

If GitHub displays matrix job names with extra suffixes, select the exact names shown in the pull request checks list.

The `Main source branch policy` check enforces that pull requests into `main` come only from `dev` or `hotfix/*` branches. Pull requests into `dev` are allowed from normal feature branches. Validation and packaging jobs depend on this policy check, so invalid `main` pull request sources fail before expensive runner validation starts.

## Self-hosted runner caution

This public repository uses a self-hosted Windows runner. Keep GitHub's approval controls for outside contributors enabled so untrusted pull request code cannot run on the self-hosted machine without approval.

For public repositories, review GitHub's Actions settings under:

```text
Settings -> Actions -> General
```

Recommended policy:

- Require approval for first-time contributors before running workflows.
- Do not automatically run workflows from untrusted fork pull requests on the self-hosted runner.

## Expected merge flow

1. Create work on a `feature/*` or `hotfix/*` branch.
2. Push the branch to `origin`.
3. Open a pull request into `dev`.
4. Wait for `Foundation Build` checks to pass.
5. Merge the pull request into `dev`.
6. The push to `dev` runs packaging again and creates the next `0.0.#` prerelease.
7. Promote `dev` to `main` through a pull request when ready.
8. Use `hotfix/*` pull requests into `main` only for urgent release fixes.
9. The push to `main` creates the next `0.#.0` release.
