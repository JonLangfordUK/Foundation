# Pi Rust Project Template

This repository is a blank Rust project template configured for Pi-assisted development.

It starts as a minimal Rust library crate so `cargo build`, `cargo test`, `cargo fmt`, and `cargo clippy` work immediately. Rename the crate and replace `src/lib.rs` when creating a real project from the template.

## Template contents
- `Cargo.toml` - minimal Rust crate manifest
- `src/lib.rs` - tiny placeholder library and test
- `AGENTS.md` - project instructions for Pi
- `.pi/skills/` - reusable skills for Rust work, feature planning, tracker updates, review handoff, and Git workflow
- `.pi/prompts/` - prompt templates for planning, implementation, review, and validation
- `docs/plans/_templates/` - feature plan and tracker templates
- `scripts/` - Windows wrappers for Cargo validation commands and optional feature-plan scaffolding

## Model policy
- Planning: `gpt-5.5`
- Implementation: `gpt-5.4`
- Review: `gpt-5.5`
- Anthropic models must not be used.

## Using this as a template
1. Copy or clone this repository for a new Rust project.
2. Remove or replace any existing `origin` remote as needed.
3. Rename the crate in `Cargo.toml`.
4. Update `description`, `license`, and package metadata in `Cargo.toml`.
5. Replace `src/lib.rs` with project-specific code.
6. If the project should be a binary instead of a library, add `src/main.rs` or adjust the Cargo targets.
7. If the project should be a Cargo workspace, convert the root `Cargo.toml` to a `[workspace]` manifest and move crates under `crates/`.

## Setup
Ensure Rust is installed and `cargo`/`rustc` are on `PATH`, then validate:

```cmd
scripts\validate-env.cmd
```

or:

```cmd
npm run validate-env
```

## Commands
### Windows wrappers
- `scripts/validate-env.cmd`
- `scripts/show-config.cmd`
- `scripts/scaffold-feature-plan.cmd` *(optional convenience helper)*
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`

### npm scripts
- `npm run validate-env`
- `npm run show-config`
- `npm run scaffold-feature-plan -- <feature-slug> [feature-name] [branch-name]`
- `npm run format-project`
- `npm run lint-project`
- `npm run test-project`
- `npm run compile-project`

## Pi workflow
From this folder, start Pi and use natural language, or use prompt templates:
- `/scaffold-feature-plan Add a parser module`
- `/feature-plan Add a parser module` *(planning only; stops for user review before implementation)*
- `/feature-implement Add a parser module` *(implementation flow; reads the tracker workflow before code edits)*
- `/compile-project`
- `/review-feature Add a parser module`

Feature workflow enforcement:
- planning a feature must use the `feature-plan-docs` skill and `gpt-5.5`
- implementing a feature must use `feature-plan-docs`, `feature-tracker-update`, and `gpt-5.4`
- optional final review must use `feature-review-handoff`, `feature-tracker-update`, and `gpt-5.5`
- implementation must not begin until `docs/plans/<new-feature>/plan.md` and `docs/plans/<new-feature>/tracker.md` exist and the user has approved moving forward
- work should be on a dedicated `feature/*` branch from `dev`
- every completed task and phase should be committed
- when remote `origin` exists, every commit and merge checkpoint should be pushed to `origin`
- if no `origin` is configured, push status should be recorded as `N/A (local-only repository)`

## Validation defaults
Rust validation defaults to:

```cmd
scripts\format-project.cmd
scripts\lint-project.cmd
scripts\test-project.cmd
scripts\compile-project.cmd
```
