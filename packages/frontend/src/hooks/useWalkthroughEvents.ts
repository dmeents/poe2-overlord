import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useCallback, useEffect, useRef, useState } from 'react';
import type {
  WalkthroughProgress,
  WalkthroughStepResult,
} from '../types/walkthrough';

/**
 * Hook for listening to walkthrough progress updates via Tauri events
 *
 * This hook provides real-time walkthrough progress updates by listening
 * to walkthrough-related events from the backend and maintaining the
 * current progress state in local state.
 *
 * @param characterId - The character ID to listen for events for
 * @returns Object containing walkthrough progress and event handlers
 */
export function useWalkthroughEvents(characterId: string | null) {
  const [progress, setProgress] = useState<WalkthroughProgress | null>(null);
  const [currentStep, setCurrentStep] = useState<WalkthroughStepResult | null>(
    null
  );
  const [previousStep, setPreviousStep] =
    useState<WalkthroughStepResult | null>(null);
  const [isListening, setIsListening] = useState(false);
  const [lastEvent, setLastEvent] = useState<string | null>(null);

  // Event listener management
  const listenerRef = useRef<(() => void) | null>(null);
  const isListeningRef = useRef(false);

  // Function to fetch previous step data
  const fetchPreviousStep = useCallback(async (stepId: string) => {
    try {
      const step = await invoke<WalkthroughStepResult>('get_walkthrough_step', {
        stepId,
      });
      setPreviousStep(step);
    } catch (error) {
      console.error('Failed to fetch previous step:', error);
    }
  }, []);

  // Function to fetch current step data
  const fetchCurrentStep = useCallback(
    async (stepId: string) => {
      try {
        const step = await invoke<WalkthroughStepResult>(
          'get_walkthrough_step',
          {
            stepId,
          }
        );
        setCurrentStep(step);

        // Also fetch previous step if available
        if (step.step.previous_step_id) {
          fetchPreviousStep(step.step.previous_step_id);
        } else {
          setPreviousStep(null);
        }
      } catch (error) {
        console.error('Failed to fetch current step:', error);
      }
    },
    [fetchPreviousStep]
  );

  // Event handler for walkthrough progress updates
  const handleWalkthroughProgressUpdated = useCallback(
    (event: { payload: unknown }) => {
      if (event.payload && typeof event.payload === 'object') {
        const payload = event.payload as Record<string, unknown>;
        if (payload.WalkthroughProgressUpdated) {
          const progressEvent = payload.WalkthroughProgressUpdated as {
            character_id: string;
            progress: WalkthroughProgress;
            timestamp: string;
          };

          if (progressEvent.character_id === characterId) {
            setProgress(progressEvent.progress);
            setLastEvent('progress_updated');

            // Fetch current step if there is one
            if (progressEvent.progress.current_step_id) {
              fetchCurrentStep(progressEvent.progress.current_step_id);
            } else {
              // No current step (campaign completed)
              setCurrentStep(null);
              setPreviousStep(null);
            }
          }
        }
      }
    },
    [characterId, fetchCurrentStep]
  );

  // Event handler for walkthrough step completed
  const handleWalkthroughStepCompleted = useCallback(
    (event: { payload: unknown }) => {
      if (event.payload && typeof event.payload === 'object') {
        const payload = event.payload as Record<string, unknown>;
        if (payload.WalkthroughStepCompleted) {
          const stepEvent = payload.WalkthroughStepCompleted as {
            character_id: string;
            step: WalkthroughStepResult;
            timestamp: string;
          };

          if (stepEvent.character_id === characterId) {
            setCurrentStep(stepEvent.step);
            setLastEvent('step_completed');

            // Fetch previous step if there is one
            if (stepEvent.step.step.previous_step_id) {
              fetchPreviousStep(stepEvent.step.step.previous_step_id);
            } else {
              setPreviousStep(null);
            }
          }
        }
      }
    },
    [characterId, fetchPreviousStep]
  );

  // Event handler for walkthrough step advanced
  const handleWalkthroughStepAdvanced = useCallback(
    (event: { payload: unknown }) => {
      if (event.payload && typeof event.payload === 'object') {
        const payload = event.payload as Record<string, unknown>;
        if (payload.WalkthroughStepAdvanced) {
          const stepEvent = payload.WalkthroughStepAdvanced as {
            character_id: string;
            from_step_id: string | null;
            to_step_id: string | null;
            timestamp: string;
          };

          if (stepEvent.character_id === characterId) {
            setLastEvent('step_advanced');
            // The progress will be updated via the progress updated event
            // We need to fetch the new current step since it's not included in this event
            // This will be handled by the progress updated event
          }
        }
      }
    },
    [characterId]
  );

  // Event handler for walkthrough campaign completed
  const handleWalkthroughCampaignCompleted = useCallback(
    (event: { payload: unknown }) => {
      if (event.payload && typeof event.payload === 'object') {
        const payload = event.payload as Record<string, unknown>;
        if (payload.WalkthroughCampaignCompleted) {
          const campaignEvent = payload.WalkthroughCampaignCompleted as {
            character_id: string;
            timestamp: string;
          };

          if (campaignEvent.character_id === characterId) {
            setLastEvent('campaign_completed');
            // The progress will be updated via the progress updated event
          }
        }
      }
    },
    [characterId]
  );

  // Set up event listeners for walkthrough events
  useEffect(() => {
    // Clean up existing listener
    if (listenerRef.current) {
      listenerRef.current();
      listenerRef.current = null;
    }

    // Prevent multiple listeners
    if (isListeningRef.current || !characterId) {
      return;
    }

    isListeningRef.current = true;

    const setupEventListeners = async () => {
      try {
        const unlistenFns: (() => void)[] = [];

        // Listen for walkthrough progress updates
        const unlistenProgress = await listen(
          'walkthrough-progress-updated',
          handleWalkthroughProgressUpdated
        );
        unlistenFns.push(unlistenProgress);

        // Listen for walkthrough step completed
        const unlistenStepCompleted = await listen(
          'walkthrough-step-completed',
          handleWalkthroughStepCompleted
        );
        unlistenFns.push(unlistenStepCompleted);

        // Listen for walkthrough step advanced
        const unlistenStepAdvanced = await listen(
          'walkthrough-step-advanced',
          handleWalkthroughStepAdvanced
        );
        unlistenFns.push(unlistenStepAdvanced);

        // Listen for walkthrough campaign completed
        const unlistenCampaignCompleted = await listen(
          'walkthrough-campaign-completed',
          handleWalkthroughCampaignCompleted
        );
        unlistenFns.push(unlistenCampaignCompleted);

        // Store cleanup function
        listenerRef.current = () => {
          unlistenFns.forEach(unlisten => unlisten());
        };

        setIsListening(true);
      } catch (error) {
        console.error('Failed to set up walkthrough event listeners:', error);
        isListeningRef.current = false;
      }
    };

    setupEventListeners();

    // Cleanup listeners
    return () => {
      if (listenerRef.current) {
        listenerRef.current();
        listenerRef.current = null;
      }
      isListeningRef.current = false;
      setIsListening(false);
    };
  }, [
    characterId,
    handleWalkthroughProgressUpdated,
    handleWalkthroughStepCompleted,
    handleWalkthroughStepAdvanced,
    handleWalkthroughCampaignCompleted,
  ]);

  // Reset state when character changes
  useEffect(() => {
    if (!characterId) {
      setProgress(null);
      setCurrentStep(null);
      setPreviousStep(null);
      setLastEvent(null);
    }
  }, [characterId]);

  return {
    progress,
    currentStep,
    previousStep,
    isListening,
    lastEvent,
    setProgress,
    setCurrentStep,
    setPreviousStep,
  };
}
