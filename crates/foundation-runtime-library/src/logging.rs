//! Foundation logging policy for game runtimes.
//!
//! Games should use [`foundation_log_plugin`] when configuring Bevy
//! `DefaultPlugins`. The helper keeps normal Foundation runs quiet by default,
//! enables visible logs only when `--log` is requested in non-shipping builds,
//! writes an overwritten per-run log file for non-shipping builds, and preserves
//! a timestamped panic log when a Rust panic reaches the panic hook.

use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    panic,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, MutexGuard},
    time::{SystemTime, UNIX_EPOCH},
};

use bevy::{
    log::{
        tracing_subscriber::{fmt::MakeWriter, Layer},
        BoxedFmtLayer, BoxedLayer, LogPlugin,
    },
    prelude::*,
};

/// Runtime argument that requests a visible Foundation log window.
pub const FOUNDATION_LOG_ARGUMENT: &str = "--log";

/// Directory below the executable root where Foundation stores runtime logs.
pub const FOUNDATION_LOG_DIRECTORY: &str = "saved/logs";

/// Per-run log file name that Foundation overwrites every non-shipping launch.
pub const FOUNDATION_LATEST_LOG_FILE_NAME: &str = "latest.log";

/// Prefix used for timestamped crash logs that Foundation never overwrites.
pub const FOUNDATION_CRASH_LOG_FILE_PREFIX: &str = "crash";

/// Runtime paths selected by Foundation logging setup.
#[derive(Clone, Debug, Resource)]
pub struct FoundationLoggingPaths {
    /// Directory that stores normal and crash logs.
    pub log_directory: PathBuf,
    /// Per-run log file that is truncated when the process starts.
    pub latest_log_file_path: PathBuf,
}

/// Returns a Bevy [`LogPlugin`] configured with Foundation's logging policy.
///
/// Add this with `DefaultPlugins.set(foundation_log_plugin())` before adding
/// [`FoundationPlugin`](crate::FoundationPlugin). Shipping builds that compile
/// without Foundation `dev-tools` suppress visible log output and skip file log
/// creation, while non-shipping builds write `saved/logs/latest.log` and show
/// formatted logs only when `--log` is present.
pub fn foundation_log_plugin() -> LogPlugin {
    LogPlugin {
        custom_layer: foundation_log_file_layer,
        fmt_layer: foundation_log_visibility_layer,
        ..default()
    }
}

/// Returns true when this build includes Foundation non-shipping logging support.
pub fn foundation_file_logging_enabled() -> bool {
    cfg!(feature = "dev-tools")
}

/// Returns true when the current process arguments request visible log output.
pub fn foundation_log_window_requested_from_environment() -> bool {
    foundation_log_window_requested(std::env::args())
}

/// Returns true when the provided arguments contain the Foundation log flag.
pub fn foundation_log_window_requested(arguments: impl IntoIterator<Item = String>) -> bool {
    arguments
        .into_iter()
        .any(|argument| argument == FOUNDATION_LOG_ARGUMENT)
}

/// Returns true when Foundation should emit visible formatted log output.
pub fn foundation_should_show_log_window(
    arguments: impl IntoIterator<Item = String>,
    file_logging_enabled: bool,
) -> bool {
    file_logging_enabled && foundation_log_window_requested(arguments)
}

/// Returns the executable-relative directory that stores Foundation logs.
pub fn foundation_log_directory_from_executable(
    executable_file_path: &Path,
) -> io::Result<PathBuf> {
    let executable_directory = executable_file_path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Executable path has no parent directory for Foundation logs.",
        )
    })?;

    Ok(executable_directory.join(FOUNDATION_LOG_DIRECTORY))
}

/// Returns the normal per-run log path under a Foundation log directory.
pub fn foundation_latest_log_file_path(log_directory: &Path) -> PathBuf {
    log_directory.join(FOUNDATION_LATEST_LOG_FILE_NAME)
}

/// Returns a timestamped crash log path that does not currently exist.
pub fn foundation_unique_crash_log_file_path(log_directory: &Path) -> PathBuf {
    let timestamp = foundation_timestamp_for_file_name(SystemTime::now());

    // Try a small monotonic suffix range so repeated panics in one second never overwrite.
    for crash_log_suffix in 0..1_000_u32 {
        let crash_log_file_name = if crash_log_suffix == 0 {
            format!("{FOUNDATION_CRASH_LOG_FILE_PREFIX}-{timestamp}.log")
        } else {
            format!("{FOUNDATION_CRASH_LOG_FILE_PREFIX}-{timestamp}-{crash_log_suffix}.log")
        };
        let crash_log_file_path = log_directory.join(crash_log_file_name);
        if !crash_log_file_path.exists() {
            return crash_log_file_path;
        }
    }

    // Fall back to nanoseconds if the suffix range was exhausted by an extreme failure loop.
    let fallback_timestamp = foundation_precise_timestamp_for_file_name(SystemTime::now());
    let crash_log_file_name =
        format!("{FOUNDATION_CRASH_LOG_FILE_PREFIX}-{fallback_timestamp}.log");
    log_directory.join(crash_log_file_name)
}

fn foundation_log_file_layer(app: &mut App) -> Option<BoxedLayer> {
    if !foundation_file_logging_enabled() {
        return None;
    }

    let logging_paths = foundation_logging_paths_from_current_executable().ok()?;
    let latest_log_file = open_latest_log_file(&logging_paths).ok()?;
    let shared_log_file = SharedLogFile::new(latest_log_file);

    install_foundation_panic_hook(logging_paths.clone(), shared_log_file.clone());
    app.insert_resource(logging_paths.clone());

    let file_log_layer = bevy::log::tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(shared_log_file)
        .boxed();

    Some(file_log_layer)
}

fn foundation_log_visibility_layer(_app: &mut App) -> Option<BoxedFmtLayer> {
    let should_show_log_window =
        foundation_should_show_log_window(std::env::args(), foundation_file_logging_enabled());

    if should_show_log_window {
        show_platform_log_window_if_available();
        return Some(Box::new(
            bevy::log::tracing_subscriber::fmt::Layer::default().with_writer(std::io::stderr),
        ));
    }

    // Bevy's default `None` would install a stderr formatter. Use a sink layer to keep
    // Foundation game launches quiet unless the user explicitly asks for visible logs.
    Some(Box::new(
        bevy::log::tracing_subscriber::fmt::Layer::default().with_writer(std::io::sink),
    ))
}

fn foundation_logging_paths_from_current_executable() -> io::Result<FoundationLoggingPaths> {
    let executable_file_path = std::env::current_exe()?;
    let log_directory = foundation_log_directory_from_executable(&executable_file_path)?;
    let latest_log_file_path = foundation_latest_log_file_path(&log_directory);

    Ok(FoundationLoggingPaths {
        log_directory,
        latest_log_file_path,
    })
}

fn open_latest_log_file(logging_paths: &FoundationLoggingPaths) -> io::Result<File> {
    fs::create_dir_all(&logging_paths.log_directory)?;
    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&logging_paths.latest_log_file_path)
}

fn install_foundation_panic_hook(
    logging_paths: FoundationLoggingPaths,
    shared_log_file: SharedLogFile,
) {
    let previous_panic_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        let _ = writeln!(
            shared_log_file.lock_for_direct_write(),
            "Foundation panic captured: {panic_info}"
        );
        let _ = shared_log_file.lock_for_direct_write().flush();
        let _ = preserve_crash_log(&logging_paths);
        previous_panic_hook(panic_info);
    }));
}

fn preserve_crash_log(logging_paths: &FoundationLoggingPaths) -> io::Result<PathBuf> {
    fs::create_dir_all(&logging_paths.log_directory)?;
    let crash_log_file_path = foundation_unique_crash_log_file_path(&logging_paths.log_directory);
    fs::copy(&logging_paths.latest_log_file_path, &crash_log_file_path)?;
    Ok(crash_log_file_path)
}

fn foundation_timestamp_for_file_name(timestamp: SystemTime) -> String {
    let timestamp_duration = timestamp.duration_since(UNIX_EPOCH).unwrap_or_default();
    timestamp_duration.as_secs().to_string()
}

fn foundation_precise_timestamp_for_file_name(timestamp: SystemTime) -> String {
    let timestamp_duration = timestamp.duration_since(UNIX_EPOCH).unwrap_or_default();
    format!(
        "{}-{}",
        timestamp_duration.as_secs(),
        timestamp_duration.subsec_nanos()
    )
}

#[cfg(windows)]
fn show_platform_log_window_if_available() {
    // Windows-subsystem games do not have a console by default. Allocating one here
    // makes `--log` useful without making normal or shipping launches noisy.
    unsafe {
        windows_sys::Win32::System::Console::AllocConsole();
    }
}

#[cfg(not(windows))]
fn show_platform_log_window_if_available() {
    // Non-Windows platforms generally inherit a terminal rather than opening a
    // Foundation-managed console window, so the stderr formatter is enough.
}

#[derive(Clone)]
struct SharedLogFile {
    file: Arc<Mutex<File>>,
}

impl SharedLogFile {
    fn new(file: File) -> Self {
        Self {
            file: Arc::new(Mutex::new(file)),
        }
    }

    fn lock_for_direct_write(&self) -> SharedLogFileGuard<'_> {
        SharedLogFileGuard {
            file_guard: self.file.lock().expect("Foundation log file lock poisoned"),
        }
    }
}

impl<'writer> MakeWriter<'writer> for SharedLogFile {
    type Writer = SharedLogFileGuard<'writer>;

    fn make_writer(&'writer self) -> Self::Writer {
        SharedLogFileGuard {
            file_guard: self.file.lock().expect("Foundation log file lock poisoned"),
        }
    }
}

struct SharedLogFileGuard<'writer> {
    file_guard: MutexGuard<'writer, File>,
}

impl Write for SharedLogFileGuard<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.file_guard.write(bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file_guard.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_argument_requests_visible_log_window_only_when_present() {
        let arguments = ["game.exe", FOUNDATION_LOG_ARGUMENT].map(str::to_string);

        assert!(foundation_log_window_requested(arguments));

        let arguments_without_log = ["game.exe", "--editor"].map(str::to_string);

        assert!(!foundation_log_window_requested(arguments_without_log));
    }

    #[test]
    fn shipping_policy_ignores_log_argument_when_file_logging_is_disabled() {
        let arguments = ["game.exe", FOUNDATION_LOG_ARGUMENT].map(str::to_string);

        assert!(!foundation_should_show_log_window(arguments, false));
    }

    #[test]
    fn non_shipping_policy_shows_log_window_when_log_argument_is_present() {
        let arguments = ["game.exe", FOUNDATION_LOG_ARGUMENT].map(str::to_string);

        assert!(foundation_should_show_log_window(arguments, true));
    }

    #[test]
    fn log_directory_is_relative_to_executable_parent() {
        let executable_file_path = Path::new("C:/FoundationGame/bin/game.exe");

        let log_directory = foundation_log_directory_from_executable(executable_file_path)
            .expect("executable parent should create a log directory");

        assert_eq!(
            log_directory,
            PathBuf::from("C:/FoundationGame/bin").join(FOUNDATION_LOG_DIRECTORY)
        );
    }

    #[test]
    fn latest_log_file_path_uses_stable_overwritten_name() {
        let log_directory = Path::new("C:/FoundationGame/saved/logs");

        let latest_log_file_path = foundation_latest_log_file_path(log_directory);

        assert_eq!(
            latest_log_file_path,
            PathBuf::from("C:/FoundationGame/saved/logs/latest.log")
        );
    }

    #[test]
    fn crash_log_path_uses_timestamped_name() {
        let temporary_log_directory = std::env::temp_dir().join(format!(
            "foundation-crash-log-test-{}",
            foundation_precise_timestamp_for_file_name(SystemTime::now())
        ));
        fs::create_dir_all(&temporary_log_directory).expect("test log directory should be created");

        let crash_log_file_path = foundation_unique_crash_log_file_path(&temporary_log_directory);
        let crash_log_file_name = crash_log_file_path
            .file_name()
            .and_then(|file_name| file_name.to_str())
            .expect("crash log should have a valid file name");

        assert!(crash_log_file_name.starts_with("crash-"));
        assert!(crash_log_file_name.ends_with(".log"));

        fs::remove_dir_all(&temporary_log_directory).expect("test log directory should be removed");
    }

    #[test]
    fn crash_log_path_does_not_reuse_existing_file() {
        let temporary_log_directory = std::env::temp_dir().join(format!(
            "foundation-crash-log-collision-test-{}",
            foundation_precise_timestamp_for_file_name(SystemTime::now())
        ));
        fs::create_dir_all(&temporary_log_directory).expect("test log directory should be created");

        let first_crash_log_file_path =
            foundation_unique_crash_log_file_path(&temporary_log_directory);
        File::create(&first_crash_log_file_path).expect("first crash log placeholder should exist");
        let second_crash_log_file_path =
            foundation_unique_crash_log_file_path(&temporary_log_directory);

        assert_ne!(first_crash_log_file_path, second_crash_log_file_path);

        fs::remove_dir_all(&temporary_log_directory).expect("test log directory should be removed");
    }
}
