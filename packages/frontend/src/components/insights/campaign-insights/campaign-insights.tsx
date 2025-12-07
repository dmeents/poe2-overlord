import { ChartBarIcon } from '@heroicons/react/24/outline';
import React from 'react';
import type { WalkthroughGuide } from '../../../types/walkthrough';
import { Card } from '../../ui/card/card';
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
  const totalSteps = Object.values(guide.acts).reduce(
    (total, act) => total + Object.keys(act.steps).length,
    0
  );

  const totalActs = Object.keys(guide.acts).length;

  return (
    <Card title='Insights' icon={<ChartBarIcon />} className={className}>
      <SectionHeader title='Overview' />
      <div>
        <DataItem label='Total Acts' value={totalActs} />
        <DataItem label='Total Steps' value={totalSteps} />
      </div>
    </Card>
  );
};
