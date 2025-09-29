/**
 * Frontend types for walkthrough functionality
 * These types mirror the backend Rust structures for type safety
 */

export interface WalkthroughProgress {
  /** Current step ID the character is on */
  current_step_id: string | null;
  /** Whether the character has completed the entire campaign */
  is_completed: boolean;
  /** When this progress was last updated */
  last_updated: string;
}

export interface WalkthroughStep {
  /** Unique identifier for this step */
  id: string;
  /** Human-readable title of the step */
  title: string;
  /** Detailed description of what to do */
  description: string;
  /** Current zone the character should be in */
  current_zone: string;
  /** Zone that indicates completion of this step */
  completion_zone: string;
  /** ID of the next step (null if this is the last step) */
  next_step_id: string | null;
  /** ID of the previous step (null if this is the first step) */
  previous_step_id: string | null;
  /** List of objectives to complete in this step */
  objectives: Objective[];
  /** Wiki items related to this step */
  wiki_items: string[];
}

export interface Objective {
  /** The main objective text */
  text: string;
  /** Additional details about the objective */
  details?: string;
  /** Whether this objective is required for step completion */
  required: boolean;
  /** Rewards for completing this objective */
  rewards: string[];
  /** Additional notes for this objective */
  notes?: string;
}

export interface WikiItem {
  /** Title of the wiki item */
  title: string;
  /** URL to the wiki page */
  url: string;
  /** Brief description of the item */
  description: string;
}

export interface WalkthroughAct {
  /** Name of the act (e.g., "Act 4") */
  act_name: string;
  /** Act number for ordering */
  act_number: number;
  /** Steps within this act */
  steps: { [key: string]: WalkthroughStep };
}

export interface WalkthroughGuide {
  /** All acts in the walkthrough */
  acts: { [key: string]: WalkthroughAct };
}

export interface WalkthroughStepResult {
  /** The step data */
  step: WalkthroughStep;
  /** The act this step belongs to */
  act: WalkthroughAct;
}

export interface CharacterWalkthroughProgress {
  /** The character's progress */
  progress: WalkthroughProgress;
  /** The next step ID (if available) */
  next_step_id: string | null;
  /** The previous step ID (if available) */
  previous_step_id: string | null;
}
