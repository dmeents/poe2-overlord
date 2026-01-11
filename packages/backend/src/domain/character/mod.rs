//! Character management with consolidated data model

pub mod commands;
pub mod models;
#[cfg(test)]
mod models_test;
pub mod repository;
pub mod service;
pub mod traits;

pub use commands::*;
pub use models::*;
pub use repository::CharacterRepositoryImpl;
pub use service::CharacterServiceImpl;
pub use traits::{CharacterRepository, CharacterService};
