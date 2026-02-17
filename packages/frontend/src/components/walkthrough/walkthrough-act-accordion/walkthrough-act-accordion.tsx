import type { StepLink, WalkthroughAct } from '../../../types/walkthrough';
import { Accordion } from '../../ui/accordion/accordion';
import { WalkthroughStepCard } from '../walkthrough-step-card/walkthrough-step-card';

interface WalkthroughActAccordionProps {
  act: WalkthroughAct;
  actIndex: number;
  isExpanded: boolean;
  currentStepId?: string;
  onToggle: (actIndex: number) => void;
  onLinkClick: (link: StepLink) => void;
  onSkipToStep?: (stepId: string) => void;
}

export function WalkthroughActAccordion({
  act,
  actIndex,
  isExpanded,
  currentStepId,
  onToggle,
  onLinkClick,
  onSkipToStep,
}: WalkthroughActAccordionProps): React.JSX.Element {
  const isCurrentStep = (stepId: string) => currentStepId === stepId;

  return (
    <Accordion
      key={`act-${actIndex}`}
      title={act.act_name}
      subtitle={`${act.steps.length} steps`}
      isExpanded={isExpanded}
      onToggle={() => onToggle(actIndex)}
      className="mb-4">
      <div className="space-y-3">
        {act.steps.map(step => (
          <WalkthroughStepCard
            key={step.id}
            step={step}
            isCurrent={isCurrentStep(step.id)}
            onLinkClick={onLinkClick}
            onSkipToStep={onSkipToStep}
          />
        ))}
      </div>
    </Accordion>
  );
}
