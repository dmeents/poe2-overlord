# POE2 Overlord Backend

This package contains the Rust backend for the POE2 Overlord Tauri 2 application, providing comprehensive game monitoring, time tracking, and configuration management capabilities for Path of Exile 2.

## Architecture Overview

The backend is organized into a modular, event-driven architecture with the following core components:

### `models/`
Core data structures and types used throughout the application:

- **`ProcessInfo`** - Information about running processes (PID, name, status)
- **`OverlayState`** - Window overlay state and properties
- **`AppConfig`** - Application configuration with persistent storage
- **`LocationSession`** - Time tracking sessions for zones/acts
- **`LocationStats`** - Aggregated statistics for game locations
- **`TimeTrackingEvent`** - Real-time events for time tracking updates
- **`SceneChangeEvent`** - Game scene transition events (zone/act changes)

### `services/`
Business logic and core functionality services:

- **`ConfigService`** - Manages application configuration with JSON persistence
- **`LogMonitorService`** - Monitors POE2 client log files for scene changes
- **`TimeTrackingService`** - Tracks time spent in different game locations
- **`ProcessMonitor`** - Detects and monitors Path of Exile 2 processes
- **`EventBroadcaster`** - Broadcasts events to multiple subscribers
- **`FileMonitor`** - Monitors file changes for real-time updates
- **`StateManager`** - Manages application state across services

### `commands/`
Tauri 2 command handlers that expose functionality to the frontend:

- **`config_commands.rs`** - Configuration management commands
- **`log_commands.rs`** - Log monitoring and file operations
- **`process_commands.rs`** - Process detection and monitoring
- **`time_tracking_commands.rs`** - Time tracking session management
- **`helpers.rs`** - Shared utility functions for commands

### `parsers/`
Specialized parsers for different data formats:

- **`SceneChangeParser`** - Parses POE2 log lines for scene transitions
- **`LogParser` trait** - Extensible interface for log parsing

### `handlers/`
Application setup and event handling:

- **`setup.rs`** - Application initialization and configuration
- **`mod.rs`** - Handler module organization

### `errors/`
Custom error types and error handling:

- **`AppError`** - Centralized error types for the application
- **`AppResult`** - Result type alias for consistent error handling

### `utils/`
Utility functions and constants:

- **`constants.rs`** - Application constants and configuration
- **`os_detection.rs`** - Cross-platform OS detection utilities

## Key Features

### 🎮 Game Monitoring
- **Real-time Log Monitoring**: Watches POE2 client log files for scene changes
- **Scene Change Detection**: Automatically detects zone and act transitions
- **Process Monitoring**: Tracks Path of Exile 2 process status using sysinfo

### ⏱️ Time Tracking
- **Session Management**: Tracks time spent in different game locations
- **Location Statistics**: Aggregates data for zones and acts
- **Real-time Updates**: Broadcasts session events to frontend subscribers
- **Persistent Storage**: Saves tracking data to JSON files

### ⚙️ Configuration Management
- **Persistent Settings**: Stores configuration in user's config directory
- **Dynamic Updates**: Supports runtime configuration changes
- **Cross-platform Paths**: Automatically detects appropriate config locations
- **Default Values**: Provides sensible defaults for all settings

### 📡 Event System
- **Broadcast Channels**: Efficient event distribution to multiple subscribers
- **Real-time Updates**: Immediate notification of game state changes
- **Type-safe Events**: Strongly typed event structures with Serde serialization

### 🔒 Security & Performance
- **Capability System**: Limited API access through Tauri 2's capability system
- **Async Architecture**: Non-blocking operations using Tokio runtime
- **Memory Safety**: Rust's ownership system ensures thread safety
- **Cross-platform**: Native performance on Windows, macOS, and Linux

## Available Commands

The backend provides comprehensive Tauri commands organized by functionality:

### Configuration Commands
- `get_config()` - Retrieve current application configuration
- `update_config(config)` - Update application settings
- `reset_config_to_defaults()` - Restore default configuration
- `get_poe_client_log_path()` - Get current log file path
- `set_poe_client_log_path(path)` - Set custom log file path
- `get_log_level()` / `set_log_level(level)` - Manage logging verbosity

### Log Monitoring Commands
- `start_log_monitoring()` - Begin monitoring POE2 client logs
- `stop_log_monitoring()` - Stop log monitoring
- `is_log_monitoring_active()` - Check monitoring status
- `get_log_file_size()` - Get current log file size
- `read_last_log_lines(count)` - Read recent log entries
- `subscribe_to_log_events()` - Subscribe to real-time log events

### Time Tracking Commands
- `start_time_tracking_session(location, type)` - Begin tracking a location
- `end_time_tracking_session(location_id)` - End an active session
- `get_active_sessions()` - List currently active sessions
- `get_completed_sessions()` - Retrieve completed session history
- `get_location_stats(location_id)` - Get statistics for a specific location
- `get_all_location_stats()` - Retrieve all location statistics
- `get_time_tracking_summary()` - Get overall tracking summary
- `clear_all_time_tracking_data()` - Reset all tracking data

### Process Commands
- `check_poe2_process()` - Check if Path of Exile 2 is running

## Dependencies

### Core Framework
- **tauri**: 2.8.3 - Cross-platform desktop app framework
- **tauri-plugin-log**: 2.6.0 - Logging infrastructure
- **tauri-plugin-shell**: 2.3.0 - Shell command execution
- **tauri-plugin-process**: 2.3.0 - Process management capabilities
- **tauri-plugin-window-state**: 2.4.0 - Window state management

### Async & Concurrency
- **tokio**: 1.x - Async runtime for background tasks and concurrency
- **sysinfo**: 0.32 - System information and process monitoring

### Data & Serialization
- **serde**: 1.0 - Serialization/deserialization with derive support
- **serde_json**: 1.0 - JSON serialization/deserialization
- **chrono**: 0.4 - Date and time handling with Serde support

### File System & Monitoring
- **notify**: 6.1 - Cross-platform file system event monitoring
- **dirs**: 5.0 - Cross-platform directory path detection

### Error Handling & Logging
- **thiserror**: 1.0 - Custom error type definitions
- **log**: 0.4 - Logging infrastructure

## Development Workflow

### Adding New Functionality

1. **Models**: Define data structures in `models/mod.rs` with Serde derive macros
2. **Services**: Implement business logic in `services/` with proper error handling
3. **Commands**: Create Tauri commands in `commands/` with `#[tauri::command]` attributes
4. **Parsers**: Add parsing logic in `parsers/` implementing the `LogParser` trait
5. **Integration**: Wire everything together in `lib.rs` and register commands

### Adding New Commands

1. Create the command function in the appropriate commands file
2. Add the `#[tauri::command]` attribute for Tauri integration
3. Register the command in `lib.rs` using `tauri::generate_handler![]`
4. Update the frontend to call the new command
5. Add comprehensive error handling returning `AppResult<T>`

### Error Handling Best Practices

- Use `AppError` types for consistent error handling across the application
- Return `AppResult<T>` from all public functions for proper error propagation
- Log errors appropriately using the `log` crate
- Provide user-friendly error messages in command responses

### Testing Strategy

The backend includes comprehensive testing with dedicated test files:

- **`config_service_tests.rs`** - Configuration service functionality
- **`log_monitor_tests.rs`** - Log monitoring and parsing
- **`time_tracking_tests.rs`** - Time tracking service operations
- **`os_detection_tests.rs`** - Cross-platform utilities
- **`constants_tests.rs`** - Application constants validation

Run tests with: `cargo test`

## Configuration

### File Locations
- **Config Directory**: `~/.config/poe2-overlord/` (Linux/macOS) or `%APPDATA%\poe2-overlord\` (Windows)
- **Config File**: `config.json` - Application settings
- **Time Tracking Data**: `time_tracking.json` - Session and statistics data

### Environment Variables
- Development vs production settings can be configured through environment variables
- Log levels can be adjusted at runtime through the configuration service

## Performance Considerations

- **Async Operations**: Uses Tokio for non-blocking I/O and background tasks
- **Efficient File Monitoring**: notify crate provides optimized file system event handling
- **Memory Management**: Proper cleanup of resources and event listeners
- **Cross-platform Optimization**: Native performance on all supported platforms
- **Event Broadcasting**: Efficient event distribution using Tokio broadcast channels

## Security Features

- **Capability System**: Limited API access through Tauri 2's capability system
- **Process Monitoring**: Minimal permissions required for process detection
- **File System Access**: Controlled access to log files and configuration
- **CSP Support**: Content Security Policy configuration available
- **Sandboxed Execution**: Tauri 2 provides application sandboxing

## Cross-Platform Support

The backend is designed to work seamlessly across different operating systems:

- **Windows**: Native Windows API integration through Tauri
- **macOS**: macOS-specific optimizations and file system handling
- **Linux**: Linux-specific path detection and system monitoring
- **Path Detection**: Automatic detection of POE2 client log locations
- **Config Paths**: Platform-appropriate configuration directory selection

## Troubleshooting

### Common Issues
- **Log File Not Found**: Ensure POE2 client log path is correctly configured
- **Permission Errors**: Check file system permissions for log files and config directories
- **Process Detection**: Verify sysinfo has appropriate permissions for process monitoring

### Debug Mode
Enable debug logging by setting log level to "debug" in configuration for detailed troubleshooting information.

## Contributing

When contributing to the backend:

1. Follow Rust coding standards and use `rustfmt` for formatting
2. Write comprehensive tests for new functionality
3. Use the existing error handling patterns with `AppError` and `AppResult`
4. Ensure all commands have proper error handling and logging
5. Test on multiple platforms when possible
6. Update this README for any architectural changes
