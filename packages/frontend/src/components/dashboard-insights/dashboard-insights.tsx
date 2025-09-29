import {
  ChartBarIcon,
  MapPinIcon,
  UserIcon,
} from '@heroicons/react/24/outline';
import { useCharacterManagement } from '../../hooks';
import { formatDuration } from '../../utils';
import { dashboardInsightsStyles } from './dashboard-insights.styles';

interface DashboardInsightsProps {
  className?: string;
}

export function DashboardInsights({ className = '' }: DashboardInsightsProps) {
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
      <div className={`${dashboardInsightsStyles.container} ${className}`}>
        <h3 className={dashboardInsightsStyles.title}>
          <ChartBarIcon className='w-5 h-5 mr-2 text-zinc-400' />
          Character Insights
        </h3>
        <div className={dashboardInsightsStyles.grid}>
          {[...Array(4)].map((_, i) => (
            <div key={i} className={dashboardInsightsStyles.loadingContainer}>
              <div className={dashboardInsightsStyles.loadingItem}></div>
              <div className={dashboardInsightsStyles.loadingValue}></div>
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
    <div className={`${dashboardInsightsStyles.container} ${className}`}>
      <h3 className={dashboardInsightsStyles.title}>
        <ChartBarIcon className='w-5 h-5 mr-2 text-zinc-400' />
        Insights
      </h3>

      {/* Current Character Status */}
      {activeCharacter && (
        <div className={dashboardInsightsStyles.featuredSection}>
          <h4 className={dashboardInsightsStyles.featuredTitle}>
            <UserIcon className='w-4 h-4 mr-2 text-zinc-400' />
            Current Character
          </h4>
          <div className={dashboardInsightsStyles.featuredValue}>
            {activeCharacter.name}
          </div>
          <div className={dashboardInsightsStyles.featuredSubtext}>
            {activeCharacter.class} • Level {activeCharacter.level} •{' '}
            {activeCharacter.league}
          </div>
        </div>
      )}

      {/* Main Stats Grid */}
      <div className={dashboardInsightsStyles.grid}>
        <div className={dashboardInsightsStyles.statItem}>
          <div className={dashboardInsightsStyles.statValue}>
            {formatDuration(todayPlayTime)}
          </div>
          <div className={dashboardInsightsStyles.statLabel}>Play Time</div>
        </div>

        <div className={dashboardInsightsStyles.statItem}>
          <div className={dashboardInsightsStyles.statValue}>
            {totalLocations}
          </div>
          <div className={dashboardInsightsStyles.statLabel}>Locations</div>
        </div>

        <div className={dashboardInsightsStyles.statItem}>
          <div className={dashboardInsightsStyles.statValue}>{totalDeaths}</div>
          <div className={dashboardInsightsStyles.statLabel}>Deaths</div>
        </div>

        <div className={dashboardInsightsStyles.statItem}>
          <div className={dashboardInsightsStyles.statValue}>
            {activeCharacter?.level || 0}
          </div>
          <div className={dashboardInsightsStyles.statLabel}>Level</div>
        </div>
      </div>

      {/* Current Location */}
      {currentLocation && (
        <div className={dashboardInsightsStyles.distributionSection}>
          <h4 className={dashboardInsightsStyles.distributionTitle}>
            <MapPinIcon className='w-4 h-4 mr-2 text-zinc-400' />
            Current Location
          </h4>
          <div className={dashboardInsightsStyles.distributionItem}>
            <span className={dashboardInsightsStyles.distributionLabel}>
              {formatCurrentLocation(currentLocation)}
            </span>
          </div>
        </div>
      )}
    </div>
  );
}
