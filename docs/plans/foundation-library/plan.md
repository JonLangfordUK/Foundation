# FoundationLibrary Baseline Plan

## Metadata
- Feature slug: `foundation-library`
- Feature area: `multi-area`
- Primary area: `game`
- Branch: `feature/foundation-library`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-06-19`
- Last updated: `2026-06-19`

## User Request
"Looks like a good idea to me. Lets call this the FoundationLibrary" followed by "Please add the baseline for this".

## Feature Summary
Add a reusable shared Rust library crate named FoundationLibrary for code that should be available to Jackdaw-style games and their editor binaries. This changes the long-term strategy from `Engine / Editor / Game` to `Editor / Game / Library`: Jackdaw supplies the editor/runtime-authoring layer, `jackdaw-editor` launches Jackdaw, game projects own game-specific code, and FoundationLibrary owns reusable Bevy/Jackdaw-compatible plugins, components, helpers, and conventions.

The baseline should be intentionally small: create the crate, expose a documented Bevy plugin, wire TemplateGame to use it in both standalone and editor binaries, and document the architecture. Do not recreate the old `engine` crate or move game-specific behavior into FoundationLibrary yet.

## Feature Area Classification
- Area: `multi-area`
- Primary area: `game`
- Rationale: The library is shared infrastructure used by games and game-specific Jackdaw editor binaries. It also affects workspace architecture and editor integration, but the first consumer is `games/template-game`.

## Codebase Research
- The root `Cargo.toml` is a workspace with members `crates/jackdaw-editor` and `games/template-game`.
- `crates/jackdaw-editor` is a Jackdaw editor launcher package named `jackdaw-editor`.
- `games/template-game` is a Jackdaw-style static game package named `template-game`, runnable from the root with `cargo run -p template-game`.
- `games/template-game/src/lib.rs` currently contains game-specific shared behavior in `TemplateGamePlugin`, including the `SpinningCube` reflected component and play-mode gate.
- `games/template-game/src/main.rs` adds `JackdawPlugin` and `template_game::TemplateGamePlugin` for standalone play.
- `games/template-game/src/bin/editor.rs` adds Jackdaw editor plugins and `template_game::TemplateGamePlugin` for game-specific editor play.
- Root workspace validation currently uses `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`, and `cargo doc --workspace --all-features --no-deps`.

## External Research
No external online research was performed because this baseline is a local workspace architecture change using standard Cargo workspace/path dependency and Bevy plugin patterns already present in the project.

## Affected Files And Systems
- `Cargo.toml`: Add `crates/foundation-library` as a workspace member; potentially add shared workspace dependency entries if useful.
- `crates/foundation-library/Cargo.toml`: New library package. Cargo package name should be `foundation-library`; Rust crate import name will be `foundation_library`.
- `crates/foundation-library/src/lib.rs`: New documented public baseline API, including a `FoundationPlugin` or `FoundationPlugins` entry point.
- `games/template-game/Cargo.toml`: Add a path dependency on `foundation-library`.
- `games/template-game/src/main.rs`: Add FoundationLibrary plugin(s) before `TemplateGamePlugin`.
- `games/template-game/src/bin/editor.rs`: Add FoundationLibrary plugin(s) before `TemplateGamePlugin` so editor/play mode sees the same reusable code.
- `games/template-game/src/lib.rs`: Keep game-specific behavior here; optionally add a small test that FoundationLibrary can be composed with TemplateGame without opening a window.
- `README.md`: Document the new Editor / Game / Library architecture and run commands.
- `docs/plans/foundation-library/tracker.md`: Record implementation progress, validation, commit, and push state.

## Proposed Implementation Approach
1. Confirm the active branch is `feature/foundation-library` and that `dev` is an ancestor of `HEAD` before implementation edits.
2. Add `crates/foundation-library` as a new root workspace member.
3. Create `crates/foundation-library/Cargo.toml` as a library package named `foundation-library`, using workspace package metadata where practical.
4. Add minimal dependencies. Prefer `bevy.workspace = true` so the plugin can implement Bevy `Plugin`. Avoid direct `jackdaw` dependency in the baseline unless a concrete editor-only API requires it.
5. Implement `crates/foundation-library/src/lib.rs` with documented public API:
   - crate-level docs explaining the library's role;
   - `FoundationPlugin` as the initial reusable plugin entry point;
   - a `prelude` module that re-exports commonly used public items;
   - at least one non-window test proving the plugin can be added to a Bevy `App`.
6. Add `foundation-library = { path = "../../crates/foundation-library" }` to `games/template-game/Cargo.toml`.
7. Add `foundation_library::FoundationPlugin` to both TemplateGame app entry points before `template_game::TemplateGamePlugin`.
8. Update `README.md` to show the new architecture:
   - `crates/jackdaw-editor` = Jackdaw editor launcher;
   - `crates/foundation-library` = reusable shared Bevy/Jackdaw-compatible library;
   - `games/template-game` = game project using FoundationLibrary.
9. Run required validation and record results in the tracker.
10. Commit and push the completed baseline feature branch.

## Alternatives Considered
- Recreate an `engine` crate: Rejected for now because Jackdaw is the editor/runtime-authoring layer and `engine` implies a larger custom engine responsibility.
- Put shared code directly in TemplateGame: Rejected because the goal is reusable code across future games.
- Put reusable code in `jackdaw-editor`: Rejected because the launcher should remain editor-shell focused and not become a game framework dependency.
- Add Jackdaw-specific editor APIs to FoundationLibrary immediately: Deferred. The baseline should remain portable and Bevy-focused unless a concrete Jackdaw editor extension need appears.

## Risks, Constraints, And Assumptions
- FoundationLibrary should not become a dumping ground. The baseline should establish clear ownership but stay minimal.
- Public APIs must be documented because this crate is intended for reuse.
- TemplateGame should remain recognizably Jackdaw-generated in shape even though it is a root workspace member.
- Adding the shared library to TemplateGame should not change runtime behavior beyond plugin composition.
- The feature branch was created from `dev` after merging the Jackdaw editor integration branch into `dev`; the old remote feature branch was intentionally left intact.

## Open Questions
- Should the initial plugin be named `FoundationPlugin` or `FoundationPlugins`? Proposed default: `FoundationPlugin` for the minimal baseline.
- Should FoundationLibrary include any actual reusable components now, or only the plugin skeleton? Proposed default: only the plugin skeleton plus tests and docs.
- Should FoundationLibrary depend on Jackdaw in the future for editor-only helpers? Proposed answer: only when a concrete feature requires it, ideally behind an `editor` feature.

## Documentation Expectations
- Public APIs added by FoundationLibrary must have Rustdoc comments.
- README should document the Editor / Game / Library strategy.
- Generated documentation must be produced before the feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Keep the first FoundationLibrary baseline small and composable.
- Do not move `SpinningCube` or `TemplateGamePlugin` out of TemplateGame in this feature unless the user explicitly asks; they are game-specific examples.
- Add FoundationLibrary to both `games/template-game/src/main.rs` and `games/template-game/src/bin/editor.rs` so standalone and editor/play-mode paths match.
- Leave Jackdaw dynamic/dylib loading out of scope.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Confirm FoundationLibrary boundaries are clear and do not reintroduce the old `engine` architecture.
- Confirm public API docs are sufficient for reuse.
- Confirm TemplateGame still runs with root package commands.

## Success Criteria
- Root workspace contains `crates/foundation-library` as package `foundation-library`.
- Rust code can import the crate as `foundation_library`.
- TemplateGame depends on FoundationLibrary and adds its plugin in both game and editor binaries.
- README documents the Editor / Game / Library architecture.
- Required Rust validation and documentation generation pass.
- Changes are committed and pushed on `feature/foundation-library` after implementation approval.

## Testing Methodology
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- `scripts/doc-project.cmd`
- `scripts/validate-project.cmd`
- Direct package checks may also be used during implementation, for example `cargo check -p foundation-library`, `cargo check -p template-game`, and `cargo check -p template-game --bin editor --features editor`.
