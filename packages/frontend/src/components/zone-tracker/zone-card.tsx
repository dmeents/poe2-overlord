import type { ZoneStats } from '@/types';
import {
  BuildingOfficeIcon,
  HomeIcon,
  MapPinIcon,
} from '@heroicons/react/24/outline';
import { TimeDisplay } from '../time-display';

interface ZoneCardProps {
  zone: ZoneStats;
  className?: string;
}

export function ZoneCard({ zone, className = '' }: ZoneCardProps) {
  const getLocationTypeColor = (type: string) => {
    switch (type) {
      case 'Zone':
        return 'text-blue-400';
      case 'Act':
        return 'text-purple-400';
      case 'Hideout':
        return 'text-green-400';
      default:
        return 'text-zinc-400';
    }
  };

  const getZoneIcon = () => {
    return <MapPinIcon className='w-3 h-3 text-blue-400' />;
  };

  const getStatusIndicator = () => {
    if (zone.is_active) {
      return (
        <div className='flex items-center space-x-1'>
          <div className='w-2 h-2 bg-emerald-400 rounded-full animate-pulse'></div>
          <span className='text-xs text-emerald-400 font-medium'>Active</span>
        </div>
      );
    }
    return null;
  };

  const getTownIndicator = () => {
    if (zone.is_town) {
      return (
        <div className='flex items-center space-x-1'>
          <HomeIcon className='w-3 h-3 text-yellow-400' />
          <span className='text-xs text-yellow-400'>Town</span>
        </div>
      );
    }
    return null;
  };

  const getHideoutIndicator = () => {
    if (zone.location_type === 'Hideout') {
      return (
        <div className='flex items-center space-x-1'>
          <BuildingOfficeIcon className='w-3 h-3 text-green-400' />
          <span className='text-xs text-green-400'>Hideout</span>
        </div>
      );
    }
    return null;
  };

  const getVisitFrequency = () => {
    if (zone.visits === 0) return 'text-zinc-500';
    return 'text-zinc-400';
  };

  const formatLastVisited = (dateString: string) => {
    const date = new Date(dateString);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

    if (diffDays === 0) return 'Today';
    if (diffDays === 1) return 'Yesterday';
    if (diffDays < 7) return `${diffDays} days ago`;
    if (diffDays < 30) return `${Math.floor(diffDays / 7)} weeks ago`;
    return date.toLocaleDateString();
  };

  return (
    <div
      className={`p-4 bg-gradient-to-br from-zinc-800/60 to-zinc-900/40 border border-zinc-700 hover:border-zinc-600 hover:from-zinc-700/60 hover:to-zinc-800/40 transition-all duration-200 ${className}`}
    >
      {/* Header */}
      <div className='flex items-start justify-between mb-3'>
        <div className='flex-1 min-w-0'>
          <div className='flex items-center space-x-2 mb-1'>
            {/* Only show location type if it's not a town or hideout */}
            {!zone.is_town && zone.location_type !== 'Hideout' && (
              <div className='flex items-center space-x-1'>
                {zone.location_type === 'Zone' && getZoneIcon()}
                <span
                  className={`text-sm font-medium ${getLocationTypeColor(zone.location_type)}`}
                >
                  {zone.location_type}
                </span>
              </div>
            )}
            {getStatusIndicator()}
            {getTownIndicator()}
            {getHideoutIndicator()}
            {zone.act && (
              <span className='text-xs text-zinc-400 font-medium'>
                {zone.act}
              </span>
            )}
          </div>
          <h3 className='text-white font-semibold text-lg truncate'>
            {zone.location_name}
          </h3>
          {zone.zone_level && (
            <p className='text-xs text-zinc-500'>Level {zone.zone_level}</p>
          )}
        </div>

        <div className='text-right'>
          <div className='text-zinc-400 font-mono text-lg'>
            <TimeDisplay seconds={zone.duration} showSeconds={false} />
          </div>
        </div>
      </div>

      {/* Compact Stats */}
      <div className='flex items-center justify-between text-sm text-zinc-400'>
        <div className='flex items-center space-x-4'>
          <span>
            {zone.visits} visit{zone.visits !== 1 ? 's' : ''}
          </span>
          {/* Only show deaths for zones where characters can actually die */}
          {!zone.is_town && zone.location_type !== 'Hideout' && (
            <span
              className={zone.deaths > 0 ? 'text-red-400' : 'text-zinc-400'}
            >
              {zone.deaths} death{zone.deaths !== 1 ? 's' : ''}
            </span>
          )}
        </div>
        <span>
          {zone.last_visited
            ? formatLastVisited(zone.last_visited)
            : 'Never visited'}
        </span>
      </div>
    </div>
  );
}
