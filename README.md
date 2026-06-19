# PiGame

PiGame is a Rust workspace for a Bevy-based game and editor.

## Workspace layout
- `Cargo.toml` - workspace manifest and shared dependency configuration
- `crates/engine` - shared Bevy launcher/window setup used by workspace applications
- `crates/game` - standalone game crate and executable
- `crates/editor` - standalone editor executable linked to shared engine and game metadata
- `AGENTS.md` - project instructions for Pi
- `.pi/skills/` - reusable skills for Rust work, feature planning, tracker updates, review handoff, and Git workflow
- `.pi/prompts/` - prompt templates for planning, implementation, review, and validation
- `docs/plans/` - feature plans, trackers, and templates
- `scripts/` - Windows wrappers for Cargo validation commands and optional feature-plan scaffolding

## Running the applications
Open the game window:

```cmd
cargo run -p pigame-game
```

Open the editor window:

```cmd
cargo run -p pigame-editor
```

Both launchers use shared setup from `pigame-engine`. The editor also links to the game crate for shared game metadata.

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
- `scripts/doc-project.cmd`
- `scripts/validate-project.cmd`

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
scripts\doc-project.cmd
```
