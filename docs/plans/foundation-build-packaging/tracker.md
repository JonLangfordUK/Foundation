# Foundation Build And Packaging Tracker

## Metadata
- Feature slug: `foundation-build-packaging`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/foundation-build-packaging`
- Overall status: `Awaiting user confirmation`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation complete with gpt-5.4; ready for user review or optional gpt-5.5 sanity review`
- Created: `2026-07-15`
- Last updated: `2026-07-15`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- `Shipping + GameEditor` is an invalid build matrix combination and must be rejected by tooling.

## Phase 1: Build Vocabulary And Configuration Design
**Status:** Complete  
**Goal:** Define the user-facing build matrix, per-game configuration schema, and local command contract before implementation spreads across scripts, manifests, and crates.

### Tasks
- [x] Define `Debug`, `Test`, and `Shipping` semantics in code/config documentation.
  - Status: Complete
  - Notes: Implemented in `crates/foundation-build` and documented in `docs/build-packaging.md`. Shipping means no debug/dev/editor tooling and uses `--no-default-features`.
- [x] Define `Game` and `GameEditor` target kinds and reject `Shipping + GameEditor`.
  - Status: Complete
  - Notes: `foundation-build` rejects `shipping` plus `game-editor`; manual invalid-combination smoke test returned the expected failure.
- [x] Extend or design the `foundation.game.toml` schema for package/build metadata.
  - Status: Complete
  - Notes: Added `[package] executable-name` and `asset-roots` support; `template-game` declares both.
- [x] Decide implementation entry point: Rust `xtask`/tooling crate, scripts, or hybrid.
  - Status: Complete
  - Notes: Implemented a Rust `foundation-build` crate with thin `scripts/foundation-build.cmd` and `scripts/package-game.cmd` wrappers.

### Validation
- Format: Passed via `scripts/format-project.cmd` on 2026-07-15
- Lint: Passed via `scripts/lint-project.cmd` on 2026-07-15
- Tests: Passed via `scripts/test-project.cmd` on 2026-07-15
- Build: Passed via `scripts/compile-project.cmd` on 2026-07-15
- Documentation generation: Passed via `scripts/doc-project.cmd` on 2026-07-15
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-07-15 before final profile tuning; focused validation reran afterward.
- User confirmation: Pending final user acceptance

### Notes
- The stable command contract is `foundation-build <build|package> --game <name> --platform <alias-or-target> --configuration <debug|test|shipping> --target-kind <game|game-editor>`.

## Phase 2: Build Mode Feature Gating
**Status:** Complete  
**Goal:** Make runtime dev tools and editor features compile/configure correctly for Debug, Test, Shipping, Game, and GameEditor builds.

### Tasks
- [x] Add Cargo features/profiles or equivalent configuration mapping for Debug/Test/Shipping.
  - Status: Complete
  - Notes: Added `foundation-test` and `foundation-shipping` profiles plus build-tool feature mapping.
- [x] Gate `foundation-runtime-library` dev tools such as console/log/debug functionality.
  - Status: Complete
  - Notes: Added `foundation-runtime-library/dev-tools`; console module, macros, and console plugin are absent when disabled.
- [x] Gate `foundation-editor-library` usage behind non-shipping `GameEditor` configuration.
  - Status: Complete
  - Notes: `template-game` editor dependency is optional and enabled only by the `editor` feature.
- [x] Update `template-game` to respect the build configuration and target kind.
  - Status: Complete
  - Notes: `template-game` gates console commands behind `dev-tools`, gates editor integration behind `editor`, and resolves packaged assets beside the executable.

### Validation
- Format: Passed via `scripts/format-project.cmd` on 2026-07-15
- Lint: Passed via `scripts/lint-project.cmd` on 2026-07-15
- Tests: Passed via `scripts/test-project.cmd` on 2026-07-15
- Build: Passed via `scripts/compile-project.cmd` on 2026-07-15
- Documentation generation: Passed via `scripts/doc-project.cmd` on 2026-07-15
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-07-15 before final profile tuning; focused validation reran afterward.
- User confirmation: Pending final user acceptance

### Notes
- Focused shipping checks passed with `cargo check -p template-game --no-default-features` and `cargo check -p template-game --no-default-features --profile foundation-shipping`.

## Phase 3: Local Build And Package Tooling
**Status:** Complete  
**Goal:** Provide a local command interface to build and package any Foundation game into deterministic output artifacts.

### Tasks
- [x] Implement command parsing for `--game`, `--platform`, `--configuration`, `--target-kind`, and output location.
  - Status: Complete
  - Notes: Implemented in `crates/foundation-build/src/lib.rs` with unit tests.
- [x] Implement target alias to Rust target triple mapping.
  - Status: Complete
  - Notes: Initial aliases are `windows-x64` and `linux-x64`; direct Rust target triples are also accepted.
- [x] Build the selected game executable with the correct features/profile/target.
  - Status: Complete
  - Notes: Build tool calls Cargo with `--no-default-features` and explicit feature/profile mapping.
- [x] Package executable, assets, manifests/config, and license/readme files.
  - Status: Complete
  - Notes: Package currently includes executable, asset roots, `foundation.game.toml`, and generated `foundation.package.toml`; license/readme expansion remains an extension point.
- [x] Produce an archive suitable for sharing or release upload.
  - Status: Complete
  - Notes: Tool creates `.tar.gz` archives via `tar` next to the package directory.
- [x] Add packaging smoke tests/checks.
  - Status: Complete
  - Notes: Unit tests cover invalid matrix and feature mapping. Manual package smoke tests produced Windows debug and shipping packages with executable, assets, manifests, and archive.

### Validation
- Format: Passed via `scripts/format-project.cmd` on 2026-07-15
- Lint: Passed via `scripts/lint-project.cmd` on 2026-07-15
- Tests: Passed via `scripts/test-project.cmd` on 2026-07-15
- Build: Passed via `scripts/compile-project.cmd` on 2026-07-15
- Documentation generation: Passed via `scripts/doc-project.cmd` on 2026-07-15
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-07-15 before final profile tuning; focused validation reran afterward.
- User confirmation: Pending final user acceptance

### Notes
- Manual smoke command passed: `scripts/foundation-build.cmd package --game template-game --platform windows-x64 --configuration debug --target-kind game`.
- Manual smoke command passed: `scripts/foundation-build.cmd package --game template-game --platform windows-x64 --configuration shipping --target-kind game`.
- Manual invalid command failed as expected: `scripts/foundation-build.cmd package --game template-game --platform windows-x64 --configuration shipping --target-kind game-editor`.

## Phase 4: CI And Release Publishing
**Status:** Complete  
**Goal:** Add GitHub workflow support for self-hosted Windows/Linux agents to validate, package, and publish release artifacts.

### Tasks
- [x] Add GitHub Actions workflow templates for build/test/package matrices.
  - Status: Complete
  - Notes: Added `.github/workflows/foundation-build.yml` with Windows/Linux self-hosted package matrix.
- [x] Add artifact upload for package outputs.
  - Status: Complete
  - Notes: Workflow uploads package outputs with `actions/upload-artifact`.
- [x] Add GitHub Release publishing path.
  - Status: Complete
  - Notes: Workflow optionally uploads `.tar.gz` packages with `softprops/action-gh-release` for tag refs.
- [x] Document cross-compilation prerequisites and known limits.
  - Status: Complete
  - Notes: `docs/build-packaging.md` documents that host-target support depends on Rust targets, linkers, SDKs, and native Bevy/wgpu requirements.
- [x] Document how to extend for store uploads, signing, installers, and additional platforms.
  - Status: Complete
  - Notes: Documented CI/release extension expectations and kept store-specific support as future adapters.

### Validation
- Format: Passed via `scripts/format-project.cmd` on 2026-07-15
- Lint: Passed via `scripts/lint-project.cmd` on 2026-07-15
- Tests: Passed via `scripts/test-project.cmd` on 2026-07-15
- Build: Passed via `scripts/compile-project.cmd` on 2026-07-15
- Documentation generation: Passed via `scripts/doc-project.cmd` on 2026-07-15
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-07-15 before final profile tuning; focused validation reran afterward.
- User confirmation: Pending final user acceptance

### Notes
- CI workflow has not been executed on real self-hosted GitHub runners in this session.

## Phase 5: Documentation, Examples, And Final Validation
**Status:** Awaiting user confirmation  
**Goal:** Make the build system usable by future games and validate the full workspace before completion.

### Tasks
- [x] Add `docs/build-packaging.md` or equivalent user guide.
  - Status: Complete
  - Notes: Added command examples, build vocabulary, package layout, manifest schema, CI usage, and cross-compilation notes.
- [x] Add examples using `template-game` for Debug/Test/Shipping and Game/GameEditor.
  - Status: Complete
  - Notes: Added valid and invalid examples.
- [x] Run full project validation.
  - Status: Complete
  - Notes: Validation passed; see validation section below.
- [ ] Commit completed tasks/phases and push to `origin` where available.
  - Status: Awaiting commit
  - Notes: Implementation changes are ready to commit and push after this tracker update.

### Validation
- Format: Passed via `scripts/format-project.cmd` on 2026-07-15
- Lint: Passed via `scripts/lint-project.cmd` on 2026-07-15
- Tests: Passed via `scripts/test-project.cmd` on 2026-07-15
- Build: Passed via `scripts/compile-project.cmd` on 2026-07-15
- Documentation generation: Passed via `scripts/doc-project.cmd` on 2026-07-15
- Full validation wrapper: Passed via `scripts/validate-project.cmd` on 2026-07-15 before final profile tuning; focused format/lint/test/build/doc validation reran afterward and passed.
- User confirmation: Pending final user acceptance

### Notes
- Final phase awaits commit/push and user acceptance.

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation.
- Use `gpt-5.5` for optional final review.
- Never use Anthropic models.
- Active branch was confirmed as `feature/foundation-build-packaging` before implementation edits.
- Keep `crates/foundation` free of concrete game dependencies.
- Do not add Jackdaw dependencies.
- Treat cross-compilation as capability-based: validate/document supported host-target pairs rather than assuming every host can build every target.
- The preferred workflow going forward is the Foundation build tool, not `cargo run -p foundation -- --game template-game`.

## Postponed Work
- Store-specific upload/signing/installer support is postponed until after GitHub Release packaging is exercised on real runners.
- macOS support is postponed because no macOS signing/notarization environment is available.
- Copying license/readme/notices into every package is noted as an extension point; the current package includes executable, assets, source manifest, generated package manifest, and archive.

## Progress Log
- `2026-07-15`: User confirmed the build system summary and clarified Unreal-inspired `Debug`, `Test`, `Shipping`, `Game`, and `GameEditor` expectations.
- `2026-07-15`: Plan and tracker created on branch `feature/foundation-build-packaging`.
- `2026-07-15`: Implementation approved by user. Active branch verified as `feature/foundation-build-packaging`; `origin/dev` is an ancestor of `HEAD`, so the branch base is verified against the current local remote-tracking `dev` state. User clarified that this build workflow should become the preferred path going forward instead of `cargo run -p foundation -- --game template-game`.
- `2026-07-15`: Implemented Foundation build/package crate, manifest package metadata, feature-gated dev/editor tooling, package scripts, documentation, and GitHub workflow template.
- `2026-07-15`: Validation passed: format, lint, tests, compile, docs, full validation wrapper, no-default-features shipping checks, invalid matrix smoke test, and Windows debug/shipping package smoke tests.
