import {
  ChartBarIcon,
  ClockIcon,
  MapPinIcon,
} from '@heroicons/react/24/outline';
import { Card, SectionHeader } from '../';
import { useCharacterManagement } from '../../../hooks';
import type { ZoneStats } from '../../../types';

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
      <Card
        title='Playtime Insights'
        icon={<ClockIcon className='w-5 h-5' />}
        className={className}
      >
        <div className='grid grid-cols-2 gap-4'>
          {[...Array(6)].map((_, i) => (
            <div key={i} className='animate-pulse'>
              <div className='h-4 bg-zinc-700 rounded mb-2'></div>
              <div className='h-6 bg-zinc-700 rounded'></div>
            </div>
          ))}
        </div>
      </Card>
    );
  }

  if (!activeCharacter || !summary) {
    return (
      <Card
        title='Playtime Insights'
        icon={<ClockIcon className='w-5 h-5' />}
        className={className}
      >
        <div className='text-center py-8'>
          <p>No character data available</p>
          <p className='text-sm text-zinc-500 mt-2'>
            Select a character to view playtime insights
          </p>
        </div>
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
    zone => !zone.is_town && zone.location_type !== 'Hideout'
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
    <Card
      title='Insights'
      icon={<ClockIcon className='w-5 h-5' />}
      className={className}
    >
      {/* Time Breakdown */}
      <div className='mt-6 space-y-4'>
        <SectionHeader
          title='Breakdown'
          icon={<ChartBarIcon className='w-4 h-4' />}
        />
        <div className='space-y-2'>
          <div className='flex items-center justify-between p-3 bg-zinc-900/80 border border-zinc-700/50'>
            <span className='text-zinc-300 font-medium'>Total Play Time</span>
            <div className='text-right'>
              <div className='text-zinc-400 text-sm'>
                {formatDurationMinutes(totalPlayTime)}
              </div>
            </div>
          </div>
          <div className='flex items-center justify-between p-3 bg-zinc-900/80 border border-zinc-700/50'>
            <span className='text-zinc-300 font-medium'>Active Play</span>
            <div className='text-right'>
              <div className='text-zinc-400 text-sm'>
                {formatDurationMinutes(activePlayTime)}
              </div>
              <div className='text-xs text-zinc-500'>
                {activePlayPercentage.toFixed(1)}%
              </div>
            </div>
          </div>
          <div className='flex items-center justify-between p-3 bg-zinc-900/80 border border-zinc-700/50'>
            <span className='text-zinc-300 font-medium'>Hideout Time</span>
            <div className='text-right'>
              <div className='text-zinc-400 text-sm'>
                {formatDurationMinutes(totalHideoutTime)}
              </div>
              <div className='text-xs text-zinc-500'>
                {hideoutPercentage.toFixed(1)}%
              </div>
            </div>
          </div>
          <div className='flex items-center justify-between p-3 bg-zinc-900/80 border border-zinc-700/50'>
            <span className='text-zinc-300 font-medium'>Deaths</span>
            <div className='text-right'>
              <div className='text-zinc-400 text-sm'>{totalDeaths}</div>
              <div className='text-xs text-zinc-500'>
                {deathRate.toFixed(2)}/hr
              </div>
            </div>
          </div>
          <div className='flex items-center justify-between p-3 bg-zinc-900/80 border border-zinc-700/50'>
            <span className='text-zinc-300 font-medium'>Avg per Zone</span>
            <div className='text-right'>
              <div className='text-zinc-400 text-sm'>
                {formatDurationMinutes(Math.round(averageTimePerZone))}
              </div>
              <div className='text-xs text-zinc-500'>
                {nonTownHideoutZones.length} zones
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* History Section */}
      <div className='mt-6 space-y-4'>
        <SectionHeader
          title='History'
          icon={<MapPinIcon className='w-4 h-4' />}
        />
        <div className='space-y-2'>
          {mostTimeSpent && (
            <div className='flex items-center justify-between p-3 bg-zinc-900/80 border border-zinc-700/50'>
              <span className='text-zinc-300 font-medium'>Most Time Spent</span>
              <div className='text-right'>
                <div className='text-zinc-400 text-sm'>
                  {mostTimeSpent.location_name}
                </div>
                <div className='text-xs text-zinc-500'>
                  {mostTimeSpentValue}
                </div>
              </div>
            </div>
          )}
          {hasDeaths && (
            <div className='flex items-center justify-between p-3 bg-zinc-900/80 border border-zinc-700/50'>
              <span className='text-zinc-300 font-medium'>Most Deaths</span>
              <div className='text-right'>
                <div className='text-zinc-400 text-sm'>
                  {mostDeaths.location_name}
                </div>
                <div className='text-xs text-zinc-500'>
                  {mostDeaths.deaths} deaths
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </Card>
  );
}
