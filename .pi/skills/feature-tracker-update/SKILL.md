---
name: feature-tracker-update
description: Update an existing docs/plans/<new-feature>/tracker.md during implementation, including progress, validation status, handoff state, notes, postponements, review findings, and completion state.
---

# Feature Tracker Update

## Purpose
Use this skill while implementing a feature to keep `docs/plans/<new-feature>/tracker.md` accurate.

The tracker is the continuity layer between `gpt-5.5` planning, `gpt-5.4` implementation, and optional `gpt-5.5` review.

## Model policy
- Use `gpt-5.4` for implementation.
- Use `gpt-5.5` for planning and review.
- Never use Anthropic models.

## Required pre-work
1. Read `docs/plans/<new-feature>/plan.md`.
2. Read `docs/plans/<new-feature>/tracker.md`.
3. Read `.pi/skills/rust-workspace-dev/SKILL.md`.
4. Read `.pi/skills/gitflow-workflow/SKILL.md` if branch, commit, or history updates are involved.
5. Confirm the active branch matches the `feature/*` branch recorded in the docs.
6. Verify the branch was created from current `dev` when possible; if it cannot be proven, record that limitation in the tracker before implementation edits.
7. Before the first implementation edit in a session, update the tracker to show implementation is starting or resuming with `gpt-5.4`.

## What to update
Update the tracker as work progresses, including:
- current task and phase status
- active implementation or review model
- format/lint/test/build validation state
- documentation generation state
- notes about issues, oversights, blockers, or discoveries
- postponed work and reasons
- progress log entries
- commit readiness, commit completion, and push state
- optional final review state and findings

## Required tracking rules
### Notes and discoveries
Record useful implementation notes, blockers, and decisions under the relevant task or phase.

### Postponements
A task or phase may be postponed only when there is a clear reason. Record the original location, new location, reason, and impact.

### Task completion
A task may only be marked complete when the required Rust validation for that task has passed and documentation generation has been recorded, unless a user-approved waiver is recorded.

Default validation:
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- documentation generation via `scripts/doc-project.cmd` when present, otherwise `cargo doc --workspace --all-features --no-deps`
- `scripts/validate-project.cmd` when present for the full validation sequence

If implementation is done but validation has not passed yet, use a non-final status such as `In progress`, `Awaiting validation`, `Blocked`, or `Postponed`.

### Phase transition
Do not start the next phase until required validation for the current phase is recorded, unless a documented feature-level waiver exists.

### Phase completion
A phase may only be marked complete when required validation has passed or a waiver is recorded, generated documentation has been recorded or waived, and required user confirmation has been recorded.

### Review handling
An optional final sanity review uses `gpt-5.5`.

If review findings exist:
- record them in the tracker
- present them to the user
- record whether the user accepted the current code, deferred the feedback, or sent the feedback back for `gpt-5.4` fixes

Do not silently apply a review loop without telling the user.

## Branch and commit expectations
Use the gitflow skill as the source of truth.

Requirements:
- The feature must remain on a dedicated `feature/*` branch created from `dev`; record branch-base verification or the reason it cannot be proven.
- Every completed task must be committed on the feature branch.
- Every completed phase must be committed on the feature branch, including the final phase.
- When remote `origin` exists, every commit/merge checkpoint must also be pushed to `origin`.
- If `origin` is not configured, record push state as `N/A (local-only repository)`.
- Regular feature commits must include current `docs/plans/<feature>/plan.md` and `docs/plans/<feature>/tracker.md` updates.

## Recommended update workflow
1. Read the current plan and tracker.
2. Update the tracker before implementation resumes.
3. Make focused Rust changes using `gpt-5.4`.
4. Update task statuses and notes.
5. Run and record validation results, including generated documentation.
6. Update phase status based on validation, documentation generation, and user confirmation.
7. Record commit/push state when applicable.
8. Append a progress log entry.

## Suggested status vocabulary
- `Planned`
- `In progress`
- `Awaiting format`
- `Awaiting lint`
- `Awaiting tests`
- `Awaiting build`
- `Awaiting documentation generation`
- `Awaiting validation`
- `Awaiting review`
- `Review feedback pending`
- `Blocked`
- `Postponed`
- `Complete`

## Suggested handoff vocabulary
- `Planning in progress with gpt-5.5`
- `Ready for gpt-5.4 implementation`
- `Implementation in progress with gpt-5.4`
- `Ready for gpt-5.5 sanity review`
- `Review feedback pending`
- `Accepted without further review changes`

## Example review entry
```md
## Review Findings
- `2026-06-19` - gpt-5.5 sanity review
  - Summary: <high-level outcome>
  - Findings:
    - <Issue or concern>
  - User decision: `Send to gpt-5.4 for fixes` / `Accept as-is` / `Defer`
```

## Output expectations
- Preserve existing tracker history.
- Make incremental updates instead of rewriting intent.
- Keep notes attached to the relevant task or phase.
- Do not mark tasks or phases complete until validation rules are satisfied.
- Treat the tracker as the durable handoff log between planning, implementation, and review models.
