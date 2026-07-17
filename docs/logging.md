# Foundation Logging

Foundation owns the default logging policy shared by Foundation games.

## Runtime behavior

By default, Foundation games keep visible log output quiet. A normal launch should
not open or rely on a log window.

Pass `--log` to request visible log output in non-shipping builds:

```cmd
cargo run -p foundation -- --game <game-name> --log
```

Packaged-style runs can forward the same runtime flag after `--`:

```cmd
scripts\foundation-build.cmd run --project <game> --configuration test --target game -- --log
```

Shipping builds ignore `--log` for visible log output. This keeps public builds
from exposing development logging windows even if a user passes the flag.

Visible log lines use Foundation's readable terminal formatter:

```text
INFO  [Bevy              ] bevy_render::renderer │ AdapterInfo { ... }
WARN  [Foundation Runtime] foundation_runtime_library::scene_stack │ Missing scene key ...
ERROR [Last Beacon       ] last_beacon::scenes │ Failed to load scene ...
```

The formatter adds severity colors and source-category colors with ANSI terminal
roles instead of hard-coded RGB values. In PowerShell or Windows Terminal, those
colors and the font come from the current terminal profile/theme. On Windows,
Foundation first tries to attach to the parent PowerShell/Windows Terminal
console before allocating a fallback console. Foundation does not set a custom
GUI font for logs; when no parent terminal exists on Windows, `--log` may fall
back to a normal Windows console.

Foundation derives categories from tracing targets, so Bevy logs are wrapped as
`Bevy` without changing the Bevy codebase. Foundation runtime, Foundation engine,
Foundation editor, Last Beacon, TemplateGame, Rust, and third-party targets get
separate labels where their crate/module targets are identifiable.

## Log files

Non-shipping builds write a normal run log beside the executable:

```text
<exe-dir>/saved/logs/latest.log
```

`latest.log` is truncated when the process starts, so each run replaces the
previous normal run log.

If a Rust panic reaches Foundation's panic hook, Foundation copies the current
normal log to a timestamped crash log:

```text
<exe-dir>/saved/logs/crash-<timestamp>.log
```

Crash logs are never intentionally overwritten. If multiple panics happen in the
same timestamp bucket, Foundation adds a numeric suffix.

Hard process aborts, operating-system access violations, GPU driver crashes, or
builds compiled with aborting panic behavior may not run Rust panic hooks. Those
failure modes may exit before Foundation can preserve a crash log.

## Game integration

Games should configure Bevy logging through Foundation before adding
`DefaultPlugins`:

```rust
use foundation_runtime_library::prelude::*;

App::new().add_plugins(DefaultPlugins.set(foundation_log_plugin()));
```

A game can continue customizing other Bevy default plugins by building the plugin
group and setting the Foundation log plugin on that group.

On Windows, games that should not create a console window by default should use a
Windows-subsystem executable entry point and let Foundation allocate a console
only when `--log` is requested in a non-shipping build.
