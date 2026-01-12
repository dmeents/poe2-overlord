import { invoke } from '@tauri-apps/api/core';
import type {
  WalkthroughGuide,
  WalkthroughProgress,
  WalkthroughStepResult,
} from '../types/walkthrough';

/**
 * Walkthrough service for interacting with the backend walkthrough functionality
 */
export class WalkthroughService {
  /**
   * Get the walkthrough guide
   */
  static async getGuide(): Promise<WalkthroughGuide> {
    return await invoke('get_walkthrough_guide');
  }

  /**
   * Get walkthrough progress for a character
   */
  static async getProgress(characterId: string): Promise<WalkthroughProgress> {
    return await invoke('get_character_walkthrough_progress', { characterId });
  }

  /**
   * Update walkthrough progress for a character
   */
  static async updateProgress(
    characterId: string,
    progress: WalkthroughProgress
  ): Promise<void> {
    return await invoke('update_character_walkthrough_progress', {
      characterId,
      progress,
    });
  }

  /**
   * Move character to a specific step
   */
  static async moveToStep(characterId: string, stepId: string): Promise<void> {
    // Get current progress first
    const currentProgress = await this.getProgress(characterId);

    // Create new progress with the target step
    const newProgress = {
      ...currentProgress,
      current_step_id: stepId,
      is_completed: false,
      last_updated: new Date().toISOString(),
    };

    return await invoke('update_character_walkthrough_progress', {
      characterId,
      progress: newProgress,
    });
  }

  /**
   * Mark character's campaign as completed
   */
  static async markCampaignCompleted(characterId: string): Promise<void> {
    // Get current progress first
    const currentProgress = await this.getProgress(characterId);

    // Create new progress marking campaign as completed
    const newProgress = {
      ...currentProgress,
      current_step_id: null,
      is_completed: true,
      last_updated: new Date().toISOString(),
    };

    return await invoke('update_character_walkthrough_progress', {
      characterId,
      progress: newProgress,
    });
  }

  /**
   * Start walkthrough for a character
   */
  static async startWalkthrough(characterId: string): Promise<void> {
    return await invoke('start_character_walkthrough', { characterId });
  }

  /**
   * Find a step in the guide by its ID
   */
  static findStepInGuide(
    guide: WalkthroughGuide,
    stepId: string
  ): WalkthroughStepResult | null {
    for (const act of Object.values(guide.acts)) {
      if (act.steps[stepId]) {
        return {
          step: act.steps[stepId],
          act_name: act.act_name,
          act_number: act.act_number,
        };
      }
    }
    return null;
  }

  /**
   * Get step details from guide using step ID
   */
  static getStepFromGuide(
    guide: WalkthroughGuide,
    stepId: string | null
  ): WalkthroughStepResult | null {
    if (!stepId) return null;
    return this.findStepInGuide(guide, stepId);
  }

  /**
   * Get multiple steps from guide using step IDs
   */
  static getStepsFromGuide(
    guide: WalkthroughGuide,
    currentStepId: string | null,
    nextStepId: string | null,
    previousStepId: string | null
  ): {
    currentStep: WalkthroughStepResult | null;
    nextStep: WalkthroughStepResult | null;
    previousStep: WalkthroughStepResult | null;
  } {
    return {
      currentStep: this.getStepFromGuide(guide, currentStepId),
      nextStep: this.getStepFromGuide(guide, nextStepId),
      previousStep: this.getStepFromGuide(guide, previousStepId),
    };
  }
}
