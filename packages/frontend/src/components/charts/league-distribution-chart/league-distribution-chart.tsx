import { GlobeAltIcon } from '@heroicons/react/24/outline';
import { getLeagueHexColor } from '@/utils/league-colors';
import { formatCountTooltip } from '../chart-tooltip/chart-tooltip';
import { DistributionDonutChart } from '../distribution-donut-chart/distribution-donut-chart';

interface LeagueDistributionChartProps {
  leagueDistribution: Record<string, number>;
  className?: string;
}

export function LeagueDistributionChart({
  leagueDistribution,
  className = '',
}: LeagueDistributionChartProps) {
  return (
    <DistributionDonutChart
      data={leagueDistribution}
      title="Leagues"
      icon={<GlobeAltIcon />}
      centerLabel="Leagues"
      getHexColor={getLeagueHexColor}
      emptyStateConfig={{
        title: 'No League Data',
        description: 'Create characters to see league distribution',
      }}
      formatTooltipContent={formatCountTooltip}
      className={className}
    />
  );
}
