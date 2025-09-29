import { ChartBarIcon, MapPinIcon } from '@heroicons/react/24/outline';
import { Card, SectionHeader, StatGrid } from '../';
import { useCharacterManagement } from '../../../hooks';
import { formatDuration } from '../../../utils';

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
      <Card
        title='Character Insights'
        icon={<ChartBarIcon className='w-5 h-5' />}
        className={className}
      >
        <StatGrid stats={Array(4).fill({ value: '', label: '' })} columns={2} />
      </Card>
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
    <Card
      title='Insights'
      icon={<ChartBarIcon className='w-5 h-5' />}
      className={className}
    >
      {/* Main Stats Grid */}
      <StatGrid stats={stats} columns={2} />

      {/* Current Location */}
      {currentLocation && (
        <div className='mt-6 space-y-4'>
          <SectionHeader
            title='Current Location'
            icon={<MapPinIcon className='w-4 h-4' />}
          />
          <div className='flex items-center justify-between p-3 bg-zinc-900/80 border border-zinc-700/50'>
            <span className='text-zinc-300 font-medium'>
              {formatCurrentLocation(currentLocation)}
            </span>
          </div>
        </div>
      )}
    </Card>
  );
}
