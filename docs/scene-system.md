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

`SceneSource::bsn_scene("template-game/main_menu")` identifies a BSN scene key. The active game catalog resolves that key to Rust-authored BSN content.

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

## BSN Authoring Rules

- Define concrete game scenes in the game crate as Rust scene functions.
- Use BSN (`bsn!`) for scene structure where practical.
- Use small imperative helpers only where scene ownership, runtime resources, or interaction wiring is clearer outside the macro.
- Keep reusable behavior in `foundation-runtime-library` and concrete scene keys/content in the game crate.

## Build Modes Direction

Foundation is intended to support:

1. **Static bundled games** for distributed single-executable builds.
2. **Loose game modules** for future development/multi-game engine installs.

The current launcher discovers game manifests and forwards to the selected game package, so the engine does not depend on concrete game crates. Static bundled builds remain a future distribution mode.
