import { useCharacterManagement } from '../../hooks';
import { formatDuration } from '../../utils';
import { quickStatsGridStyles } from './quick-stats-grid.styles';

interface QuickStatsGridProps {
  className?: string;
}

export function QuickStatsGrid({ className = '' }: QuickStatsGridProps) {
  const { activeCharacter, isLoading } = useCharacterManagement();
  // Extract tracking data from the unified character data
  const trackingData = activeCharacter
    ? {
        character_id: activeCharacter.id,
        current_location: activeCharacter.current_location,
        summary: activeCharacter.summary,
        zones: activeCharacter.zones,
        last_updated: activeCharacter.last_updated,
      }
    : null;

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

  const todayPlayTime = trackingData?.summary?.total_play_time || 0;
  const totalLocations = trackingData?.summary?.total_zones_visited || 0;
  const totalDeaths = trackingData?.summary?.total_deaths || 0;
  const currentLocation = trackingData?.current_location;

  // Helper function to format current location
  const formatCurrentLocation = (location: typeof currentLocation) => {
    if (!location) return 'Unknown';

    const parts = [];
    if (location.act) parts.push(location.act);
    if (location.scene) parts.push(location.scene);

    return parts.length > 0 ? parts.join(' - ') : 'Unknown';
  };

  return (
    <div className={`${quickStatsGridStyles.container} ${className}`}>
      <h3 className={quickStatsGridStyles.title}>Quick Stats</h3>

      <div className={quickStatsGridStyles.grid}>
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

        {/* Death Count */}
        <div className={quickStatsGridStyles.statItem}>
          <p className={quickStatsGridStyles.statLabel}>Deaths</p>
          <p className={quickStatsGridStyles.statValue}>{totalDeaths}</p>
        </div>

        {/* Current Zone */}
        <div className={quickStatsGridStyles.statItem}>
          <p className={quickStatsGridStyles.statLabel}>Current Zone</p>
          <p
            className={quickStatsGridStyles.statValueZone}
            title={formatCurrentLocation(currentLocation)}
          >
            {formatCurrentLocation(currentLocation)}
          </p>
        </div>
      </div>
    </div>
  );
}
