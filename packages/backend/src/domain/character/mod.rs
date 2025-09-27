//! Character Domain Module - Consolidated Character Management
//!
//! This module provides comprehensive character management functionality using a consolidated
//! data model that combines character metadata with tracking data in a single structure.
//!
//! ## Architecture Overview
//!
//! The new character domain uses a simplified file structure:
//! - `characters.json`: Simple index containing character IDs and active character ID
//! - `character_data_{id}.json`: Individual character files containing all character data
//!
//! ## Key Features
//!
//! - **Consolidated Data Model**: All character information (metadata + tracking) in one structure
//! - **Simplified File Structure**: Easy to manage with separate index and data files
//! - **Business Rule Enforcement**: Validates ascendency-class combinations and name uniqueness
//! - **Data Persistence**: Reliable file-based storage with automatic loading and error recovery
//! - **Concurrent Access**: Thread-safe operations using async/await and RwLock synchronization
//!
//! ## Data Model
//!
//! - **CharacterData**: Single structure containing all character information
//! - **CharactersIndex**: Simple index for managing character IDs and active character
//! - **CharacterUpdateParams**: Parameters for updating character properties
//!
//! ## File Structure
//!
//! The data directory contains:
//! - `characters.json`: Index file with character IDs and active character
//! - `character_data_{id}.json`: Individual character data files

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
pub use service::CharacterServiceImpl;
pub use traits::{CharacterRepository, CharacterService};
