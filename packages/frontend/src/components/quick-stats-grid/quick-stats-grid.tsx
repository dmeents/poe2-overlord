import { useCharacterManagement } from '../../hooks';
import { formatDuration } from '../../utils';
import { quickStatsGridStyles } from './quick-stats-grid.styles';

interface QuickStatsGridProps {
  className?: string;
}

export function QuickStatsGrid({ className = '' }: QuickStatsGridProps) {
  const { activeCharacter, isLoading } = useCharacterManagement();
  const trackingData = activeCharacter?.trackingData;

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

  const activeSessionCount =
    trackingData?.zones?.filter(zone => zone.is_active).length || 0;
  const todayPlayTime = trackingData?.summary?.total_play_time || 0;
  const totalLocations = trackingData?.summary?.total_zones_visited || 0;
  const topLocation = trackingData?.zones?.sort(
    (a, b) => b.duration - a.duration
  )[0];

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
              {topLocation.visits} visits
            </p>
          )}
        </div>
      </div>
    </div>
  );
}
