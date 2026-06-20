# Foundation Scene Stack

FoundationLibrary provides an ECS-first scene stack for Jackdaw-style games. It is designed for both gameplay levels and UI overlays while preserving Jackdaw `.jsn` files as the authored level format.

## Core model

- `SceneStack` is the authoritative stack resource.
- `SceneCommand` messages request stack changes.
- `SceneAdded`, `SceneRemoved`, `SceneFocused`, and `SceneUnfocused` messages announce lifecycle transitions.
- `SceneLoadRequested` tells bridge systems to load or assemble scene content.
- `SceneOwner` tags entities that belong to a stacked scene.

Scenes stay resident while they are on the stack. When a scene is removed, FoundationLibrary despawns entities tagged with that scene's `SceneOwner`.

## Presentation policy

Each opened scene has a `ScenePresentation`:

- `covers_previous`: lower scenes are not visible.
- `blocks_previous_input`: lower scenes are not interactive.
- `blocks_previous_update`: lower scenes do not update/simulate.

Useful presets are provided:

- `ScenePresentation::FULLSCREEN`
- `ScenePresentation::PAUSE_OVERLAY`
- `ScenePresentation::INPUT_BLOCKING_OVERLAY`
- `ScenePresentation::NON_BLOCKING_OVERLAY`

Runtime flags on stack entries are derived from the stack and presentation policies; they should not be treated as independent authoritative state.

## Command flow

Game code can queue commands through `SceneCommandsExt`:

```rust
use foundation_library::prelude::*;

fn open_level(mut commands: bevy::prelude::Commands) {
    commands.open_scene(SceneSource::jsn_level("levels/level_01.jsn"));
}
```

Available helpers include:

- `open_scene`
- `open_scene_with_options`
- `close_current_scene`
- `close_scene`
- `clear_scenes`
- `clear_and_open_scene`

Commands are processed in `PostUpdate` in queued order. Multiple commands may be issued in one frame.

## Jackdaw `.jsn` bridge

FoundationLibrary does not define a competing level format. Use `SceneSource::jsn_level(...)` for Jackdaw-authored scene files.

When a scene is opened, FoundationLibrary emits `SceneLoadRequested { scene_id, source }`. A Jackdaw-compatible loading system should:

1. Read `SceneLoadRequested`.
2. Load or assemble the requested scene source.
3. Tag every spawned entity with `SceneOwner { scene_id }`.

That ownership tag is what allows FoundationLibrary to clear scene memory when the scene leaves the stack.
