import {
  ArrowLeftIcon,
  ArrowRightIcon,
  BookOpenIcon,
  ClockIcon,
  MapPinIcon,
} from '@heroicons/react/24/outline';
import { useMemo } from 'react';
import { useCharacter } from '../../../contexts/CharacterContext';
import { useConfiguration } from '../../../contexts/ConfigurationContext';
import { useWalkthrough } from '../../../contexts/WalkthroughContext';
import { useZone } from '../../../contexts/ZoneContext';
import { useStepNavigation } from '../../../hooks/useStepNavigation';
import type { StepLink, WalkthroughStep, WalkthroughStepResult } from '../../../types/walkthrough';
import { ParsedText } from '../../../utils/text-parser';
import { getNextStepId } from '../../../utils/walkthrough';
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
  onLinkClick: (link: StepLink) => void;
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
  onLinkClick,
  onSkipToStep,
  onViewGuide,
  className = '',
}: WalkthroughStepCardProps): React.JSX.Element | null {
  const { openZone } = useZone();
  const { config: appConfig } = useConfiguration();

  // Always call hooks unconditionally (Rules of Hooks compliance)
  const walkthroughContext = useWalkthrough();
  const characterContext = useCharacter();

  // Determine if this is active variant (uses context)
  const isActiveVariant =
    variant === 'active' || (!stepProp && !stepResult && variant !== 'preview');

  // Conditionally use context data based on variant
  const progress = isActiveVariant ? walkthroughContext.progress : null;
  const guide = isActiveVariant ? walkthroughContext.guide : null;
  const currentStep = isActiveVariant ? walkthroughContext.currentStep : null;
  const previousStep = isActiveVariant ? walkthroughContext.previousStep : null;
  const activeCharacter = isActiveVariant ? characterContext.activeCharacter : null;

  // Determine the step data to use
  const stepData = stepProp || stepResult?.step || currentStep?.step;
  const hasPreviousStep = isActiveVariant && !!previousStep;

  // Filter links: exclude zone names (those are handled separately as zone buttons)
  const filteredLinks = useMemo(() => {
    if (!stepData) return [];

    const zoneNames = [stepData.current_zone, stepData.completion_zone].filter(Boolean);

    return stepData.links.filter(link => !zoneNames.includes(link.text));
  }, [stepData]);

  // Navigation hook (shared logic)
  const { advanceStep, goToPreviousStep } = useStepNavigation({
    characterId: activeCharacter?.id ?? null,
    progress,
  });

  // Use provided onLinkClick callback for handling link clicks
  const handleLinkClick = onLinkClick;

  // Compute next step ID using guide
  const nextStepId = useMemo(() => {
    if (!isActiveVariant || !guide || !stepData) return null;
    return getNextStepId(guide, stepData.id);
  }, [isActiveVariant, guide, stepData]);

  // Navigation handlers (active variant only)
  const handleAdvanceStep = async () => {
    if (!isActiveVariant) return;
    await advanceStep(nextStepId);
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
        {!appConfig?.hide_flavor_text && (
          <p className={styles.descriptionText}>
            <ParsedText
              text={stepData.description}
              links={filteredLinks}
              onLinkClick={handleLinkClick}
            />
          </p>
        )}

        {/* Objectives */}
        <StepObjectiveList
          objectives={stepData.objectives}
          links={filteredLinks}
          onLinkClick={handleLinkClick}
          hideOptionalObjectives={appConfig?.hide_optional_objectives}
          hideLeagueStartObjectives={appConfig?.hide_league_start_objectives}
          hideObjectiveDescriptions={appConfig?.hide_objective_descriptions}
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
