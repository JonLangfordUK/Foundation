# <Feature Name> Plan

## Metadata
- Feature slug: `<new-feature>`
- Feature area: `<Feature Area>`
- Primary area: `<Primary Area>`
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

## Feature Area Classification
- Area: `<Feature Area>`
- Primary area: `<Primary Area>`
- Rationale: <why this area owns the feature>

## Codebase Research
- <Relevant Rust crate/module/API finding>
- <Relevant pattern already used in the project>

## External Research
- <Finding and source>
- Or: `No external online research was performed because it was not needed or tooling was unavailable.`

## Affected Files And Systems
- `<path or subsystem>`: <why it matters>

## Proposed Implementation Approach
1. <Step>
2. <Step>
3. <Step>

## Alternatives Considered
- <Alternative>: <why rejected or deferred>

## Risks, Constraints, And Assumptions
- <Risk / constraint / assumption>

## Open Questions
- <Question, if any>

## Documentation Expectations
- Public APIs added or changed by this feature must have Rustdoc comments, or this plan must explicitly justify why they are internal/undocumented.
- Feature-level architecture or usage documentation should be added under `docs/` when Rustdoc alone is insufficient.
- Generated documentation must be produced before the feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- <Specific guidance another model must follow>

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- <Area to sanity-check later>

## Success Criteria
- <Observable outcome>
- <Observable outcome>

## Testing Methodology
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- `scripts/doc-project.cmd`
- `scripts/validate-project.cmd`
