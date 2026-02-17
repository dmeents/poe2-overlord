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

export interface StepLink {
  /** The text to match in step content */
  text: string;
  /** The URL to open when clicked */
  url: string;
}

export interface Objective {
  /** The main objective text */
  text: string;
  /** Additional details about the objective (may include merged notes) */
  details?: string;
  /** Whether this objective is required for step completion */
  required: boolean;
  /** Rewards for completing this objective */
  rewards: string[];
  /** Whether this objective only needs to be completed once per league (on first character) */
  leagueStart?: boolean;
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
  /** List of objectives to complete in this step */
  objectives: Objective[];
  /** External resource links (e.g., wiki pages) related to this step */
  links: StepLink[];
}

export interface WalkthroughAct {
  /** Name of the act (e.g., "Act 4") */
  act_name: string;
  /** Steps within this act (ordered) */
  steps: WalkthroughStep[];
}

export interface WalkthroughGuide {
  /** All acts in the walkthrough (ordered) */
  acts: WalkthroughAct[];
}

export interface WalkthroughStepResult {
  /** The step data */
  step: WalkthroughStep;
  /** The act name this step belongs to */
  act_name: string;
  /** The act number (1-based index derived from position) */
  act_number: number;
}
