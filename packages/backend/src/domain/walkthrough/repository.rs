use log::{error, info};
use serde_json;
use std::path::PathBuf;
use tokio::fs;

use crate::domain::walkthrough::models::WalkthroughGuide;
use crate::domain::walkthrough::traits::WalkthroughRepository;
use crate::errors::AppError;

/// File-based implementation of the WalkthroughRepository trait.
///
/// This repository handles persistence of walkthrough guide data using JSON files.
/// It loads the complete walkthrough guide structure from a single JSON file
/// containing all acts, steps, and objectives.
pub struct WalkthroughRepositoryImpl {
    /// Path to the walkthrough guide JSON file
    guide_path: PathBuf,
}

impl WalkthroughRepositoryImpl {
    /// Creates a new WalkthroughRepositoryImpl instance
    pub fn new(guide_path: PathBuf) -> Self {
        Self { guide_path }
    }
}

#[async_trait::async_trait]
impl WalkthroughRepository for WalkthroughRepositoryImpl {
    /// Loads the walkthrough guide from the JSON file
    async fn load_guide(&self) -> Result<WalkthroughGuide, AppError> {
        if !self.guide_path.exists() {
            error!("Guide file does not exist at {:?}", self.guide_path);
            return Err(AppError::FileSystem {
                message: format!("Walkthrough guide file not found at {:?}", self.guide_path),
            });
        }

        let content = fs::read_to_string(&self.guide_path).await.map_err(|e| {
            error!("Failed to read guide file: {}", e);
            AppError::FileSystem {
                message: format!("Failed to read walkthrough guide file: {}", e),
            }
        })?;

        let guide: WalkthroughGuide = serde_json::from_str(&content).map_err(|e| {
            error!("Failed to parse guide JSON: {}", e);
            AppError::Internal {
                message: format!("Failed to parse walkthrough guide JSON: {}", e),
            }
        })?;

        info!(
            "Successfully loaded walkthrough guide with {} acts",
            guide.acts.len()
        );

        Ok(guide)
    }
}
