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
   * Get a specific walkthrough step
   */
  static async getStep(stepId: string): Promise<WalkthroughStepResult> {
    return await invoke('get_walkthrough_step', { stepId });
  }

  /**
   * Advance character to the next step
   */
  static async advanceToNextStep(characterId: string): Promise<void> {
    return await invoke('advance_character_to_next_step', { characterId });
  }

  /**
   * Mark character's campaign as completed
   */
  static async markCampaignCompleted(characterId: string): Promise<void> {
    return await invoke('mark_character_campaign_completed', { characterId });
  }

  /**
   * Start walkthrough for a character
   */
  static async startWalkthrough(characterId: string): Promise<void> {
    return await invoke('start_character_walkthrough', { characterId });
  }

  /**
   * Set current step for a character
   */
  static async setCurrentStep(
    characterId: string,
    stepId: string
  ): Promise<void> {
    return await invoke('set_character_walkthrough_step', {
      characterId,
      stepId,
    });
  }
}
