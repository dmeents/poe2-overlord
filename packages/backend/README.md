# POE2 Overlord Backend

This package contains the Rust backend for the POE2 Overlord Tauri 2 application, providing comprehensive game monitoring, time tracking, server status monitoring, and configuration management capabilities for Path of Exile 2.

## Architecture Overview

The backend has been refactored into a **Domain-Driven Design (DDD)** architecture with clear separation of concerns, comprehensive error handling, and modular domain organization. Each domain encapsulates its own business logic, data models, persistence layer, and API commands.

### Domain Structure

The application is organized into distinct domains, each representing a core business capability:

#### `domain/character/` - Character Management Domain
The most complete domain implementation, serving as the reference architecture:

- **`models.rs`** - Core character data structures and business rules
  - `Character` - Main character entity with POE2-specific properties
  - `CharacterClass` - POE2 character classes (Warrior, Sorceress, etc.)
  - `Ascendency` - Character ascendencies with class validation
  - `League` - League types (Standard, Third Edict)
  - `CharacterUpdateParams` - Update parameters with validation
  - Business logic functions for class/ascendency validation

- **`repository.rs`** - Data persistence layer
  - `CharacterRepository` - Handles character data persistence
  - JSON file-based storage with atomic writes
  - Thread-safe operations using `Arc<RwLock<T>>`
  - Automatic config directory management

- **`service.rs`** - Business logic layer
  - `CharacterService` - Implements character business rules
  - Character creation, validation, and management
  - Active character management
  - Integration with time tracking services

- **`commands.rs`** - Tauri command handlers
  - `create_character()` - Create new characters with validation
  - `get_all_characters()` - Retrieve all characters
  - `update_character()` - Update character information
  - `delete_character()` - Remove characters
  - `set_active_character()` - Manage active character selection

#### `domain/time_tracking/` - Time Tracking Domain
Character-aware time tracking and session management:

- **`session_tracker.rs`** - Session tracking logic
- **`commands.rs`** - Time tracking command handlers
- Integration with character domain for character-specific tracking

#### `domain/configuration/` - Configuration Domain
Application configuration management (in development).

#### `domain/game_monitoring/` - Game Monitoring Domain
Game process and log monitoring (in development).

### Legacy Architecture (Being Refactored)

The following modules are being gradually refactored into the domain structure:

#### `models/`
Legacy data structures (being moved to domain models):
- `ProcessInfo` - Game process information
- `AppConfig` - Application configuration
- `LocationSession` - Time tracking sessions
- `TimeTrackingEvent` - Real-time tracking events
- `SceneChangeEvent` - Game scene transitions

#### `services/`
Legacy service layer (being replaced by domain services):
- `ConfigurationManager` - Configuration management
- `LogAnalyzer` - Log file analysis
- `ServerMonitor` - Server status monitoring
- `EventDispatcher` - Event broadcasting
- `ProcessDetector` - Game process detection

#### `commands/`
Legacy command handlers (being moved to domain commands):
- `config_commands.rs` - Configuration commands
- `log_commands.rs` - Log monitoring commands
- `command_utils.rs` - Shared utilities

#### `parsers/`
Specialized parsers for different data formats: (being migrated to new infrastructure directory)
- **`core/`** - Core parsing infrastructure
- **`parsers/`** - Specific parser implementations
- **`detection/`** - Scene type detection
- **`config/`** - Configuration parsing
- **`events/`** - Event factory
- **`utils/`** - Parsing utilities

#### `handlers/`
Application setup and runtime management: (to be refactored as well)
- `service_initializer.rs` - Application initialization
- `service_launcher.rs` - Background service launcher
- `runtime_manager.rs` - Tokio runtime management
- `task_manager.rs` - Background task management
- `log_event_handler.rs` - Log monitoring events
- `game_process_handler.rs` - Game process events
- `time_tracking_handler.rs` - Time tracking events

#### `infrastructure/`
Cross-cutting infrastructure concerns:
- **`persistence/`** - Data persistence utilities
- **`network/`** - Network connectivity and monitoring
- **`parsing/`** - Parsing infrastructure and utilities

## Domain-Driven Design Principles

### Domain Structure Pattern

Each domain follows a consistent structure:

```
domain/{domain_name}/
├── models.rs      # Domain entities, value objects, and business rules
├── repository.rs  # Data persistence and retrieval
├── service.rs     # Business logic and domain operations
├── commands.rs    # Tauri command handlers (API layer)
└── mod.rs         # Module exports and re-exports
```

### Key Design Principles

1. **Domain Isolation**: Each domain encapsulates its own business logic and data
2. **Dependency Inversion**: Services depend on abstractions (traits), not concrete implementations
3. **Single Responsibility**: Each layer has a clear, focused responsibility
4. **Async-First**: All operations are async using Tokio for optimal performance
5. **Error Handling**: Comprehensive error handling with `AppResult<T>` throughout
6. **Thread Safety**: Thread-safe operations using `Arc<RwLock<T>>` for shared state

### Service Traits

The application uses trait-based service interfaces for dependency injection:

```rust
#[async_trait]
pub trait CharacterService: Send + Sync {
    async fn create_character(&self, ...) -> AppResult<Character>;
    async fn get_all_characters(&self) -> Vec<Character>;
    // ... other methods
}
```

## Key Features

### 🎮 Character Management
- **Character Creation**: Create POE2 characters with class, ascendency, and league selection
- **Character Validation**: Business rule validation for class/ascendency combinations
- **Active Character Tracking**: Manage which character is currently active
- **Character Updates**: Update character information with validation
- **Persistent Storage**: Character data persisted to JSON with atomic writes

### ⏱️ Time Tracking
- **Character-Aware Tracking**: Time tracking tied to specific characters
- **Session Management**: Track time spent in different game locations
- **Real-time Updates**: Live session updates via event broadcasting
- **Statistics**: Aggregated time tracking statistics per character
- **Persistent Sessions**: Session data persisted across application restarts

### 🎮 Game Monitoring
- **Process Detection**: Monitor Path of Exile 2 game process status
- **Log Analysis**: Real-time analysis of POE2 client log files
- **Scene Change Detection**: Automatic detection of zone, act, and hideout transitions
- **Server Monitoring**: Real-time server status and ping monitoring
- **Event Broadcasting**: Real-time event distribution to frontend subscribers

### ⚙️ Configuration Management
- **Persistent Settings**: JSON-based configuration with validation
- **Cross-platform Paths**: Automatic config directory detection
- **Runtime Updates**: Dynamic configuration changes with validation
- **Default Values**: Sensible defaults with fallback mechanisms

## Available Commands

### Character Management Commands
- `create_character(name, class, ascendency, league, hardcore, solo_self_found)` - Create a new character
- `get_all_characters()` - Retrieve all characters
- `get_character(character_id)` - Get specific character details
- `update_character(character_id, params)` - Update character information
- `delete_character(character_id)` - Remove a character
- `set_active_character(character_id)` - Set the active character
- `get_active_character()` - Get the currently active character
- `get_character_classes()` - Get all available character classes
- `get_ascendencies_for_class(class)` - Get valid ascendencies for a class
- `get_leagues()` - Get all available leagues

### Time Tracking Commands
- `get_character_time_tracking_data(character_id)` - Get time tracking data for a character
- `clear_character_time_tracking_data(character_id)` - Clear tracking data for a character

### Legacy Commands (Being Refactored)
- `check_game_process()` - Check if POE2 is running
- `get_config()` - Get application configuration
- `update_config(config)` - Update configuration
- `is_log_monitoring_active()` - Check log monitoring status
- `read_last_log_lines(count)` - Read recent log entries

## Development Workflow

### Adding New Domains

1. **Create Domain Structure**:
   ```bash
   mkdir -p src/domain/{domain_name}
   touch src/domain/{domain_name}/{models,repository,service,commands,mod}.rs
   ```

2. **Define Domain Models** (`models.rs`):
   - Define entities, value objects, and business rules
   - Include validation logic and helper functions
   - Use Serde for serialization

3. **Implement Repository** (`repository.rs`):
   - Handle data persistence and retrieval
   - Use `Arc<RwLock<T>>` for thread-safe operations
   - Implement atomic file operations

4. **Create Service Layer** (`service.rs`):
   - Implement business logic
   - Use dependency injection with traits
   - Handle validation and error cases

5. **Add Commands** (`commands.rs`):
   - Create Tauri command handlers
   - Use `#[tauri::command]` attribute
   - Return `CommandResult<T>` for consistent error handling

6. **Register Commands** (`lib.rs`):
   - Add commands to `tauri::generate_handler![]`
   - Register services in application state

### Adding New Commands to Existing Domains

1. **Add to Service Trait** (`services/traits.rs`):
   ```rust
   async fn new_operation(&self, ...) -> AppResult<ReturnType>;
   ```

2. **Implement in Service** (`domain/{domain}/service.rs`):
   ```rust
   pub async fn new_operation(&self, ...) -> AppResult<ReturnType> {
       // Business logic implementation
   }
   ```

3. **Create Command Handler** (`domain/{domain}/commands.rs`):
   ```rust
   #[tauri::command]
   pub async fn new_command(
       // parameters
       service: State<'_, Arc<ServiceType>>,
   ) -> CommandResult<ReturnType> {
       to_command_result(service.new_operation(...).await)
   }
   ```

4. **Register Command** (`lib.rs`):
   ```rust
   tauri::generate_handler![
       // ... existing commands
       new_command,
   ]
   ```

### Error Handling Best Practices

- Use `AppError` types for consistent error handling
- Return `AppResult<T>` from all service methods
- Use `CommandResult<T>` for Tauri commands
- Log errors appropriately with context
- Provide user-friendly error messages

### Testing Strategy

- **Unit Tests**: Test individual domain components in isolation
- **Integration Tests**: Test domain interactions and service integration
- **Async Tests**: Use `tokio::test` for async operation testing
- **Mock Testing**: Use trait-based mocking for dependencies

## Dependencies

### Core Framework
- **tauri**: 2.8.3 - Cross-platform desktop app framework
- **tauri-plugin-***: Various Tauri plugins for capabilities

### Async & Concurrency
- **tokio**: 1.x - Async runtime for background tasks and I/O
- **async-trait**: 0.1 - Async trait support
- **sysinfo**: 0.32 - System information and process monitoring

### Data & Serialization
- **serde**: 1.0 - Serialization/deserialization
- **serde_json**: 1.0 - JSON serialization
- **chrono**: 0.4 - Date and time handling
- **uuid**: 1.0 - UUID generation

### File System & Monitoring
- **notify**: 6.1 - File system event monitoring
- **dirs**: 5.0 - Cross-platform directory detection

### Error Handling & Logging
- **thiserror**: 1.0 - Custom error types
- **log**: 0.4 - Logging infrastructure

## Configuration

### File Locations
- **Config Directory**: `~/.config/poe2-overlord/` (Linux/macOS) or `%APPDATA%\poe2-overlord\` (Windows)
- **Character Data**: `characters.json` - Character information
- **Time Tracking**: `time_tracking.json` - Session and statistics data
- **Configuration**: `config.json` - Application settings

## Performance Considerations

- **Async Architecture**: Non-blocking operations using Tokio
- **Thread Safety**: Proper use of `Arc<RwLock<T>>` for shared state
- **Memory Management**: Efficient resource cleanup and lifecycle management
- **Event Broadcasting**: Efficient event distribution using Tokio broadcast channels
- **Atomic Operations**: Atomic file writes for data consistency

## Security Features

- **Capability System**: Limited API access through Tauri 2's capability system
- **Input Validation**: All inputs validated and sanitized
- **Path Validation**: File paths validated before access
- **Sandboxed Execution**: Tauri 2 provides application sandboxing

## Cross-Platform Support

- **Windows**: Native Windows API integration
- **macOS**: macOS-specific optimizations
- **Linux**: Linux-specific path detection and monitoring
- **Automatic Detection**: Platform-appropriate configuration and log paths

## Contributing

When contributing to the backend:

1. **Follow Domain Structure**: Use the character domain as a reference for new domains
2. **Use Async Patterns**: Implement async operations with proper error handling
3. **Write Tests**: Include comprehensive tests for new functionality
4. **Follow Rust Conventions**: Use `rustfmt` for formatting and follow naming conventions
5. **Update Documentation**: Keep this README updated with architectural changes
6. **Use Traits**: Implement service traits for dependency injection
7. **Handle Errors**: Use `AppResult<T>` and `CommandResult<T>` consistently

## Migration Status

- ✅ **Character Domain**: Complete implementation with full CRUD operations
- ✅ **Time Tracking Domain**: Character-aware time tracking implemented
- 🔄 **Configuration Domain**: In development
- 🔄 **Game Monitoring Domain**: In development
- 🔄 **Legacy Services**: Being gradually refactored into domains

The character domain serves as the reference implementation for the new architecture. Other domains should follow the same patterns and structure established there.