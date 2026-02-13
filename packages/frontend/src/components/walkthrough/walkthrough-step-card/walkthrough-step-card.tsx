import {
  ArrowLeftIcon,
  ArrowRightIcon,
  BookOpenIcon,
  ClockIcon,
  MapPinIcon,
} from '@heroicons/react/24/outline';
import { useMemo } from 'react';
import { useCharacter } from '../../../contexts/CharacterContext';
import { useWalkthrough } from '../../../contexts/WalkthroughContext';
import { useZone } from '../../../contexts/ZoneContext';
import { useStepNavigation } from '../../../hooks/useStepNavigation';
import type { WalkthroughStep, WalkthroughStepResult } from '../../../types/walkthrough';
import { ParsedText } from '../../../utils/text-parser';
import { Button } from '../../ui/button/button';
import { Card } from '../../ui/card/card';
import { StepObjectiveList } from './step-objective-list';
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

  // Navigation hook (shared logic)
  const { advanceStep, goToPreviousStep } = useStepNavigation({
    characterId: activeCharacter?.id ?? null,
    progress,
  });

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
    if (!isActiveVariant || !stepData) return;
    await advanceStep(stepData.next_step_id);
  };

  const handlePreviousStep = async () => {
    if (!isActiveVariant || !previousStep) return;
    await goToPreviousStep(previousStep.step.id);
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

  // Right action for preview cards (skip to step)
  const rightAction =
    !isActiveVariant && onSkipToStep && !isCurrent
      ? {
          label: 'Jump to Step',
          onClick: () => onSkipToStep(stepData.id),
        }
      : undefined;

  return (
    <Card
      className={`${className} ${cardBorderClass}`}
      title={stepData.title}
      icon={<MapPinIcon />}
      accentColor={isActiveStep ? 'ember' : 'stone'}
      rightAction={rightAction}>
      <div className="space-y-4 pb-4">
        {/* Zone flow: Current → Completion */}
        <div className={styles.zoneFlow}>
          <Button
            onClick={() => openZone(stepData.current_zone)}
            variant="text"
            size="xs"
            className={styles.zoneFlowCurrent}>
            {stepData.current_zone}
          </Button>
          <ArrowRightIcon className={styles.zoneFlowArrow} />
          <Button
            onClick={() => openZone(stepData.completion_zone)}
            variant="text"
            size="xs"
            className={styles.zoneFlowTarget}>
            {stepData.completion_zone}
          </Button>
        </div>

        {/* Description (provides context) */}
        <p className={styles.descriptionText}>
          <ParsedText
            text={stepData.description}
            wikiItems={filteredWikiItems}
            onWikiClick={handleItemClick}
          />
        </p>

        {/* Objectives */}
        <StepObjectiveList
          objectives={stepData.objectives}
          wikiItems={filteredWikiItems}
          onWikiClick={handleItemClick}
        />
      </div>

      {/* Footer - only for active variant */}
      {isActiveVariant && progress && (
        <div className={styles.footer}>
          <div className={styles.footerTimestamp}>
            <ClockIcon className={styles.footerTimestampIcon} />
            Last updated: {formatLastUpdated(progress.last_updated)}
          </div>
          <div className={styles.footerActions}>
            {hasPreviousStep && (
              <Button onClick={handlePreviousStep} variant="text" size="xs" title="Previous step">
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
        </div>
      )}
    </Card>
  );
}
