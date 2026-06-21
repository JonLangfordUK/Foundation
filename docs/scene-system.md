# Foundation Scene System Guide

## Purpose

The Foundation scene system is a small ECS-first scene stack for Jackdaw-authored games. It lets gameplay and UI move between `.jsn` scenes without inventing a second scene format.

Use it to:

- Open gameplay levels, menus, splash screens, and overlays.
- Close the current scene or a named scene.
- Keep lower scenes visible, interactive, or paused depending on the top scene.
- Load Jackdaw `.jsn` files in standalone runtime and editor Play mode.
- Clean up entities that belong to a scene when that scene leaves the stack.

The system has two halves:

1. **FoundationRuntimeLibrary** owns the generic scene stack model.
2. **TemplateGame** bridges stack requests to Jackdaw scene loading and editor/runtime UI behavior.

```text
Game code / menu buttons / splash screens
                |
                v
        SceneCommand messages
                |
                v
       Foundation SceneStack
                |
                v
     SceneLoadRequested messages
                |
                v
 TemplateGame + Jackdaw load .jsn scene content
```

## Key Concepts

### Scene Stack

The scene stack is the ordered list of currently active scenes. The bottom scene is older; the top scene is newest.

```text
Top    [ Pause Menu Overlay ]  interactive, visible
       [ Gameplay Level     ]  visible, not interactive, not updating
Bottom [ Background         ]  hidden or visible depending on presentation
```

The stack is stored in `SceneStack` from `foundation_runtime_library::scene_stack`.

### SceneCommand

`SceneCommand` is the public mutation API. Systems do not edit `SceneStack` directly. They write commands such as:

- `Open`
- `CloseCurrent`
- `Close(SceneTarget)`
- `Clear`
- `ClearAndOpen`

Foundation processes these commands in `PostUpdate`, then emits lifecycle/load messages.

### SceneSource

`SceneSource` describes where scene content comes from.

Current TemplateGame usage is mostly:

```rust
SceneSource::jsn_level("main_menu.jsn")
```

That means: “Ask the active game/runtime bridge to load this Jackdaw `.jsn` asset.”

### SceneOwner

Every entity spawned for a scene-stack entry should receive:

```rust
SceneOwner { scene_id }
```

When a scene is removed from the stack, Foundation despawns top-level entities with that owner. This is what prevents old menu/gameplay entities from leaking into later scenes.

### ScenePresentation

`ScenePresentation` controls how a scene affects scenes below it.

| Presentation | Lower scene visible? | Lower scene gets input? | Lower scene updates? | Typical use |
| --- | --- | --- | --- | --- |
| `FULLSCREEN` | No | No | No | Main menu, gameplay level |
| `PAUSE_OVERLAY` | Yes | No | No | Pause menu |
| `INPUT_BLOCKING_OVERLAY` | Yes | No | Yes | Splash or modal UI overlay |
| `NON_BLOCKING_OVERLAY` | Yes | Yes | Yes | Debug overlay |

Foundation recomputes `visible`, `interactive`, `updating`, and `focused` flags whenever the stack changes.

## How Jackdaw Fits In

Jackdaw remains the scene-authoring format and editor. Foundation does not replace it.

The intended split is:

- **Jackdaw** authors and serializes scene files under `games/template-game/assets/*.jsn`.
- **FoundationRuntimeLibrary** decides which scene is on the stack and when it should load/unload.
- **TemplateGame** receives `SceneLoadRequested` and performs the actual Jackdaw `.jsn` load.

This keeps scene data editable in Jackdaw while allowing game code to use a predictable stack API.

## Standalone Game Runtime

Standalone runtime starts from `games/template-game/src/main.rs`.

### Startup flow

```text
main.rs
  -> DefaultPlugins with asset root = games/template-game/assets
  -> JackdawPlugin
  -> FoundationPlugin
  -> TemplateGamePlugin
  -> Startup: open_initial_scene
```

`open_initial_scene` writes:

1. `SceneCommand::Clear`
2. Open `splash_background.jsn` as `FULLSCREEN`
3. Open `splash_pixel_perfect.jsn` as `INPUT_BLOCKING_OVERLAY`

### Loading `.jsn` scenes

In standalone builds, `spawn_requested_jackdaw_scenes` reads each `SceneLoadRequested` message and spawns:

```rust
JackdawSceneRoot(scene_handle)
SceneOwner { scene_id }
```

Jackdaw Runtime loads the `.jsn` content through Bevy's asset system. Foundation ownership keeps cleanup tied to the scene stack.

### Runtime scene example

```text
Startup
  Stack: splash_background + splash_pixel_perfect

Pixel Perfect splash completes
  Stack: splash_background + splash_bevy

Bevy splash completes
  Stack cleared, then landing_page opens

Player presses any button
  Stack cleared, then main_menu opens

Player clicks New Game
  Stack cleared, then gameplay_level opens

Player presses Escape in gameplay
  pause_menu opens as PAUSE_OVERLAY
```

## Editor Edit Mode

Editor edit mode is Jackdaw's normal authoring mode. The game is not “playing.”

### What is active

- Jackdaw editor plugins are active.
- The open `.jsn` scene is the authoring scene.
- TemplateGame's runtime scene-stack systems are mostly gated off by `play_gate::is_playing`.
- Foundation splash runtime is disabled.
- Foundation menu systems require `SceneOwner`, so authored editor entities are not accidentally treated as runtime scene-stack copies.

### UI behavior in edit mode

TemplateGame targets authored UI roots to the editor viewport camera while editing. This makes UI scenes visible in the viewport without starting gameplay.

Important markers include:

- `TemplateGameplayUiRoot`
- `FoundationSplashUiRoot`

These tell TemplateGame which UI roots need viewport targeting.

### Mental model

```text
Jackdaw edit mode
  Open .jsn file -> editable ECS entities
  No scene stack runtime loading
  No splash/menu gameplay actions
  UI roots are previewed in the editor viewport
```

Use edit mode to arrange scene entities, author UI roots, add Foundation components, and save `.jsn` files.

## Editor Game / Play Mode

Editor Play mode runs gameplay inside the Jackdaw editor without closing the editor process.

### Entering Play

When Jackdaw enters `PlayState::Playing`, TemplateGame runs:

1. `hide_editor_authored_scene_for_play`
2. `open_initial_scene`

TemplateGame hides the edit-mode authored UI roots so the player only sees runtime scene-stack copies. It also enables Foundation splash runtime and configures UI targeting for the active editor viewport.

### Choosing the first scene

`editor_play_scene_commands` inspects the currently open Jackdaw scene:

| Open editor scene | Play-mode behavior |
| --- | --- |
| No scene or `splash_background.jsn` | Start normal TemplateGame startup flow |
| Splash scene | Open persistent background, then selected splash overlay |
| Any other scene | Clear stack and play that scene directly |

This lets developers press Play while editing a specific menu or level and test it directly.

### Loading `.jsn` scenes in Play mode

Editor Play mode cannot simply rely on the normal asset-spawn path, because runtime entities need immediate `SceneOwner` and `EditorHidden` tags.

Instead, the editor build manually:

1. Reads the requested `.jsn` file from `assets/`.
2. Parses it with `jackdaw_jsn`.
3. Loads inline assets through Jackdaw scene IO.
4. Spawns entities through Jackdaw scene loading.
5. Tags every spawned entity with `SceneOwner` and `EditorHidden`.
6. Restores authored parent/child hierarchy.

This gives Foundation cleanup precise ownership while keeping the editor hierarchy clean.

### Runtime cameras and UI

In Play mode, TemplateGame routes scene UI and cameras into the editor viewport:

- Runtime UI roots are parented into the viewport UI node when available.
- Otherwise, roots target the active viewport camera with `UiTargetCamera`.
- Runtime cameras borrow the viewport render target.
- Editor viewport cameras are reactivated when runtime cameras disappear.

### Exiting Play

When leaving Play mode, TemplateGame:

1. Writes `SceneCommand::Clear`.
2. Despawns remaining `SceneOwner` runtime entities.
3. Disables Foundation splash runtime.
4. Removes temporary splash UI target resources.
5. Resets `FoundationPauseState`.
6. Restores hidden authored edit-mode UI roots.
7. Reactivates editor viewport cameras.

This returns the editor to a clean authoring state.

## How To Use The System

### Opening a scene from Rust

Use `SceneCommand` messages or `SceneCommandsExt` helpers.

```rust
scene_commands.write(SceneCommand::open_with_options(
    SceneSource::jsn_level("options_menu.jsn"),
    OpenSceneOptions::default()
        .with_key("options-menu")
        .with_presentation(ScenePresentation::INPUT_BLOCKING_OVERLAY),
));
```

### Opening a scene from authored UI

Add `FoundationMenuButton` to a Jackdaw-authored button entity.

Common actions:

- `open_scene`
- `open_overlay_scene`
- `clear_and_open_scene`
- `close_current`
- `resume`
- `exit`
- `none`

Example authoring intent:

```text
Button: Options
  FoundationMenuButton
    action: "open_overlay_scene"
    scene_path: "options_menu.jsn"
    scene_key: "options-menu"
```

### Creating a scene asset

For a TemplateGame scene-stack scene:

1. Create or edit a `.jsn` scene in `games/template-game/assets/`.
2. Add a root marker appropriate for the scene, such as:
   - `TemplateGameplayUiRoot` for UI scenes.
   - `FoundationSplashScreen` plus `FoundationSplashUiRoot` and `FoundationSplashText` for splash scenes.
   - `FoundationSimpleGameplayLevel` for the sample generated gameplay level.
3. Add `FoundationUiOrder` to authored UI children when order matters.
4. Reference the file from code constants or authored `FoundationMenuButton` fields.
5. Add or update tests if the scene becomes part of the core flow.

### Choosing presentation

Use `FULLSCREEN` when the new scene should own the screen and stop earlier scenes.

Use `PAUSE_OVERLAY` when gameplay should remain visible but stop updating and receiving input.

Use `INPUT_BLOCKING_OVERLAY` when lower scenes should continue updating but not receive input.

Use `NON_BLOCKING_OVERLAY` only for tools/debug UI that should not block the game.

## Best Practices

- Keep reusable behavior in `foundation-runtime-library`.
- Keep TemplateGame-specific asset paths and editor glue in `games/template-game`.
- Prefer Jackdaw `.jsn` scenes for authored content instead of hard-coding complete scenes in Rust.
- Always tag runtime-spawned scene content with `SceneOwner`.
- Use `SceneCommand` instead of mutating `SceneStack` directly.
- Give important scenes a stable `SceneKey` for debugging and targeted closure.
- Keep UI roots explicit with `TemplateGameplayUiRoot` or `FoundationSplashUiRoot`.
- Add `FoundationUiOrder` to authored UI children when deterministic child order matters.
- Test scene constants and important asset references when adding core flow scenes.
- In editor-only code, keep runtime copies hidden from the editor hierarchy with `EditorHidden`.

## Common Pitfalls

### Missing `SceneOwner`

If spawned entities do not have `SceneOwner`, they will not be cleaned up when the scene closes.

### Wrong presentation

A menu opened as `FULLSCREEN` may hide the gameplay scene. A gameplay scene opened as an overlay may leave earlier scenes visible unexpectedly.

### Running gameplay systems in edit mode

Editor edit mode should preview authored content, not run stack gameplay. Use `play_gate::is_playing` for TemplateGame runtime systems that should only run during standalone or editor Play mode.

### UI not appearing in the editor viewport

Check for the correct UI root marker and ensure Play mode has an active viewport camera or viewport UI node.

### Broken scene path strings

Scene paths are asset-relative, for example `main_menu.jsn`, not absolute paths. Keep constants and authored `.jsn` references in sync.

## Quick Reference

| Task | Use |
| --- | --- |
| Open a `.jsn` scene | `SceneCommand::open(SceneSource::jsn_level(...))` |
| Open a menu over gameplay | `ScenePresentation::PAUSE_OVERLAY` or `INPUT_BLOCKING_OVERLAY` |
| Replace the whole flow | `SceneCommand::ClearAndOpen` or `OpenSceneOptions::clear_stack()` |
| Close current menu | `SceneCommand::CloseCurrent` or `FoundationMenuButton::close_current()` |
| Resume from pause | `FoundationMenuButton::resume()` |
| Clean up runtime entities | Add `SceneOwner { scene_id }` |
| Author UI for viewport/game | Add `TemplateGameplayUiRoot` |
| Author splash UI | Add `FoundationSplashScreen`, `FoundationSplashUiRoot`, and `FoundationSplashText` |
