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
**Status:** Awaiting broader validation
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
  - Status: Partial; awaiting more path tests
  - Notes: Added tests for `--project` parsing, `--game`/`--project` conflicts, external default target directory behavior, and `CARGO_TARGET_DIR` override behavior. Direct manifest and asset copy tests still need integration coverage.

### Validation
- Format: Passed via `cargo fmt --all` on 2026-07-15
- Lint: Pending
- Tests: Passed focused `cargo test -p foundation-build` on 2026-07-15
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending

## Phase 2: Standalone TemplateGame Repository
**Status:** Planned
**Goal:** Populate `https://github.com/JonLangfordUK/template-game.git` as the reference external Foundation game.

### Tasks
- [ ] Create initial repository structure with `engine/`, `game/`, `scripts/`, `docs/`, and workflow directories.
  - Status: Planned
  - Notes: The remote repo is currently empty.
- [ ] Add Foundation as the default `engine/` submodule.
  - Status: Planned
  - Notes: Use the correct Foundation branch or commit after deciding whether `dev` or a stable tag should be pinned initially.
- [ ] Move TemplateGame source, manifest, and assets into `game/`.
  - Status: Planned
  - Notes: Convert workspace dependencies and metadata to standalone game repo form.
- [ ] Add game-facing scripts for build, run, package, validation, and optional engine path association.
  - Status: Planned
  - Notes: Scripts should support default `engine/` and clear errors for missing engine association.
- [ ] Add `main` and `dev` branches and push them to the new repo.
  - Status: Planned
  - Notes: Initial commit likely lands on `main`; create `dev` from `main` or vice versa according to the final branch setup.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Package: Pending
- Workflow run: Pending

## Phase 3: Workflows And Branch Protection Readiness
**Status:** Planned
**Goal:** Ensure both repositories can enforce PR-based changes with runner-backed checks.

### Tasks
- [ ] Update Foundation workflow expectations for external TemplateGame or an intentional minimal fixture.
  - Status: Planned
  - Notes: Must account for branch-protection workflow changes if already merged.
- [ ] Add template-game workflow for PR validation and packaging on the Windows self-hosted runner.
  - Status: Planned
  - Notes: Game workflow should not publish Foundation releases.
- [ ] Add source-branch policy for template-game `main`, allowing `dev` and `hotfix/*` sources.
  - Status: Planned
  - Notes: Match the Foundation branch-protection model.
- [ ] Document branch protection setup for both repositories.
  - Status: Planned
  - Notes: Required checks must be selected after first PR workflow runs expose check names.

### Validation
- Foundation PR workflow: Pending
- TemplateGame PR workflow: Pending
- Branch protection setup walkthrough: Pending

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
- `2026-07-15`: Created planning documents for moving TemplateGame into `https://github.com/JonLangfordUK/template-game.git` and adding external Foundation game project support.
- `2026-07-15`: Inspected the new template-game repository and found it empty with no commits.
- `2026-07-15`: Noted dependency on unmerged `feature/branch-protection-ci` workflow changes.
- `2026-07-15`: User approved implementation. Confirmed active branch is `feature/external-template-game`, matching the tracker. Branch was created from `origin/dev`; branch-protection changes are already present in `origin/dev`, so no manual merge was required.
- `2026-07-15`: Added initial external `--project` support to `foundation-build`, updated Foundation scripts to work when called from outside the engine root, and validated with `cargo fmt --all` plus `cargo test -p foundation-build`.

## Git And Push State
- Foundation branch created from: `origin/dev`
- Foundation branch: `feature/external-template-game`
- Plan/tracker commit: `a6fdbe5 Add external TemplateGame plan`
- Implementation start commit: `ae83014 Start external TemplateGame implementation`
- Foundation external project support commit: Pending
- Foundation push state: Implementation start pushed; external project support push pending
- TemplateGame repo state: Empty remote inspected; no implementation commits yet
