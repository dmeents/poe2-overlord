import {
  ChartBarIcon,
  ClockIcon,
  MapPinIcon,
  UserIcon,
} from '@heroicons/react/24/outline';
import { useCharacterManagement } from '../../../hooks/useCharacterManagement';
import { formatDuration } from '../../../utils/format-duration';
import { DataCard } from '../../ui/data-card/data-card';
import { DataItem } from '../../ui/data-item/data-item';
import { SectionHeader } from '../../ui/section-header/section-header';

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

  return (
    <DataCard
      title='Insights'
      icon={<ChartBarIcon className='w-5 h-5' />}
      className={className}
    >
      {/* Character Stats Section */}
      <SectionHeader
        title='Character Stats'
        icon={<UserIcon className='w-4 h-4' />}
      />
      <div className='space-y-2'>
        <DataItem label='Level' value={activeCharacter?.level || 0} />
        <DataItem label='Total Deaths' value={totalDeaths} />
      </div>

      {/* Playtime Section */}
      <SectionHeader
        title='Playtime'
        icon={<ClockIcon className='w-4 h-4' />}
      />
      <div className='space-y-2'>
        <DataItem
          label='Total Play Time'
          value={formatDuration(todayPlayTime)}
        />
        <DataItem label='Locations Visited' value={totalLocations} />
      </div>

      {/* Current Location */}
      {currentLocation && (
        <>
          <SectionHeader
            title='Current Location'
            icon={<MapPinIcon className='w-4 h-4' />}
          />
          <div className='space-y-2'>
            <DataItem label={formatCurrentLocation(currentLocation)} value='' />
          </div>
        </>
      )}
    </DataCard>
  );
}
