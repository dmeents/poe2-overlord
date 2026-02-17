use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a link to external resources (e.g., wiki pages).
///
/// Links provide explicit text and URL pairs for enriching step content
/// with references to relevant external documentation or resources.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StepLink {
    /// The text to match in step content
    pub text: String,
    /// The URL to open when clicked
    pub url: String,
}

/// Represents a single objective within a walkthrough step.
///
/// Objectives define specific tasks or goals that must be completed to advance
/// through a walkthrough step. They can be required or optional, and may include
/// rewards and additional context information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Objective {
    /// The main objective text describing what needs to be done
    pub text: String,
    /// Additional details about the objective (may include merged notes)
    pub details: Option<String>,
    /// Whether this objective is required for step completion
    pub required: bool,
    /// Rewards for completing this objective
    pub rewards: Vec<String>,
    /// Whether this objective only needs to be completed once per league (on first character)
    #[serde(default, rename = "leagueStart")]
    pub league_start: bool,
}

/// Represents a single step in the walkthrough guide.
///
/// Steps are the individual components that make up the walkthrough progression.
/// Each step contains objectives, zone information, and external resource links
/// to guide players through the campaign. Navigation is determined by array position.
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
    /// External resource links (e.g., wiki pages) related to this step
    pub links: Vec<StepLink>,
}

/// Represents an act in the walkthrough guide.
///
/// Acts are major sections of the campaign that contain multiple walkthrough steps.
/// Each act has a name and an ordered collection of steps. Act ordering is implicit
/// in the array position within the WalkthroughGuide.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalkthroughAct {
    /// Name of the act (e.g., "Act 4")
    pub act_name: String,
    /// Steps within this act (ordered)
    pub steps: Vec<WalkthroughStep>,
}

/// Represents the complete walkthrough guide structure.
///
/// The walkthrough guide contains all acts and steps that make up the complete
/// campaign walkthrough. This is the top-level structure loaded from the JSON
/// configuration file. Acts and steps are ordered arrays, with navigation
/// determined by array position rather than explicit next/previous references.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalkthroughGuide {
    /// All acts in the walkthrough (ordered)
    pub acts: Vec<WalkthroughAct>,
}

impl WalkthroughGuide {
    /// Finds a step by its ID across all acts.
    ///
    /// Returns a tuple of (act_index, step_index) if found, None otherwise.
    pub fn find_step(&self, step_id: &str) -> Option<(usize, usize)> {
        for (act_idx, act) in self.acts.iter().enumerate() {
            for (step_idx, step) in act.steps.iter().enumerate() {
                if step.id == step_id {
                    return Some((act_idx, step_idx));
                }
            }
        }
        None
    }

    /// Checks if a step with the given ID exists in the guide.
    pub fn step_exists(&self, step_id: &str) -> bool {
        self.find_step(step_id).is_some()
    }

    /// Gets the ID of the first step in the guide.
    ///
    /// Returns None if the guide has no acts or the first act has no steps.
    pub fn first_step_id(&self) -> Option<&str> {
        self.acts
            .first()
            .and_then(|act| act.steps.first())
            .map(|step| step.id.as_str())
    }

    /// Gets the ID of the next step after the given step ID.
    ///
    /// Returns None if the step is not found or is the last step in the guide.
    pub fn next_step_id(&self, step_id: &str) -> Option<String> {
        let (act_idx, step_idx) = self.find_step(step_id)?;

        // Try next step in current act
        if let Some(next_step) = self.acts[act_idx].steps.get(step_idx + 1) {
            return Some(next_step.id.clone());
        }

        // Try first step of next act
        if let Some(next_act) = self.acts.get(act_idx + 1) {
            if let Some(first_step) = next_act.steps.first() {
                return Some(first_step.id.clone());
            }
        }

        None
    }

    /// Gets the ID of the previous step before the given step ID.
    ///
    /// Returns None if the step is not found or is the first step in the guide.
    pub fn previous_step_id(&self, step_id: &str) -> Option<String> {
        let (act_idx, step_idx) = self.find_step(step_id)?;

        // Try previous step in current act
        if step_idx > 0 {
            return Some(self.acts[act_idx].steps[step_idx - 1].id.clone());
        }

        // Try last step of previous act
        if act_idx > 0 {
            if let Some(last_step) = self.acts[act_idx - 1].steps.last() {
                return Some(last_step.id.clone());
            }
        }

        None
    }
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
    /// The act number (1-based index derived from position)
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
