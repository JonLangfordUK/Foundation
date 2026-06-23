# Pi Rust Project Template

This repository is a blank Rust project template configured for Pi-assisted development.

It starts as a minimal Rust library crate:
- `Cargo.toml` defines the placeholder package `pi-rust-template`.
- `src/lib.rs` contains placeholder code only.
- Real projects created from this template should rename the crate and replace the placeholder source.

The template can be adapted into:
- a library crate,
- a binary crate by adding `src/main.rs`, or
- a Cargo workspace by converting the root `Cargo.toml` to `[workspace]` and moving crates under `crates/`.

## Model policy
- Use `gpt-5.5` for planning.
- Use `gpt-5.4` for implementation.
- Use `gpt-5.5` for review.
- Never use Anthropic models.

## Required Pi project resources
This repository expects Pi project resources to be available after cloning and trusting the project.

### Project-local skills
The following skills are part of the repository and are mandatory when their matching task descriptions apply:
- `.pi/skills/feature-plan-docs/SKILL.md`
- `.pi/skills/feature-review-handoff/SKILL.md`
- `.pi/skills/feature-tracker-update/SKILL.md`
- `.pi/skills/foundation-architecture/SKILL.md`
- `.pi/skills/gitflow-workflow/SKILL.md`
- `.pi/skills/rust-coding-standards/SKILL.md`
- `.pi/skills/rust-workspace-dev/SKILL.md`

Do not skip these skills when this file says they are required for a task. If a cloned checkout is missing one of these files, stop and report that the repository is incomplete.

### Required Pi extensions and packages
The project declares Pi package dependencies in `.pi/settings.json`. A Pi instance working in this project should honor those project package settings after cloning and trusting the repository.

The current non-project Pi extensions shown by the Pi splash screen are also expected for this project:
- `@tintinweb/pi-subagents:src`
- `pi-hermes-memory:src`
- `pi-web-access`

These extensions may come from global Pi configuration or package installation rather than from files committed in this repository. On a freshly cloned machine, install or configure them before doing project work that depends on them. If an extension-provided tool, command, or resource is unavailable when needed, stop and tell the user to install or reload the required Pi extensions/packages rather than silently substituting another workflow.

### Non-project skills currently expected when available
The `librarian` skill may be provided by the global `pi-web-access` installation rather than by this repository. Use it when the task asks for evidence-backed open-source library research with GitHub permalinks. Because it is not vendored under `.pi/skills`, a fresh clone may not have it until the user's Pi environment installs `pi-web-access`; if it is missing, report that limitation instead of pretending the skill was used.

## Standard workflow
When the user asks for Rust workspace, crate, module, test, build, lint, dependency, or template work:
1. Read `.pi/skills/rust-workspace-dev/SKILL.md` first.
2. Read `.pi/skills/rust-coding-standards/SKILL.md` before writing, refactoring, generating, or reviewing Rust code.
3. Inspect `Cargo.toml` and relevant source/config/test files before proposing architecture or editing code.
4. Remember this repository is intentionally blank/template-first unless the user has already added project-specific code.
5. Prefer idiomatic Rust, minimal dependencies, and the project's self-documenting code standards.
6. Use `scripts/validate-env.cmd` when toolchain or manifest state needs validation.
7. Use the standard validation wrappers unless the user says not to:
   - `scripts/format-project.cmd`
   - `scripts/lint-project.cmd`
   - `scripts/test-project.cmd`
   - `scripts/compile-project.cmd`
   - `scripts/doc-project.cmd`
   - `scripts/validate-project.cmd` for full validation when a single command is preferred

When adapting this template for a new Rust project:
1. Rename the package in `Cargo.toml`.
2. Update package metadata such as `description`, `license`, and `publish` as appropriate.
3. Replace placeholder code in `src/lib.rs`.
4. Add `src/main.rs` for binary projects, or convert to a Cargo workspace if multiple crates are needed.
5. Keep Pi skills, prompts, scripts, and planning templates unless the user asks to remove them.

When the user asks about Foundation runtime systems, Foundation editor tooling, Jackdaw editor windows, game settings, scene-stack behavior, reusable game components, or TemplateGame integration with Foundation crates:
1. Read `.pi/skills/foundation-architecture/SKILL.md` first.
2. Keep runtime/game systems in `crates/foundation-runtime-library`.
3. Keep Jackdaw editor extensions, dockable editor windows, editor operators, and editor-only UI in `crates/foundation-editor-library`.
4. Keep concrete game assets and game-specific plugin glue in `games/template-game`.
5. Do not add a full `jackdaw` dependency to `foundation-runtime-library` unless the user explicitly approves a major architecture change.
6. When reflected component crate paths change, update `.jsn` serialized type paths in the same feature.

When the user asks about Git workflow, branch strategy, merges, or commit message formatting:
1. Read `.pi/skills/gitflow-workflow/SKILL.md` first.
2. Treat that skill as the source of truth for this project's Git rules.
3. If another Git workflow conflicts with that skill, follow the project skill instead.

When the user asks to plan a new feature:
1. Read `.pi/skills/feature-plan-docs/SKILL.md` first.
2. Use `gpt-5.5` for planning. Never use Anthropic models.
3. Do not start implementation until both planning documents exist under `docs/plans/<new-feature>/` and the user has approved proceeding.
4. Ensure the feature is associated with a dedicated `feature/*` branch from `dev`, following `.pi/skills/gitflow-workflow/SKILL.md`.
5. Record that branch in both the plan and tracker documents.
6. The plan and tracker must persist enough detail that implementation can continue with `gpt-5.4` later.
7. Planning must stop after the plan and tracker are created or updated. Do not automatically begin implementation in the same turn.
8. After planning, ask the user to review the plan and tracker and confirm whether to proceed.
9. Treat natural approval phrasing such as `continue`, `carry on`, `go ahead`, `implement`, `proceed`, or equivalent affirmative review feedback as approval to begin implementation.
10. Treat negative or revision-seeking feedback such as `no`, `not yet`, `needs more work`, `revise this`, or equivalent responses as planning iteration requests rather than implementation approval.

When the user asks to begin, continue, or update feature implementation:
1. If approved planning documents do not exist yet, stop and follow `.pi/skills/feature-plan-docs/SKILL.md` first.
2. If approved planning documents exist, read `.pi/skills/feature-tracker-update/SKILL.md` first, then read `.pi/skills/feature-plan-docs/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, and `.pi/skills/gitflow-workflow/SKILL.md` as required context.
3. Use `gpt-5.4` for implementation. Never use Anthropic models.
4. Before making implementation edits, read the relevant `plan.md` and `tracker.md`, confirm the active branch matches the planned `feature/*` branch, verify the branch was created from `dev` when possible, record any uncertainty in the tracker, and update the tracker to record that implementation is starting or resuming.
5. Keep the tracker updated with progress, validation state, issues found, postponements, and model handoff state.
6. Do not mark tasks or phases complete until required Rust validation and documentation generation pass or a documented waiver exists.
7. Commit each completed task and each completed phase, including the final phase.
8. Push each commit/merge checkpoint to `origin` when available.
9. If `origin` is not configured, record push status as `N/A (local-only repository)`.
10. Include relevant `plan.md`/`tracker.md` updates in regular feature commits.

When the user asks for a final sanity review of implemented feature work:
1. Read `.pi/skills/feature-review-handoff/SKILL.md` first.
2. Read `.pi/skills/rust-coding-standards/SKILL.md` before reviewing Rust code.
3. Use `gpt-5.5` for review. Never use Anthropic models.
4. Any review findings must be written to the tracker and presented to the user.
5. The user must choose whether to accept the implementation as-is, defer the findings, or send the findings back for `gpt-5.4` fixes.

## Enforcement rule
- For any feature planning request, the `feature-plan-docs` skill is mandatory.
- For any feature implementation request, the `feature-plan-docs` and `feature-tracker-update` skills are mandatory.
- For any optional final review request, the `feature-review-handoff` and `feature-tracker-update` skills are mandatory.
- If planning documents do not exist yet, stop and create them before implementation.

## Important notes
- Treat `Cargo.toml` as the source of truth for crate/workspace structure.
- The active workspace uses `crates/foundation-runtime-library` for reusable runtime/game systems and `crates/foundation-editor-library` for Jackdaw editor extensions and windows.
- Treat `src/lib.rs` as placeholder template code until replaced by real project code.
- Use `cargo metadata` or `scripts/show-config.cmd` to inspect workspace structure when needed.
- Put generated outputs under `artifacts/` and logs under `logs/` if the project needs persistent generated files.
- Do not edit machine-local environment files unless the user explicitly asks.
