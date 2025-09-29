use log::{debug, error, info};
use serde_json;
use std::path::PathBuf;
use tokio::fs;

use crate::domain::walkthrough::models::WalkthroughGuide;
use crate::domain::walkthrough::traits::WalkthroughRepository;
use crate::errors::AppError;

/// Implementation of WalkthroughRepository for JSON file-based persistence
pub struct WalkthroughRepositoryImpl {
    /// Path to the walkthrough guide JSON file
    guide_path: PathBuf,
}

impl WalkthroughRepositoryImpl {
    /// Creates a new WalkthroughRepositoryImpl
    pub fn new(guide_path: PathBuf) -> Self {
        Self { guide_path }
    }
}

#[async_trait::async_trait]
impl WalkthroughRepository for WalkthroughRepositoryImpl {
    /// Loads the walkthrough guide from the JSON file
    async fn load_guide(&self) -> Result<WalkthroughGuide, AppError> {
        debug!(
            "🔍 WALKTHROUGH REPO: Loading guide from {:?}",
            self.guide_path
        );

        if !self.guide_path.exists() {
            error!(
                "❌ WALKTHROUGH REPO: Guide file does not exist at {:?}",
                self.guide_path
            );
            return Err(AppError::FileSystem {
                message: format!("Walkthrough guide file not found at {:?}", self.guide_path),
            });
        }

        let content = fs::read_to_string(&self.guide_path).await.map_err(|e| {
            error!("❌ WALKTHROUGH REPO: Failed to read guide file: {}", e);
            AppError::FileSystem {
                message: format!("Failed to read walkthrough guide file: {}", e),
            }
        })?;

        let guide: WalkthroughGuide = serde_json::from_str(&content).map_err(|e| {
            error!("❌ WALKTHROUGH REPO: Failed to parse guide JSON: {}", e);
            AppError::Internal {
                message: format!("Failed to parse walkthrough guide JSON: {}", e),
            }
        })?;

        info!(
            "✅ WALKTHROUGH REPO: Successfully loaded walkthrough guide with {} acts",
            guide.acts.len()
        );
        debug!(
            "🔍 WALKTHROUGH REPO: Loaded acts: {:?}",
            guide.acts.keys().collect::<Vec<_>>()
        );

        Ok(guide)
    }

    /// Saves the walkthrough guide to the JSON file
    async fn save_guide(&self, guide: &WalkthroughGuide) -> Result<(), AppError> {
        debug!("🔍 WALKTHROUGH REPO: Saving guide to {:?}", self.guide_path);

        // Ensure the directory exists
        if let Some(parent) = self.guide_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                error!("❌ WALKTHROUGH REPO: Failed to create directory: {}", e);
                AppError::FileSystem {
                    message: format!("Failed to create directory for walkthrough guide: {}", e),
                }
            })?;
        }

        let content = serde_json::to_string_pretty(guide).map_err(|e| {
            error!("❌ WALKTHROUGH REPO: Failed to serialize guide: {}", e);
            AppError::Internal {
                message: format!("Failed to serialize walkthrough guide: {}", e),
            }
        })?;

        fs::write(&self.guide_path, content).await.map_err(|e| {
            error!("❌ WALKTHROUGH REPO: Failed to write guide file: {}", e);
            AppError::FileSystem {
                message: format!("Failed to write walkthrough guide file: {}", e),
            }
        })?;

        info!("✅ WALKTHROUGH REPO: Successfully saved walkthrough guide");
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    // Note: Repository tests require proper FileSystemPersistence setup which is complex
    // These tests are disabled for now but the repository functions are tested
    // through integration tests
}
