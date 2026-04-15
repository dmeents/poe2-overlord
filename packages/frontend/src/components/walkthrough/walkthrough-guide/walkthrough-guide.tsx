import { BookOpenIcon } from '@heroicons/react/24/outline';
import { open } from '@tauri-apps/plugin-shell';
import { useState } from 'react';

import { useStepNavigation } from '../../../hooks/useStepNavigation';
import type {
  StepLink,
  WalkthroughGuide as WalkthroughGuideType,
} from '../../../types/walkthrough';
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
  const [expandedActs, setExpandedActs] = useState<Set<number>>(new Set());

  // Use shared navigation hook
  const { skipToStep } = useStepNavigation({
    characterId: characterId ?? null,
    progress: null, // Guide doesn't need current progress
  });

  const toggleAct = (actIndex: number) => {
    const newExpanded = new Set(expandedActs);
    if (newExpanded.has(actIndex)) {
      newExpanded.delete(actIndex);
    } else {
      newExpanded.add(actIndex);
    }
    setExpandedActs(newExpanded);
  };

  const handleLinkClick = async (link: StepLink) => {
    try {
      await open(link.url);
    } catch (error) {
      console.error('Failed to open link:', error);
    }
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
        {guide.acts.map((act, actIndex) => (
          <WalkthroughActAccordion
            // biome-ignore lint/suspicious/noArrayIndexKey: acts have no natural unique key
            key={`act-${actIndex}`}
            act={act}
            actIndex={actIndex}
            isExpanded={expandedActs.has(actIndex)}
            currentStepId={currentStepId}
            onToggle={toggleAct}
            onLinkClick={handleLinkClick}
            onSkipToStep={handleSkipToStep}
          />
        ))}
      </div>
    </div>
  );
}
