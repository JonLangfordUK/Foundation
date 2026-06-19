---
name: feature-review-handoff
description: Perform an optional final sanity review of an implemented feature, record findings in the tracker, and hand review feedback back to implementation only if the user chooses.
---

# Feature Review Handoff

## Purpose
Use this skill after implementation when the user wants an additional sanity pass over code that was already written.

## Model policy
- Use `gpt-5.5` for review.
- Send accepted fix work back to `gpt-5.4` implementation.
- Never use Anthropic models.

## Required pre-work
1. Read `docs/plans/<new-feature>/plan.md`.
2. Read `docs/plans/<new-feature>/tracker.md`.
3. Read `.pi/skills/rust-workspace-dev/SKILL.md`.
4. Inspect the changed implementation files.
5. Review format, lint, test, build, and documentation generation results if they exist.
6. Identify plan requirements, success criteria, documentation expectations, risks, or review focus areas that should shape the review.

## Review scope
Review against:
- the user request
- the recorded plan
- the tracker state
- changed files and implementation details
- validation results already performed, including documentation generation
- risks, assumptions, and deferred work recorded during planning or implementation

Focus on practical issues such as:
- correctness concerns
- Rust ownership/lifetime/error-handling problems
- API compatibility and feature flag concerns
- architectural mismatches with the plan
- missing edge-case handling
- maintainability concerns
- missing validation coverage
- missing public API documentation or missing generated documentation evidence

## Output requirements
When performing the review:
1. Summarize the overall result clearly.
2. Separate must-fix concerns from optional improvements when possible.
3. Record findings in `tracker.md` under a review or handoff section.
4. Present the findings to the user.
5. Ask whether to accept the implementation as-is, send the review feedback back to `gpt-5.4` for fixes, or defer the findings.

Do not automatically start a fix pass unless the user chooses it.

## Suggested tracker addition
```md
## Review Findings
- `2026-06-19` - gpt-5.5 sanity review
  - Overall result: <pass / pass with concerns / needs follow-up>
  - Must-fix:
    - <Issue>
  - Optional improvements:
    - <Improvement>
  - User decision: `Accept as-is` / `Send to gpt-5.4 for fixes` / `Defer`
```
