# Codebase Cleanup Review Plan

## Metadata
- Feature slug: `codebase-cleanup-review`
- Feature area: `multi-area`
- Primary area: `game`
- Branch: `feature/codebase-cleanup-review`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-06-21`
- Last updated: `2026-06-21`

## User Request
Review the entire `template_game` and `foundation_library` codebase and ensure the code is clean, well implemented, and robust.

Additional approved requirement: after the user confirms all features are still working following the code review/refactor, write a clear, concise, illustrative instruction-manual style breakdown of the scene system. The document must teach a new developer what the system is, how it works, how it interacts with Jackdaw, and how to use it in standalone game runtime, editor edit mode, and editor game/play mode.

## Feature Summary
This feature is a quality and robustness pass over the reusable Foundation game library and the TemplateGame example/application code. The implementation should audit Rust modules, tests, manifests, launcher configuration, and TemplateGame `.jsn` assets; fix discovered correctness, maintainability, robustness, documentation, and validation issues; and leave the codebase with clear validation evidence.

## Feature Area Classification
- Area: `multi-area`
- Primary area: `game`
- Rationale: The requested work spans shared engine-like library code in `foundation-library`, TemplateGame runtime/editor integration code, and TemplateGame authored assets. The primary area is `game` because the user specifically named `template_game` and the requested outcome is a robust game template built on the shared library. Please confirm this classification before implementation if you want a different taxonomy.

## Branch Status
- Current branch created for this work: `feature/codebase-cleanup-review`.
- Branch base: created from `dev` during planning.
- Remote: `origin` is configured as `https://github.com/JonLangfordUK/Foundation.git`; implementation commits should be pushed after each checkpoint.

## Codebase Research
- Root `Cargo.toml` is a workspace with `crates/foundation-library`, `crates/jackdaw-editor`, and `games/template-game` as members. This feature focuses on `crates/foundation-library` and `games/template-game`, but validation is workspace-wide unless a narrower command is explicitly justified.
- `crates/foundation-library` is an edition 2021 library crate depending on workspace `bevy` and `jackdaw_runtime`. It exposes `FoundationPlugin`, `FoundationSettings`, `FoundationActor`, and a prelude that re-exports menu, scene-stack, and splash-screen APIs.
- `crates/foundation-library/src/scene_stack.rs` owns the scene-stack resource, commands, lifecycle messages, runtime flags, and cleanup. Existing tests cover stack opening, replacement, focus changes, key closure, cleanup, and load requests.
- `crates/foundation-library/src/menu.rs` owns reusable menu actions, pause state, placeholder/options UI generation, gameplay placeholder generation, escape handling, and generated UI ownership. Existing tests cover constructors, defaults, and options tab order.
- `crates/foundation-library/src/splash_screen.rs` owns authored/generate splash UI selection, timing phases, cleanup, and scene transition commands. Existing tests cover timings, transition command construction, and phase alpha progression.
- `games/template-game` is an edition 2024 game crate with a default standalone binary, an optional `editor` binary, a library used by both binaries, and optional editor-only dependencies (`jackdaw`, `jackdaw_jsn`, `serde_json`).
- `games/template-game/src/lib.rs` contains most TemplateGame runtime/editor integration: startup scene commands, editor play-mode scene loading, UI root targeting/parenting, scene-stack bridge, authored UI text completion, landing/main menu setup, and tests.
- `games/template-game/src/main.rs` configures standalone Bevy/Jackdaw runtime plugins, asset root selection, shared Foundation and TemplateGame plugins, and a default UI camera.
- `games/template-game/src/bin/editor.rs` configures the TemplateGame editor binary, project auto-open behavior, editor plugins, shared Foundation and TemplateGame plugins, and editor asset roots.
- `games/template-game/tests/template_components.rs` is editor-feature gated and currently smoke-tests that a user component reaches Jackdaw's component picker.
- `games/template-game/assets/*.jsn` include the startup splash/background flow, landing page, main menu, options/load/pause menus, gameplay level, and a sample scene. The assets reference Foundation and TemplateGame components and should be included in the robustness audit, especially path consistency and authored component expectations.
- Quick static scan found only test `expect(...)` calls in `scene_stack.rs` and one `#[allow(dead_code)]` in `games/template-game/src/lib.rs`; these are candidate review points but not necessarily defects.

## External Research
No external online research was performed because this is an internal cleanup/audit feature and the existing code already identifies the relevant Bevy, Jackdaw, and project APIs. If implementation uncovers an API-specific uncertainty, research the exact Bevy/Jackdaw API and record the finding in the tracker before changing behavior.

## Affected Files And Systems
- `crates/foundation-library/Cargo.toml`: Dependency/workspace consistency and package metadata.
- `crates/foundation-library/src/lib.rs`: Public plugin composition, public prelude, reflection registration, and public API docs.
- `crates/foundation-library/src/scene_stack.rs`: Scene-stack correctness, lifecycle message ordering, focus/runtime flag computation, cleanup robustness, and tests.
- `crates/foundation-library/src/menu.rs`: Menu action safety, pause state transitions, generated entity ownership, UI child wiring, query filters, and tests.
- `crates/foundation-library/src/splash_screen.rs`: Splash runtime gating, authored/generated UI behavior, timing edge cases, scene transition semantics, cleanup, and tests.
- `games/template-game/Cargo.toml`: Workspace dependency consistency, feature gating, binary metadata, and editor dependencies.
- `games/template-game/.cargo/config.toml`: Local cargo aliases for editor/play workflows.
- `games/template-game/jackdaw.toml`: Jackdaw run configuration consistency with actual binary names.
- `games/template-game/src/lib.rs`: TemplateGame plugin registration, editor/standalone cfg boundaries, scene loading bridge, UI targeting, input handling, and tests.
- `games/template-game/src/main.rs`: Standalone startup, asset root handling, plugin ordering, error handling, and camera defaults.
- `games/template-game/src/bin/editor.rs`: Editor startup, project root handling, asset root handling, plugin ordering, and stop/play integration.
- `games/template-game/tests/template_components.rs`: Editor-feature integration coverage.
- `games/template-game/assets/*.jsn`: Authored component paths, scene path references, parent/order metadata, UI root markers, and template flow consistency.
- `docs/plans/codebase-cleanup-review/plan.md` and `tracker.md`: Required planning and handoff state.

## Proposed Implementation Approach
1. Confirm the feature area classification and implementation scope with the user before edits begin.
2. Read this plan, the tracker, and mandatory project skills before implementation. Verify the active branch is `feature/codebase-cleanup-review` and that it was created from `dev` when possible.
3. Establish a baseline by running the standard validation wrappers. Record failures before fixing them.
4. Audit `foundation-library` module by module for correctness, robust error handling, public API clarity, naming/readability standards, scene lifecycle safety, test gaps, and documentation gaps.
5. Audit `template-game` module by module for standalone/editor behavior, cfg boundaries, asset path handling, UI ownership/parenting, scene-stack bridge behavior, input edge cases, readability, and test gaps.
6. Audit TemplateGame `.jsn` assets and launcher/config files for consistency with constants, component registrations, scene-stack assumptions, and editor/play flows.
7. Make focused fixes in small tasks. Prefer minimal, behavior-preserving cleanup unless a correctness issue requires behavior change.
8. Add or update tests for any behavior changes, and add regression tests for robustness issues where practical.
9. Update Rustdoc and inline comments where public APIs or non-obvious scene/editor behavior need clearer handoff context.
10. Run format, lint, tests, build, documentation generation, and full validation. Record all results in the tracker.
11. Commit each completed task and phase with the required Gitflow commit message format and push to `origin` after each commit when available.
12. After implementation, offer an optional `gpt-5.5` final sanity review focused on any high-risk systems or broad cleanup changes.

## Alternatives Considered
- **Only run formatting/linting:** Rejected because the user asked for a full quality and robustness pass, not only mechanical cleanup.
- **Rewrite major systems up front:** Rejected because existing code already has tests and project-specific behavior. The safer approach is evidence-driven audit and focused fixes.
- **Limit scope to Rust files only:** Rejected because TemplateGame behavior depends on `.jsn` assets and launcher configuration as much as Rust code.
- **Include `crates/jackdaw-editor`:** Deferred unless validation failures or direct dependencies require touching it. The user specifically named `template_game` and `foundation_library`.

## Risks, Constraints, And Assumptions
- Broad cleanup can accidentally change runtime/editor behavior. Keep commits focused and back changes with tests or validation notes.
- Some editor behavior may require manual verification in Jackdaw Editor beyond automated cargo tests. Record any manual-verification gaps.
- Bevy/Jackdaw APIs are ECS- and schedule-sensitive; preserve system ordering unless a defect is identified.
- TemplateGame uses cfg-gated editor code; both default and `editor` feature validation must be considered.
- Authored `.jsn` assets may encode component type paths and scene paths that tests do not fully cover.
- The plan assumes the requested scope is `foundation-library` and `games/template-game`; broader workspace cleanup is out of scope except for validation blockers.

## Open Questions
- Please confirm that `multi-area` with primary area `game` is the desired classification.
- Should implementation include manual editor launch/play testing, or only automated Rust validation unless needed?
- Are behavior-changing improvements acceptable if the audit finds weak design, or should changes be limited to correctness/readability/test/documentation cleanup?

## Documentation Expectations
- Public APIs added or changed by this feature must have Rustdoc comments, or this plan must explicitly justify why they are internal/undocumented.
- Non-obvious ECS scheduling, scene-stack ownership, UI parenting, editor/play-mode boundaries, and asset-loading assumptions should have clear comments.
- After user confirmation that the reviewed/refactored features still work, add a dedicated scene-system instruction manual under `docs/` that explains the scene stack, Jackdaw interaction, standalone runtime flow, editor edit-mode flow, editor play-mode flow, and recommended usage patterns.
- Generated documentation must be produced before the feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Read `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/feature-plan-docs/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, and `.pi/skills/gitflow-workflow/SKILL.md` before making implementation edits.
- Confirm the active branch is `feature/codebase-cleanup-review`; if not, stop and resolve branch state before editing.
- Keep the tracker current before and after each phase/task.
- Do not mark tasks complete until validation and documentation generation are recorded or a waiver is approved.
- Commit each completed task and phase. Include changed plan/tracker files in normal feature commits. Push to `origin` after each commit when available.
- Treat generated `target/` content as out of scope and do not edit committed source based on generated artifacts alone.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Review scene-stack lifecycle correctness and message ordering after cleanup.
- Review editor-only scene loading and UI targeting for stale entity, missing entity, and parent/child race conditions.
- Review public API/docs consistency for `foundation-library::prelude`.
- Review validation evidence and any waived/manual checks.

## Success Criteria
- `foundation-library` and `template-game` Rust code is cleaner, more maintainable, and aligned with project Rust coding standards.
- Robustness issues discovered during the audit are fixed or explicitly documented as postponed with rationale.
- Public APIs and non-obvious ECS/editor behavior are documented clearly enough for future maintenance.
- After user confirmation that features still work, a scene-system instruction manual exists under `docs/`, is clear and concise, and teaches standalone, editor edit-mode, editor play-mode, and Jackdaw integration behavior.
- Relevant test coverage is added or improved for behavior changes and identified edge cases.
- TemplateGame asset references and launcher/config files are consistent with code constants and expected flows.
- Standard validation passes, including documentation generation, or any deviations are explicitly waived by the user and recorded.
- Tracker records baseline findings, changes made, validation results, push status, and handoff state.

## Testing Methodology
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- `scripts/doc-project.cmd`
- `scripts/validate-project.cmd`
- If needed for editor-specific coverage: targeted `cargo test -p template-game --features editor` or equivalent wrapper-compatible command, recorded in the tracker.
- If requested/needed: manual standalone launch and editor play-mode smoke checks, with results recorded in the tracker.
