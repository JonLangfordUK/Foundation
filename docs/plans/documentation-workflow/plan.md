# Documentation Workflow Plan

## Metadata
- Feature slug: `documentation-workflow`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/documentation-workflow`
- Status: `Awaiting user confirmation`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-06-19`
- Last updated: `2026-06-19`

## User Request
Update the project development workflow so feature completion requires generated documentation, feature planning asks whether work belongs to the game, engine, or editor area, and the supporting scripts/templates/prompts are reviewed for a rock-solid pipeline.

## Feature Summary
This feature hardens the Pi-assisted Rust development workflow. It aligns skills, prompts, planning templates, and validation scripts around a consistent process: classify each feature by area, keep plan/tracker documents authoritative, require documentation generation before feature completion, and provide a one-command full validation wrapper.

## Feature Area Classification
- Area: `multi-area`
- Primary area: `engine`
- Rationale: This is workflow infrastructure that applies to game, engine, and editor features. The primary area is marked as `engine` because the workflow primarily protects reusable foundation/API development and documentation quality.

## Codebase Research
- `.pi/skills/*.md` define the mandatory planning, implementation, review, Rust validation, and Gitflow workflow rules.
- `.pi/prompts/*.md` are user-facing command prompts that must stay consistent with the skills.
- `docs/plans/_templates/*.md` are used by `scripts/scaffold-feature-plan.cmd`; stale templates can generate non-compliant plan/tracker files.
- `scripts/Invoke-RustWorkspace.ps1` centralizes wrapper commands used by the `.cmd` scripts.
- `AGENTS.md` injects project-level rules and must mention any required validation wrapper.

## External Research
- No external online research was performed because this feature concerns repository-local workflow files and scripts.

## Affected Files And Systems
- `.pi/skills/rust-workspace-dev/SKILL.md`: documents standard Rust validation, including docs.
- `.pi/skills/feature-plan-docs/SKILL.md`: requires feature area classification and documentation expectations.
- `.pi/skills/feature-tracker-update/SKILL.md`: requires documentation generation to be tracked before completion.
- `.pi/skills/feature-review-handoff/SKILL.md`: includes documentation evidence in review scope.
- `.pi/prompts/*.md`: keeps prompt entry points aligned with the workflow.
- `docs/plans/_templates/*.md`: ensures scaffolded plans include required metadata and documentation validation.
- `scripts/Invoke-RustWorkspace.ps1`: adds `doc-project` and full `validate-project` commands.
- `scripts/doc-project.cmd`: generates Rust documentation.
- `scripts/validate-project.cmd`: runs full validation in one command.
- `scripts/Scaffold-FeaturePlan.ps1`: supports feature area and primary area replacement.
- `AGENTS.md`: updates repository-level instructions to include documentation generation.

## Proposed Implementation Approach
1. Update skill files so documentation generation and feature area classification are mandatory parts of the workflow.
2. Add `scripts/doc-project.cmd` and `doc-project` handling in `scripts/Invoke-RustWorkspace.ps1`.
3. Review all workflow entry points for consistency.
4. Update prompts, templates, and `AGENTS.md` so they match the skill rules.
5. Extend the scaffold script to support feature area and primary area metadata.
6. Add `scripts/validate-project.cmd` for one-command full validation.
7. Create retrospective plan/tracker documents for this workflow-hardening branch.
8. Run full validation and commit/push the completed workflow updates.

## Alternatives Considered
- Only update skills and rely on manual fallback commands: rejected because prompts/templates/scripts would drift and create non-compliant plans.
- Require separate validation commands only: rejected because a single full validation wrapper reduces missed steps.
- Make documentation generation optional: rejected because the user explicitly requested documentation generation to be enforced at feature completion.

## Risks, Constraints, And Assumptions
- This branch began before retrospective plan/tracker documents existed; this plan records that process correction.
- The feature area taxonomy is intentionally limited to `game`, `engine`, `editor`, and `multi-area`; workflow-only changes should use `multi-area` with a primary area rationale.
- `cargo doc --workspace --all-features --no-deps` can become expensive after Bevy/Jackdaw are added, but it is still the correct completion gate unless waived.
- `scripts/validate-project.cmd` runs each validation step sequentially and can take time on larger workspaces.

## Open Questions
- None for this workflow-hardening pass.

## Documentation Expectations
- Workflow docs in skills, prompts, templates, and `AGENTS.md` must consistently mention documentation generation.
- Public API documentation enforcement in Rust crates will be handled by future crate setup, likely using `#![warn(missing_docs)]` or `#![deny(missing_docs)]` for engine crates when appropriate.
- Generated documentation must be produced before this feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Keep all workflow entry points consistent: skills, prompts, templates, scripts, and `AGENTS.md`.
- Run `scripts/validate-project.cmd` after edits.
- Commit and push on `feature/documentation-workflow`.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Check for contradictions between `AGENTS.md`, skills, prompts, and templates.
- Confirm scaffolded plan/tracker files would include feature area and documentation validation.
- Confirm wrapper scripts execute the intended Cargo commands.

## Success Criteria
- Feature planning requires and records feature area classification.
- Feature completion requires documentation generation evidence or a documented waiver.
- Prompt files, templates, skills, and `AGENTS.md` agree on the workflow.
- `scripts/doc-project.cmd` generates Rust documentation.
- `scripts/validate-project.cmd` runs format, lint, test, build, and documentation generation.
- Full validation passes.

## Testing Methodology
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- `scripts/doc-project.cmd`
- `scripts/validate-project.cmd`
