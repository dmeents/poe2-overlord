import { BookOpenIcon } from '@heroicons/react/24/outline';
import { useState } from 'react';

import { useStepNavigation } from '../../../hooks/useStepNavigation';
import type { WalkthroughGuide as WalkthroughGuideType } from '../../../types/walkthrough';
import { handleWikiClick } from '../../../utils/wiki-utils';
import { SectionHeader } from '../../ui/section-header/section-header';
import { WalkthroughActAccordion } from '../walkthrough-act-accordion/walkthrough-act-accordion';

interface WalkthroughGuideProps {
  guide: WalkthroughGuideType;
  currentStepId?: string;
  characterId?: string;
  className?: string;
}

export function WalkthroughGuide({
  guide,
  currentStepId,
  characterId,
  className = '',
}: WalkthroughGuideProps): React.JSX.Element {
  const [expandedActs, setExpandedActs] = useState<Set<string>>(new Set());

  // Use shared navigation hook
  const { skipToStep } = useStepNavigation({
    characterId: characterId ?? null,
    progress: null, // Guide doesn't need current progress
  });

  const toggleAct = (actId: string) => {
    const newExpanded = new Set(expandedActs);
    if (newExpanded.has(actId)) {
      newExpanded.delete(actId);
    } else {
      newExpanded.add(actId);
    }
    setExpandedActs(newExpanded);
  };

  const handleSkipToStep = async (stepId: string) => {
    await skipToStep(stepId);
  };

  return (
    <div>
      <SectionHeader
        title="Guide"
        icon={<BookOpenIcon className="w-4 h-4" />}
        className={className}
      />
      <div className="space-y-4">
        {Object.entries(guide.acts)
          .sort(([, a], [, b]) => a.act_number - b.act_number)
          .map(([actKey, act]) => (
            <WalkthroughActAccordion
              key={actKey}
              act={act}
              actKey={actKey}
              isExpanded={expandedActs.has(actKey)}
              currentStepId={currentStepId}
              onToggle={toggleAct}
              onWikiClick={handleWikiClick}
              onSkipToStep={handleSkipToStep}
            />
          ))}
      </div>
    </div>
  );
}
