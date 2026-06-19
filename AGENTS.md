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

## Standard workflow
When the user asks for Rust workspace, crate, module, test, build, lint, dependency, or template work:
1. Read `.pi/skills/rust-workspace-dev/SKILL.md` first.
2. Inspect `Cargo.toml` and relevant source/config/test files before proposing architecture or editing code.
3. Remember this repository is intentionally blank/template-first unless the user has already added project-specific code.
4. Prefer idiomatic Rust and minimal dependencies.
5. Use `scripts/validate-env.cmd` when toolchain or manifest state needs validation.
6. Use the standard validation wrappers unless the user says not to:
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

When the user asks about Git workflow, branch strategy, merges, or commit message formatting:
1. Read `.pi/skills/gitflow-workflow/SKILL.md` first.
2. Treat that skill as the source of truth for this project's Git rules.
3. If another Git workflow conflicts with that skill, follow the project skill instead.

When the user asks to plan a new feature or begin implementing one:
1. Read `.pi/skills/feature-plan-docs/SKILL.md` first.
2. Use `gpt-5.5` for planning. Never use Anthropic models.
3. Do not start implementation until both planning documents exist under `docs/plans/<new-feature>/` and the user has approved proceeding.
4. Ensure the feature is associated with a dedicated branch from `dev`, following `.pi/skills/gitflow-workflow/SKILL.md`.
5. Record that branch in both the plan and tracker documents.
6. The plan and tracker must persist enough detail that implementation can continue with `gpt-5.4` later.
7. Planning must stop after the plan and tracker are created or updated. Do not automatically begin implementation in the same turn.
8. After planning, ask the user to review the plan and tracker and confirm whether to proceed.
9. Treat natural approval phrasing such as `continue`, `carry on`, `go ahead`, `implement`, `proceed`, or equivalent affirmative review feedback as approval to begin implementation.
10. Treat negative or revision-seeking feedback such as `no`, `not yet`, `needs more work`, `revise this`, or equivalent responses as planning iteration requests rather than implementation approval.

When the user asks to continue or update in-progress feature work:
1. Read `.pi/skills/feature-tracker-update/SKILL.md` first.
2. Use `gpt-5.4` for implementation. Never use Anthropic models.
3. Before making implementation edits, read the relevant `plan.md` and `tracker.md`, then update the tracker to record that implementation is starting or resuming.
4. Keep the tracker updated with progress, validation state, issues found, postponements, and model handoff state.
5. Do not mark tasks or phases complete until required Rust validation and documentation generation pass or a documented waiver exists.
6. Commit each completed task and each completed phase, including the final phase.
7. Push each commit/merge checkpoint to `origin` when available.
8. If `origin` is not configured, record push status as `N/A (local-only repository)`.
9. Include relevant `plan.md`/`tracker.md` updates in regular feature commits.

When the user asks for a final sanity review of implemented feature work:
1. Read `.pi/skills/feature-review-handoff/SKILL.md` first.
2. Use `gpt-5.5` for review. Never use Anthropic models.
3. Any review findings must be written to the tracker and presented to the user.
4. The user must choose whether to accept the implementation as-is, defer the findings, or send the findings back for `gpt-5.4` fixes.

## Enforcement rule
- For any feature planning request, the `feature-plan-docs` skill is mandatory.
- For any feature implementation request, the `feature-plan-docs` and `feature-tracker-update` skills are mandatory.
- For any optional final review request, the `feature-review-handoff` and `feature-tracker-update` skills are mandatory.
- If planning documents do not exist yet, stop and create them before implementation.

## Important notes
- Treat `Cargo.toml` as the source of truth for crate/workspace structure.
- Treat `src/lib.rs` as placeholder template code until replaced by real project code.
- Use `cargo metadata` or `scripts/show-config.cmd` to inspect workspace structure when needed.
- Put generated outputs under `artifacts/` and logs under `logs/` if the project needs persistent generated files.
- Do not edit machine-local environment files unless the user explicitly asks.
