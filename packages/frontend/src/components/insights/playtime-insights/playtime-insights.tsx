import { ClockIcon } from '@heroicons/react/24/outline';
import { useCharacter } from '../../../contexts/CharacterContext';
import type { ZoneStats } from '../../../types/character';
import { Card } from '../../ui/card/card';
import { EmptyState } from '../../ui/empty-state/empty-state';
import { DataItem } from '../../ui/data-item/data-item';
import { LoadingSpinner } from '../../ui/loading-spinner/loading-spinner';
import { SectionHeader } from '../../ui/section-header/section-header';

// Format duration without seconds, rounding to nearest minute
function formatDurationMinutes(seconds: number): string {
  if (seconds === 0) return '0m';

  const hours = Math.floor(seconds / 3600);
  const minutes = Math.round(seconds / 60);

  if (hours > 0) {
    const remainingMinutes = minutes % 60;
    return remainingMinutes > 0
      ? `${hours}h ${remainingMinutes}m`
      : `${hours}h`;
  }

  return `${minutes}m`;
}

interface PlaytimeInsightsProps {
  zones?: ZoneStats[];
}

export function PlaytimeInsights({ zones: propZones }: PlaytimeInsightsProps) {
  const { activeCharacter, isLoading } = useCharacter();
  const summary = activeCharacter?.summary;
  const zones = propZones || activeCharacter?.zones || [];

  if (isLoading) {
    return (
      <Card
        title='Playtime Insights'
        icon={<ClockIcon />}
        className={className}
      >
        <LoadingSpinner message='Loading playtime insights...' />
      </Card>
    );
  }

  if (!activeCharacter || !summary) {
    return (
      <Card
        title='Playtime Insights'
        icon={<ClockIcon />}
        className={className}
      >
        <EmptyState
          icon={<ClockIcon className='w-8 h-8' />}
          title='No Character Data Available'
          description='Select a character to view playtime insights'
        />
      </Card>
    );
  }

  // Calculate additional insights
  const totalPlayTime = summary.total_play_time || 0;
  const totalHideoutTime = summary.total_hideout_time || 0;
  const activePlayTime = totalPlayTime - totalHideoutTime;
  const totalDeaths = summary.total_deaths || 0;

  // Filter out towns and hideouts for average time per zone calculation
  const nonTownHideoutZones = zones.filter(
    zone => !zone.is_town && !zone.zone_name.includes('Hideout')
  );
  const averageTimePerZone =
    nonTownHideoutZones.length > 0
      ? nonTownHideoutZones.reduce((sum, zone) => sum + zone.duration, 0) /
        nonTownHideoutZones.length
      : 0;
  const deathRate =
    totalPlayTime > 0 ? totalDeaths / (totalPlayTime / 3600) : 0; // deaths per hour

  // Find most time spent location
  const mostTimeSpent = zones.length > 0 ? zones[0] : null;
  const mostTimeSpentValue = mostTimeSpent
    ? formatDurationMinutes(mostTimeSpent.duration)
    : 'N/A';

  // Find location with most deaths
  const mostDeaths = zones.reduce(
    (max, zone) => (zone.deaths > max.deaths ? zone : max),
    zones[0] || { deaths: 0, location_name: 'N/A' }
  );
  const hasDeaths = mostDeaths.deaths > 0;

  // Calculate efficiency metrics
  const hideoutPercentage =
    totalPlayTime > 0 ? (totalHideoutTime / totalPlayTime) * 100 : 0;
  const activePlayPercentage = 100 - hideoutPercentage;

  return (
    <Card title='Insights' icon={<ClockIcon />} className='py-0'>
      <DataItem
        label='Total Play Time'
        value={formatDurationMinutes(totalPlayTime)}
      />
      <DataItem
        label='Active Play'
        value={formatDurationMinutes(activePlayTime)}
        subValue={`${activePlayPercentage.toFixed(1)}%`}
      />
      <DataItem
        label='Hideout Time'
        value={formatDurationMinutes(totalHideoutTime)}
        subValue={`${hideoutPercentage.toFixed(1)}%`}
      />
      <DataItem
        label='Deaths'
        value={totalDeaths}
        subValue={`${deathRate.toFixed(2)}/hr`}
      />
      <DataItem
        label='Avg Time per Zone'
        value={formatDurationMinutes(Math.round(averageTimePerZone))}
        subValue={`${nonTownHideoutZones.length} zones`}
      />
      {mostTimeSpent && (
        <DataItem
          label='Most Time Spent'
          value={mostTimeSpent.zone_name}
          subValue={mostTimeSpentValue}
        />
      )}
      {hasDeaths && (
        <DataItem
          label='Most Deaths'
          value={mostDeaths.zone_name}
          subValue={`${mostDeaths.deaths} deaths`}
        />
      )}
    </Card>
  );
}
