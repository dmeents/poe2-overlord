import { invoke } from '@tauri-apps/api/core';
import type { WalkthroughProgress } from '../types/walkthrough';

interface UseStepNavigationOptions {
  characterId: string | null;
  progress: WalkthroughProgress | null;
}

interface UseStepNavigationResult {
  advanceStep: (nextStepId: string | null) => Promise<void>;
  goToPreviousStep: (previousStepId: string) => Promise<void>;
  skipToStep: (stepId: string) => Promise<void>;
}

/**
 * Shared hook for walkthrough step navigation
 * Extracts common progress update logic used across multiple components
 */
export function useStepNavigation({
  characterId,
  progress,
}: UseStepNavigationOptions): UseStepNavigationResult {
  const advanceStep = async (nextStepId: string | null) => {
    if (!characterId || !progress || !nextStepId) {
      if (!nextStepId) {
        console.error('No next step available. Campaign may be completed.');
      }
      return;
    }

    try {
      const newProgress: WalkthroughProgress = {
        ...progress,
        current_step_id: nextStepId,
        is_completed: false,
        last_updated: new Date().toISOString(),
      };

      await invoke('update_character_walkthrough_progress', {
        characterId,
        progress: newProgress,
      });
    } catch (err) {
      console.error('Failed to advance step:', err);
    }
  };

  const goToPreviousStep = async (previousStepId: string) => {
    if (!characterId || !progress) return;

    try {
      const newProgress: WalkthroughProgress = {
        ...progress,
        current_step_id: previousStepId,
        is_completed: false,
        last_updated: new Date().toISOString(),
      };

      await invoke('update_character_walkthrough_progress', {
        characterId,
        progress: newProgress,
      });
    } catch (err) {
      console.error('Failed to go to previous step:', err);
    }
  };

  const skipToStep = async (stepId: string) => {
    if (!characterId) return;

    try {
      const newProgress: WalkthroughProgress = {
        current_step_id: stepId,
        is_completed: false,
        last_updated: new Date().toISOString(),
      };

      await invoke('update_character_walkthrough_progress', {
        characterId,
        progress: newProgress,
      });
    } catch (err) {
      console.error('Failed to skip to step:', err);
    }
  };

  return {
    advanceStep,
    goToPreviousStep,
    skipToStep,
  };
}
