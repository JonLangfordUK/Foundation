# Credits Scene Plan

## Metadata
- Feature slug: `credits-scene`
- Feature area: `multi-area`
- Primary area: `game`
- Branch: `feature/credits-scene`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-06-22`
- Last updated: `2026-06-22`

## User Request
Add a credits scene to the project. The main menu should include a Credits button under the Options button. The credits scene should have a black background and text moving from bottom to top. The credits content should be authored in JSON and support an unbounded number of nested groups, where each group has the same recursive schema: a name, people, and child groups. People have a name and role. Group headers must visually shrink as nesting depth increases so top-level groups read as major headings and deeper child groups read as subordinate headings.

Example data shape requested:

```json
{
  "groups": [
    {
      "name": "Studio A",
      "people": [],
      "groups": [
        {
          "name": "Team 1",
          "people": [
            { "name": "Alice", "role": "Developer" },
            { "name": "Bob", "role": "Designer" }
          ],
          "groups": []
        }
      ]
    }
  ]
}
```

## Feature Summary
This feature adds a TemplateGame credits flow built on the existing Foundation scene-stack/menu architecture. A new main-menu button opens a `credits.jsn` scene. That scene contains an authored marker component that loads `credits.json`, recursively flattens an unbounded nested group tree into display rows, and spawns a vertically scrolling Bevy UI credits roll over a black background. Group header text size decreases by nesting depth, while remaining above a readable minimum size. The scene can be closed with Escape and should include a Back button or comparable return behavior so players can return to the main menu.

## Feature Area Classification
- Area: `multi-area` (`game` + `engine`)
- Primary area: `game`
- Rationale: The visible credits scene, concrete JSON credits file, and main-menu button are TemplateGame-specific game content. Reusable JSON schema parsing and scrolling credits UI may belong in `foundation-runtime-library` if implemented as a generic `FoundationCredits` runtime component. The primary ownership remains game because the requested credits content and menu integration are concrete TemplateGame behavior.

## Branch Status
- Dedicated feature branch: `feature/credits-scene`
- Branch creation status: Created from `dev` on 2026-06-22 during planning.
- Remote status: Not pushed during planning.
- Working tree note: `games/template-game/.jsn/project.jsn` was already modified before planning began and appears unrelated to this feature. Do not overwrite it unless the user confirms it is part of the credits work.

## Codebase Research
- `Cargo.toml` defines a workspace with `crates/foundation-runtime-library`, `crates/foundation-editor-library`, `crates/jackdaw-editor`, and `games/template-game`.
- `crates/foundation-runtime-library/src/lib.rs` installs `scene_stack::FoundationSceneStackPlugin`, `splash_screen::FoundationSplashScreenPlugin`, and `menu::FoundationMenuPlugin`, then re-exports reusable runtime APIs from `prelude`.
- `crates/foundation-runtime-library/src/menu.rs` already owns reusable menu primitives, including `FoundationMenuButton`, `FoundationCloseOnEscape`, `FoundationGeneratedMenuUi`, `FoundationUiOrder`, menu colors, and runtime generation patterns for placeholder/options menu UI.
- `crates/foundation-runtime-library/Cargo.toml` already depends on `serde`, but not `serde_json`. JSON credits parsing in the runtime crate would require adding `serde_json` to workspace/runtime dependencies or parsing in the game crate where `serde_json` is currently optional for editor-only code.
- `games/template-game/src/lib.rs` defines scene path constants, registers TemplateGame reflected marker components, loads `.jsn` scenes via scene-stack `SceneLoadRequested`, completes Jackdaw-authored UI text components, and initializes authored marker components such as `TemplateLandingPage` and `TemplateMainMenu`.
- `games/template-game/assets/main_menu.jsn` currently has buttons in this order: New Game, Load Game, Options, Exit. The requested Credits button should be inserted under Options and above Exit, with `FoundationMenuButton` opening `credits.jsn` using a stable key such as `credits`.
- `games/template-game/src/lib.rs` tests assert that scene path constants match existing assets and that `main_menu.jsn` references known scene paths. These tests must be updated to include the new credits scene and asset references.
- `games/template-game/src/main.rs` already sets `ClearColor` to black for standalone runs, but the credits scene should still author or generate a black full-screen background so it is correct in editor play and regardless of other scene backgrounds.

## External Research
No external online research was performed because this plan can be derived from existing Bevy/Jackdaw patterns already present in the repository and no third-party API uncertainty was identified during planning.

## Affected Files And Systems
- `crates/foundation-runtime-library/Cargo.toml`: likely add `serde_json` if credits parsing is implemented in the reusable runtime crate.
- Root `Cargo.toml`: likely add workspace dependency `serde_json = "1"` if shared by runtime/game crates.
- `crates/foundation-runtime-library/src/lib.rs`: register and re-export a new credits module/plugin/component if credits is implemented as reusable Foundation runtime behavior.
- `crates/foundation-runtime-library/src/credits.rs` (new): likely home for generic credits JSON data types, loading, flattening, scrolling UI generation, and tests.
- `crates/foundation-runtime-library/src/menu.rs`: may reuse existing helpers/patterns; avoid broad menu refactors unless needed.
- `games/template-game/Cargo.toml`: if credits parsing stays game-local, add non-optional `serde`/`serde_json` dependencies as needed. Prefer runtime crate if the behavior is reusable.
- `games/template-game/src/lib.rs`: add `CREDITS_SCENE` constant, include it in tests, and possibly register any game-local credits marker if not using a Foundation marker.
- `games/template-game/assets/main_menu.jsn`: insert Credits button under Options and adjust `FoundationUiOrder` values so Exit remains after Credits.
- `games/template-game/assets/credits.jsn` (new): authored credits scene with black background, gameplay UI root, close-on-escape/back behavior, and credits marker component.
- `games/template-game/assets/credits.json` (new): concrete JSON credits data using the requested nested schema.
- `games/template-game/tests/template_components.rs`: update reflection/component/asset expectations if applicable.
- `games/template-game/.jsn/project.jsn`: may need update only if Jackdaw project metadata tracks assets or reflected component availability; preserve existing unrelated user modifications.

## Proposed Implementation Approach
1. Confirm the ownership split during implementation: prefer reusable credits runtime in `foundation-runtime-library` if it can stay generic and not depend on `jackdaw`; otherwise keep only game-specific glue in `games/template-game`.
2. Add credits data model types for JSON:
   - `CreditsDocument { groups: Vec<CreditsGroup> }`
   - `CreditsGroup { name: String, people: Vec<CreditPerson>, groups: Vec<CreditsGroup> }`
   - `CreditPerson { name: String, role: String }`
   Derive `Deserialize`, `Serialize` where useful for tests/docs, and `Debug/Clone/PartialEq` for validation.
3. Add a reflected marker component for authored scenes, likely `FoundationCreditsRoll` if reusable or `TemplateCreditsRoll` if game-local. Suggested fields: `credits_path`, `scroll_speed_pixels_per_second`, `start_offset_pixels`, `end_padding_pixels`, and optional text sizes/spacing.
4. Implement a loader/system that reads the JSON from the game asset directory or project-relative assets directory, parses it, recursively flattens nested groups in pre-order for any depth, and spawns generated UI text rows under the credits root. Each flattened group row must carry its nesting depth so UI generation can apply indentation and depth-based heading sizes. In editor mode, follow existing UI target/parent behavior used by TemplateGame UI roots.
5. Apply group heading typography by depth: top-level groups use the largest header size, each nested level steps down, and very deep levels clamp to a readable minimum header size. Person rows should remain consistently readable and should be indented or otherwise associated with their containing group.
6. Implement scrolling by moving a generated content node from bottom to top over time. When the credits finish, either keep the final off-screen position or allow looping only if a component field explicitly enables it. Default should be a one-way roll matching the request.
7. Include a black full-screen background in `credits.jsn` using existing `TemplateFullscreenBackground` or authored UI background color.
8. Add a Back button or close-on-escape behavior. Escape should close the current credits scene through `FoundationCloseOnEscape`. A Back button should use `FoundationMenuButton::close_current` or equivalent authored component.
9. Add `CREDITS_SCENE = "credits.jsn"` to TemplateGame constants and update tests that validate scene paths and authored references.
10. Edit `main_menu.jsn` to add a Credits button under Options and above Exit. Use `FoundationMenuButton` action `open_scene`, scene path `credits.jsn`, and scene key `credits`.
11. Add unit tests for JSON parsing/flattening order, arbitrary-depth recursion, depth-to-heading-size clamping, default component values, scene constant existence, and `main_menu.jsn` referencing `credits.jsn`.
12. Run formatting, lint, tests, build, docs, and full validation wrappers before marking tasks complete.

## Alternatives Considered
- Game-only implementation in `games/template-game`: simpler and keeps concrete behavior near assets, but duplicates reusable credits-roll logic if future games need credits.
- Reusable `foundation-runtime-library` implementation: preferred if the marker and JSON schema stay game-agnostic. This matches existing reusable menu/splash systems and keeps standalone games independent of the full Jackdaw editor crate.
- Author all credits text directly in `credits.jsn`: rejected because the user explicitly requested JSON-authored credits with nested groups.
- Use a hard-coded Rust credits list: rejected because it would require recompiling to edit credits and does not satisfy the JSON requirement.

## Risks, Constraints, And Assumptions
- Bevy `AssetServer` does not directly parse arbitrary JSON without adding asset loader code. A direct file read from the assets directory may be simpler but must work in both standalone and editor play. The implementation should centralize path resolution and handle missing/invalid files gracefully.
- `foundation-runtime-library` currently does not depend on `serde_json`; adding it is small but must be justified and reflected in workspace dependencies.
- Editor Play uses current working directory plus `assets` for `.jsn` scene loading; credits JSON path resolution should match this behavior.
- Jackdaw `.jsn` type paths must match any new reflected component crate path exactly. If component ownership changes, serialized `.jsn` paths must be updated in the same feature.
- The existing working tree has unrelated `games/template-game/.jsn/project.jsn` modifications. Implementation must avoid accidentally committing unrelated local changes unless confirmed.
- The requested recursive group schema assumes empty `people` and `groups` arrays are valid at every depth; implementation should default missing arrays to empty only if that is intentionally documented.
- Deeply nested groups must not cause recursion bugs, stack overflows in practical content, unreadably tiny text, or negative/invalid font sizes. Depth-based header sizing must clamp to a readable minimum.
- The exact typography and scroll timing are not specified; implementation should choose readable defaults and expose simple component fields for tuning.

## Open Questions
- Should the reusable credits-roll logic live in `foundation-runtime-library` as a Foundation feature, or should this first version stay TemplateGame-specific? Plan recommendation: reusable runtime component in Foundation with concrete TemplateGame assets.
- Should the credits automatically return to the main menu when the roll finishes, loop, or remain closable by Back/Escape? Plan recommendation: remain closable; no automatic navigation unless requested.
- Should the JSON schema allow optional fields/defaults or require all arrays/strings exactly as shown? Plan recommendation: require `groups` at the document root and default omitted nested `people`/`groups` to empty only if tests document it.

## Documentation Expectations
- Public APIs added or changed by this feature must have Rustdoc comments, especially new credits data types, marker component fields, and any reusable plugin/module.
- If the JSON schema is implemented in Foundation, document the schema in Rustdoc and consider adding a short `docs/credits-scene.md` or equivalent if Rustdoc is insufficient for content authors.
- Generated documentation must be produced before the feature is considered complete via `scripts/doc-project.cmd`.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Before editing, read `.pi/skills/feature-tracker-update/SKILL.md`, this `plan.md`, `tracker.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, `.pi/skills/gitflow-workflow/SKILL.md`, and `.pi/skills/foundation-architecture/SKILL.md`.
- Confirm the active branch is `feature/credits-scene` and record any uncertainty in `tracker.md`.
- Preserve pre-existing unrelated `games/template-game/.jsn/project.jsn` modifications unless the user explicitly includes them.
- Keep runtime/game systems out of `foundation-editor-library` unless editor-only tooling is required.
- Do not add a full `jackdaw` dependency to `foundation-runtime-library`.
- Update serialized `.jsn` component paths in the same commit as any component crate-path decisions.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Verify JSON parsing and recursive group flattening handles an arbitrary number of nested group levels in the requested order.
- Verify credits UI owns generated entities with `SceneOwner` so closing the credits scene cleans them up.
- Verify editor Play and standalone game paths both locate `credits.json`.
- Verify main menu visual order places Credits under Options and above Exit.
- Verify no unrelated `project.jsn` changes are included accidentally.

## Success Criteria
- Main menu displays a Credits button directly under Options and above Exit.
- Pressing Credits opens a credits scene through the existing scene stack.
- Credits scene has a black background.
- Credits text scrolls from bottom to top.
- Credits content is loaded from JSON using the requested recursive `groups`/`people` schema.
- Nested groups and people render in deterministic pre-order with visible group names, person names, and roles for any practical nesting depth.
- Group headers get progressively smaller by nesting depth and clamp to a readable minimum size for very deep groups.
- Credits scene can be exited with Escape and/or a Back button.
- Standalone game and editor Play behavior both work or any limitation is explicitly documented and approved.
- Tests cover parsing/flattening and asset/scene integration.
- Required validation and documentation generation pass before implementation is marked complete.

## Testing Methodology
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- `scripts/doc-project.cmd`
- `scripts/validate-project.cmd`

Focused validation should include targeted `cargo test` runs for the affected runtime/game crates before full validation when practical.
