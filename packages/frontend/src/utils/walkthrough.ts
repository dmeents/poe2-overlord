import { invoke } from '@tauri-apps/api/core';
import type {
  WalkthroughGuide,
  WalkthroughProgress,
  WalkthroughStepResult,
} from '../types/walkthrough';

/**
 * Get the walkthrough guide
 */
export async function getGuide(): Promise<WalkthroughGuide> {
  return await invoke('get_walkthrough_guide');
}

/**
 * Get walkthrough progress for a character
 */
export async function getProgress(characterId: string): Promise<WalkthroughProgress> {
  return await invoke('get_character_walkthrough_progress', { characterId });
}

/**
 * Update walkthrough progress for a character
 */
export async function updateProgress(
  characterId: string,
  progress: WalkthroughProgress,
): Promise<void> {
  return await invoke('update_character_walkthrough_progress', {
    characterId,
    progress,
  });
}

/**
 * Move character to a specific step
 */
export async function moveToStep(characterId: string, stepId: string): Promise<void> {
  // Get current progress first
  const currentProgress = await getProgress(characterId);

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
export async function markCampaignCompleted(characterId: string): Promise<void> {
  // Get current progress first
  const currentProgress = await getProgress(characterId);

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
export async function startWalkthrough(characterId: string): Promise<void> {
  return await invoke('start_character_walkthrough', { characterId });
}

/**
 * Find a step in the guide by its ID
 */
export function findStepInGuide(
  guide: WalkthroughGuide,
  stepId: string,
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
export function getStepFromGuide(
  guide: WalkthroughGuide,
  stepId: string | null,
): WalkthroughStepResult | null {
  if (!stepId) return null;
  return findStepInGuide(guide, stepId);
}

/**
 * Get multiple steps from guide using step IDs
 */
export function getStepsFromGuide(
  guide: WalkthroughGuide,
  currentStepId: string | null,
  nextStepId: string | null,
  previousStepId: string | null,
): {
  currentStep: WalkthroughStepResult | null;
  nextStep: WalkthroughStepResult | null;
  previousStep: WalkthroughStepResult | null;
} {
  return {
    currentStep: getStepFromGuide(guide, currentStepId),
    nextStep: getStepFromGuide(guide, nextStepId),
    previousStep: getStepFromGuide(guide, previousStepId),
  };
}

/**
 * WalkthroughService namespace for backward compatibility
 * @deprecated Use the exported functions directly instead
 */
export const WalkthroughService = {
  getGuide,
  getProgress,
  updateProgress,
  moveToStep,
  markCampaignCompleted,
  startWalkthrough,
  findStepInGuide,
  getStepFromGuide,
  getStepsFromGuide,
};
