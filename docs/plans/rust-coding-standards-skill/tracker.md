# Rust Coding Standards Skill Tracker

## Metadata
- Feature slug: `rust-coding-standards-skill`
- Feature area: `scaffolding`
- Primary area: `scaffolding`
- Branch: `feature/rust-coding-standards-skill`
- Overall status: `Planned`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for gpt-5.4 implementation after user approval`
- Created: `2026-06-21`
- Last updated: `2026-06-21`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- User-specific coding standards override the Rust Style Guide if conflicts are found.

## Phase 1: Add Coding Standards Skill
**Status:** Planned  
**Goal:** Add a version-controlled Pi skill that defines project Rust coding standards.

### Tasks
- [ ] Create `.pi/skills/rust-coding-standards/SKILL.md`.
  - Status: Planned
  - Notes: Must include all user-requested readability rules and Rust Style Guide fallback guidance.
- [ ] Add examples/checklist to make the standards enforceable during implementation and review.
  - Status: Planned
  - Notes: Include naming examples, direct-value-to-named-variable examples, comment guidance, and conflict precedence.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending

### Notes
- The skill should avoid vague wording where the user requested strict rules.

## Phase 2: Wire Skill Into Rust Workflow
**Status:** Planned  
**Goal:** Ensure future Rust code work loads and applies the new standards.

### Tasks
- [ ] Update `.pi/skills/rust-workspace-dev/SKILL.md` to read/apply the coding standards skill for Rust implementation and review.
  - Status: Planned
  - Notes: This is the main enforcement point for Rust code work.
- [ ] Update `AGENTS.md` so repository-level Rust workflow instructions mention the coding standards skill.
  - Status: Planned
  - Notes: Keep Gitflow and feature-plan enforcement intact.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending

### Notes
- No Rust API changes are expected.

## Phase 3: Validate And Handoff
**Status:** Planned  
**Goal:** Confirm the documentation-only workflow change is clean and ready for review/merge.

### Tasks
- [ ] Run project validation wrappers or record a user-approved waiver if full Rust validation is unnecessary for documentation-only workflow changes.
  - Status: Planned
  - Notes: Default plan is to run `scripts/validate-project.cmd` unless time or lock issues prevent it.
- [ ] Update this tracker with validation results and implementation notes.
  - Status: Planned
  - Notes: Include any lock/tooling issues if encountered.
- [ ] Commit and push the completed feature branch after validation.
  - Status: Planned
  - Notes: Commit message must follow `.pi/skills/gitflow-workflow/SKILL.md`.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending

### Notes
- The feature branch `feature/rust-coding-standards-skill` was created from `dev` during planning.

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Read `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/feature-plan-docs/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, and `.pi/skills/gitflow-workflow/SKILL.md` before implementation.
- Keep this tracker updated before and after edits, validation, commits, and push attempts.
- The planned files are `.pi/skills/rust-coding-standards/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `AGENTS.md`, and these plan/tracker documents.

## Postponed Work
- Automated lint rules for every readability standard are postponed. The first implementation should enforce these standards through the Pi skill and review checklist.

## Progress Log
- `2026-06-21`: Created feature branch `feature/rust-coding-standards-skill` from `dev` for planning.
- `2026-06-21`: Researched existing project skills, repository workflow instructions, workspace manifest, and Rust Style Guide guidance.
- `2026-06-21`: Plan and tracker created; waiting for user approval before implementation.
- `2026-06-21`: Updated feature classification from `multi-area` / `engine` to `scaffolding` / `scaffolding` after user review.
