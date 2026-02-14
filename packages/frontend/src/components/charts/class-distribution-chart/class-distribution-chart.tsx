import { UsersIcon } from '@heroicons/react/24/outline';
import { getClassHexColor } from '@/utils/class-colors';
import { formatCountTooltip } from '../chart-tooltip/chart-tooltip';
import { DistributionDonutChart } from '../distribution-donut-chart/distribution-donut-chart';

interface ClassDistributionChartProps {
  classDistribution: Record<string, number>;
  className?: string;
}

export function ClassDistributionChart({
  classDistribution,
  className = '',
}: ClassDistributionChartProps) {
  return (
    <DistributionDonutChart
      data={classDistribution}
      title="Classes"
      icon={<UsersIcon />}
      centerLabel="Classes"
      getHexColor={getClassHexColor}
      emptyStateConfig={{
        title: 'No Class Data',
        description: 'Create characters to see class distribution',
      }}
      formatTooltipContent={formatCountTooltip}
      className={className}
    />
  );
}
