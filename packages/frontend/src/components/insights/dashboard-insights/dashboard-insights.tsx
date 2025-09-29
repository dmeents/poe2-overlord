import { ChartBarIcon, MapPinIcon } from '@heroicons/react/24/outline';
import { StatGrid } from '../';
import { useCharacterManagement } from '../../../hooks';
import { formatDuration } from '../../../utils';
import { DataCard, DataItem, SectionHeader } from '../../ui';

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
      <DataCard
        title='Character Insights'
        icon={<ChartBarIcon className='w-5 h-5' />}
        isLoading={true}
        className={className}
      >
        <div></div>
      </DataCard>
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

  const stats = [
    { value: formatDuration(todayPlayTime), label: 'Play Time' },
    { value: totalLocations, label: 'Locations' },
    { value: totalDeaths, label: 'Deaths' },
    { value: activeCharacter?.level || 0, label: 'Level' },
  ];

  return (
    <DataCard
      title='Insights'
      icon={<ChartBarIcon className='w-5 h-5' />}
      className={className}
    >
      {/* Main Stats Grid */}
      <StatGrid stats={stats} columns={2} />

      {/* Current Location */}
      {currentLocation && (
        <SectionHeader
          title='Current Location'
          icon={<MapPinIcon className='w-4 h-4' />}
        >
          <DataItem label={formatCurrentLocation(currentLocation)} value='' />
        </SectionHeader>
      )}
    </DataCard>
  );
}
