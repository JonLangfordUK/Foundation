---
name: rust-workspace-dev
description: Rust workspace workflow for this project. Use when planning, implementing, formatting, linting, testing, building, benchmarking, or validating Rust code.
---

# Rust Workspace Development

## Purpose
Use this skill for Rust project work in this pi workspace.

The repository may be either:
- a single Rust crate with `Cargo.toml` in the root, or
- a Cargo workspace with a root `Cargo.toml` containing `[workspace]`.

Do not assume the crate layout before inspecting `Cargo.toml` and the relevant source files.

## Required pre-work
1. Read `.pi/skills/rust-coding-standards/SKILL.md` before writing, refactoring, generating, or reviewing Rust code.
2. Inspect `Cargo.toml` before planning architecture or commands.
3. Inspect `src/`, `crates/`, `tests/`, `benches/`, `examples/`, and relevant config files when present.
4. Prefer existing project patterns over introducing new structure.
5. If online research is needed and tools are available, research the specific crates/APIs involved and cite useful findings in the plan.

## Standard commands
Use these defaults unless the project defines a more specific command:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --workspace --all-features
cargo doc --workspace --all-features --no-deps
```

For a non-workspace crate, remove `--workspace` if Cargo reports it is invalid.

## Scripts
Convenience wrappers are available:

- `scripts/validate-env.cmd` - verify `cargo` and `rustc` are available and show versions
- `scripts/show-config.cmd` - show detected Cargo manifest/workspace information
- `scripts/format-project.cmd` - run `cargo fmt --all -- --check`
- `scripts/lint-project.cmd` - run `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `scripts/test-project.cmd` - run `cargo test --workspace --all-features`
- `scripts/compile-project.cmd` - run `cargo build --workspace --all-features`
- `scripts/doc-project.cmd` - when present, run project documentation generation; otherwise use `cargo doc --workspace --all-features --no-deps`
- `scripts/validate-project.cmd` - run the full project validation sequence: format, lint, test, build, and documentation generation

## Workflow

### 1. Validate environment
Before the first Rust task in a session, validate the toolchain if needed:

```bash
scripts/validate-env.cmd
```

If validation fails, explain whether `cargo`, `rustc`, or the manifest is missing.

### 2. Plan a feature
When the user asks to plan a Rust feature:
1. Read `Cargo.toml` and relevant crate/module files.
2. Identify affected modules, public APIs, feature flags, dependency changes, tests, and compatibility risks.
3. Produce a plan with validation steps appropriate for the change.
4. Use `gpt-5.5` for planning. Never use Anthropic models.

### 3. Implement a feature
When the user asks to implement a feature:
1. Review the approved plan and tracker before code edits.
2. Read and apply `.pi/skills/rust-coding-standards/SKILL.md` before Rust edits.
3. Use `gpt-5.4` for implementation. Never use Anthropic models.
4. Keep changes focused, idiomatic, and consistent with existing Rust style and project coding standards.
5. Add or update tests for behavior changes where practical.
6. Run formatting, linting, tests, build checks, and documentation generation unless the user says not to.

### 4. Review a feature
When the user asks for a review:
1. Read and apply `.pi/skills/rust-coding-standards/SKILL.md` before reviewing Rust code.
2. Use `gpt-5.5` for review. Never use Anthropic models.
3. Review against the plan, tracker, user request, changed files, validation results, and project coding standards.
4. Record findings in the tracker if the feature workflow is in use.

## Rust-specific guidance
- Prefer small modules with explicit ownership and error handling.
- Avoid unnecessary `unwrap()`/`expect()` in library/runtime code unless a panic is intentional and documented.
- Preserve public API compatibility unless the plan explicitly allows a breaking change.
- Document new public items when appropriate.
- Keep dependency additions minimal and justified in `Cargo.toml`.
- Consider feature flags, target platforms, async runtime choices, and `no_std`/WASM constraints when relevant.
- Treat `cargo fmt`, `cargo clippy`, `cargo test`, `cargo build`, and `cargo doc --workspace --all-features --no-deps` results as the default validation evidence.
