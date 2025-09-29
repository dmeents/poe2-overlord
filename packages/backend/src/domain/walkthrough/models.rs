use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a single objective within a walkthrough step
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Objective {
    /// The main objective text
    pub text: String,
    /// Additional details about the objective
    pub details: Option<String>,
    /// Whether this objective is required for step completion
    pub required: bool,
    /// Rewards for completing this objective
    pub rewards: Vec<String>,
    /// Additional notes for this objective
    pub notes: Option<String>,
}

/// Represents a single step in the walkthrough guide
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalkthroughStep {
    /// Unique identifier for this step
    pub id: String,
    /// Title of the step
    pub title: String,
    /// Detailed description of what needs to be done
    pub description: String,
    /// List of objectives for this step
    pub objectives: Vec<Objective>,
    /// The zone where this step begins
    pub current_zone: String,
    /// The zone that indicates this step is complete
    pub completion_zone: String,
    /// ID of the next step (None if this is the last step)
    pub next_step_id: Option<String>,
    /// ID of the previous step (None if this is the first step)
    pub previous_step_id: Option<String>,
    /// Terms that should be enriched with wiki links in the UI
    pub wiki_items: Vec<String>,
}

/// Represents an act in the walkthrough guide
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalkthroughAct {
    /// Name of the act (e.g., "Act 4")
    pub act_name: String,
    /// Act number for ordering
    pub act_number: u32,
    /// Steps within this act
    pub steps: HashMap<String, WalkthroughStep>,
}

/// Represents the complete walkthrough guide structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalkthroughGuide {
    /// All acts in the walkthrough
    pub acts: HashMap<String, WalkthroughAct>,
}

/// Represents a character's progress through the walkthrough
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalkthroughProgress {
    /// Current step ID the character is on (defaults to "act_4_step_1" for initial implementation)
    pub current_step_id: Option<String>,
    /// Whether the character has completed the entire campaign
    pub is_completed: bool,
    /// When this progress was last updated
    pub last_updated: DateTime<Utc>,
}

impl WalkthroughProgress {
    /// Creates new walkthrough progress for a character starting at the beginning
    pub fn new() -> Self {
        Self {
            current_step_id: Some("act_4_step_1".to_string()),
            is_completed: false,
            last_updated: Utc::now(),
        }
    }

    /// Creates walkthrough progress for a character who has completed the campaign
    pub fn completed() -> Self {
        Self {
            current_step_id: None,
            is_completed: true,
            last_updated: Utc::now(),
        }
    }

    /// Updates the progress to the next step
    pub fn advance_to_next_step(&mut self, next_step_id: Option<String>) {
        self.current_step_id = next_step_id.clone();
        self.is_completed = next_step_id.is_none();
        self.last_updated = Utc::now();
    }

    /// Updates the progress to a specific step
    pub fn set_current_step(&mut self, step_id: String) {
        self.current_step_id = Some(step_id);
        self.is_completed = false;
        self.last_updated = Utc::now();
    }

    /// Marks the campaign as completed
    pub fn mark_completed(&mut self) {
        self.current_step_id = None;
        self.is_completed = true;
        self.last_updated = Utc::now();
    }

    /// Updates the last_updated timestamp
    pub fn touch(&mut self) {
        self.last_updated = Utc::now();
    }
}

impl Default for WalkthroughProgress {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents the result of a walkthrough step lookup
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalkthroughStepResult {
    /// The step data
    pub step: WalkthroughStep,
    /// The act this step belongs to
    pub act_name: String,
    /// The act number
    pub act_number: u32,
}

/// Represents the result of getting a character's walkthrough progress
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CharacterWalkthroughProgress {
    /// The character's progress
    pub progress: WalkthroughProgress,
    /// The current step data (if not completed)
    pub current_step: Option<WalkthroughStepResult>,
    /// The next step data (if available)
    pub next_step: Option<WalkthroughStepResult>,
    /// The previous step data (if available)
    pub previous_step: Option<WalkthroughStepResult>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walkthrough_progress_new() {
        let progress = WalkthroughProgress::new();
        assert_eq!(progress.current_step_id, Some("act_4_step_1".to_string()));
        assert!(!progress.is_completed);
    }

    #[test]
    fn test_walkthrough_progress_completed() {
        let progress = WalkthroughProgress::completed();
        assert_eq!(progress.current_step_id, None);
        assert!(progress.is_completed);
    }

    #[test]
    fn test_walkthrough_progress_advance() {
        let mut progress = WalkthroughProgress::new();
        progress.advance_to_next_step(Some("act_4_step_2".to_string()));
        assert_eq!(progress.current_step_id, Some("act_4_step_2".to_string()));
        assert!(!progress.is_completed);

        progress.advance_to_next_step(None);
        assert_eq!(progress.current_step_id, None);
        assert!(progress.is_completed);
    }

    #[test]
    fn test_walkthrough_progress_set_current_step() {
        let mut progress = WalkthroughProgress::new();
        progress.set_current_step("act_4_step_5".to_string());
        assert_eq!(progress.current_step_id, Some("act_4_step_5".to_string()));
        assert!(!progress.is_completed);
    }

    #[test]
    fn test_walkthrough_progress_mark_completed() {
        let mut progress = WalkthroughProgress::new();
        progress.mark_completed();
        assert_eq!(progress.current_step_id, None);
        assert!(progress.is_completed);
    }
}
