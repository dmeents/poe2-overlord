/* eslint-disable react-refresh/only-export-components */
import type {
  WalkthroughGuide,
  WalkthroughProgress,
  WalkthroughStepResult,
} from '@/types/walkthrough';
import { useAppEventListener } from '@/hooks/useAppEventListener';
import { useCharacter } from './CharacterContext';
import {
  createContext,
  useContext,
  useState,
  useEffect,
  useCallback,
  useMemo,
} from 'react';
import { WalkthroughService } from '@/utils/walkthrough';
import {
  EVENT_KEYS,
  type ExtractPayload,
  type WalkthroughStepCompletedEvent,
  type WalkthroughStepAdvancedEvent,
  type WalkthroughCampaignCompletedEvent,
} from '@/utils/events/registry';
import { useWalkthroughGuide } from '@/queries/walkthrough';

interface WalkthroughContextValue {
  // Guide data
  guide: WalkthroughGuide | null;
  guideLoading: boolean;
  guideError: string | null;

  // Progress data (from character data)
  progress: WalkthroughProgress | null;
  currentStep: WalkthroughStepResult | null;
  previousStep: WalkthroughStepResult | null;
  lastEvent: string | null;

  // Status
  isListening: boolean;
  characterId: string | null;
}

const WalkthroughContext = createContext<WalkthroughContextValue | undefined>(
  undefined
);

export function WalkthroughProvider({ children }: React.PropsWithChildren) {
  // Get active character from CharacterContext (includes real-time updates)
  const { activeCharacter, isLoading: characterLoading } = useCharacter();

  const characterId = activeCharacter?.id || null;

  // Fetch guide using react-query
  const {
    data: guide,
    isLoading: guideLoading,
    error: guideError,
  } = useWalkthroughGuide();

  // Extract progress from character data
  const progress = activeCharacter?.walkthrough_progress || null;

  // Local state for derived data and events
  const [currentStep, setCurrentStep] = useState<WalkthroughStepResult | null>(
    null
  );
  const [previousStep, setPreviousStep] =
    useState<WalkthroughStepResult | null>(null);
  const [lastEvent, setLastEvent] = useState<string | null>(null);

  // Helper function to get step from guide
  const getStepFromGuide = useCallback(
    (stepId: string | null): WalkthroughStepResult | null => {
      if (!guide || !stepId) return null;
      return WalkthroughService.getStepFromGuide(guide, stepId);
    },
    [guide]
  );

  // Update current and previous steps when progress or guide changes
  useEffect(() => {
    if (!progress || !guide) {
      setCurrentStep(null);
      setPreviousStep(null);
      return;
    }

    // Update current step from guide
    if (progress.current_step_id) {
      const currentStepData = getStepFromGuide(progress.current_step_id);
      setCurrentStep(currentStepData);

      // Update previous step from current step's metadata
      if (currentStepData?.step.previous_step_id) {
        const previousStepData = getStepFromGuide(
          currentStepData.step.previous_step_id
        );
        setPreviousStep(previousStepData);
      } else {
        setPreviousStep(null);
      }
    } else {
      // No current step (campaign completed or not started)
      setCurrentStep(null);
      setPreviousStep(null);
    }
  }, [progress, guide, getStepFromGuide]);

  // Event listeners for walkthrough events
  const { isListening } = useAppEventListener(
    [
      {
        eventType: EVENT_KEYS.WalkthroughStepCompleted,
        handler: (payload: unknown) => {
          const { character_id } =
            payload as ExtractPayload<WalkthroughStepCompletedEvent>;

          // Only process events for the current character
          if (character_id !== characterId) return;

          setLastEvent('step_completed');
          // Don't update steps here - wait for step-advanced or progress-updated events
        },
      },
      {
        eventType: EVENT_KEYS.WalkthroughStepAdvanced,
        handler: (payload: unknown) => {
          const { character_id, from_step_id } =
            payload as ExtractPayload<WalkthroughStepAdvancedEvent>;

          // Only process events for the current character
          if (character_id !== characterId) return;

          setLastEvent('step_advanced');

          // Update previous step in local state for UI context
          if (from_step_id) {
            const newPreviousStep = getStepFromGuide(from_step_id);
            setPreviousStep(newPreviousStep);
          }
        },
      },
      {
        eventType: EVENT_KEYS.WalkthroughCampaignCompleted,
        handler: (payload: unknown) => {
          const { character_id } =
            payload as ExtractPayload<WalkthroughCampaignCompletedEvent>;

          // Only process events for the current character
          if (character_id !== characterId) return;

          setLastEvent('campaign_completed');
          // The progress will be updated via the progress updated event
        },
      },
    ],
    [characterId, getStepFromGuide]
  );

  // Reset event state when character changes
  useEffect(() => {
    if (!characterId) {
      setLastEvent(null);
    }
  }, [characterId]);

  // Memoize the context value to prevent unnecessary re-renders
  const contextValue = useMemo(
    () => ({
      guide: guide || null,
      guideLoading: guideLoading || characterLoading,
      guideError: guideError ? String(guideError) : null,
      progress,
      currentStep,
      previousStep,
      lastEvent,
      isListening,
      characterId,
    }),
    [
      guide,
      guideLoading,
      characterLoading,
      guideError,
      progress,
      currentStep,
      previousStep,
      lastEvent,
      isListening,
      characterId,
    ]
  );

  return (
    <WalkthroughContext.Provider value={contextValue}>
      {children}
    </WalkthroughContext.Provider>
  );
}

export function useWalkthrough() {
  const context = useContext(WalkthroughContext);

  if (context === undefined) {
    throw new Error('useWalkthrough must be used within WalkthroughProvider');
  }

  return context;
}
