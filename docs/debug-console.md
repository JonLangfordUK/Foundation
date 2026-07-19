# Foundation Debug Console Guide

## Purpose

The Foundation debug console is a reusable runtime console for Foundation games. It opens as a Foundation scene-stack overlay and is available to the engine plus the currently compiled game binary.

Commands from other games are not loaded and filtered at runtime. They are unavailable because those game crates are not linked into the selected game's executable.

## Opening The Console

Press the backtick key:

```text
`
```

The console opens at the bottom of the screen. Press backtick again, or press Escape while the console input is focused, to close it.

The console uses an input-blocking scene-stack overlay. Gameplay continues updating underneath, but lower scenes should not receive input while the console is open.

## Running Commands

Most commands use a command name followed by named arguments:

```text
example.say-hello name=Jon
```

Named arguments use `name=value` syntax. Invalid tokens are reported in the console output.

The built-in scene command uses positional scene arguments instead:

```text
open last-beacon/main_menu
open last-beacon/gameplay_level last-beacon/pause_menu
open scenes/main_menu.bsn
```

`open` clears the current Foundation scene stack and opens the supplied BSN scenes in order. Registered scene keys are resolved through `FoundationBsnSceneRegistry`. Direct `.bsn` paths are also accepted when typed explicitly, and those paths are relative to the active assets directory.

The console provides:

- Enter to run the current command.
- Tab to complete the first sorted suggestion.
- A floating suggestion list above the history/output area with all matching predictions.
- Registered BSN scene key predictions for `open` arguments, such as `open las` predicting `last-beacon/main_menu`.
- Up and Down to navigate command history.
- Mouse wheel to scroll console output.
- Persistent command history at `<executable-dir>/saved/console/history.json`.

## Declaring A Command With Inputs

Commands are ordinary Rust functions annotated with `#[console_command]`. User-provided command arguments are grouped in a named input struct that derives `ConsoleCommandInput`.

```rust
use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

#[derive(Clone, Debug, ConsoleCommandInput)]
pub struct SayHelloInputs {
    /// Name that should be greeted by the command.
    pub name: String,
}

#[console_command(name = "example.say-hello")]
pub fn say_hello(inputs: ConsoleInputs<SayHelloInputs>) {
    info!("Hello, {}!", inputs.name);
}
```

The input struct fields become the console's named parameters. For the example above, the console expects:

```text
example.say-hello name=<String>
```

## Command Names

By default, a command uses the Rust function name:

```rust
#[console_command]
pub fn template_game_greeting(inputs: ConsoleInputs<GreetingInputs>) {
    // Registered as `template_game_greeting`.
}
```

Use `name = "..."` to override the in-console name:

```rust
#[console_command(name = "example.say-hello")]
pub fn say_hello(inputs: ConsoleInputs<SayHelloInputs>) {
    // Registered as `example.say-hello`.
}
```

## Bevy System Parameters

Console commands can use Bevy-system-style parameters for engine/game state, and `ConsoleInputs<T>` for user input.

```rust
#[derive(Clone, Debug, ConsoleCommandInput)]
pub struct SetClearColorInputs {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[console_command]
pub fn set_clear_color(
    mut clear_color: ResMut<ClearColor>,
    inputs: ConsoleInputs<SetClearColorInputs>,
) {
    clear_color.0 = Color::srgb(inputs.red, inputs.green, inputs.blue);
}
```

In this example:

- Bevy fills `ResMut<ClearColor>`.
- The console fills `ConsoleInputs<SetClearColorInputs>` from named command arguments.

## Crate Requirements

Crates that define console commands should depend on:

```toml
foundation-runtime-library.workspace = true
linkme.workspace = true
```

`linkme` is currently required because command registration uses a distributed slice in the linked game binary.

## TemplateGame Example

TemplateGame includes a command named:

```text
example.say-hello
```

Example invocation:

```text
example.say-hello name=Jon
```
