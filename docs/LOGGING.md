# Logging in DJ Engine

DJ Engine uses a high-performance tracing-based logging system that mirrors all engine activity to both the terminal and a persistent log file.

## 📂 Log Location

Logs are saved to the user's home directory:

- **Linux/WSL**: `~/.dj_engine/logs/engine.log`
- **Windows**: `%USERPROFILE%\.dj_engine\logs\engine.log`

History is preserved between runs, allowing for deep debugging of intermittent issues.

## 🛠️ Implementation Details

The logging system is based on `tracing-subscriber` and `tracing-appender`.

- **Stdout Layer**: Colorized output with module targets for terminal readability.
- **File Layer**: Plaintext output (ANSI codes stripped) for file readability and searchability.
- **Initialization**: Managed by `dj_engine::core::logging::init_logging()`.

## ⚙️ Configuration

Logging and log levels are currently configured at compile-time and via environment variables.

To see more verbose output (e.g., debug or trace):

```bash
RUST_LOG=dj_engine=debug cargo run
```

## 🔌 Asset/Script Logging

Scripts executed via Lua are logged using the standard `info!`, `warn!`, and `error!` macros from the engine side, ensuring their output appears in both the terminal and `engine.log`.
