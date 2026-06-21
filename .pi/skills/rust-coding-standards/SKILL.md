---
name: rust-coding-standards
description: Project Rust coding standards for readable, self-documenting code. Use for every Rust implementation, refactor, test, review, and generated Rust code change.
---

# Rust Coding Standards

## Purpose
Use this skill for every Rust code change in this repository.

The goal is clean, self-documenting Rust code that is easy to review and easy to
maintain. These standards apply to production code, tests, examples, generated
Rust snippets, and review feedback.

## Precedence
Apply standards in this order:
1. Follow explicit user instructions in the current conversation.
2. Follow this project coding-standards skill.
3. Follow `.pi/skills/rust-workspace-dev/SKILL.md` for workspace validation.
4. Follow the Rust Style Guide and its child pages:
   - `https://doc.rust-lang.org/style-guide/`
   - `items.html`
   - `statements.html`
   - `expressions.html`
   - `types.html`
   - `advice.html`
   - `cargo.html`
   - `principles.html`
5. Follow existing local code patterns when they do not conflict with the rules
   above.

If this skill conflicts with the Rust Style Guide, this skill wins.

## Naming Standards
Use descriptive names that explain the value's role in the current context.

Rules:
- Do not use abbreviated parameter names.
- Do not use short parameter names such as `i`, `j`, `k`, `x`, `y`, or `z`.
- Do not use shorthand names such as `idx`, `num`, `val`, `ptr`, `ctx`, `cfg`,
  or `cmd`.
- Do not use generic names such as `index` when a more descriptive name is
  available.
- Use names that describe what is being indexed, counted, loaded, configured, or
  mutated.
- Apply the same naming discipline to local variables, closure parameters, loop
  bindings, tuple destructuring, test values, and helper functions.

Examples:
- Prefer `scene_entity` over `entity` when the entity specifically owns a scene.
- Prefer `child_entity_index` over `index` when iterating over child entities.
- Prefer `loaded_scene_count` over `count`.
- Prefer `scene_file_path` over `path` when multiple path meanings are nearby.
- Prefer `menu_button_interaction` over `interaction` when the source is unclear.

Short names are only acceptable when they are established Rust type names or
mathematical domain terms whose meaning is clearer than a long replacement, such
as `T` in a generic type parameter. Even then, prefer descriptive names when the
code would be clearer.

## Named Values Before Function Calls
Do not hide important meaning inside a function argument list.

Rules:
- When passing literal values directly into a function, store them in a named
  variable first unless the function call is a trivial constructor with obvious
  meaning.
- When passing computed expressions directly into a function, store the computed
  value in a named variable first when it improves readability.
- Use the variable name to explain why the value exists.
- Prefer a short preparation block before the call over a dense call with mixed
  literals and expressions.

Avoid:
```rust
commands.spawn(Transform::from_xyz(4.0, 3.0, 6.0));
```

Prefer:
```rust
let camera_position = Vec3::new(4.0, 3.0, 6.0);
commands.spawn(Transform::from_translation(camera_position));
```

Avoid:
```rust
open_scene("pause_menu.jsn", true);
```

Prefer:
```rust
let pause_menu_scene_path = "pause_menu.jsn";
let should_clear_scene_stack = true;
open_scene(pause_menu_scene_path, should_clear_scene_stack);
```

Acceptable exceptions:
- Zero-argument constructors such as `Default::default()` or `Vec::new()`.
- Standard derive or builder syntax where adding a variable would reduce clarity.
- Test assertions where the expected value is already obvious and the assertion
  message or test name explains the intent.

## Comments
Use frequent single-line comments to explain intent, non-obvious logic, and
important state transitions.

Rules:
- Prefer `//` line comments over block comments.
- Place comments on their own line when practical.
- Use one space after `//`.
- Write comments as complete sentences with a capital letter and punctuation.
- Explain why code exists, why a branch is needed, or why an ordering matters.
- Do not write comments that simply repeat the code.
- Add comments around logic that crosses systems, schedules, ECS ownership,
  scene lifecycle, asset loading, UI ownership, or editor/runtime boundaries.

Avoid:
```rust
// increment counter
loaded_scene_count += 1;
```

Prefer:
```rust
// Track loaded scenes so validation can report incomplete scene batches.
loaded_scene_count += 1;
```

## Rust Style Guide Fallback
After applying the project-specific readability rules above, follow the Rust
Style Guide defaults.

Important defaults:
- Use `rustfmt` formatting.
- Use spaces, not tabs.
- Use 4 spaces for each indentation level.
- Keep lines at or below 100 characters where practical.
- Prefer block indentation over visual indentation.
- Use trailing commas in multiline comma-separated lists.
- Do not leave trailing whitespace.
- Prefer line comments and line doc comments over block comments.
- Put each attribute on its own line.
- Use one `derive` attribute per item.
- Follow Rust Style Guide conventions for module items, statements, expressions,
  types and bounds, non-formatting advice, and `Cargo.toml` layout.

## Review Checklist
Before considering Rust code complete, verify:
- Parameter names are descriptive and not abbreviated.
- Loop variables, closure parameters, and tuple destructuring names are
  descriptive.
- Generic names such as `index` have been replaced with context-specific names.
- Literal or computed function arguments have named variables when that improves
  readability.
- Single-line comments explain non-obvious code and logic frequently enough for a
  new maintainer to follow the flow.
- Comments explain why, not merely what.
- `cargo fmt` or `scripts/format-project.cmd` has been run or recorded.
- `cargo clippy` or `scripts/lint-project.cmd` has been run or recorded.
- Tests, build, and documentation generation have been run or recorded according
  to `.pi/skills/rust-workspace-dev/SKILL.md`.

## Enforcement
When writing or reviewing Rust code:
1. Read this skill before editing.
2. Apply these rules while designing the change, not only during final cleanup.
3. Treat violations as review findings unless the user explicitly grants an
   exception.
4. If an exception is necessary, document why the exception improves readability
   or preserves an established Rust convention.
