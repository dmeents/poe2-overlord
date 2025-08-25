# POE2 Overlord Backend

This package contains the Rust backend for the POE2 Overlord Tauri 2 application, organized in a modular architecture for better maintainability and separation of concerns.

## Architecture

The backend is organized into the following modules:

### `models/`

Contains data structures and types used throughout the application:

- `ProcessInfo` - Information about running processes including PID, name, and status
- `OverlayState` - State of the overlay window and its properties

### `services/`

Contains business logic and core functionality:

- `ProcessMonitor` - Handles Path of Exile 2 process detection and monitoring using sysinfo
- `WindowManager` - Manages window operations, overlay behavior, and positioning

### `commands/`

Contains Tauri 2 command handlers that expose functionality to the frontend:

- `process_commands.rs` - Process monitoring commands (`check_poe2_process`)
- `window_commands.rs` - Window management commands (visibility, positioning, always-on-top)

### `handlers/`

Contains application setup and event handling:

- `setup.rs` - Application initialization, window configuration, and event setup

### `lib.rs`

Main library entry point that orchestrates all modules and provides the `run()` function with Tauri 2 configuration.

## Key Features

- **Process Monitoring**: Automatically detects Path of Exile 2 processes using sysinfo
- **Overlay Management**: Configures windows for overlay behavior with transparency support
- **Window Controls**: Provides commands for window positioning, visibility, and always-on-top behavior
- **Event Emission**: Sends process status updates to the frontend in real-time
- **Cross-Platform**: Works on Windows, macOS, and Linux with native performance

## Available Commands

The backend provides the following Tauri commands to the frontend:

- `check_poe2_process()` - Check if Path of Exile 2 is currently running
- `toggle_overlay_visibility()` - Show/hide the overlay window
- `set_window_position(x, y)` - Move the overlay window to specific coordinates
- `get_window_position()` - Get current window position
- `set_always_on_top(enabled)` - Control always-on-top behavior

## Dependencies

- **tauri**: 2.8.3 - Cross-platform desktop app framework
- **tauri-plugin-shell**: 2.3.0 - Shell command execution
- **tauri-plugin-process**: 2.3.0 - Process management capabilities
- **tauri-plugin-log**: 2.6.0 - Logging infrastructure
- **sysinfo**: 0.32 - System information and process monitoring
- **tokio**: 1.x - Async runtime for background tasks
- **serde**: 1.0 - Serialization/deserialization
- **log**: 0.4 - Logging infrastructure

## Development

To add new functionality:

1. **Models**: Add new data structures to `models/mod.rs`
2. **Services**: Implement business logic in `services/`
3. **Commands**: Create Tauri commands in `commands/` with proper error handling
4. **Handlers**: Add setup logic in `handlers/`
5. **Integration**: Wire everything together in `lib.rs` and register commands

### Adding New Commands

1. Create the command function in the appropriate commands file
2. Add the `#[tauri::command]` attribute
3. Register the command in `lib.rs` using `tauri::generate_handler![]`
4. Update the frontend to call the new command

### Error Handling

All commands should return `Result<T, String>` for proper error handling:
- Use descriptive error messages
- Log errors appropriately
- Return user-friendly error strings

## Configuration

The backend configuration is handled through:

- `tauri.conf.json` - Window settings, build configuration, and security policies
- `Cargo.toml` - Rust dependencies and build settings
- Environment variables for development vs production settings

## Security Features

- **Capability System**: Limited API access through Tauri 2's capability system
- **Process Monitoring**: Minimal permissions required for process detection
- **Window Management**: Controlled access to window operations
- **CSP Support**: Content Security Policy configuration available

## Performance Considerations

- **Async Operations**: Uses tokio for non-blocking process monitoring
- **Efficient Process Detection**: sysinfo provides optimized system information access
- **Memory Management**: Proper cleanup of resources and event listeners
- **Cross-Platform Optimization**: Native performance on all supported platforms
