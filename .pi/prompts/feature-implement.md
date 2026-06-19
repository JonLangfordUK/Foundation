---
description: Begin or continue Rust implementation from an approved feature plan, using the tracker-update workflow before code edits.
---
Implement the approved Rust feature request: $@

Required workflow:
- Use `gpt-5.4` for implementation. Never use Anthropic models.
- Read `.pi/skills/feature-tracker-update/SKILL.md` first.
- Read `.pi/skills/rust-workspace-dev/SKILL.md` and `.pi/skills/gitflow-workflow/SKILL.md`.
- Read the relevant `docs/plans/<feature>/plan.md` and `docs/plans/<feature>/tracker.md` before any code edits.
- Confirm the active branch matches the planned `feature/*` branch and verify it was created from `dev` when possible; record any uncertainty in the tracker.
- Update the tracker to record that `gpt-5.4` implementation is starting or resuming before editing implementation files.
- Keep the tracker updated as work progresses.
- Run `scripts/format-project.cmd`, `scripts/lint-project.cmd`, `scripts/test-project.cmd`, `scripts/compile-project.cmd`, `scripts/doc-project.cmd`, and preferably `scripts/validate-project.cmd` unless the user says not to or the tracker records an approved waiver.
- Do not move to the next phase until required validation is recorded for the current phase unless an explicit feature-level waiver is documented.
- Commit every completed task.
- Commit every completed phase, including the final phase.
- When remote `origin` exists, push after every commit and merge checkpoint.
- If `origin` is not configured, record push state as `N/A (local-only repository)`.
- Include up-to-date `docs/plans/<feature>/plan.md` and `docs/plans/<feature>/tracker.md` in regular feature progress commits.
- Do not mark tasks or phases complete until the tracker skill validation rules are satisfied, including generated documentation.
