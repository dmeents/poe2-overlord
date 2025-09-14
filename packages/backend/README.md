# POE2 Overlord Backend

This package contains the Rust backend for the POE2 Overlord Tauri 2 application, providing comprehensive game monitoring, time tracking, server status monitoring, and configuration management capabilities for Path of Exile 2.

## Architecture Overview

The backend has been completely refactored into a modern, modular, event-driven architecture with clear separation of concerns and comprehensive error handling:

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

- **`ConfigurationManager`** - Manages application configuration with JSON persistence and validation
- **`LogAnalyzer`** - Comprehensive log file analysis and monitoring with real-time updates
- **`SessionTracker`** - Advanced time tracking for game locations with persistent storage
- **`ServerMonitor`** - Real-time server status monitoring with ping tracking
- **`EventDispatcher`** - Event broadcasting system using Tokio broadcast channels
- **`LogFileWatcher`** - File system monitoring for real-time log file updates
- **`LocationTracker`** - Player location state management (scene and act tracking)
- **`ProcessDetector`** - Game process detection and monitoring with async operations

### `commands/`
Tauri 2 command handlers that expose functionality to the frontend:

- **`config_commands.rs`** - Configuration management commands with validation
- **`log_commands.rs`** - Log monitoring, file operations, and event subscriptions
- **`time_tracking_commands.rs`** - Time tracking session management and statistics
- **`command_utils.rs`** - Shared utility functions and error handling for commands
- **`helpers.rs`** - Common helper functions used across commands

### `parsers/`
Specialized parsers for different data formats with modular architecture:

- **`core/`** - Core parsing infrastructure with factory pattern and manager
  - **`LogParserManager`** - Manages multiple parsers with type-safe registration
  - **`ParserFactory`** - Factory for creating parser instances
  - **`LogParser` trait** - Extensible interface for log parsing
- **`parsers/`** - Specific parser implementations
  - **`SceneChangeParser`** - Parses POE2 log lines for scene transitions (zones, acts, hideouts)
  - **`ServerConnectionParser`** - Parses server connection events and status
- **`detection/`** - Scene type detection and classification
- **`config/`** - Configuration parsing and validation
- **`events/`** - Event factory for creating typed events
- **`utils/`** - Parsing utilities for pattern matching and content extraction

### `handlers/`
Application setup, event handling, and runtime management:

- **`service_initializer.rs`** - Application initialization and service setup with dependency injection
- **`service_launcher.rs`** - Background service launcher for async operations
- **`runtime_manager.rs`** - Tokio runtime management and task coordination
- **`task_manager.rs`** - Background task management and lifecycle control
- **`log_event_handler.rs`** - Handles log monitoring events and real-time updates
- **`game_process_handler.rs`** - Manages game process monitoring events
- **`time_tracking_handler.rs`** - Handles time tracking events and session management
- **`event_utils.rs`** - Event handling utilities and helpers

### `errors/`
Comprehensive error handling system:

- **`AppError`** - Centralized error types for the application with proper categorization
- **`AppResult`** - Result type alias for consistent error handling across all modules
- **`Error variants`** - Specific error types for different failure scenarios

### `utils/`
Utility functions, constants, and cross-platform utilities:

- **`constants.rs`** - Application constants and configuration values
- **`os_detection.rs`** - Cross-platform OS detection utilities
- **`network.rs`** - Network utilities for server monitoring and ping operations
- **`validation.rs`** - Input validation and sanitization utilities

## Key Features

### 🎮 Game Monitoring
- **Real-time Log Monitoring**: Watches POE2 client log files for scene changes using notify crate with async operations
- **Scene Change Detection**: Automatically detects zone, act, and hideout transitions with intelligent parsing
- **Game Process Monitoring**: Tracks Path of Exile 2 game process status using sysinfo with async operations
- **File Change Events**: Real-time file system monitoring for immediate log updates
- **Server Connection Tracking**: Monitors server connections and tracks connection events
- **Server Status Monitoring**: Real-time server ping monitoring with status tracking

### ⏱️ Time Tracking
- **Session Management**: Tracks time spent in different game locations (zones, acts, hideouts) with proper state management
- **Location Statistics**: Aggregates data for zones, acts, and hideouts with persistent storage
- **Real-time Updates**: Broadcasts session events to frontend subscribers using Tokio broadcast channels
- **Persistent Storage**: Saves tracking data to JSON files with proper error handling and atomic writes
- **Session History**: Maintains complete history of completed sessions with detailed statistics
- **Process Integration**: Tracks time since POE2 process start for comprehensive play time analysis


### ⚙️ Configuration Management
- **Persistent Settings**: Stores configuration in user's config directory with JSON format and atomic writes
- **Dynamic Updates**: Supports runtime configuration changes with validation and error handling
- **Cross-platform Paths**: Automatically detects appropriate config locations for each OS
- **Default Values**: Provides sensible defaults for all settings with fallback mechanisms
- **Path Validation**: Ensures configured paths exist and are accessible with comprehensive error reporting
- **Log Level Control**: Runtime log level adjustment with immediate effect
- **Auto-detection**: Automatic detection of default POE2 client log paths for each platform

### 📡 Event System
- **Broadcast Channels**: Efficient event distribution to multiple subscribers using Tokio broadcast channels
- **Real-time Updates**: Immediate notification of game state changes with minimal latency
- **Type-safe Events**: Strongly typed event structures with Serde serialization
- **Event Subscriptions**: Frontend can subscribe to specific event types for targeted updates
- **Event Filtering**: Support for filtering events by type and content
- **Event Persistence**: Optional event persistence for debugging and analysis
- **Multiple Event Types**: Support for log events, time tracking events, and server status events

### 🔒 Security & Performance
- **Capability System**: Limited API access through Tauri 2's capability system with fine-grained permissions
- **Async Architecture**: Non-blocking operations using Tokio runtime for optimal performance and scalability
- **Memory Safety**: Rust's ownership system ensures thread safety and prevents data races
- **Cross-platform**: Native performance on Windows, macOS, and Linux with platform-specific optimizations
- **Resource Management**: Proper cleanup of file watchers, event listeners, and async tasks with lifecycle management
- **Error Handling**: Comprehensive error handling with proper error propagation and user-friendly messages
- **Input Validation**: All inputs are validated and sanitized before processing

## Available Commands

The backend provides comprehensive Tauri commands organized by functionality:

### Configuration Commands
- `get_config()` - Retrieve current application configuration
- `get_default_config()` - Get default configuration values
- `update_config(config)` - Update application settings with validation
- `reset_config_to_defaults()` - Restore default configuration

### Log Monitoring Commands
- `is_log_monitoring_active()` - Check if log monitoring is currently active
- `get_log_file_size()` - Get current log file size in bytes
- `read_last_log_lines(count)` - Read recent log entries from the file

### Time Tracking Commands
- `get_character_time_tracking_data(character_id)` - Get comprehensive time tracking data for a specific character including active sessions, completed sessions, and statistics
- `clear_character_time_tracking_data(character_id)` - Reset all tracking data for a specific character

### Game Process Commands
- `check_game_process()` - Check if Path of Exile 2 game is running with detailed status including PID and start time

## Dependencies

### Core Framework
- **tauri**: 2.8.3 - Cross-platform desktop app framework with capability system
- **tauri-plugin-log**: 2.6.0 - Logging infrastructure for development
- **tauri-plugin-shell**: 2.3.0 - Shell command execution capabilities
- **tauri-plugin-process**: 2.3.0 - Process management capabilities
- **tauri-plugin-window-state**: 2.4.0 - Window state management

### Async & Concurrency
- **tokio**: 1.x - Async runtime for background tasks, file monitoring, and event broadcasting
- **sysinfo**: 0.32 - System information and process monitoring with async support

### Data & Serialization
- **serde**: 1.0 - Serialization/deserialization with derive support
- **serde_json**: 1.0 - JSON serialization/deserialization
- **chrono**: 0.4 - Date and time handling with Serde support and timezone support

### File System & Monitoring
- **notify**: 6.1 - Cross-platform file system event monitoring with async support
- **dirs**: 5.0 - Cross-platform directory path detection

### Error Handling & Logging
- **thiserror**: 1.0 - Custom error type definitions with derive macros
- **log**: 0.4 - Logging infrastructure

### Development Dependencies
- **tempfile**: 3.8 - Temporary file creation for testing
- **tokio-test**: 0.4 - Testing utilities for async code
- **mockall**: 0.12 - Mocking framework for testing
- **anyhow**: 1.0 - Error handling utilities for testing

## Development Workflow

### Adding New Functionality

1. **Models**: Define data structures in `models/mod.rs` with Serde derive macros
2. **Services**: Implement business logic in `services/` with proper error handling and async patterns
3. **Commands**: Create Tauri commands in `commands/` with `#[tauri::command]` attributes
4. **Parsers**: Add parsing logic in `parsers/` implementing the `LogParser` trait
5. **Integration**: Wire everything together in `lib.rs` and register commands

### Adding New Commands

1. Create the command function in the appropriate commands file
2. Add the `#[tauri::command]` attribute for Tauri integration
3. Register the command in `lib.rs` using `tauri::generate_handler![]`
4. Update the frontend to call the new command
5. Add comprehensive error handling returning `AppResult<T>`
6. Ensure proper async handling if the command performs I/O operations

### Error Handling Best Practices

- Use `AppError` types for consistent error handling across the application
- Return `AppResult<T>` from all public functions for proper error propagation
- Log errors appropriately using the `log` crate with proper log levels
- Provide user-friendly error messages in command responses
- Handle async errors properly with `.await` and error propagation

### Testing Strategy

The backend includes comprehensive testing with dedicated test files in the `tests/` directory:

- **`concurrency_tests.rs`** - Concurrency and async operation testing
- **`model_tests.rs`** - Data model and serialization testing
- **`scene_type_detector_tests.rs`** - Scene type detection and classification testing
- **`serialization_tests.rs`** - Serialization and deserialization testing
- **`system_tests.rs`** - System integration and end-to-end testing
- **`utility_tests.rs`** - Utility function and helper testing

#### Running Tests
- **All tests**: `cargo test`
- **Specific test file**: `cargo test --test test_file_name`
- **Verbose output**: `cargo test --verbose`
- **Watch mode**: `cargo watch -x test`

#### Test Coverage
- **Unit tests**: Individual function and method testing
- **Integration tests**: Service integration and interaction testing
- **Async tests**: Tokio-based async operation testing
- **Mock testing**: Using mockall for isolated component testing

## Configuration

### File Locations
- **Config Directory**: `~/.config/poe2-overlord/` (Linux/macOS) or `%APPDATA%\poe2-overlord\` (Windows)
- **Config File**: `config.json` - Application settings with validation
- **Time Tracking Data**: `time_tracking.json` - Session and statistics data

### Environment Variables
- Development vs production settings can be configured through environment variables
- Log levels can be adjusted at runtime through the configuration service

## Performance Considerations

- **Async Operations**: Uses Tokio for non-blocking I/O, file monitoring, and background tasks
- **Efficient File Monitoring**: notify crate provides optimized file system event handling
- **Memory Management**: Proper cleanup of resources, event listeners, and async tasks
- **Cross-platform Optimization**: Native performance on all supported platforms
- **Event Broadcasting**: Efficient event distribution using Tokio broadcast channels
- **Resource Pooling**: Reuses file watchers and event streams where possible

## Security Features

- **Capability System**: Limited API access through Tauri 2's capability system
- **Game Process Monitoring**: Minimal permissions required for game process detection
- **File System Access**: Controlled access to log files and configuration directories
- **CSP Support**: Content Security Policy configuration available
- **Sandboxed Execution**: Tauri 2 provides application sandboxing
- **Path Validation**: All file paths are validated before access

## Cross-Platform Support

The backend is designed to work seamlessly across different operating systems:

- **Windows**: Native Windows API integration through Tauri with proper path handling
- **macOS**: macOS-specific optimizations and file system handling
- **Linux**: Linux-specific path detection and system monitoring
- **Path Detection**: Automatic detection of POE2 client log locations for each platform
- **Config Paths**: Platform-appropriate configuration directory selection
- **File Watching**: Platform-optimized file system event monitoring

## Troubleshooting

### Common Issues
- **Log File Not Found**: Ensure POE2 client log path is correctly configured and accessible
- **Permission Errors**: Check file system permissions for log files and config directories
- **Game Process Detection**: Verify sysinfo has appropriate permissions for game process monitoring
- **File Watching Issues**: Ensure the file system supports the notify crate's watching capabilities

### Debug Mode
Enable debug logging by setting log level to "debug" in configuration for detailed troubleshooting information.

## Contributing

When contributing to the backend:

1. Follow Rust coding standards and use `rustfmt` for formatting
2. Write comprehensive tests for new functionality using the existing test patterns
3. Use the existing error handling patterns with `AppError` and `AppResult`
4. Ensure all commands have proper error handling, logging, and async support
5. Test on multiple platforms when possible
6. Update this README for any architectural changes
7. Follow the modular architecture patterns established in the refactoring
8. Use proper async patterns with Tokio for I/O operations
