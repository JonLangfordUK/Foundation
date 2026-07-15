# Foundation Build And Packaging Plan

## Metadata
- Feature slug: `foundation-build-packaging`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/foundation-build-packaging`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-07-15`
- Last updated: `2026-07-15`

## User Request
The user needs a comprehensive build solution for Foundation games that can build, test, package, and eventually distribute any Foundation-built game as a standalone executable named after that game. The system should target selected platforms and configurations, support editor-enabled and game-only builds where appropriate, produce shareable/store-ready packages, and be driven by GitHub workflows running on local Windows and/or Linux agents. The design should be heavily inspired by Unreal's build setup.

The requested build concepts are:
- `Debug`: full debugging, no optimizations.
- `Test`: development features enabled, including console commands, log windows, and other dev tools.
- `Shipping`: no debugging, no dev tools, just the game and assets.
- Target: `Game` or `GameEditor`, with `GameEditor` excluded from `Shipping`.

## Feature Summary
Add a Foundation build and packaging pipeline that turns loose development workspace games into deterministic distributable outputs. The pipeline should define a stable vocabulary for game selection, platform selection, build configuration, target, asset/package layout, and release publishing. It should be configurable per game and per platform, extensible for future stores/signing/installers, and usable both locally and from GitHub Actions self-hosted runners.

## Feature Area Classification
- Area: `multi-area`
- Primary area: `engine`
- Rationale: The feature primarily belongs to Foundation engine/tooling because it defines how games are discovered, built, configured, and packaged. It also touches game manifests/assets and workspace CI/release automation, so it spans engine, game, and workflow/tooling concerns.

## Codebase Research
- `Cargo.toml` is a Cargo workspace with `crates/foundation`, `crates/foundation-runtime-library`, `crates/foundation-editor-library`, `crates/foundation-console-macros`, and `games/template-game`.
- `crates/foundation` is the loose development launcher. It discovers `games/*/foundation.game.toml`, parses `--game <name>` and `--editor`, then spawns `cargo run -p <game-package> -- [--editor]`.
- `games/template-game/foundation.game.toml` currently contains `[game].name = "template-game"` and `[launch].package = "template-game"`; this is the natural place to extend per-game build/package metadata.
- `games/template-game` currently has a thin binary wrapper in `src/main.rs`, a library entry point in `src/lib.rs`, and an `assets/` directory. Packaged builds must preserve the game-owned asset root and produce an executable named for the game.
- `games/template-game/src/lib.rs` detects editor mode from `--editor` at runtime and always depends on `foundation-editor-library`. Shipping/test/debug configuration will need compile-time feature/profile gating in addition to runtime arguments so shipping builds exclude dev/editor systems.
- `crates/foundation-runtime-library` owns reusable runtime systems, including console/debug-related functionality. Build configurations must be able to include or exclude runtime dev tools cleanly.
- `crates/foundation-editor-library` owns editor-time extension points and should only be included in `GameEditor` or equivalent non-shipping configurations.
- Existing project wrappers under `scripts/` provide validation commands that the build system should integrate with rather than replace.

## External Research
- Unreal's packaging documentation describes build configurations as presets that determine how a project is compiled and what code/content are available; Shipping is for distribution where users should not have console/dev capabilities, while Debug/Development-style builds retain debugging and console-oriented functionality. Source: https://dev.epicgames.com/documentation/en-us/unreal-engine/packaging-your-project
- Unreal's build configuration reference uses named configurations such as Debug and Shipping, and combines configuration with target concepts. Source: https://docs.unrealengine.com/4.27/en-US/ProductionPipelines/DevelopmentSetup/BuildConfigurations/
- Rust release automation examples commonly use target matrices, cross-compilation helpers, archives, and GitHub Releases from workflows. Examples found include `houseabsolute/actions-rust-release`, `michaelklishin/rust-build-package-release-action`, and cargo cross-compilation actions. These are references only; the project should prefer a repo-owned build interface that can run locally and inside self-hosted GitHub runners.

## Affected Files And Systems
- `Cargo.toml`: likely add workspace profiles and/or feature unification for Foundation build configurations.
- `crates/foundation`: may remain the loose development launcher, but should not be the final packaged executable for each game unless explicitly requested. It may gain shared manifest/build metadata parsing support or a companion tooling crate may own that responsibility.
- `crates/foundation-runtime-library`: gate dev-only runtime features such as console commands, debug/log windows, and other tools behind explicit features or build configuration resources.
- `crates/foundation-editor-library`: ensure editor-only functionality is only included for `GameEditor` style builds and never for `Shipping`.
- `games/*/foundation.game.toml`: extend to include build/package metadata such as display name, executable name, asset roots, package include/exclude rules, supported targets, and build defaults.
- `games/*/Cargo.toml`: add features for dev tools/editor support and release-oriented binary naming where needed.
- `games/*/src/main.rs` and `games/*/src/lib.rs`: route runtime/editor/dev-tool activation through configuration that can be controlled by build mode.
- `scripts/`: add local wrappers for build, package, test, and release preparation.
- `.github/workflows/`: add self-hosted runner workflow templates for matrix builds, tests, artifacts, and release publishing.
- `docs/`: add user-facing documentation for build modes, targets, configuration files, local commands, CI workflows, and known cross-compilation limits.

## Proposed Implementation Approach
1. Define the Foundation build vocabulary and compatibility matrix:
   - Build configurations: `Debug`, `Test`, `Shipping`.
   - Targets: `Game`, `GameEditor`.
   - Invalid combination: `Shipping + GameEditor`.
   - Platform target: Rust target triple plus a friendly Foundation platform alias such as `windows-x64` or `linux-x64`.
2. Add/extend per-game manifest metadata:
   - Game name, package name, executable name, asset roots, package include/exclude globs, supported platforms, and optional store/release metadata.
   - Keep the manifest data-driven so Foundation does not depend on concrete game crates.
3. Add compile-time feature/profile mapping:
   - `Debug`: debug profile or equivalent, debug assertions enabled, no optimization-focused release stripping.
   - `Test`: optimized or semi-optimized test/development build with dev tools enabled.
   - `Shipping`: release/LTO/strip-oriented build with debug/dev/editor features disabled.
   - `GameEditor`: enables editor features and uses non-shipping-compatible dependencies only.
4. Create a repo-owned build/package command interface:
   - Prefer a small Rust `xtask`/tooling crate or equivalent script entry point that accepts `--game`, `--platform`, `--configuration`, `--target`, and `--output`.
   - Scripts should call this command so local and CI usage share the same logic.
5. Implement packaging layout:
   - Produce deterministic output under `artifacts/packages/<game>/<platform>/<configuration>/<target>/`.
   - Copy executable, assets, manifests/config, licenses/notices, and any platform runtime files.
   - Produce an archive suitable for sharing/uploading.
6. Add validation commands:
   - Validate build matrix inputs and reject invalid combinations early.
   - Run existing format/lint/test/build/doc wrappers.
   - Add packaging smoke checks that verify executable presence, asset presence, and manifest metadata.
7. Add GitHub workflow support for self-hosted runners:
   - Matrix over game/platform/configuration/target.
   - Separate jobs for validation, package artifacts, and release upload.
   - Support Windows and Linux runner labels.
   - Document cross-compilation prerequisites and fallbacks when a target cannot be built from a host.
8. Add release publishing support:
   - Start with GitHub release artifact upload.
   - Keep store upload/signing/installers as extension points rather than initial hard dependencies.

## Build Matrix Draft
| Configuration | Game target | GameEditor target | Dev tools | Debugging | Optimization intent | Distribution intent |
| --- | --- | --- | --- | --- | --- | --- |
| `Debug` | Allowed | Allowed | Enabled | Full | None/minimal | Local debugging only |
| `Test` | Allowed | Allowed | Enabled | Limited/useful | Development/testing optimized | Internal QA/shareable test builds |
| `Shipping` | Allowed | Not allowed | Disabled | Disabled/stripped | Release/store optimized | Public/store distribution |

## Alternatives Considered
- Hard-code one script per game/platform/configuration: rejected because it would not scale to multiple Foundation games and would make store/distribution extension difficult.
- Use only GitHub Actions YAML without a local build tool: rejected because local agents and developers need the same behavior outside CI.
- Make `crates/foundation` the packaged executable for every game: deferred/likely rejected for shipping because the user wants standalone executables named after each game, and the Foundation architecture notes static bundled builds as a future distribution mode distinct from loose development launching.
- Depend entirely on existing third-party Rust release actions: rejected as the main interface because Foundation needs game/editor/shipping semantics; third-party actions may still be useful implementation references.

## Risks, Constraints, And Assumptions
- Cross-compiling games that use Bevy/wgpu may require platform SDKs, linkers, graphics/runtime libraries, and target-specific setup that cannot always be made universal from any host.
- Windows-to-Linux and Linux-to-Windows builds may be possible for some targets, but store-ready packages may still require native signing, installer tooling, or platform-specific validation.
- Cargo feature unification can accidentally include dev/editor code in shipping builds if feature boundaries are not designed carefully.
- The existing `template-game` depends directly on `foundation-editor-library`; shipping exclusion may require feature-gating that dependency.
- Asset packaging must avoid copying dev-only files into shipping output.
- Store requirements vary substantially, so initial support should focus on deterministic packages and GitHub Releases, with stores as future adapters.

## Open Questions
- Should the initial implementation use an `xtask` Rust crate, PowerShell scripts, or both? Recommendation: a Rust `xtask` owns logic, scripts are thin wrappers.
- Which exact target aliases should be supported first: `windows-x64`, `linux-x64`, and/or musl Linux?
- Should `Test` builds use Cargo `release` with dev-tool features, a custom Cargo profile, or a separate profile derived from `release`?
- Which files are mandatory in every package: license, readme, credits, game manifest, default settings, crash/log config?
- Should release upload create GitHub Releases in this repo only, or support pushing artifacts to another repository as a first-class option?
- What store targets should be prioritized after GitHub Releases?

## Documentation Expectations
- Public APIs added or changed by this feature must have Rustdoc comments, or this plan must explicitly justify why they are internal/undocumented.
- Add feature-level documentation under `docs/`, likely `docs/build-packaging.md`, covering build vocabulary, manifest schema, commands, package layout, examples, CI runner setup, and cross-compilation limitations.
- Document the exact meaning of `Debug`, `Test`, and `Shipping`, and the `Game` versus `GameEditor` target matrix.
- Document how to add a new game to the build system.
- Generated documentation must be produced before the feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Read `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/feature-plan-docs/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, `.pi/skills/gitflow-workflow/SKILL.md`, and `.pi/skills/foundation-architecture/SKILL.md` before implementation edits.
- Confirm the active branch is `feature/foundation-build-packaging` before implementation edits.
- Keep `crates/foundation` independent from concrete game crates.
- Do not add Jackdaw dependencies.
- Treat `Shipping + GameEditor` as invalid from the first implementation phase.
- Be conservative with cross-compilation claims; validate and document actual host/target support rather than promising unsupported combinations.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Verify shipping builds cannot include console commands, log windows, editor plugins, or debug-only systems by accident.
- Verify the build/package tool is data-driven and does not hard-code `template-game` except in tests/examples.
- Verify GitHub workflows use self-hosted runner labels safely and do not assume secrets are available on untrusted forks.
- Verify package contents are deterministic and exclude dev-only files.

## Success Criteria
- A developer can run one command to build/package a selected game for a selected platform, configuration, and target.
- The produced executable is named after the selected game.
- `Debug`, `Test`, and `Shipping` have documented and enforced semantics.
- `Game` and `GameEditor` targets are supported where valid, and `Shipping + GameEditor` is rejected.
- Shipping packages exclude dev tools, editor features, and debug-only assets/configuration.
- Test packages include dev tools such as console/log tooling.
- The build configuration is manifest/config driven and extendable per game/platform.
- GitHub workflow templates can build/test/package on self-hosted Windows/Linux runners and upload artifacts/releases.
- Documentation explains local usage, CI usage, package layout, and cross-compilation prerequisites/limits.

## Testing Methodology
- `scripts/format-project.cmd`
- `scripts/lint-project.cmd`
- `scripts/test-project.cmd`
- `scripts/compile-project.cmd`
- `scripts/doc-project.cmd`
- `scripts/validate-project.cmd`
- Focused tests for build matrix parsing and invalid combinations.
- Focused tests for manifest package metadata parsing.
- Packaging smoke tests that verify output executable/assets/manifests exist and dev-only files are excluded from shipping packages.
- CI workflow dry-run/review where full self-hosted runner execution is unavailable.
