---
description: Plan a Rust feature only. Create/update the plan and tracker, then stop for user review before implementation.
---
Plan the Rust feature request: $@

Required workflow:
- Use `gpt-5.5` for planning. Never use Anthropic models.
- Read `.pi/skills/feature-plan-docs/SKILL.md` first.
- Follow `.pi/skills/rust-workspace-dev/SKILL.md` and `.pi/skills/gitflow-workflow/SKILL.md`.
- Ask or confirm whether this is a `game`, `engine`, `editor`, or multi-area feature, and record the primary area in both planning documents. Workflow/tooling-only changes should use `multi-area` with a primary area rationale unless the taxonomy is expanded later.
- Create or update `docs/plans/<feature>/plan.md` and `docs/plans/<feature>/tracker.md`.
- Do planning and research only in this flow. Do not edit implementation files.
- Include documentation expectations and generated documentation validation in the plan and tracker.
- End by summarizing the plan and tracker and asking the user to review them.
- Do not begin implementation automatically.
- Only treat later explicit user approval such as `continue`, `carry on`, `go ahead`, `implement`, `proceed`, or equivalent affirmative review feedback as permission to start implementation.
- If the user says `no`, `not yet`, `needs more work`, `revise`, or equivalent, keep iterating on the planning docs instead of implementing.
