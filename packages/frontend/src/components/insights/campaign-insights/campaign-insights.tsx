import { ChartBarIcon } from '@heroicons/react/24/outline';

import type { WalkthroughGuide } from '../../../types/walkthrough';
import { Card } from '../../ui/card/card';
import { DataItem } from '../../ui/data-item/data-item';

interface CampaignInsightsProps {
  guide: WalkthroughGuide;
  className?: string;
}

export function CampaignInsights({
  guide,
  className = '',
}: CampaignInsightsProps): React.JSX.Element {
  const totalSteps = guide.acts.reduce((total, act) => total + act.steps.length, 0);

  const totalActs = guide.acts.length;

  return (
    <Card title="Insights" icon={<ChartBarIcon />} className={className}>
      <div>
        <DataItem label="Total Acts" value={totalActs} />
        <DataItem label="Total Steps" value={totalSteps} />
      </div>
    </Card>
  );
}
