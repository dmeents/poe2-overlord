/// Character domain module providing comprehensive character management functionality.
///
/// This module implements the character domain following Domain-Driven Design (DDD) principles,
/// providing a complete solution for managing Path of Exile 2 characters within the application.
///
/// ## Architecture Overview
///
/// The character domain is organized into several layers:
///
/// - **Models** (`models.rs`): Core domain entities and value objects representing characters,
///   classes, ascendencies, and leagues. Includes business rules and validation logic.
///
/// - **Traits** (`traits.rs`): Service and repository interfaces defining contracts for
///   business logic and data persistence operations.
///
/// - **Repository** (`repository.rs`): Data access layer implementation providing file-based
///   persistence with in-memory caching for optimal performance.
///
/// - **Service** (`service.rs`): Business logic layer orchestrating character operations,
///   enforcing business rules, and coordinating between components.
///
/// - **Commands** (`commands.rs`): Tauri command handlers exposing domain functionality
///   to the frontend through a clean API interface.
///
/// ## Key Features
///
/// - **Character Management**: Create, read, update, and delete characters with full validation
/// - **Active Character Tracking**: Maintain a single active character with automatic state management
/// - **Business Rule Enforcement**: Validate ascendency-class combinations and ensure name uniqueness
/// - **Data Persistence**: Reliable file-based storage with automatic loading and error recovery
/// - **Concurrent Access**: Thread-safe operations using async/await and RwLock synchronization
/// - **Integration**: Seamless integration with time tracking and other domain services
///
// Module declarations
pub mod commands;
pub mod models;
pub mod repository;
pub mod service;
pub mod traits;

// Public API exports
pub use commands::*;
pub use models::*;
pub use repository::CharacterRepositoryImpl;
pub use service::CharacterService;
pub use traits::{CharacterRepository, CharacterService as CharacterServiceTrait};
