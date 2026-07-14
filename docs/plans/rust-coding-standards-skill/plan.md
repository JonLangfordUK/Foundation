# Rust Coding Standards Skill Plan

## Metadata
- Feature slug: `rust-coding-standards-skill`
- Feature area: `scaffolding`
- Primary area: `scaffolding`
- Branch: `feature/rust-coding-standards-skill`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-06-21`
- Last updated: `2026-06-21`

## User Request
Create a Pi skill for Rust coding standards that is enforced for any Rust code. The skill must prioritize the user's readability rules over the official Rust Style Guide when conflicts exist:
- Clean, self-documenting code.
- No abbreviated parameter names.
- No short parameter names such as `i`, `j`, or `k`.
- No shorthand parameters such as `index`; names must be descriptive.
- Do not pass raw values directly into functions when a named variable would make the value's meaning clearer; store the value in a descriptive variable first.
- Use frequent single-line comments to explain code and logic.
- Otherwise follow `https://doc.rust-lang.org/style-guide/` and its child pages.

## Feature Summary
Add a project skill document under `.pi/skills/` that captures the repository's Rust coding standards and make the project's Rust workflow load that skill before Rust implementation or review work. The new skill should be reusable by future coding agents and clear enough to resolve conflicts between user-specific readability requirements and Rust's default style guidance.

## Feature Area Classification
- Area: `scaffolding`
- Primary area: `scaffolding`
- Rationale: This is a project scaffolding/workflow feature. It defines reusable coding-standard guidance for future Rust work rather than adding game, engine, or editor runtime behavior.

## Codebase Research
- `.pi/skills/` currently contains project skills for feature planning, feature tracking, feature review handoff, Gitflow, and Rust workspace development. There is no existing coding-standards skill.
- `.pi/skills/rust-workspace-dev/SKILL.md` is the mandatory skill for Rust implementation, formatting, linting, tests, builds, and docs. It is the best enforcement point for loading the new coding-standards skill.
- `AGENTS.md` tells agents to read `.pi/skills/rust-workspace-dev/SKILL.md` for Rust workspace, crate, module, test, build, lint, dependency, or template work. It should also mention the coding standards skill so enforcement is visible from the repository-level instructions.
- `Cargo.toml` defines a Rust workspace with `foundation-library`, `jackdaw-editor`, and `template-game`; the standards should apply to all workspace Rust code rather than a single crate.

## External Research
- The Rust Style Guide states that Rust formatting should normally be handled mechanically by tools such as `rustfmt`, and that consistent formatting reduces comprehension cost.
- The Rust Style Guide's default formatting conventions include 4-space indentation, spaces instead of tabs, a 100-character maximum line width, trailing commas in multiline comma-separated lists, no trailing whitespace, and block indentation instead of visual indentation.
- The Rust Style Guide recommends line comments over block comments, comments on their own line when practical, a single space after `//`, and comments that are generally complete sentences with capitalization and punctuation.
- Child pages cover formatting for module-level items, statements, expressions, types and bounds, Cargo.toml conventions, non-formatting advice, and the principles behind the guide. The skill should reference these pages as the fallback standard after the user's stricter readability requirements.

## Affected Files And Systems
- `.pi/skills/rust-coding-standards/SKILL.md`: New skill containing the coding standards and conflict-resolution rules.
- `.pi/skills/rust-workspace-dev/SKILL.md`: Add a required step to read and apply the coding standards skill for Rust code work.
- `AGENTS.md`: Update the standard Rust workflow to mention the coding standards skill as mandatory for Rust code implementation/review.
- `docs/plans/rust-coding-standards-skill/plan.md`: Feature plan.
- `docs/plans/rust-coding-standards-skill/tracker.md`: Feature tracker.

## Proposed Implementation Approach
1. Create `.pi/skills/rust-coding-standards/SKILL.md` with front matter, purpose, precedence rules, naming standards, literal/value readability standards, comment standards, Rust Style Guide fallback rules, and review checklist.
2. Explicitly state that user-specified readability rules override the Rust Style Guide when there is a conflict.
3. Include examples of unacceptable and acceptable parameter names and direct-value calls.
4. Update `.pi/skills/rust-workspace-dev/SKILL.md` so Rust implementation/review work reads the coding standards skill before editing or reviewing Rust code.
5. Update `AGENTS.md` so repository-level instructions also require the coding standards skill for Rust code work.
6. Validate formatting-sensitive files manually and run project validation wrappers where appropriate.

## Alternatives Considered
- Only update `AGENTS.md`: Rejected because the user specifically requested a skill document, and a reusable skill is easier for agents to load directly.
- Put all standards into `rust-workspace-dev`: Rejected because a dedicated coding-standards skill is clearer and can be referenced by multiple workflows.
- Add automated lints for every rule: Deferred because several rules, especially comments and named intermediate values, require judgment and are better enforced by agent/review workflow first.

## Risks, Constraints, And Assumptions
- Frequent comments can become noisy if applied mechanically. The skill should require useful single-line comments for non-obvious code and logic rather than restating every line.
- The direct-value rule should not force pointless variables for self-explanatory values when the function signature or surrounding code already makes meaning obvious unless the user wants it strictly universal.
- Some common Rust idioms use short closure names or tuple destructuring. The skill should favor descriptive names and document that exceptions require strong justification.
- This feature changes workflow documentation only; it does not alter Rust runtime behavior.

## Open Questions
- During implementation, decide whether the direct-value rule is absolute for all function calls or scoped to calls where readability would otherwise suffer. The user's wording suggests strong enforcement, so the skill should phrase it as mandatory except for unavoidable language constructs or trivial zero-argument/default constructors.

## Documentation Expectations
- The new skill itself is the primary documentation deliverable.
- No public Rust APIs are expected to be added or changed.
- Generated documentation should still be produced before the feature is complete because project workflow requires it, unless the user grants a waiver for a documentation-only workflow change.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Work on `feature/rust-coding-standards-skill`, which was created from `dev` during planning.
- Keep the skill concise enough to be usable, but explicit enough to enforce all requested rules.
- In conflicts, user-specific readability rules must take precedence over the official Rust Style Guide.
- Do not use the `skill_manage` tool as the primary implementation path for this project skill; create the project file under `.pi/skills/rust-coding-standards/SKILL.md` so it is version-controlled with the repository.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Check that the new skill is loaded by the Rust workflow and visible from `AGENTS.md`.
- Check that standards are actionable and do not contradict Gitflow or Rust workspace validation instructions.
- Check that examples accurately reflect the user's requested naming and readability rules.

## Success Criteria
- `.pi/skills/rust-coding-standards/SKILL.md` exists and contains the requested coding standards.
- The skill references the Rust Style Guide and its child pages as fallback guidance.
- The skill clearly states that user-requested standards override the Rust Style Guide on conflict.
- Rust workflow instructions require reading/applying the new skill for Rust code work.
- Validation is recorded in the tracker.

## Testing Methodology
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- `scripts/doc-project.cmd`
- `scripts/validate-project.cmd` for the full validation sequence when practical
- Manual review of markdown skill content for clarity, precedence, and enforceability
