//! File management infrastructure for reading bundled JSON data

pub mod paths;
pub mod service;

pub use paths::{expand_tilde, AppPaths};
pub use service::FileService;
