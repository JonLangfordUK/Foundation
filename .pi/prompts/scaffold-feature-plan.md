---
description: Optionally scaffold docs/plans/<new-feature>/plan.md and tracker.md before Rust implementation
---
Use the `feature-plan-docs`, `rust-workspace-dev`, and `gitflow-workflow` skills.

Optionally scaffold planning documents for this Rust feature before implementation starts:
$@

Requirements:
- Use `gpt-5.5` for planning-oriented scaffolding. Never use Anthropic models.
- This is an optional convenience step and not a replacement for the required planning workflow.
- Do not implement the feature yet.
- Create `docs/plans/<new-feature>/plan.md` and `docs/plans/<new-feature>/tracker.md`.
- When using the helper, call `scripts/scaffold-feature-plan.cmd <feature-slug> <feature-name> <branch-name> <feature-area> <primary-area>` with all arguments supplied.
- Use `docs/plans/_templates/plan.template.md` and `docs/plans/_templates/tracker.template.md` as the starting point when helpful.
- Ask or confirm whether this is a `game`, `engine`, `editor`, or multi-area feature, then record the feature area and primary area in both documents. Workflow/tooling-only changes should use `multi-area` with a primary area rationale unless the taxonomy is expanded later.
- Record the dedicated `feature/*` branch created from `dev` in both documents.
- Include codebase research, external research if useful, success criteria, testing methodology, planning model, preferred implementation model, and durable handoff notes.
- If testing was not specified, default to format, lint, test, build, documentation generation, and full validation wrapper validation.
