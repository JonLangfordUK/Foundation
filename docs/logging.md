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
