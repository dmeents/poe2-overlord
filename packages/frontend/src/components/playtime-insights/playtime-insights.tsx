import {
  ChartBarIcon,
  ClockIcon,
  MapPinIcon,
} from '@heroicons/react/24/outline';
import { useCharacterManagement } from '../../hooks';
import type { ZoneStats } from '../../types';
import { playtimeInsightsStyles } from './playtime-insights.styles';

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
      <div className={`${playtimeInsightsStyles.container} ${className}`}>
        <h3 className={playtimeInsightsStyles.title}>
          <ClockIcon className='w-5 h-5 mr-2 text-zinc-400' />
          Playtime Insights
        </h3>
        <div className={playtimeInsightsStyles.grid}>
          {[...Array(6)].map((_, i) => (
            <div key={i} className={playtimeInsightsStyles.loadingContainer}>
              <div className={playtimeInsightsStyles.loadingItem}></div>
              <div className={playtimeInsightsStyles.loadingValue}></div>
            </div>
          ))}
        </div>
      </div>
    );
  }

  if (!activeCharacter || !summary) {
    return (
      <div className={`${playtimeInsightsStyles.container} ${className}`}>
        <h3 className={playtimeInsightsStyles.title}>
          <ClockIcon className='w-5 h-5 mr-2 text-zinc-400' />
          Playtime Insights
        </h3>
        <div className={playtimeInsightsStyles.emptyState}>
          <p>No character data available</p>
          <p className={playtimeInsightsStyles.emptyStateSubtext}>
            Select a character to view playtime insights
          </p>
        </div>
      </div>
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
    <div className={`${playtimeInsightsStyles.container} ${className}`}>
      <h3 className={playtimeInsightsStyles.title}>
        <ClockIcon className='w-5 h-5 mr-2 text-zinc-400' />
        Insights
      </h3>

      {/* Time Breakdown */}
      <div className={playtimeInsightsStyles.efficiencySection}>
        <h4 className={playtimeInsightsStyles.efficiencyTitle}>
          <ChartBarIcon className='w-4 h-4 mr-2 text-zinc-400' />
          Breakdown
        </h4>
        <div className={playtimeInsightsStyles.efficiencyGrid}>
          <div className={playtimeInsightsStyles.efficiencyItem}>
            <span className={playtimeInsightsStyles.efficiencyLabel}>
              Total Play Time
            </span>
            <div className='text-right'>
              <div className={playtimeInsightsStyles.efficiencyValue}>
                {formatDurationMinutes(totalPlayTime)}
              </div>
            </div>
          </div>
          <div className={playtimeInsightsStyles.efficiencyItem}>
            <span className={playtimeInsightsStyles.efficiencyLabel}>
              Active Play
            </span>
            <div className='text-right'>
              <div className={playtimeInsightsStyles.efficiencyValue}>
                {formatDurationMinutes(activePlayTime)}
              </div>
              <div className='text-xs text-zinc-500'>
                {activePlayPercentage.toFixed(1)}%
              </div>
            </div>
          </div>
          <div className={playtimeInsightsStyles.efficiencyItem}>
            <span className={playtimeInsightsStyles.efficiencyLabel}>
              Hideout Time
            </span>
            <div className='text-right'>
              <div className={playtimeInsightsStyles.efficiencyValue}>
                {formatDurationMinutes(totalHideoutTime)}
              </div>
              <div className='text-xs text-zinc-500'>
                {hideoutPercentage.toFixed(1)}%
              </div>
            </div>
          </div>
          <div className={playtimeInsightsStyles.efficiencyItem}>
            <span className={playtimeInsightsStyles.efficiencyLabel}>
              Deaths
            </span>
            <div className='text-right'>
              <div className={playtimeInsightsStyles.efficiencyValue}>
                {totalDeaths}
              </div>
              <div className='text-xs text-zinc-500'>
                {deathRate.toFixed(2)}/hr
              </div>
            </div>
          </div>
          <div className={playtimeInsightsStyles.efficiencyItem}>
            <span className={playtimeInsightsStyles.efficiencyLabel}>
              Avg per Zone
            </span>
            <div className='text-right'>
              <div className={playtimeInsightsStyles.efficiencyValue}>
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
      <div className={playtimeInsightsStyles.efficiencySection}>
        <h4 className={playtimeInsightsStyles.efficiencyTitle}>
          <MapPinIcon className='w-4 h-4 mr-2 text-zinc-400' />
          History
        </h4>
        <div className={playtimeInsightsStyles.efficiencyGrid}>
          {mostTimeSpent && (
            <div className={playtimeInsightsStyles.efficiencyItem}>
              <span className={playtimeInsightsStyles.efficiencyLabel}>
                Most Time Spent
              </span>
              <div className='text-right'>
                <div className={playtimeInsightsStyles.efficiencyValue}>
                  {mostTimeSpent.location_name}
                </div>
                <div className='text-xs text-zinc-500'>
                  {mostTimeSpentValue}
                </div>
              </div>
            </div>
          )}
          {hasDeaths && (
            <div className={playtimeInsightsStyles.efficiencyItem}>
              <span className={playtimeInsightsStyles.efficiencyLabel}>
                Most Deaths
              </span>
              <div className='text-right'>
                <div className={playtimeInsightsStyles.efficiencyValue}>
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
    </div>
  );
}
