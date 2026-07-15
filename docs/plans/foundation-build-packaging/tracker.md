# Foundation Build And Packaging Tracker

## Metadata
- Feature slug: `foundation-build-packaging`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/foundation-build-packaging`
- Overall status: `Planned`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for user review before gpt-5.4 implementation`
- Created: `2026-07-15`
- Last updated: `2026-07-15`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.
- Never use Anthropic models.
- `Shipping + GameEditor` is an invalid build matrix combination and must be rejected by tooling.

## Phase 1: Build Vocabulary And Configuration Design
**Status:** Planned  
**Goal:** Define the user-facing build matrix, per-game configuration schema, and local command contract before implementation spreads across scripts, manifests, and crates.

### Tasks
- [ ] Define `Debug`, `Test`, and `Shipping` semantics in code/config documentation.
  - Status: Planned
  - Notes: Shipping means no debugging, no dev tools, game/assets only. Test means dev tools enabled. Debug means full debugging with no optimizations.
- [ ] Define `Game` and `GameEditor` target kinds and reject `Shipping + GameEditor`.
  - Status: Planned
  - Notes: Editor target kind is only valid for non-shipping configurations.
- [ ] Extend or design the `foundation.game.toml` schema for package/build metadata.
  - Status: Planned
  - Notes: Must remain data-driven and scalable beyond `template-game`.
- [ ] Decide implementation entry point: Rust `xtask`/tooling crate, scripts, or hybrid.
  - Status: Planned
  - Notes: Plan recommendation is Rust-owned logic with thin script wrappers.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending / Not required yet
- User confirmation: Pending

### Notes
- This phase should produce the stable contract used by later phases.

## Phase 2: Build Mode Feature Gating
**Status:** Planned  
**Goal:** Make runtime dev tools and editor features compile/configure correctly for Debug, Test, Shipping, Game, and GameEditor builds.

### Tasks
- [ ] Add Cargo features/profiles or equivalent configuration mapping for Debug/Test/Shipping.
  - Status: Planned
  - Notes: Must account for Cargo feature unification and avoid shipping accidentally enabling dev/editor code.
- [ ] Gate `foundation-runtime-library` dev tools such as console/log/debug functionality.
  - Status: Planned
  - Notes: Test builds should include these; shipping builds should not.
- [ ] Gate `foundation-editor-library` usage behind non-shipping `GameEditor` configuration.
  - Status: Planned
  - Notes: Existing `template-game` dependency may need to become optional.
- [ ] Update `template-game` to respect the build configuration and target kind.
  - Status: Planned
  - Notes: Runtime `--editor` alone is insufficient for shipping exclusion.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending / Not required yet

### Notes
- Review should focus heavily on shipping exclusion guarantees.

## Phase 3: Local Build And Package Tooling
**Status:** Planned  
**Goal:** Provide a local command interface to build and package any Foundation game into deterministic output artifacts.

### Tasks
- [ ] Implement command parsing for `--game`, `--platform`, `--configuration`, `--target-kind`, and output location.
  - Status: Planned
  - Notes: Input validation must reject unsupported games/platforms/configuration combinations.
- [ ] Implement target alias to Rust target triple mapping.
  - Status: Planned
  - Notes: Start with Windows/Linux targets chosen during implementation.
- [ ] Build the selected game executable with the correct features/profile/target.
  - Status: Planned
  - Notes: Executable output should use the game name.
- [ ] Package executable, assets, manifests/config, and license/readme files.
  - Status: Planned
  - Notes: Shipping packages must exclude dev-only files.
- [ ] Produce an archive suitable for sharing or release upload.
  - Status: Planned
  - Notes: Archive format may vary by platform if justified.
- [ ] Add packaging smoke tests/checks.
  - Status: Planned
  - Notes: Verify executable and asset presence; verify invalid matrix rejection.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending / Not required yet

### Notes
- Package outputs should live under `artifacts/` and logs under `logs/` if persistent outputs are needed.

## Phase 4: CI And Release Publishing
**Status:** Planned  
**Goal:** Add GitHub workflow support for self-hosted Windows/Linux agents to validate, package, and publish release artifacts.

### Tasks
- [ ] Add GitHub Actions workflow templates for build/test/package matrices.
  - Status: Planned
  - Notes: Use self-hosted runner labels and support Windows/Linux agents.
- [ ] Add artifact upload for package outputs.
  - Status: Planned
  - Notes: Keep generated packages accessible from workflow runs.
- [ ] Add GitHub Release publishing path.
  - Status: Planned
  - Notes: Support creating/uploading release artifacts; document required permissions/secrets.
- [ ] Document cross-compilation prerequisites and known limits.
  - Status: Planned
  - Notes: Do not promise universal cross-build support without validated toolchains.
- [ ] Document how to extend for store uploads, signing, installers, and additional platforms.
  - Status: Planned
  - Notes: Store integration can remain adapter-ready unless the user prioritizes a store now.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending / Not required yet

### Notes
- CI should reuse the local build/package command rather than duplicating logic in YAML.

## Phase 5: Documentation, Examples, And Final Validation
**Status:** Planned  
**Goal:** Make the build system usable by future games and validate the full workspace before completion.

### Tasks
- [ ] Add `docs/build-packaging.md` or equivalent user guide.
  - Status: Planned
  - Notes: Include build vocabulary, examples, package layout, CI usage, and troubleshooting.
- [ ] Add examples using `template-game` for Debug/Test/Shipping and Game/GameEditor.
  - Status: Planned
  - Notes: Exclude Shipping+GameEditor example except as an invalid input example.
- [ ] Run full project validation.
  - Status: Planned
  - Notes: Use standard wrappers and record results here.
- [ ] Commit completed tasks/phases and push to `origin` where available.
  - Status: Planned
  - Notes: Gitflow skill requires pushing after commits when origin exists.

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending

### Notes
- Final phase cannot be complete until validation and documentation generation are recorded or waived.

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation.
- Use `gpt-5.5` for optional final review.
- Never use Anthropic models.
- Read the plan before implementation and keep this tracker current as work starts, pauses, validates, or completes.
- Confirm active branch is `feature/foundation-build-packaging` before implementation edits.
- Keep `crates/foundation` free of concrete game dependencies.
- Do not add Jackdaw dependencies.
- Treat cross-compilation as capability-based: validate/document supported host-target pairs rather than assuming every host can build every target.

## Postponed Work
- Store-specific upload/signing/installer support may be postponed until after GitHub Release packaging is working, unless the user selects a store target during planning review.
- macOS support may be postponed if no macOS signing/notarization environment is available.

## Progress Log
- `2026-07-15`: User confirmed the build system summary and clarified Unreal-inspired `Debug`, `Test`, `Shipping`, `Game`, and `GameEditor` expectations.
- `2026-07-15`: Plan and tracker created on branch `feature/foundation-build-packaging`.
