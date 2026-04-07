---
name: feature-plan-docs
description: Create a feature plan and tracker under docs/plans/<new-feature>/ before implementation starts, including branch tracking, research, validation expectations, and model handoff details.
---

# Feature Plan And Tracker Creation

## Purpose
Use this skill before any implementation starts for a new feature.

This skill creates or updates:
- `docs/plans/<new-feature>/plan.md`
- `docs/plans/<new-feature>/tracker.md`

Do not start implementation until both documents exist and the user has approved moving from planning to implementation.

## Model policy
- Planning model: `gpt-5.5`
- Implementation model recorded for handoff: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Never use Anthropic models for planning, implementation, or review.

The plan and tracker are the persistence layer between models. Record enough detail that implementation or review can continue later without relying on chat-only context.

## Required pre-work
1. Read `.pi/skills/rust-workspace-dev/SKILL.md` for Rust workflow.
2. Read `.pi/skills/gitflow-workflow/SKILL.md` for branch and commit rules.
3. Inspect `Cargo.toml` and relevant Rust source/config/test files before writing the plan.
4. If online research tools are available and useful, perform external research relevant to the feature.
5. If online research tools are not available or not needed, explicitly note that in the plan.

## Branch requirement
Every feature must have a dedicated branch from `dev`.

Rules:
- Use a `feature/<work-being-done>` branch name following the gitflow skill.
- Record the branch name in both `plan.md` and `tracker.md`.
- If the branch does not exist yet, propose the branch name and clearly mark branch creation as required before implementation starts.
- Do not plan implementation work directly on `main` or `dev`.

## Directory and file creation
For feature slug `<new-feature>`, create:
- `docs/plans/<new-feature>/plan.md`
- `docs/plans/<new-feature>/tracker.md`

Start from these templates when helpful:
- `docs/plans/_templates/plan.template.md`
- `docs/plans/_templates/tracker.template.md`

Optional helper:
- `scripts/scaffold-feature-plan.cmd <feature-slug> [feature-name] [branch-name]`

The feature slug should usually match the feature branch suffix.

## Plan document requirements
`plan.md` must include:
1. Feature name and user request
2. Branch name and current status
3. Codebase research findings
4. External research findings or a note that none was performed
5. Affected Rust crates/modules/APIs/configuration
6. Proposed implementation approach
7. Alternatives considered when relevant
8. Risks, constraints, assumptions, and open questions
9. Success criteria
10. Testing and validation methodology
11. Planning model used: `gpt-5.5`
12. Handoff notes for `gpt-5.4` implementation
13. Optional review focus areas for `gpt-5.5`

## Tracker document requirements
`tracker.md` must include:
1. Feature name and slug
2. Branch name
3. Overall status
4. Ordered phases and tasks
5. Validation state for each task and phase
6. Notes/issues/oversights discovered during work
7. Postponed work and reasons
8. Progress log entries
9. Planning model: `gpt-5.5`
10. Preferred implementation model: `gpt-5.4`
11. Optional final review model: `gpt-5.5`
12. Current handoff state

## Validation rules
Default Rust validation, unless the plan states a justified alternative:
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`

A task may only be marked complete when required validation for that task has passed or a user-approved waiver is recorded.

A phase may only be marked complete when:
- required validation has passed or a waiver is recorded, and
- the user has confirmed the phase is suitable when user confirmation is required.

## Commit and history expectations
Use the gitflow skill as the source of truth.

Requirements:
- All work must happen on a dedicated `feature/*` branch from `dev`.
- Every completed task must be committed.
- Every completed phase must be committed, including the final phase.
- When remote `origin` exists, every commit and merge checkpoint must also be pushed to `origin`.
- If `origin` is not configured, record push status as `N/A (local-only repository)`.
- Regular feature commits must include current `docs/plans/<feature>/plan.md` and `docs/plans/<feature>/tracker.md` changes.

## Mandatory stop after planning
After `plan.md` and `tracker.md` are created or updated:
1. Stop planning work.
2. Summarize the plan and tracker for the user.
3. Ask the user to review and confirm whether implementation should begin.
4. Do not begin implementation in the same turn unless the user explicitly asks to proceed after that review checkpoint.

Treat clear affirmative responses such as `continue`, `carry on`, `go ahead`, `implement`, or `proceed` as approval to begin implementation. Treat negative or revision-seeking responses as planning iteration requests.

## Suggested `plan.md` template
```md
# <Feature Name> Plan

## Metadata
- Feature slug: `<new-feature>`
- Branch: `feature/<work-being-done>`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `<YYYY-MM-DD>`
- Last updated: `<YYYY-MM-DD>`

## User Request
<What the user asked for>

## Feature Summary
<What the feature is and why it exists>

## Codebase Research
- <Relevant Rust crate/module/API finding>

## External Research
- <Finding and source>
- Or: `No external online research was performed because it was not needed or tooling was unavailable.`

## Affected Files And Systems
- `<path or subsystem>`: <why it matters>

## Proposed Implementation Approach
1. <Step>

## Alternatives Considered
- <Alternative>: <why rejected or deferred>

## Risks, Constraints, And Assumptions
- <Risk / constraint / assumption>

## Open Questions
- <Question, if any>

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- <Specific guidance another model must follow>

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- <Area to sanity-check later>

## Success Criteria
- <Observable outcome>

## Testing Methodology
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
```

## Suggested `tracker.md` template
```md
# <Feature Name> Tracker

## Metadata
- Feature slug: `<new-feature>`
- Branch: `feature/<work-being-done>`
- Overall status: `Planned`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for gpt-5.4 implementation`
- Created: `<YYYY-MM-DD>`
- Last updated: `<YYYY-MM-DD>`

## Validation Rules
- Task complete only after required Rust validation passes or a waiver is recorded.
- Phase complete only after required validation passes and required user confirmation is recorded.

## Phase 1: <Phase name>
**Status:** Planned  
**Goal:** <testable deliverable>

### Tasks
- [ ] <Task 1>
  - Status: Planned
  - Notes: None

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- User confirmation: Pending / Not required yet

## Implementation / Review Handoff Notes
- None

## Postponed Work
- None

## Progress Log
- `<YYYY-MM-DD>`: Plan and tracker created.
```
