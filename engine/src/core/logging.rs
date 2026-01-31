//! Logging utilities for DJ Engine.
//!
//! Provides automatic file-based logging using `tracing-appender`.

use tracing_subscriber::{fmt, prelude::*, Registry};
use tracing_appender::non_blocking::WorkerGuard;
use std::path::PathBuf;

/// Initializes the global tracing subscriber with both stdout and file output.
///
/// Returns a `WorkerGuard` that must be held in `main` to ensure logs are flushed.
pub fn init_logging() -> WorkerGuard {
    let log_dir = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".dj_engine")
        .join("logs");

    if !log_dir.exists() {
        let _ = std::fs::create_dir_all(&log_dir);
    }

    let file_appender = tracing_appender::rolling::never(log_dir, "engine.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let stdout_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_writer(std::io::stdout);

    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_target(true)
        .with_writer(non_blocking);

    let subscriber = Registry::default()
        .with(stdout_layer)
        .with(file_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global tracing subscriber");

    guard
}
