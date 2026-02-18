//! Character management with consolidated data model

pub mod commands;
pub mod models;
#[cfg(test)]
mod models_test;
pub mod service;
pub mod sqlite_repository;
pub mod traits;

pub use commands::*;
pub use models::*;
pub use service::CharacterServiceImpl;
pub use sqlite_repository::CharacterSqliteRepository;
pub use traits::{CharacterRepository, CharacterService};
