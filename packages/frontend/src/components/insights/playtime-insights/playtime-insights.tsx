import {
  ChartBarIcon,
  ClockIcon,
  MapPinIcon,
} from '@heroicons/react/24/outline';
import { useCharacterManagement } from '../../../hooks/useCharacterManagement';
import type { ZoneStats } from '../../../types/character';
import { DataCard } from '../../ui/data-card/data-card';
import { DataItem } from '../../ui/data-item/data-item';
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
  className?: string;
  zones?: ZoneStats[];
}

export function PlaytimeInsights({
  className = '',
  zones: propZones,
}: PlaytimeInsightsProps) {
  const { activeCharacter, isLoading } = useCharacterManagement();
  const summary = activeCharacter?.summary;
  const zones = propZones || activeCharacter?.zones || [];

  if (isLoading) {
    return (
      <DataCard
        title='Playtime Insights'
        icon={<ClockIcon className='w-5 h-5' />}
        isLoading={true}
        className={className}
      >
        <div></div>
      </DataCard>
    );
  }

  if (!activeCharacter || !summary) {
    return (
      <DataCard
        title='Playtime Insights'
        icon={<ClockIcon className='w-5 h-5' />}
        isEmpty={true}
        emptyTitle='No Character Data Available'
        emptyDescription='Select a character to view playtime insights'
        className={className}
      >
        <div></div>
      </DataCard>
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
    <DataCard
      title='Insights'
      icon={<ClockIcon className='w-5 h-5' />}
      className={className}
    >
      {/* Time Breakdown */}
      <SectionHeader
        title='Breakdown'
        icon={<ChartBarIcon className='w-4 h-4' />}
      />
      <div className='space-y-2'>
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
          label='Avg per Zone'
          value={formatDurationMinutes(Math.round(averageTimePerZone))}
          subValue={`${nonTownHideoutZones.length} zones`}
        />
      </div>

      {/* History Section */}
      <SectionHeader
        title='History'
        icon={<MapPinIcon className='w-4 h-4' />}
      />
      <div className='space-y-2'>
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
      </div>
    </DataCard>
  );
}
