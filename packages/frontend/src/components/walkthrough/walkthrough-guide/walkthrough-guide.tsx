import { BookOpenIcon } from '@heroicons/react/24/outline';
import { invoke } from '@tauri-apps/api/core';
import { useState } from 'react';

import type {
  WalkthroughGuide as WalkthroughGuideType,
  WalkthroughProgress,
} from '../../../types/walkthrough';
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
    if (!characterId) return;

    try {
      // Create new progress with the selected step
      const newProgress: WalkthroughProgress = {
        current_step_id: stepId,
        is_completed: false,
        last_updated: new Date().toISOString(),
      };

      await invoke('update_character_walkthrough_progress', {
        characterId,
        progress: newProgress,
      });
      // Events will handle the UI update
    } catch (err) {
      console.error('Failed to skip to step:', err);
    }
  };

  return (
    <div>
      <SectionHeader
        title='Guide'
        icon={<BookOpenIcon className='w-4 h-4' />}
        className={className}
      />
      <div className='space-y-4'>
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
