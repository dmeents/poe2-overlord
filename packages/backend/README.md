# POE2 Overlord Backend

A comprehensive Path of Exile 2 companion application built with Rust and Tauri, featuring a domain-driven architecture for character management, time tracking, and game monitoring.

## Architecture Overview

This application follows **Domain-Driven Design (DDD)** principles with a clean architecture pattern, organized into three distinct layers:

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                        │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐ │
│  │ Service Registry│ │ App Setup       │ │ Service         │ │
│  │ (DI Container)  │ │ (Bootstrap)     │ │ Orchestrator    │ │
│  └─────────────────┘ └─────────────────┘ └─────────────────┘ │
└─────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────┐
│                      Domain Layer                           │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────┐ │
│  │ Character   │ │ Time        │ │ Game        │ │ Log     │ │
│  │ Management  │ │ Tracking    │ │ Monitoring  │ │ Analysis│ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────┘ │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────┐ │
│  │ Configuration│ │ Location    │ │ Server      │ │ Event   │ │
│  │ Management  │ │ Tracking    │ │ Monitoring  │ │ Management│ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────┘ │
└─────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────┐
│                  Infrastructure Layer                       │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────┐ │
│  │ Persistence │ │ Monitoring  │ │ Parsing     │ │ Tauri   │ │
│  │ (File I/O)  │ │ (Processes) │ │ (Log Files) │ │ Integration│ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────┘ │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────┐ │
│  │ Runtime     │ │ Network     │ │ System      │ │ Time    │ │
│  │ Management  │ │ Connectivity│ │ Detection   │ │ Utils   │ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## Domain Layer - Core Business Logic

The domain layer contains eight distinct bounded contexts, each representing a specific area of business functionality:

### 1. Character Management (`domain/character/`)
**Purpose**: Complete character lifecycle management for Path of Exile 2 characters.

**Key Components**:
- **Models**: `Character`, `CharacterClass`, `Ascendency`, `League` with business validation
- **Service**: Business logic orchestration and character operations
- **Repository**: File-based persistence with in-memory caching
- **Commands**: Tauri command handlers for frontend integration

**Features**:
- Create, read, update, delete characters with full validation
- Active character tracking with automatic state management
- Business rule enforcement (ascendency-class combinations, name uniqueness)
- Thread-safe operations with async/await and RwLock synchronization

### 2. Time Tracking (`domain/time_tracking/`)
**Purpose**: Comprehensive playtime monitoring and analytics for characters.

**Key Components**:
- **Models**: `ZoneStats`, `TrackingSummary`, `LocationState`
- **Service**: Zone tracking and statistics calculation
- **Repository**: Persistent storage with caching
- **Events**: Real-time time tracking updates

**Features**:
- Zone-based time tracking (hideout, acts, zones)
- Detailed statistics and analytics
- Zone overlap validation
- Real-time frontend updates

### 3. Game Monitoring (`domain/game_monitoring/`)
**Purpose**: Process detection and game state monitoring.

**Key Components**:
- **Models**: `GameProcessStatus`, `GameMonitoringConfig`
- **Service**: Process detection and state coordination
- **Events**: Game state change notifications

**Features**:
- Automatic game process detection
- Integration with time tracking (zone entry/exit)
- Real-time game state updates
- Cross-platform process monitoring

### 4. Log Analysis (`domain/log_analysis/`)
**Purpose**: Real-time log parsing and event extraction from game logs.

**Key Components**:
- **Models**: `LogAnalysisSession`, `LogAnalysisStats`, `LogLineAnalysis`
- **Service**: Log monitoring and event extraction
- **Repository**: Session and statistics persistence

**Features**:
- Real-time log file monitoring
- Pattern matching for game events
- Event extraction (scene changes, server connections, character events)
- Integration with character and server monitoring

### 5. Configuration Management (`domain/configuration/`)
**Purpose**: Application settings and configuration with real-time change events.

**Key Components**:
- **Models**: `AppConfig`, `ConfigurationFileInfo`, `ConfigurationValidationResult`
- **Service**: Configuration loading, validation, and change management
- **Events**: Real-time configuration change notifications

**Features**:
- Atomic configuration file operations
- Dynamic logging level configuration
- POE client log path management
- Configuration validation and error handling

### 6. Location Tracking (`domain/location_tracking/`)
**Purpose**: Scene and zone tracking with configurable monitoring rules.

**Key Components**:
- **Models**: `LocationState`, `ZoneStats`, `SceneTypeConfig`
- **Service**: Location state management and zone tracking
- **Events**: Location change notifications

**Features**:
- Scene type detection (hideout, act, zone)
- Configurable monitoring rules
- Location history and statistics
- Integration with time tracking

### 7. Server Monitoring (`domain/server_monitoring/`)
**Purpose**: Network connectivity and server status monitoring.

**Key Components**:
- **Models**: `ServerStatus`, `ServerInfo`, `ServerMonitoringStats`
- **Service**: Network connectivity monitoring and status tracking
- **Repository**: Server information and statistics persistence

**Features**:
- Periodic server ping monitoring
- Network connectivity tracking
- Server status change notifications
- Statistics collection and persistence

### 8. Event Management (`domain/event_management/`)
**Purpose**: Publish-subscribe event system for loose coupling between components.

**Key Components**:
- **Models**: `EventChannel`, `EventSubscription`, `EventManagementSession`
- **Service**: Event broadcasting and subscription management
- **Repository**: Subscription and session persistence

**Features**:
- Typed event channels with configurable capacity
- Subscription lifecycle management
- Event statistics and monitoring
- Loose coupling between domain services

## Application Layer - Service Orchestration

The application layer serves as the orchestration and coordination layer, implementing dependency injection and managing service lifecycles.

### Service Registry (`application/service_registry.rs`)
**Purpose**: Dependency injection container and service lifecycle management.

**Key Features**:
- Service initialization in dependency order
- Tauri app state registration
- Comprehensive error handling during initialization
- Service instance management

**Initialization Order**:
1. Configuration Service (no dependencies)
2. Event Dispatcher (no dependencies)
3. Character Service (depends on configuration)
4. Time Tracking Service (depends on configuration)
5. Server Monitor (depends on event dispatcher)
6. Log Analyzer (depends on server monitor and character service)
7. Game Monitoring Service (depends on time tracking, event publisher, and process detector)

### Application Setup (`application/app_setup.rs`)
**Purpose**: Complete application bootstrap and configuration.

**Bootstrap Process**:
1. Service initialization through service registry
2. Configuration loading and logging setup
3. Asynchronous data loading (character time tracking data)
4. Runtime management system initialization
5. Background service startup

**Key Features**:
- Dynamic logging configuration from settings
- Non-blocking data loading
- Coordinated background service startup
- Comprehensive error handling

### Service Orchestrator (`application/service_orchestrator.rs`)
**Purpose**: Background task lifecycle and coordination management.

**Background Services**:
- **Game Process Monitoring**: Detects POE2 processes and manages game state
- **Log Monitoring**: Real-time log file analysis and event extraction
- **Time Tracking Emission**: Periodic time data emission to frontend
- **Server Ping Monitoring**: Network connectivity and server status checks

## Infrastructure Layer - External Integrations

The infrastructure layer provides concrete implementations of domain traits and handles external system interactions.

### Persistence (`infrastructure/persistence/`)
**Purpose**: Data persistence and file operations.

**Components**:
- **Atomic Writer**: Safe file writing with backup and rollback
- **JSON Storage**: Serialization/deserialization utilities
- **Directory Manager**: File system operations
- **Repository Traits**: Data access abstractions

### Monitoring (`infrastructure/monitoring/`)
**Purpose**: System monitoring implementations.

**Components**:
- **Process Monitor**: Game process detection and monitoring
- **Server Monitor**: Network connectivity and server status tracking

### Parsing (`infrastructure/parsing/`)
**Purpose**: Log file parsing and analysis.

**Components**:
- **Log Analyzer**: Main log analysis service
- **Parser Factory**: Parser creation and management
- **Parsers**: Specialized parsers for different log event types
  - Character Parser: Character-related events
  - Scene Change Parser: Location and scene transitions
  - Server Connection Parser: Network and server events

### Tauri Integration (`infrastructure/tauri/`)
**Purpose**: Frontend communication and Tauri-specific operations.

**Components**:
- **Event Dispatcher**: Event broadcasting to frontend
- **Event Publisher**: Typed event publishing
- **Command Utils**: Tauri command handling utilities

### Runtime Management (`infrastructure/runtime/`)
**Purpose**: Async task orchestration and lifecycle management.

**Components**:
- **Runtime Manager**: Task spawning and lifecycle management
- **Task Manager**: Task tracking and coordination

### System Utilities (`infrastructure/system/`)
**Purpose**: OS detection and path management.

**Components**:
- **OS Detection**: Cross-platform operating system detection
- **Path Management**: POE client log path resolution

### Time Utilities (`infrastructure/time/`)
**Purpose**: Time calculations and validation.

**Components**:
- **Duration Calculations**: Zone duration and time calculations
- **Validation**: Time data validation and overlap detection

## Key Architectural Patterns

### 1. Domain-Driven Design (DDD)
- **Bounded Contexts**: Each domain module represents a distinct business area
- **Aggregates**: Core entities with business rules and invariants
- **Domain Events**: Loose coupling through event-driven communication
- **Repository Pattern**: Data access abstraction

### 2. Clean Architecture
- **Dependency Inversion**: Domain layer independent of infrastructure
- **Interface Segregation**: Small, focused interfaces
- **Single Responsibility**: Each module has a single, well-defined purpose

### 3. Event-Driven Architecture
- **Publish-Subscribe**: Loose coupling through events
- **Async Communication**: Non-blocking event processing
- **Real-time Updates**: Frontend receives live updates

### 4. Dependency Injection
- **Service Registry**: Centralized service management
- **Lifecycle Management**: Proper initialization and cleanup
- **Testability**: Easy mocking and testing

## Technology Stack

- **Runtime**: Tokio async runtime
- **Framework**: Tauri for desktop application
- **Serialization**: Serde for JSON handling
- **Logging**: Log crate with Tauri plugin
- **File I/O**: Standard library with atomic operations
- **Process Monitoring**: Platform-specific process detection
- **Network**: HTTP client for server monitoring

## Development Guidelines

### Code Organization
- Each domain module follows consistent structure: `models/`, `traits/`, `service/`, `repository/`, `commands/`, `events/`
- Infrastructure concerns are separated from domain logic
- Clear separation between application orchestration and business logic

### Error Handling
- Comprehensive error handling with custom error types
- Graceful degradation and recovery
- Detailed logging for debugging and monitoring

### Testing Strategy
- Unit tests in dedicated test files (not in implementation files)
- Integration tests for domain services
- Mock implementations for external dependencies

### Performance Considerations
- In-memory caching for frequently accessed data
- Async operations for I/O-bound tasks
- Efficient data structures and algorithms
- Background task coordination

## Getting Started

### Prerequisites
- Rust 1.70+
- Node.js 18+ (for frontend)
- Yarn package manager

### Building
```bash
# Build the backend
cd packages/backend
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Configuration
The application uses a configuration file for settings:
- Log levels and output configuration
- POE client log file paths
- Monitoring intervals and thresholds
- Character data storage locations

## API Reference

### Tauri Commands
The application exposes numerous Tauri commands for frontend integration:

**Character Management**:
- `create_character`, `update_character`, `delete_character`
- `get_all_characters`, `get_character`, `get_active_character`
- `set_active_character`, `update_character_level`

**Time Tracking**:
- `get_character_time_tracking_data`
- `get_character_active_zone`, `get_character_zone_stats`
- `get_character_total_play_time`

**Configuration**:
- `get_config`, `update_config`, `validate_config`
- `get_poe_client_log_path`, `set_poe_client_log_path`

**Log Monitoring**:
- `is_log_monitoring_active`, `get_log_file_size`
- `read_last_log_lines`

### Event System
The application emits various events to the frontend:
- Character updates and changes
- Time tracking data updates
- Game process status changes
- Server connectivity updates
- Configuration changes

This architecture provides a solid foundation for a complex desktop application with clear separation of concerns, excellent testability, and maintainable code structure.
