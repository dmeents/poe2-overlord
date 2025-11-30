import {
  BookOpenIcon,
  ChartBarIcon,
  ListBulletIcon,
} from '@heroicons/react/24/outline';
import React from 'react';
import type { WalkthroughGuide } from '../../../types/walkthrough';
import { DataCard } from '../../ui/data-card/data-card';
import { DataItem } from '../../ui/data-item/data-item';
import { SectionHeader } from '../../ui/section-header/section-header';

interface CampaignInsightsProps {
  guide: WalkthroughGuide;
  className?: string;
}

export const CampaignInsights: React.FC<CampaignInsightsProps> = ({
  guide,
  className = '',
}) => {
  // Calculate statistics from the guide
  const totalSteps = Object.values(guide.acts).reduce(
    (total, act) => total + Object.keys(act.steps).length,
    0
  );

  const totalObjectives = Object.values(guide.acts).reduce(
    (total, act) =>
      total +
      Object.values(act.steps).reduce(
        (stepTotal, step) => stepTotal + step.objectives.length,
        0
      ),
    0
  );

  const totalWikiLinks = Object.values(guide.acts).reduce(
    (total, act) =>
      total +
      Object.values(act.steps).reduce(
        (stepTotal, step) => stepTotal + step.wiki_items.length,
        0
      ),
    0
  );

  const totalActs = Object.keys(guide.acts).length;

  return (
    <DataCard
      title='Insights'
      icon={<ChartBarIcon className='w-5 h-5' />}
      className={className}
    >
      {/* Overview Section */}
      <SectionHeader
        title='Overview'
        icon={<BookOpenIcon className='w-4 h-4' />}
      />
      <div className='space-y-2'>
        <DataItem label='Total Acts' value={totalActs} />
        <DataItem label='Total Steps' value={totalSteps} />
      </div>

      {/* Content Section */}
      <SectionHeader
        title='Content'
        icon={<ListBulletIcon className='w-4 h-4' />}
      />
      <div className='space-y-2'>
        <DataItem label='Objectives' value={totalObjectives} />
        <DataItem label='Wiki Links' value={totalWikiLinks} />
      </div>
    </DataCard>
  );
};
