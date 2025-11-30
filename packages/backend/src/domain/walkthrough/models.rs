use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a single objective within a walkthrough step.
///
/// Objectives define specific tasks or goals that must be completed to advance
/// through a walkthrough step. They can be required or optional, and may include
/// rewards and additional context information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Objective {
    /// The main objective text describing what needs to be done
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

/// Represents a single step in the walkthrough guide.
///
/// Steps are the individual components that make up the walkthrough progression.
/// Each step contains objectives, zone information, and navigation context
/// to guide players through the campaign.
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

/// Represents an act in the walkthrough guide.
///
/// Acts are major sections of the campaign that contain multiple walkthrough steps.
/// Each act has a name, number for ordering, and a collection of steps that
/// guide players through that portion of the campaign.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalkthroughAct {
    /// Name of the act (e.g., "Act 4")
    pub act_name: String,
    /// Act number for ordering
    pub act_number: u32,
    /// Steps within this act
    pub steps: HashMap<String, WalkthroughStep>,
}

/// Represents the complete walkthrough guide structure.
///
/// The walkthrough guide contains all acts and steps that make up the complete
/// campaign walkthrough. This is the top-level structure loaded from the JSON
/// configuration file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalkthroughGuide {
    /// All acts in the walkthrough
    pub acts: HashMap<String, WalkthroughAct>,
}

/// Represents a character's progress through the walkthrough.
///
/// This struct tracks where a character is in their walkthrough progression,
/// including the current step, completion status, and when progress was last updated.
/// Progress is stored as part of the character's data and updated as they advance.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalkthroughProgress {
    /// Current step ID the character is on (defaults to "act_1_step_1")
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
            current_step_id: Some("act_1_step_1".to_string()),
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

    /// Updates the last_updated timestamp to current time
    pub fn touch(&mut self) {
        self.last_updated = Utc::now();
    }
}

impl Default for WalkthroughProgress {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents the result of a walkthrough step lookup.
///
/// This struct combines step data with its act context, providing complete
/// information about a step including which act it belongs to.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalkthroughStepResult {
    /// The step data
    pub step: WalkthroughStep,
    /// The act this step belongs to
    pub act_name: String,
    /// The act number
    pub act_number: u32,
}

/// Represents the result of getting a character's walkthrough progress.
///
/// This struct combines a character's progress with navigation context,
/// providing information about the current step and adjacent steps for UI navigation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CharacterWalkthroughProgress {
    /// The character's progress
    pub progress: WalkthroughProgress,
    /// The next step ID (if available)
    pub next_step_id: Option<String>,
    /// The previous step ID (if available)
    pub previous_step_id: Option<String>,
}
