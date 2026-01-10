import type { WalkthroughAct } from '../../../types/walkthrough';
import { Accordion } from '../../ui/accordion/accordion';
import { WalkthroughStepCard } from '../walkthrough-step-card/walkthrough-step-card';

interface WalkthroughActAccordionProps {
  act: WalkthroughAct;
  actKey: string;
  isExpanded: boolean;
  currentStepId?: string;
  onToggle: (actKey: string) => void;
  onWikiClick: (itemName: string) => void;
  onSkipToStep?: (stepId: string) => void;
}

export function WalkthroughActAccordion({
  act,
  actKey,
  isExpanded,
  currentStepId,
  onToggle,
  onWikiClick,
  onSkipToStep,
}: WalkthroughActAccordionProps): React.JSX.Element {
  const isCurrentStep = (stepId: string) => currentStepId === stepId;

  return (
    <Accordion
      key={actKey}
      title={act.act_name}
      subtitle={`${Object.keys(act.steps).length} steps`}
      isExpanded={isExpanded}
      onToggle={() => onToggle(actKey)}
      className='mb-4'
    >
      <div className='space-y-3'>
        {Object.values(act.steps)
          .sort((a, b) => {
            const aStepNum = parseInt(a.id.split('_').pop() || '0');
            const bStepNum = parseInt(b.id.split('_').pop() || '0');
            return aStepNum - bStepNum;
          })
          .map(step => (
            <WalkthroughStepCard
              key={step.id}
              step={step}
              isCurrent={isCurrentStep(step.id)}
              onWikiClick={onWikiClick}
              onSkipToStep={onSkipToStep}
            />
          ))}
      </div>
    </Accordion>
  );
}
