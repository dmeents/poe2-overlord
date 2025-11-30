//! File management infrastructure for atomic JSON operations

pub mod paths;
pub mod service;
pub mod traits;

pub use paths::{expand_tilde, AppPaths};
pub use service::FileService;
pub use traits::FileOperations;
