import { useCharacterTimeTracking } from '../../hooks/useCharacterTimeTracking';
import { formatDuration } from '../../utils';
import { quickStatsGridStyles } from './quick-stats-grid.styles';

interface QuickStatsGridProps {
  className?: string;
}

export function QuickStatsGrid({ className = '' }: QuickStatsGridProps) {
  const { timeTrackingData, activeSessions, isLoading } =
    useCharacterTimeTracking();

  if (isLoading) {
    return (
      <div className={`${quickStatsGridStyles.container} ${className}`}>
        <h3 className={quickStatsGridStyles.title}>Quick Stats</h3>
        <div className={quickStatsGridStyles.grid}>
          {[...Array(4)].map((_, i) => (
            <div key={i} className={quickStatsGridStyles.loadingContainer}>
              <div className={quickStatsGridStyles.loadingItem}></div>
              <div className={quickStatsGridStyles.loadingValue}></div>
            </div>
          ))}
        </div>
      </div>
    );
  }

  const activeSessionCount = activeSessions.length;
  const todayPlayTime = timeTrackingData?.summary?.total_play_time_seconds || 0;
  const totalLocations =
    timeTrackingData?.summary?.total_locations_tracked || 0;
  const topLocation = timeTrackingData?.summary?.top_locations?.[0];

  return (
    <div className={`${quickStatsGridStyles.container} ${className}`}>
      <h3 className={quickStatsGridStyles.title}>Quick Stats</h3>

      <div className={quickStatsGridStyles.grid}>
        {/* Active Sessions */}
        <div className={quickStatsGridStyles.statItem}>
          <p className={quickStatsGridStyles.statLabel}>Active Sessions</p>
          <p className={quickStatsGridStyles.statValue}>{activeSessionCount}</p>
        </div>

        {/* Today's Play Time */}
        <div className={quickStatsGridStyles.statItem}>
          <p className={quickStatsGridStyles.statLabel}>Total Play Time</p>
          <p className={quickStatsGridStyles.statValue}>
            {formatDuration(todayPlayTime)}
          </p>
        </div>

        {/* Locations Visited */}
        <div className={quickStatsGridStyles.statItem}>
          <p className={quickStatsGridStyles.statLabel}>Locations Visited</p>
          <p className={quickStatsGridStyles.statValue}>{totalLocations}</p>
        </div>

        {/* Most Visited Location */}
        <div className={quickStatsGridStyles.statItem}>
          <p className={quickStatsGridStyles.statLabel}>Most Visited</p>
          <p
            className={quickStatsGridStyles.statValueSmall}
            title={topLocation?.location_name}
          >
            {topLocation?.location_name || 'None'}
          </p>
          {topLocation && (
            <p className={quickStatsGridStyles.statSubtext}>
              {topLocation.total_visits} visits
            </p>
          )}
        </div>
      </div>
    </div>
  );
}
