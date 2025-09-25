pub mod commands;
pub mod models;
pub mod repository;
pub mod service;
pub mod traits;

// Re-export main types for backward compatibility
pub use commands::*;
pub use models::*;
pub use repository::CharacterRepository;
pub use service::CharacterService;
pub use traits::CharacterService as CharacterServiceTrait;
