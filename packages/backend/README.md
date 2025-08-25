# Backend Package

This package contains the Rust backend for the POE2 Master Tauri application, organized in a modular architecture for better maintainability and separation of concerns.

## Architecture

The backend is organized into the following modules:

### `models/`
Contains data structures and types used throughout the application:
- `ProcessInfo` - Information about running processes
- `OverlayState` - State of the overlay window

### `services/`
Contains business logic and core functionality:
- `ProcessMonitor` - Handles process detection and monitoring
- `WindowManager` - Manages window operations and overlay behavior

### `commands/`
Contains Tauri command handlers that expose functionality to the frontend:
- `process_commands.rs` - Process-related commands
- `window_commands.rs` - Window management commands

### `handlers/`
Contains application setup and event handling:
- `setup.rs` - Application initialization and configuration

### `lib.rs`
Main library entry point that orchestrates all modules and provides the `run()` function.

## Key Features

- **Process Monitoring**: Automatically detects Path of Exile 2 processes
- **Overlay Management**: Configures windows for overlay behavior
- **Window Controls**: Provides commands for window positioning and visibility
- **Event Emission**: Sends process status updates to the frontend

## Usage

The backend is automatically initialized when the Tauri application starts. It provides the following commands to the frontend:

- `check_poe2_process()` - Check if POE2 is running
- `toggle_overlay_visibility()` - Show/hide the overlay
- `set_window_position()` - Move the overlay window
- `get_window_position()` - Get current window position
- `set_always_on_top()` - Control always-on-top behavior

## Development

To add new functionality:

1. **Models**: Add new data structures to `models/mod.rs`
2. **Services**: Implement business logic in `services/`
3. **Commands**: Create Tauri commands in `commands/`
4. **Handlers**: Add setup logic in `handlers/`
5. **Integration**: Wire everything together in `lib.rs`

## Dependencies

- **tauri**: Cross-platform desktop app framework
- **sysinfo**: System information and process monitoring
- **tokio**: Async runtime for background tasks
- **serde**: Serialization/deserialization
- **log**: Logging infrastructure
