import type { ZoneStats } from '@/types/character';
import { ArrowTopRightOnSquareIcon } from '@heroicons/react/24/outline';
import { open } from '@tauri-apps/plugin-shell';
import { memo } from 'react';
import { TimeDisplay } from '../../insights/time-display/time-display';
import {
  getZoneActiveIndicatorClasses,
  getZoneActiveTextClasses,
  getZoneCardClasses,
  getZoneCardHeaderClasses,
  getZoneDurationClasses,
  getZoneMetadataContainerClasses,
  getZoneMetadataItemsClasses,
  getZonePillBaseClasses,
  getZonePillClasses,
  getZonePillColorClasses,
  getZoneStatsContainerClasses,
  getZoneStatusIndicatorClasses,
  getZoneTitleClasses,
  getZoneTitleContainerClasses,
  getZoneWikiButtonClasses,
} from './zone-card.styles';

interface ZoneCardProps {
  zone: ZoneStats;
  className?: string;
}

export const ZoneCard = memo(function ZoneCard({
  zone,
  className = '',
}: ZoneCardProps) {
  const getStatusIndicator = () => {
    if (zone.is_active) {
      return (
        <div className={getZoneStatusIndicatorClasses()}>
          <div className={getZoneActiveIndicatorClasses()}></div>
          <span className={getZoneActiveTextClasses()}>Active</span>
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
      .replace(/[^a-zA-Z0-9_'-.]/g, '') // Remove special characters except underscores, apostrophes, hyphens, and periods
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
    <div className={`${getZoneCardClasses()} ${className}`}>
      {/* Header */}
      <div className={getZoneCardHeaderClasses()}>
        <div className={getZoneTitleContainerClasses()}>
          <div className='flex items-center space-x-2'>
            {getStatusIndicator()}
            <h3 className={getZoneTitleClasses()}>{zone.zone_name}</h3>
          </div>
        </div>

        <div className={getZoneStatsContainerClasses()}>
          <div className={getZoneDurationClasses()}>
            <TimeDisplay seconds={zone.duration} showSeconds={false} />
          </div>
        </div>
      </div>

      {/* Compact Stats */}
      <div className={getZoneMetadataContainerClasses()}>
        <div className={getZoneMetadataItemsClasses()}>
          {/* Pill-formatted metadata */}
          {zone.is_town && (
            <span
              className={`${getZonePillClasses()} ${getZonePillColorClasses('Town')} ${getZonePillBaseClasses()}`}
            >
              Town
            </span>
          )}
          {zone.zone_name.toLowerCase().includes('hideout') && (
            <span
              className={`${getZonePillClasses()} ${getZonePillColorClasses('Hideout')} ${getZonePillBaseClasses()}`}
            >
              Hideout
            </span>
          )}
          {!zone.is_town &&
            !zone.zone_name.toLowerCase().includes('hideout') && (
              <span
                className={`${getZonePillClasses()} ${getZonePillColorClasses('Zone')} ${getZonePillBaseClasses()}`}
              >
                Zone
              </span>
            )}
          {zone.act && (
            <span
              className={`${getZonePillClasses()} ${getZonePillBaseClasses()}`}
            >
              Act {zone.act}
            </span>
          )}
          {zone.area_level && (
            <span
              className={`${getZonePillClasses()} ${getZonePillBaseClasses()}`}
            >
              Level {zone.area_level}
            </span>
          )}
          {/* Wiki link */}
          <button
            onClick={() => handleWikiClick(zone.zone_name)}
            className={getZoneWikiButtonClasses()}
            title='Open in wiki'
          >
            <span>Wiki</span>
            <ArrowTopRightOnSquareIcon className='w-3 h-3' />
          </button>
          {/* Regular stats */}
          <span
            className={`${getZonePillClasses()} ${getZonePillBaseClasses()}`}
          >
            {zone.visits} visit{zone.visits !== 1 ? 's' : ''}
          </span>
          {/* Only show deaths for zones where characters can actually die */}
          {!zone.is_town &&
            !zone.zone_name.toLowerCase().includes('hideout') && (
              <span
                className={`${getZonePillClasses()} ${
                  zone.deaths > 0 ? 'text-red-400' : 'text-zinc-400'
                } ${getZonePillBaseClasses()}`}
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
});
