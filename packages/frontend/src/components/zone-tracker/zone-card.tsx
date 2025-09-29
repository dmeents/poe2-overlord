import type { ZoneStats } from '@/types';
import { ArrowTopRightOnSquareIcon } from '@heroicons/react/24/outline';
import { open } from '@tauri-apps/plugin-shell';
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

  const formatLastVisited = (dateString: string) => {
    const date = new Date(dateString);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMinutes = Math.floor(diffMs / (1000 * 60));
    const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
    const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

    if (diffMinutes < 1) return 'Just now';
    if (diffMinutes < 60)
      return `${diffMinutes} minute${diffMinutes !== 1 ? 's' : ''} ago`;
    if (diffHours < 24)
      return `${diffHours} hour${diffHours !== 1 ? 's' : ''} ago`;
    if (diffDays === 1) return 'Yesterday';
    if (diffDays < 7) return `${diffDays} days ago`;
    if (diffDays < 30)
      return `${Math.floor(diffDays / 7)} week${Math.floor(diffDays / 7) !== 1 ? 's' : ''} ago`;
    return date.toLocaleDateString();
  };

  const getWikiUrl = (zoneName: string) => {
    // Convert zone name to capitalized snake case for wiki URL
    const capitalizedSnakeCase = zoneName
      .replace(/\s+/g, '_') // Replace spaces with underscores
      .replace(/[^a-zA-Z0-9_]/g, '') // Remove special characters except underscores
      .split('_')
      .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
      .join('_');

    return `https://www.poe2wiki.net/wiki/${capitalizedSnakeCase}`;
  };

  const handleWikiClick = async (zoneName: string) => {
    const url = getWikiUrl(zoneName);
    try {
      await open(url);
    } catch (error) {
      console.error('Failed to open wiki link:', error);
    }
  };

  return (
    <div
      className={`p-4 bg-gradient-to-br from-zinc-800/60 to-zinc-900/40 border border-zinc-700 hover:border-zinc-600 hover:from-zinc-700/60 hover:to-zinc-800/40 transition-all duration-200 ${className}`}
    >
      {/* Header */}
      <div className='flex items-center justify-between mb-3'>
        <div className='flex-1 min-w-0'>
          <div className='flex items-center space-x-2'>
            {getStatusIndicator()}
            <h3 className='text-white font-semibold text-lg truncate'>
              {zone.location_name}
            </h3>
          </div>
        </div>

        <div className='text-right'>
          <div className='text-zinc-400 font-mono text-lg'>
            <TimeDisplay seconds={zone.duration} showSeconds={false} />
          </div>
        </div>
      </div>

      {/* Compact Stats */}
      <div className='flex items-center justify-between text-sm text-zinc-400'>
        <div className='flex items-center space-x-2 flex-wrap'>
          {/* Pill-formatted metadata */}
          {zone.is_town && (
            <span className='text-xs text-yellow-400 font-medium bg-zinc-700/50 px-2 py-0.5 rounded'>
              Town
            </span>
          )}
          {zone.location_type === 'Hideout' && (
            <span className='text-xs text-green-400 font-medium bg-zinc-700/50 px-2 py-0.5 rounded'>
              Hideout
            </span>
          )}
          {!zone.is_town && zone.location_type !== 'Hideout' && (
            <span
              className={`text-xs font-medium px-2 py-0.5 rounded ${getLocationTypeColor(zone.location_type)} bg-zinc-700/50`}
            >
              {zone.location_type}
            </span>
          )}
          {zone.act && (
            <span className='text-xs text-zinc-400 font-medium bg-zinc-700/50 px-2 py-0.5 rounded'>
              {zone.act}
            </span>
          )}
          {zone.zone_level && (
            <span className='text-xs text-zinc-400 font-medium bg-zinc-700/50 px-2 py-0.5 rounded'>
              Level {zone.zone_level}
            </span>
          )}
          {/* Wiki link */}
          <button
            onClick={() => handleWikiClick(zone.location_name)}
            className='text-xs text-zinc-400 font-medium bg-zinc-700/50 px-2 py-0.5 rounded hover:bg-zinc-600/50 transition-colors duration-200 flex items-center space-x-1'
            title='Open in wiki'
          >
            <span>Wiki</span>
            <ArrowTopRightOnSquareIcon className='w-3 h-3' />
          </button>
          {/* Regular stats */}
          <span className='text-xs text-zinc-400 font-medium bg-zinc-700/50 px-2 py-0.5 rounded'>
            {zone.visits} visit{zone.visits !== 1 ? 's' : ''}
          </span>
          {/* Only show deaths for zones where characters can actually die */}
          {!zone.is_town && zone.location_type !== 'Hideout' && (
            <span
              className={`text-xs font-medium px-2 py-0.5 rounded ${
                zone.deaths > 0 ? 'text-red-400' : 'text-zinc-400'
              } bg-zinc-700/50`}
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
