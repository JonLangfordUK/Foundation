# External Template Game Tracker

## Metadata
- Feature slug: `external-template-game`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/external-template-game`
- Overall status: `Implementation in progress`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation in progress with gpt-5.4`
- Created: `2026-07-15`
- Last updated: `2026-07-15`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- Keep Foundation and TemplateGame commits in their respective repositories.
- Push after each completed commit when `origin` is configured.

## Phase 0: Branch Protection Dependency
**Status:** In progress
**Goal:** Ensure the branch-protection workflow feature is either merged first or its changes are preserved before this feature updates workflows.

### Tasks
- [x] Confirm status of `feature/branch-protection-ci`.
  - Status: Complete
  - Notes: The branch exists on `origin` and includes PR triggers plus `Main source branch policy`. It is not merged into `origin/dev` at implementation start.
- [ ] Decide whether to merge branch protection first or rebase/cherry-pick compatible workflow changes.
  - Status: In progress
  - Notes: Because implementation is proceeding before that feature is merged to `dev`, this feature will merge `origin/feature/branch-protection-ci` into `feature/external-template-game` to preserve workflow behavior.

### Validation
- Git state inspection: Pending
- User confirmation: Pending

## Phase 1: Foundation External Project Support
**Status:** Complete
**Goal:** Let Foundation build, run, and package a game project outside the Foundation repository by relative or absolute path.

### Tasks
- [x] Add `--project <path>` support to `foundation-build` while preserving `--game <name>` compatibility.
  - Status: Awaiting broader validation
  - Notes: Added `--project` parsing and validation. It accepts either `--game` or `--project`, not both.
- [x] Resolve manifest, game source, game asset, engine asset, output, and Cargo paths correctly for external projects.
  - Status: Awaiting broader validation
  - Notes: External project paths can point at a directory containing `foundation.game.toml` or the manifest file directly. Game asset roots resolve relative to the game directory. Engine assets under `engine/assets` package to `assets/engine` when present.
- [x] Update build/run/package commands for external game Cargo manifests.
  - Status: Awaiting broader validation
  - Notes: External builds use `cargo build --manifest-path <game Cargo.toml>`. In-workspace `--game` builds continue to use `cargo build -p`.
- [x] Add focused tests for relative project paths, absolute project paths, direct manifest paths, asset root resolution, and `CARGO_TARGET_DIR` compatibility.
  - Status: Complete with integration coverage
  - Notes: Added tests for `--project` parsing, `--game`/`--project` conflicts, external default target directory behavior, and `CARGO_TARGET_DIR` override behavior. Local package validation covered absolute external project path and asset copy behavior.

### Validation
- Format: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Lint: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Tests: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Build: Passed via `scripts/validate-project.cmd` on 2026-07-15; external shipping package build also passed via `scripts/foundation-build.cmd package --project E:/GameDev/template-game/game --platform windows-x64 --configuration shipping --target game`
- Documentation generation: Passed via `scripts/validate-project.cmd` on 2026-07-15
- Full validation wrapper: Passed `scripts/validate-project.cmd` on 2026-07-15

## Phase 2: Standalone TemplateGame Repository
**Status:** Complete pending PR workflow
**Goal:** Populate `https://github.com/Perfect-Pixel-Games/template-game.git` as the reference external Foundation game.

### Tasks
- [x] Create initial repository structure with `engine/`, `game/`, `scripts/`, `docs/`, and workflow directories.
  - Status: Complete
  - Notes: Created the standalone structure in `E:/GameDev/template-game`.
- [x] Add Foundation as the default `engine/` submodule.
  - Status: Complete
  - Notes: Added Foundation as an `engine/` submodule tracking `feature/external-template-game` for the initial integration branch.
- [x] Move TemplateGame source, manifest, and assets into `game/`.
  - Status: Complete
  - Notes: Copied the current TemplateGame source, manifest, and assets into `game/`, and converted `game/Cargo.toml` to standalone path dependencies through `../engine`.
- [x] Add game-facing scripts for build, run, package, validation, and optional engine path association.
  - Status: Complete
  - Notes: Added wrappers that default to `engine/` and allow `FOUNDATION_ENGINE_PATH` for alternate engine checkouts.
- [x] Add `main` and `dev` branches and push them to the new repo.
  - Status: Complete
  - Notes: Bootstrapped `main` with the initial README, pushed `main`, created `dev`, pushed `dev`, and created `feature/external-template-game` from `dev` for implementation.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Passed `cargo test --manifest-path game/Cargo.toml --no-default-features` on 2026-07-15
- Build: External package build passed through Foundation on 2026-07-15
- Package: Passed `scripts/foundation-build.cmd package --project E:/GameDev/template-game/game --platform windows-x64 --configuration shipping --target game` from Foundation on 2026-07-15
- Workflow run: Pending after push/PR

## Phase 3: Workflows And Branch Protection Readiness
**Status:** In progress
**Goal:** Ensure both repositories can enforce PR-based changes with runner-backed checks.

### Tasks
- [x] Update Foundation workflow expectations for external TemplateGame or an intentional minimal fixture.
  - Status: Awaiting PR workflow
  - Notes: Foundation workflow now checks out the external TemplateGame repository, replaces its `engine/` directory with a junction to the current Foundation checkout, and packages via `--project external\\template-game\\game`.
- [x] Add template-game workflow for PR validation and packaging on the Windows self-hosted runner.
  - Status: Awaiting PR workflow
  - Notes: Added `.github/workflows/template-game-build.yml`; it validates and packages but does not publish Foundation releases.
- [x] Add source-branch policy for template-game `main`, allowing `dev` and `hotfix/*` sources.
  - Status: Awaiting PR workflow
  - Notes: Added `Main source branch policy` matching the Foundation branch-protection model.
- [x] Document branch protection setup for both repositories.
  - Status: Awaiting PR workflow check names
  - Notes: Added TemplateGame branch-protection docs and updated Foundation build docs. Required checks must be selected after first PR workflow runs expose check names.

### Validation
- Foundation PR workflow: Pending after opening PR
- TemplateGame PR workflow: Pending after opening PR
- Branch protection setup walkthrough: Ready to provide after PR checks expose names

## Phase 4: Documentation And Cleanup
**Status:** Planned
**Goal:** Finish the transition from in-repo TemplateGame to external reference game.

### Tasks
- [ ] Update Foundation docs to describe external games and the TemplateGame repo.
  - Status: Planned
  - Notes: Include examples for `--project` and engine/game asset roots.
- [ ] Update TemplateGame docs to describe clone, submodule, engine association, scripts, workflows, and branch protection.
  - Status: Planned
  - Notes: Include `git clone --recurse-submodules` instructions.
- [ ] Remove or replace in-repo `games/template-game` according to the approved plan.
  - Status: Planned
  - Notes: If a minimal fixture remains, document why it exists and avoid presenting it as the real TemplateGame.

### Validation
- Documentation generation: Pending
- Full Foundation validation: Pending
- Full TemplateGame validation: Pending
- User confirmation: Pending

## Postponed Work
- Game-specific release versioning for `template-game` is postponed unless the user asks to define it now.
- Linux runner support remains postponed until a Linux runner/toolchain exists.

## Progress Log
- `2026-07-15`: Created planning documents for moving TemplateGame into `https://github.com/Perfect-Pixel-Games/template-game.git` and adding external Foundation game project support.
- `2026-07-15`: Inspected the new template-game repository and found it empty with no commits.
- `2026-07-15`: Noted dependency on unmerged `feature/branch-protection-ci` workflow changes.
- `2026-07-15`: User approved implementation. Confirmed active branch is `feature/external-template-game`, matching the tracker. Branch was created from `origin/dev`; branch-protection changes are already present in `origin/dev`, so no manual merge was required.
- `2026-07-15`: Added initial external `--project` support to `foundation-build`, updated Foundation scripts to work when called from outside the engine root, and validated with `cargo fmt --all` plus `cargo test -p foundation-build`.
- `2026-07-15`: Removed `games/template-game` from the Foundation workspace, updated Foundation workflow packaging to use the external TemplateGame repository, and updated Foundation build documentation.
- `2026-07-15`: Bootstrapped the new TemplateGame repository with `main`, `dev`, and `feature/external-template-game`; added Foundation as the default `engine/` submodule and copied TemplateGame into `game/`.
- `2026-07-15`: Pushed TemplateGame implementation commit `c09e4b8 Add external Foundation TemplateGame` to `origin/feature/external-template-game`.
- `2026-07-15`: Ran full Foundation validation with `scripts/validate-project.cmd`; formatting, linting, tests, build, and documentation generation all passed.
- `2026-07-15`: User reported Foundation PR workflow failure in `external_project_uses_game_target_directory_by_default` when `CARGO_TARGET_DIR` was set by CI. Updated the test to call `built_executable_path_with_target_directory(None)` so it verifies the default external game target directory without being affected by CI's environment override. Verified with `CARGO_TARGET_DIR='C:/actions-runner/cargo-target/Foundation' cargo test -p foundation-build`.
- `2026-07-15`: Updated local Foundation `origin` remote to `https://github.com/Perfect-Pixel-Games/Foundation-Engine.git` after repository move.
- `2026-07-15`: Updated Foundation workflow and planning docs to use `Perfect-Pixel-Games/template-game` after the TemplateGame repository move.
- `2026-07-15`: User decided Foundation should no longer produce GitHub releases now that games are external. Removed Foundation release jobs while keeping package artifacts available as workflow artifacts.
- `2026-07-15`: User clarified Foundation should still produce version tags. Added tag-only Foundation jobs for protected `dev` and `main` pushes; they create version tags after package validation succeeds but do not publish GitHub Releases.
- `2026-07-15`: Added TemplateGame release jobs on `feature/game-releases` so game-owned package artifacts publish from the game repository instead of the engine repository.

## Git And Push State
- Foundation branch created from: `origin/dev`
- Foundation branch: `feature/external-template-game`
- Plan/tracker commit: `a6fdbe5 Add external TemplateGame plan`
- Implementation start commit: `ae83014 Start external TemplateGame implementation`
- Foundation external project support commit: `6c6ed8f Add external game project support`
- Foundation move TemplateGame commit: `3d0c31f Move TemplateGame out of Foundation`
- Foundation tracker update commit: `e11e9fc Update external TemplateGame tracker`
- Foundation validation tracker commit: `a8c6336 Record external TemplateGame validation`
- Foundation CI test fix commit: `0126ade Fix external project target directory test`
- Foundation repository URL update commit: `476beb7 Update TemplateGame repository references`
- Foundation release removal commit: `bba4ab7 Stop Foundation release publishing`
- Foundation tag-only commit: Pending
- TemplateGame game release commit: `7ef3d92 Publish TemplateGame releases`
- Foundation push state: Release removal pushed; tag-only update pending
- TemplateGame repo state: `main` and `dev` bootstrapped and pushed; feature implementation commit `c09e4b8 Add external Foundation TemplateGame` pushed on `feature/external-template-game`
