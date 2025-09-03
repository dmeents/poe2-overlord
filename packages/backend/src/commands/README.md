# Commands Module

This module contains all Tauri commands for the POE2 Overlord backend application. The commands have been refactored to provide consistent error handling, better logging, and improved maintainability.

## Architecture

### Module Structure

- **`mod.rs`** - Main module file with exports and common types
- **`helpers.rs`** - Helper macros and utilities for error handling
- **`command_utils.rs`** - Command-specific utilities and response wrappers
- **`config_commands.rs`** - Configuration management commands
- **`log_commands.rs`** - Log monitoring commands
- **`server_status_commands.rs`** - Server status and process monitoring commands
- **`time_tracking_commands.rs`** - Time tracking and session management commands

### Key Types

- **`CommandResult<T>`** - Type alias for Tauri command results (`Result<T, String>`)
- **`CommandResponse<T>`** - Structured response wrapper with metadata
- **`AppResult<T>`** - Internal application result type (`Result<T, AppError>`)

## Error Handling

### Consistent Error Patterns

All commands now use the unified `CommandResult<T>` return type and consistent error handling:

```rust
#[tauri::command]
pub async fn example_command(
    service: State<'_, ServiceType>,
) -> CommandResult<ReturnType> {
    to_command_result(service.method().await.map_err(|e| {
        error!("Operation failed: {}", e);
        AppError::Internal(format!("Operation failed: {}", e))
    }))
}
```

### Helper Macros

- **`command_service_call!`** - For synchronous service calls
- **`async_command_service_call!`** - For asynchronous service calls
- **`handle_service_call!`** - For internal service calls
- **`handle_async_service_call!`** - For internal async service calls

### Error Conversion

- **`to_command_result()`** - Converts `AppResult<T>` to `CommandResult<T>`
- **`error_to_command_result()`** - Creates error results with context

## Command Utilities

### Response Wrappers

The `CommandResponse<T>` struct provides structured responses:

```rust
let response = CommandResponse::success(data, "Operation completed");
let error_response = CommandResponse::error("Operation failed");
```

### Logging Macros

- **`command_entry!(command_name)`** - Logs command invocation
- **`command_exit!(command_name, result)`** - Logs command completion
- **`command_log!(level, command, message)`** - Structured command logging

### Parameter Validation

- **`validate_string_param()`** - Ensures non-empty string parameters
- **`validate_positive_number()`** - Validates positive numeric parameters

### Performance Monitoring

- **`log_command_execution()`** - Wraps commands with timing and logging

## Usage Examples

### Basic Command

```rust
#[tauri::command]
pub async fn get_data(service: State<'_, DataService>) -> CommandResult<Vec<Data>> {
    Ok(service.get_all_data())
}
```

### Command with Error Handling

```rust
#[tauri::command]
pub async fn update_data(
    service: State<'_, DataService>,
    data: Data,
) -> CommandResult<()> {
    to_command_result(service.update(data).await.map_err(|e| {
        error!("Failed to update data: {}", e);
        AppError::Internal(format!("Failed to update data: {}", e))
    }))
}
```

### Command with Validation

```rust
#[tauri::command]
pub async fn create_item(
    name: String,
    service: State<'_, ItemService>,
) -> CommandResult<Item> {
    validate_string_param(&name, "item name")?;
    
    to_command_result(service.create(name).await.map_err(|e| {
        error!("Failed to create item: {}", e);
        AppError::Internal(format!("Failed to create item: {}", e))
    }))
}
```

## Testing

The commands module includes comprehensive tests in `tests/command_tests.rs` that cover:

- Error handling patterns
- Response wrapper functionality
- Parameter validation
- Command execution logging
- Service integration

## Best Practices

1. **Always use `CommandResult<T>`** for Tauri command return types
2. **Use helper macros** for consistent error handling
3. **Validate parameters** before processing
4. **Log command entry/exit** for debugging
5. **Use structured error messages** with context
6. **Convert internal errors** to `AppError` types before returning

## Migration Notes

### From Old Pattern

```rust
// Old: Inconsistent error handling
pub async fn old_command() -> Result<String, String> {
    service.method().map_err(|e| format!("Error: {}", e))
}
```

### To New Pattern

```rust
// New: Consistent error handling
pub async fn new_command() -> CommandResult<String> {
    to_command_result(service.method().await.map_err(|e| {
        error!("Operation failed: {}", e);
        AppError::Internal(format!("Operation failed: {}", e))
    }))
}
```

## Future Improvements

- Add command rate limiting
- Implement command caching
- Add command metrics collection
- Create command dependency injection system
- Add command validation schemas
