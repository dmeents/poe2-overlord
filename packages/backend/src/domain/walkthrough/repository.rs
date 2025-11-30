use crate::domain::walkthrough::models::WalkthroughGuide;
use crate::domain::walkthrough::traits::WalkthroughRepository;
use crate::errors::AppError;
use crate::infrastructure::file_management::FileService;
use log::info;
use std::path::PathBuf;

pub struct WalkthroughRepositoryImpl {
    guide_path: PathBuf,
}

impl WalkthroughRepositoryImpl {
    pub fn new(guide_path: PathBuf) -> Self {
        Self { guide_path }
    }
}

#[async_trait::async_trait]
impl WalkthroughRepository for WalkthroughRepositoryImpl {
    async fn load_guide(&self) -> Result<WalkthroughGuide, AppError> {
        let guide: WalkthroughGuide = FileService::read_json(&self.guide_path).await?;
        info!(
            "Successfully loaded walkthrough guide with {} acts",
            guide.acts.len()
        );
        Ok(guide)
    }
}
