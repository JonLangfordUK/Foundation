# External Template Game Plan

## Metadata
- Feature slug: `external-template-game`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/external-template-game`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-07-15`
- Last updated: `2026-07-15`

## User Request
Move the current `template-game` out of the Foundation repository into the new public GitHub repository `https://github.com/Perfect-Pixel-Games/template-game.git`. The new game repository should use an Unreal-like layout where `engine/` contains Foundation as a submodule, checkout, junction, or externally supplied path, and `game/` contains the game source and game assets. Foundation should support building, running, and packaging external game projects by relative or absolute path. Both repositories should have working build systems and branch-protection-ready workflows using `dev` and `main` branches.

## Feature Summary
Foundation will evolve from an engine repository that contains `games/template-game` as an in-workspace test game into an engine repository that can build external Foundation game projects. The `template-game` repository will become the reference external game project and will pin Foundation through an `engine/` submodule by default, while still supporting alternate engine locations through scripts or path association.

## Feature Area Classification
- Area: `multi-area`
- Primary area: `engine`
- Rationale: The feature changes Foundation build tooling, workflow expectations, and the TemplateGame integration shape. It also creates a game repository, but the durable Foundation-side capability is external project support.

## Codebase Research
- `crates/foundation-build/src/lib.rs` currently discovers the Foundation workspace by finding a `Cargo.toml` with a `games/` directory and resolves games by scanning `games/<name>/foundation.game.toml`.
- `foundation-build` currently requires `--game <name>` and builds with `cargo build -p <manifest launch package>` from the Foundation workspace root.
- Packaging currently resolves asset roots relative to `games/<game-name>/` and copies them into package output.
- `games/template-game/foundation.game.toml` contains `[game] name = "template-game"`, `[launch] package = "template-game"`, and package asset roots of `["assets"]`.
- `games/template-game/Cargo.toml` uses workspace dependencies and workspace package metadata, so it must be converted for standalone game repo use or generated from a game workspace root.
- `games/template-game/src/lib.rs` currently uses `env!("CARGO_MANIFEST_DIR")` and `FOUNDATION_ASSET_ROOT` for asset root discovery. This can continue to work if the standalone game crate owns `game/assets` and packaging sets/copies the correct roots.
- `.github/workflows/foundation-build.yml` on `origin/dev` currently validates and packages the in-repo `template-game` and publishes releases on pushes to `dev` and `main`. The separate `feature/branch-protection-ci` branch adds PR checks and main source-branch policy but is not yet merged into `dev` at plan creation time.
- The new `https://github.com/Perfect-Pixel-Games/template-game.git` repository is currently empty with no commits at the time of inspection.

## External Research
No external online research was performed because the work is primarily repository restructuring and existing Cargo/GitHub Actions behavior is already known from the current project workflows.

## Affected Files And Systems
- `crates/foundation-build/src/lib.rs`: add external project manifest support, path resolution, build/run/package behavior for external game projects, tests, and help text.
- `scripts/foundation-build.cmd`: keep as Foundation entry point; may not need changes if CLI parsing stays inside `foundation-build`.
- `scripts/package-game.cmd`: update if it assumes only `--game` or in-repo game names.
- `Cargo.toml`: remove `games/template-game` from Foundation workspace only after Foundation no longer requires it as an in-repo member for validation.
- `games/template-game/**`: move/copy into the external `template-game` repo, then remove or replace with a minimal smoke fixture if needed.
- `.github/workflows/foundation-build.yml`: update Foundation workflow to validate the engine without relying on an in-repo real game, or deliberately use a lightweight fixture/reference game.
- `docs/build-packaging.md`: document external game project support and engine association model.
- `docs/branch-protection.md`: keep branch protection guidance if `feature/branch-protection-ci` is merged before or alongside this feature.
- New `template-game` repo: create `main` and `dev`, add `engine/` submodule, `game/` project, scripts, workflows, docs, and branch-protection setup guidance.

## Proposed Implementation Approach
1. Complete or account for the pending `feature/branch-protection-ci` feature first, because its workflow changes are the baseline for PR validation and branch-protection setup.
2. Add Foundation external project CLI support using a clear argument such as `--project <path>` while preserving `--game <name>` for in-repo compatibility during the transition.
3. Allow `--project` to accept either a directory containing `foundation.game.toml` or a direct path to `foundation.game.toml`.
4. Resolve game asset roots relative to the game manifest directory, and resolve engine assets relative to the Foundation engine root.
5. Package game assets under a game-owned asset location and engine assets under an engine-owned asset location to avoid collisions.
6. Update build logic so external games can be built from their own Cargo manifest/workspace while still using Foundation profiles/configuration where practical.
7. Convert `template-game` into the new standalone repository layout:
   - `engine/` as a Foundation submodule by default.
   - `game/` containing `Cargo.toml`, `foundation.game.toml`, `src/`, and `assets/`.
   - `scripts/` containing game-facing wrappers for build, run, package, validation, and engine association checks.
8. Configure the external game repo with `main` and `dev` branches.
9. Add GitHub Actions workflows in the game repo that validate/package PRs and pushes using the self-hosted Windows runner, without publishing Foundation releases.
10. Update Foundation docs and TemplateGame docs to describe the engine association model: submodule, normal checkout, junction/symlink, or explicit relative/absolute engine path.
11. Validate both repositories locally and with GitHub PR workflows before branch protection is enforced.

## Alternatives Considered
- Keep the real/template game inside Foundation: rejected because it dilutes the Foundation repository identity and couples engine releases to game releases.
- Use Git dependencies instead of a submodule: rejected for this phase because Foundation is under active development and the user wants to edit engine and game side by side.
- Use crates.io: rejected because the engine is not ready for published package versioning and the user wants GitHub/self-contained development.
- Require only a fixed `engine/` submodule path: too restrictive. The default should be `engine/`, but scripts/tooling should also support explicit relative or absolute engine paths.

## Risks, Constraints, And Assumptions
- Cargo path dependencies are static. If `game/Cargo.toml` points at `../engine/...`, supporting an arbitrary external engine path may require a local `.cargo/config.toml`, generated local config, a junction/symlink at `engine/`, or a wrapper workspace strategy.
- Git submodules add developer and CI complexity; scripts and docs must clearly use `--recurse-submodules` and `submodules: recursive` checkout.
- Removing `games/template-game` from Foundation may leave no in-repo game for engine validation. The plan must decide whether to keep a minimal fixture, use the external repo in workflow tests, or validate only engine crates in Foundation.
- Public repositories with self-hosted runners must keep GitHub approval controls for untrusted fork PRs.
- The external `template-game` repo is empty at planning time, so initial branch setup requires creating the first commit and pushing both `main` and `dev`.
- The branch protection feature is currently on `feature/branch-protection-ci`; this feature should either rebase after it merges or incorporate compatible workflow changes carefully.

## Open Questions
- Should Foundation keep a tiny internal smoke-test game after moving TemplateGame out, or should all game integration validation move to the external template-game repo?
- Should the external project CLI be named `--project`, `--game-project`, or `--manifest`? Default recommendation: `--project` for directory-or-manifest input.
- Should external engine path support be implemented by explicit CLI argument, generated local Cargo config, or a required `engine/` association directory that may be a submodule, checkout, symlink, or junction? Default recommendation: keep `engine/` as the Cargo-facing path and provide scripts to associate it with another path.
- Should game repository releases use the same versioning scheme as Foundation or a separate game-specific scheme? Default recommendation: separate game release scheme later; initial work should validate/package only.

## Documentation Expectations
- New or changed public Rust APIs in `foundation-build` should have Rustdoc where they are public; internal functions should remain clear and tested.
- Update `docs/build-packaging.md` with external project usage examples.
- Add game repo docs explaining clone, submodule, engine path association, build/run/package, workflows, and branch protection.
- Generated documentation must be produced before the Foundation feature is considered complete unless the user approves a waiver for workflow-only parts.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Start by reading this plan, the tracker, `feature-tracker-update`, `rust-workspace-dev`, `rust-coding-standards`, `foundation-architecture`, and `gitflow-workflow`.
- Confirm the active Foundation branch is `feature/external-template-game` and based on `dev` before edits.
- Coordinate carefully with `feature/branch-protection-ci`; if that branch is not merged, record the dependency and avoid overwriting its workflow behavior.
- Keep commits small and push both Foundation and template-game repository changes after each completed task.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Verify external project path handling works with relative paths, absolute paths, and direct manifest paths.
- Verify package output includes game assets and engine assets without ownership collisions.
- Verify Foundation release workflows still release only Foundation, and game workflows do not create Foundation releases.
- Verify branch protection setup instructions and workflow check names for both repositories.

## Success Criteria
- `template-game` exists as a standalone public repository with `main` and `dev` branches.
- The standalone repo has `engine/` as a Foundation submodule by default and `game/` as the game project containing source and assets.
- Foundation can build, run, and package the external template game by relative or absolute project path.
- Game-facing scripts can build, run, package, and validate TemplateGame through the associated engine.
- Foundation no longer treats the full TemplateGame as an ordinary in-repo game, unless a minimal smoke fixture is intentionally retained and documented.
- Foundation and template-game workflows validate PRs through the Windows self-hosted runner without creating unintended releases.
- Branch protection can be configured on `dev` and `main` for both repositories using documented required checks.

## Testing Methodology
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- `scripts/doc-project.cmd`
- `scripts/validate-project.cmd` for Foundation full validation when practical
- TemplateGame repo validation wrapper once created
- Local external project smoke checks, including at minimum:
  - `scripts\foundation-build.cmd build --project <template-game>\game --configuration test --target game`
  - `scripts\foundation-build.cmd package --project <template-game>\game --configuration shipping --target game`
  - game repo wrapper scripts for build/run/package where feasible
- GitHub PR workflow run for Foundation
- GitHub PR workflow run for template-game
