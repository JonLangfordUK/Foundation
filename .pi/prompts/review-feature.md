---
description: Run an optional final sanity review on an implemented Rust feature
---
Use the `feature-review-handoff`, `feature-tracker-update`, `rust-workspace-dev`, and `gitflow-workflow` skills.

Review the following implemented Rust feature:
$@

Requirements:
- Use `gpt-5.5` for the review pass. Never use Anthropic models.
- Read the existing `docs/plans/<new-feature>/plan.md` and `docs/plans/<new-feature>/tracker.md`.
- Review the implementation against the plan, tracker, changed files, validation results, and generated documentation evidence.
- Record review findings in the tracker, including missing public API documentation or missing generated documentation evidence.
- Present issues and recommendations to the user clearly.
- Ask the user whether to accept the implementation as-is, defer the findings, or send the findings back for a `gpt-5.4` implementation pass.
