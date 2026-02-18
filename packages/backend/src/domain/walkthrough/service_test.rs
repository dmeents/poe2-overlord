#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    use crate::domain::character::models::{
        Ascendency, CharacterClass, CharacterData, CharacterDataResponse, CharacterUpdateParams,
        CharactersIndex, League, LocationState,
    };
    use crate::domain::character::traits::CharacterService;
    use crate::domain::walkthrough::models::{
        Objective, WalkthroughAct, WalkthroughGuide, WalkthroughProgress, WalkthroughStep,
    };
    use crate::domain::walkthrough::service::WalkthroughServiceImpl;
    use crate::domain::walkthrough::traits::{WalkthroughRepository, WalkthroughService};
    use crate::errors::AppError;
    use crate::infrastructure::events::EventBus;

    // ============= Mock Implementations =============

    /// Mock WalkthroughRepository that returns configurable guide data
    struct MockWalkthroughRepository {
        guide: WalkthroughGuide,
        should_fail: bool,
    }

    impl MockWalkthroughRepository {
        fn new(guide: WalkthroughGuide) -> Self {
            Self {
                guide,
                should_fail: false,
            }
        }

        fn with_failure() -> Self {
            Self {
                guide: WalkthroughGuide { acts: Vec::new() },
                should_fail: true,
            }
        }
    }

    #[async_trait]
    impl WalkthroughRepository for MockWalkthroughRepository {
        async fn load_guide(&self) -> Result<WalkthroughGuide, AppError> {
            if self.should_fail {
                return Err(AppError::FileSystem {
                    message: "Failed to load guide".to_string(),
                });
            }
            Ok(self.guide.clone())
        }
    }

    /// Mock CharacterService that stores character data in memory
    struct MockCharacterService {
        characters: Arc<RwLock<HashMap<String, CharacterData>>>,
        should_fail_load: bool,
        should_fail_save: bool,
    }

    impl MockCharacterService {
        fn new() -> Self {
            Self {
                characters: Arc::new(RwLock::new(HashMap::new())),
                should_fail_load: false,
                should_fail_save: false,
            }
        }

        fn with_character(character: CharacterData) -> Self {
            let mut characters = HashMap::new();
            characters.insert(character.id.clone(), character);
            Self {
                characters: Arc::new(RwLock::new(characters)),
                should_fail_load: false,
                should_fail_save: false,
            }
        }

        fn with_load_failure() -> Self {
            Self {
                characters: Arc::new(RwLock::new(HashMap::new())),
                should_fail_load: true,
                should_fail_save: false,
            }
        }

        fn with_save_failure(character: CharacterData) -> Self {
            let mut characters = HashMap::new();
            characters.insert(character.id.clone(), character);
            Self {
                characters: Arc::new(RwLock::new(characters)),
                should_fail_load: false,
                should_fail_save: true,
            }
        }
    }

    #[async_trait]
    impl CharacterService for MockCharacterService {
        async fn load_character_data(&self, character_id: &str) -> Result<CharacterData, AppError> {
            if self.should_fail_load {
                return Err(AppError::Validation {
                    message: format!("Character {} not found", character_id),
                });
            }
            let characters = self.characters.read().await;
            characters
                .get(character_id)
                .cloned()
                .ok_or_else(|| AppError::Validation {
                    message: format!("Character {} not found", character_id),
                })
        }

        async fn save_character_data(
            &self,
            character_data: &CharacterData,
        ) -> Result<(), AppError> {
            if self.should_fail_save {
                return Err(AppError::FileSystem {
                    message: "Failed to save character".to_string(),
                });
            }
            let mut characters = self.characters.write().await;
            characters.insert(character_data.id.clone(), character_data.clone());
            Ok(())
        }

        async fn get_character(
            &self,
            character_id: &str,
        ) -> Result<CharacterDataResponse, AppError> {
            let characters = self.characters.read().await;
            let character =
                characters
                    .get(character_id)
                    .cloned()
                    .ok_or_else(|| AppError::Validation {
                        message: format!("Character {} not found", character_id),
                    })?;
            Ok(CharacterDataResponse::from(character))
        }

        // Unused methods - panic if called unexpectedly
        async fn create_character(
            &self,
            _name: String,
            _class: CharacterClass,
            _ascendency: Ascendency,
            _league: League,
            _hardcore: bool,
            _solo_self_found: bool,
        ) -> Result<CharacterDataResponse, AppError> {
            panic!("create_character should not be called in walkthrough tests")
        }

        async fn get_all_characters(&self) -> Result<Vec<CharacterDataResponse>, AppError> {
            panic!("get_all_characters should not be called in walkthrough tests")
        }

        async fn update_character(
            &self,
            _character_id: &str,
            _update_params: CharacterUpdateParams,
        ) -> Result<CharacterDataResponse, AppError> {
            panic!("update_character should not be called in walkthrough tests")
        }

        async fn delete_character(&self, _character_id: &str) -> Result<(), AppError> {
            panic!("delete_character should not be called in walkthrough tests")
        }

        async fn set_active_character(&self, _character_id: Option<&str>) -> Result<(), AppError> {
            panic!("set_active_character should not be called in walkthrough tests")
        }

        async fn get_active_character(&self) -> Result<Option<CharacterDataResponse>, AppError> {
            panic!("get_active_character should not be called in walkthrough tests")
        }

        async fn get_characters_index(&self) -> Result<CharactersIndex, AppError> {
            panic!("get_characters_index should not be called in walkthrough tests")
        }

        async fn is_name_unique(
            &self,
            _name: &str,
            _exclude_id: Option<&str>,
        ) -> Result<bool, AppError> {
            panic!("is_name_unique should not be called in walkthrough tests")
        }

        async fn update_character_level(
            &self,
            _character_id: &str,
            _new_level: u32,
        ) -> Result<(), AppError> {
            panic!("update_character_level should not be called in walkthrough tests")
        }

        async fn get_current_location(
            &self,
            _character_id: &str,
        ) -> Result<Option<LocationState>, AppError> {
            panic!("get_current_location should not be called in walkthrough tests")
        }

        async fn enter_zone(&self, _character_id: &str, _zone_name: &str) -> Result<(), AppError> {
            panic!("enter_zone should not be called in walkthrough tests")
        }

        async fn leave_zone(&self, _character_id: &str, _zone_name: &str) -> Result<(), AppError> {
            panic!("leave_zone should not be called in walkthrough tests")
        }

        async fn record_death(&self, _character_id: &str) -> Result<(), AppError> {
            panic!("record_death should not be called in walkthrough tests")
        }

        async fn finalize_all_active_zones(&self) -> Result<(), AppError> {
            panic!("finalize_all_active_zones should not be called in walkthrough tests")
        }

        async fn sync_zone_metadata(&self, _character_id: &str) -> Result<(), AppError> {
            panic!("sync_zone_metadata should not be called in walkthrough tests")
        }
    }

    // ============= Test Data Factories =============

    /// Creates a minimal valid walkthrough guide with 2 acts and 3 steps
    fn create_test_guide() -> WalkthroughGuide {
        WalkthroughGuide {
            acts: vec![
                WalkthroughAct {
                    act_name: "Act 1".to_string(),
                    steps: vec![
                        WalkthroughStep {
                            id: "act_1_step_1".to_string(),
                            title: "First Step".to_string(),
                            description: "Start your journey".to_string(),
                            objectives: vec![Objective {
                                text: "Talk to NPC".to_string(),
                                details: None,
                                required: true,
                                rewards: vec![],
                                league_start: false,
                            }],
                            current_zone: "Starting Area".to_string(),
                            completion_zone: "The Coast".to_string(),
                            links: vec![],
                        },
                        WalkthroughStep {
                            id: "act_1_step_2".to_string(),
                            title: "Second Step".to_string(),
                            description: "Continue your journey".to_string(),
                            objectives: vec![],
                            current_zone: "The Coast".to_string(),
                            completion_zone: "Town Square".to_string(),
                            links: vec![],
                        },
                    ],
                },
                WalkthroughAct {
                    act_name: "Act 2".to_string(),
                    steps: vec![WalkthroughStep {
                        id: "act_2_step_1".to_string(),
                        title: "Final Step".to_string(),
                        description: "Complete the campaign".to_string(),
                        objectives: vec![],
                        current_zone: "Town Square".to_string(),
                        completion_zone: "Final Boss Arena".to_string(),
                        links: vec![],
                    }],
                },
            ],
        }
    }

    /// Creates a character with default walkthrough progress (at act_1_step_1)
    fn create_test_character(id: &str) -> CharacterData {
        let mut character = CharacterData::new(
            id.to_string(),
            "TestCharacter".to_string(),
            CharacterClass::Warrior,
            Ascendency::Titan,
            League::Standard,
            false,
            false,
        );
        character.walkthrough_progress = WalkthroughProgress::new();
        character
    }

    /// Creates a character at a specific step
    fn create_character_at_step(id: &str, step_id: &str) -> CharacterData {
        let mut character = create_test_character(id);
        character.walkthrough_progress.current_step_id = Some(step_id.to_string());
        character
    }

    /// Creates a character with completed campaign
    fn create_completed_character(id: &str) -> CharacterData {
        let mut character = create_test_character(id);
        character.walkthrough_progress = WalkthroughProgress::completed();
        character
    }

    /// Creates a WalkthroughServiceImpl with test dependencies
    fn create_test_service(
        repository: Arc<dyn WalkthroughRepository + Send + Sync>,
        character_service: Arc<dyn CharacterService + Send + Sync>,
    ) -> WalkthroughServiceImpl {
        let event_bus = Arc::new(EventBus::new());
        WalkthroughServiceImpl::new(repository, character_service, event_bus)
    }

    // ============= get_guide() Tests =============

    #[tokio::test]
    async fn test_get_guide_success() {
        let guide = create_test_guide();
        let repository = Arc::new(MockWalkthroughRepository::new(guide.clone()));
        let character_service = Arc::new(MockCharacterService::new());
        let service = create_test_service(repository, character_service);

        let result = service.get_guide().await;

        assert!(result.is_ok());
        let returned_guide = result.unwrap();
        assert_eq!(returned_guide.acts.len(), 2);
        assert_eq!(returned_guide.acts[0].act_name, "Act 1");
        assert_eq!(returned_guide.acts[1].act_name, "Act 2");
    }

    #[tokio::test]
    async fn test_get_guide_repository_error() {
        let repository = Arc::new(MockWalkthroughRepository::with_failure());
        let character_service = Arc::new(MockCharacterService::new());
        let service = create_test_service(repository, character_service);

        let result = service.get_guide().await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_guide_empty() {
        let empty_guide = WalkthroughGuide { acts: Vec::new() };
        let repository = Arc::new(MockWalkthroughRepository::new(empty_guide));
        let character_service = Arc::new(MockCharacterService::new());
        let service = create_test_service(repository, character_service);

        let result = service.get_guide().await;

        assert!(result.is_ok());
        let returned_guide = result.unwrap();
        assert_eq!(returned_guide.acts.len(), 0);
    }

    // ============= get_character_progress() Tests =============

    #[tokio::test]
    async fn test_get_character_progress_with_current_step() {
        let guide = create_test_guide();
        let character = create_character_at_step("char-1", "act_1_step_1");

        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let character_service = Arc::new(MockCharacterService::with_character(character));
        let service = create_test_service(repository, character_service);

        let result = service.get_character_progress("char-1").await;

        assert!(result.is_ok());
        let progress = result.unwrap();
        assert_eq!(
            progress.progress.current_step_id,
            Some("act_1_step_1".to_string())
        );
        assert_eq!(progress.next_step_id, Some("act_1_step_2".to_string()));
        assert_eq!(progress.previous_step_id, None);
        assert!(!progress.progress.is_completed);
    }

    #[tokio::test]
    async fn test_get_character_progress_at_middle_step() {
        let guide = create_test_guide();
        let character = create_character_at_step("char-1", "act_1_step_2");

        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let character_service = Arc::new(MockCharacterService::with_character(character));
        let service = create_test_service(repository, character_service);

        let result = service.get_character_progress("char-1").await;

        assert!(result.is_ok());
        let progress = result.unwrap();
        assert_eq!(
            progress.progress.current_step_id,
            Some("act_1_step_2".to_string())
        );
        assert_eq!(progress.next_step_id, Some("act_2_step_1".to_string()));
        assert_eq!(progress.previous_step_id, Some("act_1_step_1".to_string()));
    }

    #[tokio::test]
    async fn test_get_character_progress_at_last_step() {
        let guide = create_test_guide();
        let character = create_character_at_step("char-1", "act_2_step_1");

        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let character_service = Arc::new(MockCharacterService::with_character(character));
        let service = create_test_service(repository, character_service);

        let result = service.get_character_progress("char-1").await;

        assert!(result.is_ok());
        let progress = result.unwrap();
        assert_eq!(
            progress.progress.current_step_id,
            Some("act_2_step_1".to_string())
        );
        assert_eq!(progress.next_step_id, None); // Last step has no next
        assert_eq!(progress.previous_step_id, Some("act_1_step_2".to_string()));
    }

    #[tokio::test]
    async fn test_get_character_progress_completed_campaign() {
        let guide = create_test_guide();
        let character = create_completed_character("char-1");

        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let character_service = Arc::new(MockCharacterService::with_character(character));
        let service = create_test_service(repository, character_service);

        let result = service.get_character_progress("char-1").await;

        assert!(result.is_ok());
        let progress = result.unwrap();
        assert!(progress.progress.is_completed);
        assert_eq!(progress.progress.current_step_id, None);
        assert_eq!(progress.next_step_id, None);
        assert_eq!(progress.previous_step_id, None);
    }

    #[tokio::test]
    async fn test_get_character_progress_invalid_step_id() {
        let guide = create_test_guide();
        let character = create_character_at_step("char-1", "nonexistent_step");

        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let character_service = Arc::new(MockCharacterService::with_character(character));
        let service = create_test_service(repository, character_service);

        let result = service.get_character_progress("char-1").await;

        // Should succeed but with None navigation since step not found
        assert!(result.is_ok());
        let progress = result.unwrap();
        assert_eq!(
            progress.progress.current_step_id,
            Some("nonexistent_step".to_string())
        );
        assert_eq!(progress.next_step_id, None);
        assert_eq!(progress.previous_step_id, None);
    }

    #[tokio::test]
    async fn test_get_character_progress_character_not_found() {
        let guide = create_test_guide();
        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let character_service = Arc::new(MockCharacterService::with_load_failure());
        let service = create_test_service(repository, character_service);

        let result = service.get_character_progress("nonexistent").await;

        assert!(result.is_err());
    }

    // ============= update_character_progress() Tests =============

    #[tokio::test]
    async fn test_update_character_progress_success() {
        let guide = create_test_guide();
        let character = create_test_character("char-1");
        let character_service = Arc::new(MockCharacterService::with_character(character));

        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let service = create_test_service(repository, character_service.clone());

        let mut new_progress = WalkthroughProgress::new();
        new_progress.set_current_step("act_1_step_2".to_string());

        let result = service
            .update_character_progress("char-1", new_progress)
            .await;

        assert!(result.is_ok());

        // Verify character was updated
        let characters = character_service.characters.read().await;
        let updated_char = characters.get("char-1").unwrap();
        assert_eq!(
            updated_char.walkthrough_progress.current_step_id,
            Some("act_1_step_2".to_string())
        );
    }

    #[tokio::test]
    async fn test_update_character_progress_character_not_found() {
        let guide = create_test_guide();
        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let character_service = Arc::new(MockCharacterService::new());
        let service = create_test_service(repository, character_service);

        let new_progress = WalkthroughProgress::new();

        let result = service
            .update_character_progress("nonexistent", new_progress)
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_character_progress_save_failure() {
        let guide = create_test_guide();
        let character = create_test_character("char-1");
        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let character_service = Arc::new(MockCharacterService::with_save_failure(character));
        let service = create_test_service(repository, character_service);

        let new_progress = WalkthroughProgress::new();

        let result = service
            .update_character_progress("char-1", new_progress)
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_character_progress_invalid_step_id_rejected() {
        let guide = create_test_guide();
        let character = create_test_character("char-1");
        let character_service = Arc::new(MockCharacterService::with_character(character));

        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let service = create_test_service(repository, character_service);

        // Try to set an invalid step ID
        let mut invalid_progress = WalkthroughProgress::new();
        invalid_progress.set_current_step("nonexistent_step".to_string());

        let result = service
            .update_character_progress("char-1", invalid_progress)
            .await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        match error {
            AppError::Validation { message } => {
                assert!(message.contains("Invalid step ID"));
                assert!(message.contains("nonexistent_step"));
            }
            _ => panic!("Expected Validation error, got {:?}", error),
        }
    }

    #[tokio::test]
    async fn test_update_character_progress_completed_with_no_step_id_allowed() {
        let guide = create_test_guide();
        let character = create_test_character("char-1");
        let character_service = Arc::new(MockCharacterService::with_character(character));

        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let service = create_test_service(repository, character_service.clone());

        // Set progress to completed with no step ID - should be allowed
        let completed_progress = WalkthroughProgress::completed();

        let result = service
            .update_character_progress("char-1", completed_progress)
            .await;

        assert!(result.is_ok());

        // Verify character was updated
        let characters = character_service.characters.read().await;
        let updated_char = characters.get("char-1").unwrap();
        assert!(updated_char.walkthrough_progress.is_completed);
        assert_eq!(updated_char.walkthrough_progress.current_step_id, None);
    }

    // ============= handle_scene_change() Tests - Basic Scenarios =============

    #[tokio::test]
    async fn test_handle_scene_change_advances_step() {
        let guide = create_test_guide();
        let character = create_character_at_step("char-1", "act_1_step_1");
        let character_service = Arc::new(MockCharacterService::with_character(character));

        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let service = create_test_service(repository, character_service.clone());

        // Scene change to completion zone should advance step
        let result = service.handle_scene_change("char-1", "The Coast").await;

        assert!(result.is_ok());

        // Verify character was advanced to next step
        let characters = character_service.characters.read().await;
        let updated_char = characters.get("char-1").unwrap();
        assert_eq!(
            updated_char.walkthrough_progress.current_step_id,
            Some("act_1_step_2".to_string())
        );
    }

    #[tokio::test]
    async fn test_handle_scene_change_no_match_no_advancement() {
        let guide = create_test_guide();
        let character = create_character_at_step("char-1", "act_1_step_1");
        let character_service = Arc::new(MockCharacterService::with_character(character));

        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let service = create_test_service(repository, character_service.clone());

        // Scene change to non-completion zone should not advance
        let result = service.handle_scene_change("char-1", "Random Zone").await;

        assert!(result.is_ok());

        // Verify character was NOT advanced
        let characters = character_service.characters.read().await;
        let updated_char = characters.get("char-1").unwrap();
        assert_eq!(
            updated_char.walkthrough_progress.current_step_id,
            Some("act_1_step_1".to_string())
        );
    }

    #[tokio::test]
    async fn test_handle_scene_change_empty_zone_name() {
        let guide = create_test_guide();
        let character = create_character_at_step("char-1", "act_1_step_1");
        let character_service = Arc::new(MockCharacterService::with_character(character));

        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let service = create_test_service(repository, character_service.clone());

        // Empty zone name should not cause any changes
        let result = service.handle_scene_change("char-1", "").await;

        assert!(result.is_ok());

        // Verify character was NOT changed
        let characters = character_service.characters.read().await;
        let updated_char = characters.get("char-1").unwrap();
        assert_eq!(
            updated_char.walkthrough_progress.current_step_id,
            Some("act_1_step_1".to_string())
        );
    }

    #[tokio::test]
    async fn test_handle_scene_change_whitespace_zone_name() {
        let guide = create_test_guide();
        let character = create_character_at_step("char-1", "act_1_step_1");
        let character_service = Arc::new(MockCharacterService::with_character(character));

        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let service = create_test_service(repository, character_service.clone());

        // Whitespace-only zone name should not cause any changes
        let result = service.handle_scene_change("char-1", "   \t\n   ").await;

        assert!(result.is_ok());

        // Verify character was NOT changed
        let characters = character_service.characters.read().await;
        let updated_char = characters.get("char-1").unwrap();
        assert_eq!(
            updated_char.walkthrough_progress.current_step_id,
            Some("act_1_step_1".to_string())
        );
    }

    #[tokio::test]
    async fn test_handle_scene_change_completed_campaign_skips_processing() {
        let guide = create_test_guide();
        let character = create_completed_character("char-1");
        let character_service = Arc::new(MockCharacterService::with_character(character));

        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let service = create_test_service(repository, character_service.clone());

        // Completed campaign should skip processing entirely
        let result = service.handle_scene_change("char-1", "The Coast").await;

        assert!(result.is_ok());

        // Verify character remains completed
        let characters = character_service.characters.read().await;
        let updated_char = characters.get("char-1").unwrap();
        assert!(updated_char.walkthrough_progress.is_completed);
        assert_eq!(updated_char.walkthrough_progress.current_step_id, None);
    }

    // ============= handle_scene_change() Tests - Campaign Completion =============

    #[tokio::test]
    async fn test_handle_scene_change_completes_campaign() {
        let guide = create_test_guide();
        // Character at last step
        let character = create_character_at_step("char-1", "act_2_step_1");
        let character_service = Arc::new(MockCharacterService::with_character(character));

        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let service = create_test_service(repository, character_service.clone());

        // Scene change to final completion zone
        let result = service
            .handle_scene_change("char-1", "Final Boss Arena")
            .await;

        assert!(result.is_ok());

        // Verify campaign is marked completed
        let characters = character_service.characters.read().await;
        let updated_char = characters.get("char-1").unwrap();
        assert!(updated_char.walkthrough_progress.is_completed);
        assert_eq!(updated_char.walkthrough_progress.current_step_id, None);
    }

    // ============= handle_scene_change() Tests - Error Cases =============

    #[tokio::test]
    async fn test_handle_scene_change_missing_current_step_in_guide() {
        let guide = create_test_guide();
        // Character at step that doesn't exist in guide
        let character = create_character_at_step("char-1", "nonexistent_step");
        let character_service = Arc::new(MockCharacterService::with_character(character));

        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let service = create_test_service(repository, character_service.clone());

        // Scene change - should handle gracefully
        let result = service.handle_scene_change("char-1", "The Coast").await;

        assert!(result.is_ok()); // No error, just no action
    }

    #[tokio::test]
    async fn test_handle_scene_change_no_current_step_id() {
        let guide = create_test_guide();
        let mut character = create_test_character("char-1");
        // Set inconsistent state: not completed but no current step
        character.walkthrough_progress.current_step_id = None;
        character.walkthrough_progress.is_completed = false;

        let character_service = Arc::new(MockCharacterService::with_character(character));

        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let service = create_test_service(repository, character_service.clone());

        // Scene change - should handle gracefully
        let result = service.handle_scene_change("char-1", "The Coast").await;

        assert!(result.is_ok()); // No error, just no action
    }

    #[tokio::test]
    async fn test_handle_scene_change_character_not_found() {
        let guide = create_test_guide();
        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let character_service = Arc::new(MockCharacterService::new());
        let service = create_test_service(repository, character_service);

        let result = service
            .handle_scene_change("nonexistent", "The Coast")
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_handle_scene_change_guide_load_failure() {
        let repository = Arc::new(MockWalkthroughRepository::with_failure());
        let character = create_character_at_step("char-1", "act_1_step_1");
        let character_service = Arc::new(MockCharacterService::with_character(character));
        let service = create_test_service(repository, character_service);

        let result = service.handle_scene_change("char-1", "The Coast").await;

        assert!(result.is_err());
    }

    // ============= Multi-Step Progression Tests =============

    #[tokio::test]
    async fn test_progress_through_multiple_steps() {
        let guide = create_test_guide();
        let character = create_character_at_step("char-1", "act_1_step_1");
        let character_service = Arc::new(MockCharacterService::with_character(character));

        let repository = Arc::new(MockWalkthroughRepository::new(guide));
        let service = create_test_service(repository, character_service.clone());

        // Step 1 -> Step 2
        service
            .handle_scene_change("char-1", "The Coast")
            .await
            .unwrap();
        {
            let characters = character_service.characters.read().await;
            let char = characters.get("char-1").unwrap();
            assert_eq!(
                char.walkthrough_progress.current_step_id,
                Some("act_1_step_2".to_string())
            );
        }

        // Step 2 -> Step 3 (Act 2)
        service
            .handle_scene_change("char-1", "Town Square")
            .await
            .unwrap();
        {
            let characters = character_service.characters.read().await;
            let char = characters.get("char-1").unwrap();
            assert_eq!(
                char.walkthrough_progress.current_step_id,
                Some("act_2_step_1".to_string())
            );
        }

        // Step 3 -> Completed
        service
            .handle_scene_change("char-1", "Final Boss Arena")
            .await
            .unwrap();
        {
            let characters = character_service.characters.read().await;
            let char = characters.get("char-1").unwrap();
            assert!(char.walkthrough_progress.is_completed);
            assert_eq!(char.walkthrough_progress.current_step_id, None);
        }
    }
}
