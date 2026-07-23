# Foundation Scene System Guide

## Purpose

The Foundation scene system is an ECS-first scene stack for Bevy games. Foundation owns stack state, lifecycle messages, scene ownership, and reusable scene behaviors. Games own concrete scene catalogs.

TemplateGame currently defines scenes in Rust with Bevy 0.19 BSN (`bsn!`). Bevy does not currently ship a first-party `.bsn` asset loader, so BSN scene definitions are code-authored for now. The TemplateGame scene catalog lives under `games/template-game/src/scenes/`, with each splash/menu scene in its own Rust module.

## Architecture

```text
Foundation engine (`cargo run -p foundation -- --game template-game`)
        |
        v
Game extension manifest (`games/template-game/foundation.game.toml`)
        |
        v
FoundationRuntimeLibrary
  - SceneStack
  - SceneCommand messages
  - SceneLoadRequested messages
  - SceneOwner cleanup
        |
        v
TemplateGame scene catalog
  - BSN scene functions
  - scene-key routing
  - game-specific plugin glue
```

`foundation-editor-library` remains as a Bevy-only editor-time extension point. Launching with `--editor` enables that shell:

```cmd
cargo run -p foundation -- --game template-game --editor
```

## Scene Stack

The scene stack is an ordered list of active scenes. The bottom scene is older; the top scene is newest.

```text
Top    [ Pause Menu Overlay ]  visible, focused, blocks gameplay input/update
       [ Gameplay Level     ]  visible, paused by overlay
Bottom [ Main Menu          ]  removed before gameplay in the current flow
```

Systems mutate the stack by writing `SceneCommand` messages. Foundation processes commands, emits lifecycle/load messages, and removes entities tagged with `SceneOwner` when their scene leaves the stack.

## Scene Sources

`SceneSource::bsn_scene("template-game/main_menu")` identifies a BSN scene key. Foundation can now resolve these keys through the temporary `.bsn` asset bridge installed by `FoundationBsnAssetPlugin`.

Games register stable scene keys with `FoundationBsnSceneRegistry`:

```rust
registry.register_scene("last-beacon/main_menu", "scenes/main_menu.bsn");
```

If a key is not registered, Foundation treats the key as a direct asset path relative to the active assets directory. This makes `SceneSource::bsn_scene("levels/intro.bsn")` useful for simple level and prefab loading without a separate catalog.

Non-shipping builds can also use the startup scene override argument. A single value opens one BSN scene instead of the game's default startup flow:

```cmd
--scene last-beacon/main_menu
--scene scenes/main_menu.bsn
```

A bracketed list opens scenes in order as a startup stack. Foundation trims whitespace around commas, so all of these list separators are valid:

```cmd
--scene "[last-beacon/gameplay_level,scenes/testing_mode.bsn]"
--scene "[last-beacon/gameplay_level, scenes/testing_mode.bsn]"
--scene "[last-beacon/gameplay_level , scenes/testing_mode.bsn]"
```

The first override scene clears the stack; later entries are opened above it. If no `--scene` argument is present, games should emit their normal default startup scene commands.

Non-shipping builds can also use the debug console to reopen scenes while the game is already running:

```text
open last-beacon/main_menu
open last-beacon/gameplay_level last-beacon/pause_menu
open scenes/main_menu.bsn
```

The `open` command clears the current scene stack, opens the first scene fresh, and then opens each later scene above it in order. Console predictions for `open` arguments list registered scene keys from `FoundationBsnSceneRegistry`, such as `last-beacon/main_menu`. These predictions update while the user types and can appear before `open` is fully typed, so `op` can already preview full commands such as `open last-beacon/main_menu`. Scene-key prediction searches within registered keys, so `open map` can match a registered key such as `last-beacon/mapmap`. Direct asset-relative `.bsn` paths remain valid when typed explicitly, but they are not predicted unless registered as scene keys.

`SceneSource::runtime(SceneKey::new("debug-overlay"))` remains available for system-authored runtime scenes.

## Scene Presentation

| Presentation | Lower visible? | Lower input? | Lower updates? | Typical use |
| --- | --- | --- | --- | --- |
| `FULLSCREEN` | No | No | No | Splash, main menu, gameplay |
| `PAUSE_OVERLAY` | Yes | No | No | Pause menu |
| `INPUT_BLOCKING_OVERLAY` | Yes | No | Yes | Options/modal menu |
| `NON_BLOCKING_OVERLAY` | Yes | Yes | Yes | Debug overlay |

## Current TemplateGame Flow

```text
Startup
  -> open template-game/splash_pixel_perfect

Pixel Perfect splash completes
  -> template-game/splash_bevy

Bevy splash completes
  -> clear stack and open template-game/main_menu

Main menu
  -> New Game: clear stack and open template-game/gameplay_level
  -> Options: open template-game/options_menu overlay
  -> Exit: request AppExit

Gameplay
  -> Escape: open template-game/pause_menu as pause overlay

Pause menu
  -> Resume: close pause overlay and unpause
  -> Options: open options overlay
  -> Main Menu: clear stack and open main menu
```

## Ownership Rules

Every root entity spawned for a stack scene should receive:

```rust
SceneOwner { scene_id }
```

Foundation cleanup removes owned entities when a scene leaves the stack. Generated UI or gameplay entities should inherit the same owner so they do not leak across scene transitions.

## Async Scene Preparation And Cache

Foundation scene opens now flow through a preparation/activation lifecycle instead of constructing BSN scene content directly on the click-to-transition path.

### Preparation lifecycle

A scene source can be requested in two ways:

- explicitly through `SceneCommand::Preload` / `SceneCommandsExt::preload_scene`, or
- implicitly because an `Open` / `ClearAndOpen` transition batch needs that source before the stack can mutate.

Foundation tracks public preparation state in `ScenePreparationRegistry`:

- `Requested`
- `AssetLoading`
- `Resolving`
- `ApplyingTopLevel`
- `DiscoveringNestedWork`
- `PreparingNestedWork`
- `Ready`
- `Activating`
- `Active`
- `Failed`

BSN scene preparation stays asynchronous from the caller's perspective:

1. Foundation queues `ScenePreloadRequested` for the target `SceneSource`.
2. `FoundationBsnAssetPlugin` spawns a hidden prepared root and starts the Bevy asset load.
3. The `.bsn` asset resolves and `ScenePatch::apply(...)` runs against the hidden prepared root, never against a visible transition target.
4. Foundation propagates `ScenePreparationContext` through the hidden prepared subtree so nested loaders and runtime scene generators can register readiness tokens against the source before it enters the stack.
5. `ScenePreloadReady` is emitted only after top-level apply is complete and every registered readiness token has settled. In other words, `Ready` means fully spawned, hidden, non-interactable, dependency-settled, and cached for activation — not merely "the top-level BSN patch applied."
6. A queued transition batch activates only after every required scene source is `Ready`.

If preparation fails, Foundation records `Failed` and emits `ScenePreloadFailed` / `SceneTransitionFailed` so callers can recover rather than hanging forever on a hidden or half-built scene.

### Transition batches

Foundation now queues stack mutations in batches. Any batch containing one or more scene opens waits until all target scene sources are prepared. While that wait is in progress, `SceneTransitionStatus` exposes:

- whether a transition is pending,
- which sources the active batch is still waiting on, and
- how many transition batches are queued.

This is the intended backbone for future loading-screen UI: a loading scene can inspect `SceneTransitionStatus` and react to `ScenePreloadReady` / `ScenePreloadFailed` without re-implementing scene-open logic.

### Cached activation

Prepared BSN roots are cached off-stack in a hidden state. When a queued batch finally activates, Foundation reuses the prepared root instead of re-running the expensive BSN resolve/apply work on the transition frame.

Activation does three things:

1. remove the prepared-cache marker,
2. assign the new active `SceneOwner` to the root and all authored descendants, and
3. immediately request a refill preload for the same source so frequently-used scenes can stay warm for the next transition.

The cached prepared root is consumed by activation; Foundation then begins refilling the cache entry in the background. Refill work is preparation work, not active-scene readiness work: it must never make the active scene unready again.

`FoundationBsnPreparationBudget` limits how many ready BSN patches Foundation applies in one frame. The default is one apply per frame, which prevents a preload/refill burst from applying every prepared scene in the same gameplay frame. This does not make a single `ScenePatch::apply` call itself multi-threaded, but it keeps multiple scene dependencies from stacking into one large hitch.

### Scene-authored preload relationships

Games can register likely next-scene relationships through `ScenePreloadRegistry`. When a scene is added or regains focus, Foundation requests preloads for its registered targets. This keeps common UI transitions warm without every game re-implementing scene prediction logic.

## Readiness Gating (Scene Visibility)

A scene's `visible` stack flag (from `ScenePresentation`) and its actual on-screen visibility are two separate concerns. Stack presentation decides whether a scene *should* be visible (not covered by another scene); readiness decides whether its content is *actually built and styled* yet.

`SceneContentLoading` is a marker component games and Foundation systems attach to a `SceneOwner`-tagged entity — the scene root or any descendant — while that entity's content is still loading or applying:

```rust
commands.entity(entity).insert(SceneContentLoading);
// ... once the entity's content is final (success or failure) ...
commands.entity(entity).remove::<SceneContentLoading>();
```

`sync_scene_entity_visibility` only shows a scene-owned root once **both** are true:

- the scene stack says the scene should be visible (`SceneStack::is_visible`), and
- no entity that scene owns still carries `SceneContentLoading`.

This closes the gap where a scene used to become visible the instant it was pushed onto the stack, before any of its authored content existed — producing a visible "pop" once the content actually finished loading a few frames later. Foundation's `.bsn` asset bridge (`bsn_assets.rs`) uses this directly: every scene-owned BSN root spawns `Visibility::Hidden` + `SceneContentLoading`, and both are cleared once `scene_patch.apply(...)` completes (whether it succeeds or fails — a broken load must still reveal whatever content exists rather than hiding the scene forever). Standalone (non scene-stack) BSN prefabs spawned via `spawn_bsn_asset` are not gated by the scene stack; they reveal themselves directly once their own apply completes.

Game code that starts its own nested asset loads in an active scene can follow the same pattern: insert `SceneContentLoading` on the entity that starts a new load, and remove it once that load settles. Doing so keeps the owning scene hidden until every nested load finishes too, instead of the scene's shell appearing first and individual pieces popping in afterward.

For off-stack prepared scenes, nested loaders should use readiness tokens instead. `ScenePreparationContext` identifies the source being prepared before a `SceneOwner` exists, and `ScenePreparationRegistry::request_readiness_token` / `settle_readiness_token` let game or runtime systems block `ScenePreloadReady` until their nested work has settled. This is the preferred path for reusable widget BSNs and runtime-generated scene content that must be ready before cached activation.

Readiness gating is purely additive to presentation: a fully-ready scene that is covered by a scene above it still hides, and a stack-visible-but-loading scene stays hidden regardless of presentation. Both conditions must hold together.

### Profiling BSN apply stalls

Foundation's temporary `.bsn` bridge emits tracing spans around scene resolution and `ScenePatch::apply` under `foundation_bsn_instance` and `foundation_bsn_apply`. Enable Bevy's profiling features (for example `bevy/trace_chrome` or `bevy/trace_tracy`) to capture these spans in a timeline profiler.

For lightweight log-based profiling, set `FOUNDATION_BSN_PROFILE_MS=<milliseconds>` before launching a game. Foundation logs any BSN resolve/apply step that takes at least that long, including the asset path and root entity. This is intended to distinguish scene-stack visibility/readiness timing from the synchronous main-thread cost of constructing a large BSN scene.

## BSN Asset Authoring Rules

Foundation includes a temporary `.bsn` asset bridge for Bevy 0.19. Bevy currently ships the `bsn!` macro, but not the official file-backed `.bsn` asset loader. Foundation's bridge is intentionally isolated in `foundation-runtime-library` so it can be removed when Bevy provides first-party support.

Use `.bsn` assets for ECS-authored levels and prefabs:

```text
game/assets/scenes/main_menu.bsn
game/assets/scenes/gameplay_level.bsn
game/assets/prefabs/loot_crate.bsn
```

Rules:

- Keep concrete game scene and prefab files in the game asset directory.
- Use `.bsn` for static entity/component hierarchy where practical.
- Keep Rust glue for runtime behavior that cannot live in static assets, such as systems, resources, scene transition drivers, and strongly typed callbacks.
- Register stable scene keys with `FoundationBsnSceneRegistry` when user-facing keys should not expose asset paths.
- Keep reusable loader, spawn, and hot-reload behavior in `foundation-runtime-library`.
- Do not use `.bsn` as a general data format for arbitrary non-ECS content unless a later plan explicitly expands the scope.

### Hot reload behavior

When Bevy reports that a loaded `.bsn` `ScenePatch` changed, Foundation's bridge performs whole-instance replacement:

1. Find live `FoundationBsnInstance` roots using that asset.
2. Despawn each old root recursively, removing authored children with it.
3. Spawn a fresh root from the reloaded `.bsn` asset.
4. Reapply Foundation scene ownership and parent attachment context that belongs to the instance.

Foundation does not attempt in-place diffing or gameplay-state preservation. Entity references into a reloaded prefab or level may become stale after replacement. This is an accepted development-time tradeoff for simple, deterministic hot reload.

## Build Modes Direction

Foundation is intended to support:

1. **Static bundled games** for distributed single-executable builds.
2. **Loose game modules** for future development/multi-game engine installs.

The current launcher discovers game manifests and forwards to the selected game package, so the engine does not depend on concrete game crates. Static bundled builds remain a future distribution mode.
