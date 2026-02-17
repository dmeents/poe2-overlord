/* eslint-disable react-refresh/only-export-components */

import { createContext, useCallback, useContext, useEffect, useMemo, useState } from 'react';
import { useAppEventListener } from '@/hooks/useAppEventListener';
import { useWalkthroughGuide } from '@/queries/walkthrough';
import type {
  WalkthroughGuide,
  WalkthroughProgress,
  WalkthroughStepResult,
} from '@/types/walkthrough';
import {
  EVENT_KEYS,
  type ExtractPayload,
  type WalkthroughCampaignCompletedEvent,
  type WalkthroughStepAdvancedEvent,
  type WalkthroughStepCompletedEvent,
} from '@/utils/events/registry';
import { getPreviousStepId, getStepFromGuide as getStepFromGuideUtil } from '@/utils/walkthrough';
import { useCharacter } from './CharacterContext';

interface WalkthroughContextValue {
  guide: WalkthroughGuide | null;
  guideLoading: boolean;
  guideError: string | null;
  progress: WalkthroughProgress | null;
  currentStep: WalkthroughStepResult | null;
  previousStep: WalkthroughStepResult | null;
  lastEvent: string | null;
  characterId: string | null;
}

const WalkthroughContext = createContext<WalkthroughContextValue | undefined>(undefined);

export function WalkthroughProvider({ children }: React.PropsWithChildren) {
  const { activeCharacter, isLoading: characterLoading } = useCharacter();
  const characterId = activeCharacter?.id || null;

  const { data: guide, isLoading: guideLoading, error: guideError } = useWalkthroughGuide();

  const progress = activeCharacter?.walkthrough_progress || null;

  const [currentStep, setCurrentStep] = useState<WalkthroughStepResult | null>(null);

  const [previousStep, setPreviousStep] = useState<WalkthroughStepResult | null>(null);

  const [lastEvent, setLastEvent] = useState<string | null>(null);

  const getStepFromGuide = useCallback(
    (stepId: string | null): WalkthroughStepResult | null => {
      if (!guide || !stepId) return null;
      return getStepFromGuideUtil(guide, stepId);
    },
    [guide],
  );

  useEffect(() => {
    if (!progress || !guide) {
      setCurrentStep(null);
      setPreviousStep(null);
      return;
    }

    if (progress.current_step_id) {
      const currentStepData = getStepFromGuide(progress.current_step_id);
      setCurrentStep(currentStepData);

      // Use navigation helper to get previous step ID
      const prevStepId = getPreviousStepId(guide, progress.current_step_id);
      if (prevStepId) {
        const previousStepData = getStepFromGuide(prevStepId);
        setPreviousStep(previousStepData);
      } else {
        setPreviousStep(null);
      }
    } else {
      setCurrentStep(null);
      setPreviousStep(null);
    }
  }, [progress, guide, getStepFromGuide]);

  useAppEventListener(
    [
      {
        eventType: EVENT_KEYS.WalkthroughStepCompleted,
        handler: (payload: unknown) => {
          const { character_id } = payload as ExtractPayload<WalkthroughStepCompletedEvent>;

          if (character_id !== characterId) return;
          setLastEvent('step_completed');
        },
      },
      {
        eventType: EVENT_KEYS.WalkthroughStepAdvanced,
        handler: (payload: unknown) => {
          const { character_id, from_step_id } =
            payload as ExtractPayload<WalkthroughStepAdvancedEvent>;

          if (character_id !== characterId) return;

          setLastEvent('step_advanced');

          if (from_step_id) {
            const newPreviousStep = getStepFromGuide(from_step_id);
            setPreviousStep(newPreviousStep);
          }
        },
      },
      {
        eventType: EVENT_KEYS.WalkthroughCampaignCompleted,
        handler: (payload: unknown) => {
          const { character_id } = payload as ExtractPayload<WalkthroughCampaignCompletedEvent>;

          if (character_id !== characterId) return;
          setLastEvent('campaign_completed');
        },
      },
    ],
    [characterId, getStepFromGuide],
  );

  useEffect(() => {
    if (!characterId) setLastEvent(null);
  }, [characterId]);

  const contextValue = useMemo(
    () => ({
      guide: guide || null,
      guideLoading: guideLoading || characterLoading,
      guideError: guideError ? String(guideError) : null,
      progress,
      currentStep,
      previousStep,
      lastEvent,
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
      characterId,
    ],
  );

  return <WalkthroughContext.Provider value={contextValue}>{children}</WalkthroughContext.Provider>;
}

export function useWalkthrough() {
  const context = useContext(WalkthroughContext);

  if (context === undefined) {
    throw new Error('useWalkthrough must be used within WalkthroughProvider');
  }

  return context;
}
