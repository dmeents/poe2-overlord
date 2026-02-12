import {
  ArrowLeftIcon,
  ArrowRightIcon,
  BookOpenIcon,
  CheckCircleIcon,
  ClockIcon,
  GiftIcon,
  MapPinIcon,
  StarIcon,
} from '@heroicons/react/24/outline';
import { invoke } from '@tauri-apps/api/core';
import { useMemo } from 'react';
import { useCharacter } from '../../../contexts/CharacterContext';
import { useWalkthrough } from '../../../contexts/WalkthroughContext';
import { useZone } from '../../../contexts/ZoneContext';
import type { WalkthroughStep, WalkthroughStepResult } from '../../../types/walkthrough';
import { ParsedText } from '../../../utils/text-parser';
import { Button } from '../../ui/button/button';
import { Card } from '../../ui/card/card';

interface WalkthroughStepCardProps {
  // Data sources (mutually exclusive with variant)
  step?: WalkthroughStep;
  stepResult?: WalkthroughStepResult;

  // Display mode
  variant?: 'active' | 'preview';

  // State flags
  isCurrent?: boolean;

  // Callbacks
  onWikiClick: (itemName: string) => void;
  onSkipToStep?: (stepId: string) => void;
  onViewGuide?: () => void;

  // Style
  className?: string;
}

export function WalkthroughStepCard({
  step: stepProp,
  stepResult,
  variant,
  isCurrent = false,
  onWikiClick,
  onSkipToStep,
  onViewGuide,
  className = '',
}: WalkthroughStepCardProps): React.JSX.Element | null {
  const { openZone } = useZone();

  // Always call hooks unconditionally (Rules of Hooks compliance)
  const walkthroughContext = useWalkthrough();
  const characterContext = useCharacter();

  // Determine if this is active variant (uses context)
  const isActiveVariant =
    variant === 'active' || (!stepProp && !stepResult && variant !== 'preview');

  // Conditionally use context data based on variant
  const progress = isActiveVariant ? walkthroughContext.progress : null;
  const currentStep = isActiveVariant ? walkthroughContext.currentStep : null;
  const previousStep = isActiveVariant ? walkthroughContext.previousStep : null;
  const activeCharacter = isActiveVariant ? characterContext.activeCharacter : null;

  // Determine the step data to use
  const stepData = stepProp || stepResult?.step || currentStep?.step;
  const hasPreviousStep = isActiveVariant && !!previousStep;

  // Wiki items filtering
  const filteredWikiItems = useMemo(() => {
    if (!stepData) return [];

    const zoneNames = [stepData.current_zone, stepData.completion_zone].filter(Boolean);

    return stepData.wiki_items.filter(item => !zoneNames.includes(item));
  }, [stepData]);

  // Handle item clicks (zones vs wiki)
  const handleItemClick = (itemName: string) => {
    if (!stepData) return;

    const zoneNames = [stepData.current_zone, stepData.completion_zone];

    if (zoneNames.includes(itemName)) {
      openZone(itemName);
    } else {
      onWikiClick(itemName);
    }
  };

  // Navigation handlers (active variant only)
  const handleAdvanceStep = async () => {
    if (!isActiveVariant || !stepData || !progress || !activeCharacter) return;

    try {
      const nextStepId = stepData.next_step_id;
      if (!nextStepId) {
        console.error('No next step available. Campaign may be completed.');
        return;
      }

      const newProgress = {
        ...progress,
        current_step_id: nextStepId,
        is_completed: false,
        last_updated: new Date().toISOString(),
      };

      await invoke('update_character_walkthrough_progress', {
        characterId: activeCharacter.id,
        progress: newProgress,
      });
    } catch (err) {
      console.error('Failed to advance step:', err);
    }
  };

  const handlePreviousStep = async () => {
    if (!isActiveVariant || !previousStep || !progress || !activeCharacter) return;

    try {
      const newProgress = {
        ...progress,
        current_step_id: previousStep.step.id,
        is_completed: false,
        last_updated: new Date().toISOString(),
      };

      await invoke('update_character_walkthrough_progress', {
        characterId: activeCharacter.id,
        progress: newProgress,
      });
    } catch (err) {
      console.error('Failed to go to previous step:', err);
    }
  };

  const formatLastUpdated = (timestamp: string) => {
    const date = new Date(timestamp);
    return date.toLocaleString();
  };

  // Active variant: Check for special states
  if (isActiveVariant) {
    if (!progress) return null;

    // Completion state
    if (progress.is_completed) {
      return (
        <Card className={`${className} border-green-500 bg-green-500/10`}>
          <div className="flex items-center gap-4 p-4">
            <CheckCircleIcon className="w-8 h-8 text-green-500" />
            <h4 className="text-xl font-semibold text-white">Campaign Complete!</h4>
          </div>
          <div className="flex justify-between items-center pt-3 pb-1 px-4 border-t border-zinc-700/30">
            <div className="flex items-center gap-2 text-xs text-zinc-500">
              <ClockIcon className="w-3 h-3" />
              Last updated: {formatLastUpdated(progress.last_updated)}
            </div>
            {onViewGuide && (
              <Button onClick={onViewGuide} variant="text" size="xs" title="View guide">
                <BookOpenIcon className="w-3 h-3" />
                Guide
              </Button>
            )}
          </div>
        </Card>
      );
    }

    // No active step state
    if (!stepData) {
      return (
        <Card className={`${className} border-blue-500 bg-blue-500/10`}>
          <div className="text-center py-8">
            <BookOpenIcon className="w-16 h-16 text-zinc-400 mx-auto mb-4" />
            <h4 className="text-lg font-semibold text-white mb-2">No Active Step</h4>
            <p className="text-zinc-300 mb-4">Start your walkthrough to begin tracking progress.</p>
            {onViewGuide && (
              <Button onClick={onViewGuide} variant="primary" size="md">
                <BookOpenIcon className="w-4 h-4" />
                View Walkthrough Guide
              </Button>
            )}
          </div>
        </Card>
      );
    }
  }

  // No step data in preview mode
  if (!stepData) return null;

  // Determine card styling
  const isActiveStep = isActiveVariant || isCurrent;
  const cardBorderClass = isActiveStep ? 'border-blue-500 bg-blue-500/10' : '';

  return (
    <Card
      className={`${className} ${cardBorderClass}`}
      title={stepData.title}
      subtitle={stepData.current_zone}
      icon={<MapPinIcon />}
      accentColor={isActiveStep ? 'blue' : 'zinc'}>
      <div className="p-4 space-y-4">
        {/* Completion Zone */}
        <div className="bg-blue-500/5 border border-blue-500/20 p-3">
          <div className="flex items-center gap-2">
            <MapPinIcon className="w-4 h-4 text-blue-400 flex-shrink-0" />
            <span className="text-zinc-300 font-medium text-sm">
              Enter{' '}
              <button
                type="button"
                onClick={() => openZone(stepData.completion_zone)}
                className="text-zinc-300 hover:text-zinc-200 underline decoration-blue-400 hover:decoration-blue-300 cursor-pointer font-medium">
                {stepData.completion_zone}
              </button>
            </span>
          </div>
        </div>

        {/* Description */}
        <div className="bg-zinc-800/30 border border-zinc-700/20 p-3">
          <p className="text-sm text-zinc-300">
            <ParsedText
              text={stepData.description}
              wikiItems={filteredWikiItems}
              onWikiClick={handleItemClick}
            />
          </p>
        </div>

        {/* Objectives */}
        {stepData.objectives.length > 0 && (
          <div className="bg-zinc-800/40 p-4 border border-zinc-700/30">
            <h5 className="text-sm font-medium text-zinc-200 mb-3">
              Objectives ({stepData.objectives.length}):
            </h5>
            <ul className="space-y-4">
              {stepData.objectives.map((objective, objectiveIndex) => (
                <li
                  key={`objective-${objectiveIndex}-${objective.text.slice(0, 20)}`}
                  className="text-xs">
                  <div className="flex items-start gap-2">
                    <div className="w-1.5 h-1.5 rounded-full bg-zinc-400 mt-1.5 flex-shrink-0" />
                    <div className="flex-1 space-y-1">
                      <div className="font-medium text-zinc-200 flex items-center gap-2">
                        {objective.required !== undefined && (
                          <StarIcon
                            className={`w-3 h-3 ${
                              objective.required ? 'text-orange-400' : 'text-zinc-400'
                            }`}
                            title={objective.required ? 'Required' : 'Optional'}
                          />
                        )}
                        <ParsedText
                          text={objective.text}
                          wikiItems={filteredWikiItems}
                          onWikiClick={handleItemClick}
                        />
                      </div>
                      {(objective.details ||
                        objective.notes ||
                        (objective.rewards && objective.rewards.length > 0)) && (
                        <div className="border-l-2 border-zinc-500 pl-2 ml-1.5 space-y-1">
                          {objective.details && (
                            <div className="text-xs text-zinc-400">
                              <ParsedText
                                text={objective.details}
                                wikiItems={filteredWikiItems}
                                onWikiClick={handleItemClick}
                              />
                            </div>
                          )}
                          {objective.notes && (
                            <div className="text-xs text-blue-400 italic">
                              Note:{' '}
                              <ParsedText
                                text={objective.notes}
                                wikiItems={filteredWikiItems}
                                onWikiClick={handleItemClick}
                              />
                            </div>
                          )}
                          {objective.rewards && objective.rewards.length > 0 && (
                            <div className="text-xs flex items-center gap-1">
                              <GiftIcon className="w-3 h-3 text-purple-400" title="Rewards" />
                              <ParsedText
                                text={objective.rewards.join(', ')}
                                wikiItems={filteredWikiItems}
                                onWikiClick={handleItemClick}
                              />
                            </div>
                          )}
                        </div>
                      )}
                    </div>
                  </div>
                </li>
              ))}
            </ul>
          </div>
        )}
      </div>

      {/* Footer */}
      {(isActiveVariant || (onSkipToStep && !isCurrent)) && (
        <div className="flex justify-between items-center py-2 px-4 border-t border-zinc-700/30">
          {isActiveVariant && progress ? (
            <>
              <div className="flex items-center gap-2 text-xs text-zinc-500">
                <ClockIcon className="w-3 h-3" />
                Last updated: {formatLastUpdated(progress.last_updated)}
              </div>
              <div className="flex gap-2">
                {hasPreviousStep && (
                  <Button
                    onClick={handlePreviousStep}
                    variant="text"
                    size="xs"
                    title="Previous step">
                    <ArrowLeftIcon className="mr-1 w-3 h-3" />
                    Previous
                  </Button>
                )}
                {!progress.is_completed && (
                  <Button onClick={handleAdvanceStep} variant="text" size="xs" title="Next step">
                    Next
                    <ArrowRightIcon className="ml-1 w-3 h-3" />
                  </Button>
                )}
                {onViewGuide && (
                  <Button
                    onClick={onViewGuide}
                    variant="text"
                    size="xs"
                    className="gap-1"
                    title="View guide">
                    <BookOpenIcon className="w-3 h-3" />
                    Guide
                  </Button>
                )}
              </div>
            </>
          ) : (
            <div className="flex justify-end w-full">
              <Button
                onClick={() => onSkipToStep?.(stepData.id)}
                variant="text"
                size="xs"
                className="gap-1"
                title="Go to this step">
                <ArrowRightIcon className="w-3 h-3" />
                Go Here
              </Button>
            </div>
          )}
        </div>
      )}
    </Card>
  );
}
