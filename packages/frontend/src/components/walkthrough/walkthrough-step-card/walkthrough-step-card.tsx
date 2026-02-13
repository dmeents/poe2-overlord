import {
  ArrowLeftIcon,
  ArrowRightIcon,
  BookOpenIcon,
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
import { walkthroughStepCardStyles as styles } from './walkthrough-step-card.styles';

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

    // No active step state
    if (!stepData) {
      return (
        <Card className={`${className} ${styles.noStepCard}`}>
          <div className={styles.noStepContent}>
            <BookOpenIcon className={styles.noStepIcon} />
            <h4 className={styles.noStepTitle}>No Active Step</h4>
            <p className={styles.noStepText}>Start your walkthrough to begin tracking progress.</p>
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
  const cardBorderClass = isActiveStep ? styles.activeCard : '';

  return (
    <Card
      className={`${className} ${cardBorderClass}`}
      title={stepData.title}
      subtitle={stepData.current_zone}
      icon={<MapPinIcon />}
      accentColor={isActiveStep ? 'arcane' : 'stone'}>
      <div className="p-4 space-y-4">
        {/* Completion Zone */}
        <div className={styles.completionZoneContainer}>
          <div className={styles.completionZoneContent}>
            <MapPinIcon className={styles.completionZoneIcon} />
            <span className={styles.completionZoneLabel}>
              Enter{' '}
              <button
                type="button"
                onClick={() => openZone(stepData.completion_zone)}
                className={styles.completionZoneLink}>
                {stepData.completion_zone}
              </button>
            </span>
          </div>
        </div>

        {/* Description */}
        <div className={styles.descriptionContainer}>
          <p className={styles.descriptionText}>
            <ParsedText
              text={stepData.description}
              wikiItems={filteredWikiItems}
              onWikiClick={handleItemClick}
            />
          </p>
        </div>

        {/* Objectives */}
        {stepData.objectives.length > 0 && (
          <div className={styles.objectivesContainer}>
            <h5 className={styles.objectivesTitle}>Objectives ({stepData.objectives.length}):</h5>
            <ul className={styles.objectivesList}>
              {stepData.objectives.map((objective, objectiveIndex) => (
                <li
                  key={`objective-${objectiveIndex}-${objective.text.slice(0, 20)}`}
                  className={styles.objectiveItem}>
                  <div className={styles.objectiveContent}>
                    <div className={styles.objectiveBullet} />
                    <div className={styles.objectiveInner}>
                      <div className={styles.objectiveText}>
                        {objective.required !== undefined && (
                          <StarIcon
                            className={
                              objective.required
                                ? styles.objectiveRequired
                                : styles.objectiveOptional
                            }
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
                        <div className={styles.objectiveDetails}>
                          {objective.details && (
                            <div className={styles.objectiveDetailsText}>
                              <ParsedText
                                text={objective.details}
                                wikiItems={filteredWikiItems}
                                onWikiClick={handleItemClick}
                              />
                            </div>
                          )}
                          {objective.notes && (
                            <div className={styles.objectiveNotesText}>
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
                              <GiftIcon className={styles.rewardIcon} title="Rewards" />
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
        <div className={styles.footer}>
          {isActiveVariant && progress ? (
            <>
              <div className={styles.footerTimestamp}>
                <ClockIcon className={styles.footerTimestampIcon} />
                Last updated: {formatLastUpdated(progress.last_updated)}
              </div>
              <div className={styles.footerActions}>
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
            <div className={styles.footerActionsEnd}>
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
