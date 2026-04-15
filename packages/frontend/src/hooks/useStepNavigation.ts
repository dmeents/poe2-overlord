import { invoke } from '@tauri-apps/api/core';
import { useCallback } from 'react';
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
  const advanceStep = useCallback(
    async (nextStepId: string | null) => {
      if (!characterId || !progress || !nextStepId) {
        if (!nextStepId) {
          console.warn('No next step available. Campaign may be completed.');
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
    },
    [characterId, progress],
  );

  const goToPreviousStep = useCallback(
    async (previousStepId: string) => {
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
    },
    [characterId, progress],
  );

  const skipToStep = useCallback(
    async (stepId: string) => {
      if (!characterId) return;

      try {
        const newProgress: WalkthroughProgress = {
          ...progress,
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
    },
    [characterId, progress],
  );

  return {
    advanceStep,
    goToPreviousStep,
    skipToStep,
  };
}
