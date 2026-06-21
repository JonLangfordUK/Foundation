# Rust Coding Standards Skill Tracker

## Metadata
- Feature slug: `rust-coding-standards-skill`
- Feature area: `scaffolding`
- Primary area: `scaffolding`
- Branch: `feature/rust-coding-standards-skill`
- Overall status: `Complete`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for gpt-5.5 sanity review or user acceptance`
- Created: `2026-06-21`
- Last updated: `2026-06-21`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- User-specific coding standards override the Rust Style Guide if conflicts are found.

## Phase 1: Add Coding Standards Skill
**Status:** Complete  
**Goal:** Add a version-controlled Pi skill that defines project Rust coding standards.

### Tasks
- [x] Create `.pi/skills/rust-coding-standards/SKILL.md`.
  - Status: Complete
  - Notes: Created with user-requested readability rules, precedence, Rust Style Guide fallback guidance, and enforcement expectations.
- [x] Add examples/checklist to make the standards enforceable during implementation and review.
  - Status: Complete
  - Notes: Added naming examples, direct-value-to-named-variable examples, comment guidance, and a review checklist.

### Validation
- Format: Passed via `scripts/validate-project.cmd` on 2026-06-21
- Lint: Passed via `scripts/validate-project.cmd` on 2026-06-21
- Tests: Passed via `scripts/validate-project.cmd` on 2026-06-21
- Build: Passed via `scripts/validate-project.cmd` on 2026-06-21
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-06-21
- Full validation wrapper: Passed on 2026-06-21
- User confirmation: Received approval to continue implementation

### Notes
- The skill avoids vague wording where the user requested strict rules and records limited exceptions for established Rust conventions.

## Phase 2: Wire Skill Into Rust Workflow
**Status:** Complete  
**Goal:** Ensure future Rust code work loads and applies the new standards.

### Tasks
- [x] Update `.pi/skills/rust-workspace-dev/SKILL.md` to read/apply the coding standards skill for Rust implementation and review.
  - Status: Complete
  - Notes: Added the coding standards skill to required pre-work, implementation, and review workflow steps.
- [x] Update `AGENTS.md` so repository-level Rust workflow instructions mention the coding standards skill.
  - Status: Complete
  - Notes: Added the coding standards skill to Rust workflow, feature implementation context, and final review guidance.

### Validation
- Format: Passed via `scripts/validate-project.cmd` on 2026-06-21
- Lint: Passed via `scripts/validate-project.cmd` on 2026-06-21
- Tests: Passed via `scripts/validate-project.cmd` on 2026-06-21
- Build: Passed via `scripts/validate-project.cmd` on 2026-06-21
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-06-21
- Full validation wrapper: Passed on 2026-06-21
- User confirmation: Received approval to continue implementation

### Notes
- No Rust API changes were required.

## Phase 3: Validate And Handoff
**Status:** Complete  
**Goal:** Confirm the documentation-only workflow change is clean and ready for review/merge.

### Tasks
- [x] Run project validation wrappers or record a user-approved waiver if full Rust validation is unnecessary for documentation-only workflow changes.
  - Status: Complete
  - Notes: `scripts/validate-project.cmd` passed on 2026-06-21.
- [x] Update this tracker with validation results and implementation notes.
  - Status: Complete
  - Notes: Validation results and implementation status recorded.
- [x] Commit and push the completed feature branch after validation.
  - Status: Complete
  - Notes: Commit and push completed after this tracker update was staged with the feature changes.

### Validation
- Format: Passed via `scripts/validate-project.cmd` on 2026-06-21
- Lint: Passed via `scripts/validate-project.cmd` on 2026-06-21
- Tests: Passed via `scripts/validate-project.cmd` on 2026-06-21
- Build: Passed via `scripts/validate-project.cmd` on 2026-06-21
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-06-21
- Full validation wrapper: Passed on 2026-06-21
- User confirmation: Received approval to continue implementation

### Notes
- The feature branch `feature/rust-coding-standards-skill` was created from `dev` during planning.

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation fixes if review findings are sent back.
- Never use Anthropic models.
- Read `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/feature-plan-docs/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, and `.pi/skills/gitflow-workflow/SKILL.md` before future implementation.
- The implemented files are `.pi/skills/rust-coding-standards/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `AGENTS.md`, and these plan/tracker documents.

## Postponed Work
- Automated lint rules for every readability standard are postponed. The first implementation enforces these standards through the Pi skill and review checklist.

## Progress Log
- `2026-06-21`: `scripts/validate-project.cmd` passed; tracker updated for completion.
- `2026-06-21`: Implemented `.pi/skills/rust-coding-standards/SKILL.md` and wired it into `.pi/skills/rust-workspace-dev/SKILL.md` and `AGENTS.md`.
- `2026-06-21`: Implementation started with `gpt-5.4`; active branch matches `feature/rust-coding-standards-skill`, and branch contains current `dev` merge base `2af3929`.
- `2026-06-21`: Created feature branch `feature/rust-coding-standards-skill` from `dev` for planning.
- `2026-06-21`: Researched existing project skills, repository workflow instructions, workspace manifest, and Rust Style Guide guidance.
- `2026-06-21`: Plan and tracker created; waiting for user approval before implementation.
- `2026-06-21`: Updated feature classification from `multi-area` / `engine` to `scaffolding` / `scaffolding` after user review.
