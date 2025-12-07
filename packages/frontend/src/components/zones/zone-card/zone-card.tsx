import type { ZoneStats } from '@/types/character';
import { MapIcon } from '@heroicons/react/24/outline';
import { memo, useState } from 'react';
import { TimeDisplay } from '../../insights/time-display/time-display';
import { ZoneDetailsModal } from '../zone-details-modal/zone-details-modal';
import { getZonePillBaseClasses, getZonePillClasses } from './zone-card.styles';
import { formatTimeAgo } from '@/utils/format-time-ago';

interface ZoneCardProps {
  zone: ZoneStats;
  allZones: ZoneStats[];
  className?: string;
}

export const ZoneCard = memo(function ZoneCard({
  zone,
  allZones,
  className = '',
}: ZoneCardProps) {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [selectedZone, setSelectedZone] = useState<ZoneStats | null>(null);

  const getActiveBorderClass = () => {
    return zone.is_active ? 'border-emerald-500' : 'border-zinc-700';
  };

  const handleCardClick = () => {
    console.log('Card clicked, opening modal for:', zone.zone_name);
    setSelectedZone(zone);
    setIsModalOpen(true);
  };

  const handleZoneChange = (newZone: ZoneStats | null) => {
    console.log('Zone change requested:', newZone?.zone_name);
    setSelectedZone(newZone);
  };

  return (
    <>
      <ZoneDetailsModal
        key={selectedZone?.zone_name || 'no-zone'}
        zone={selectedZone}
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        allZones={allZones}
        onZoneChange={handleZoneChange}
      />
      <div
        onClick={handleCardClick}
        className={`bg-zinc-900/80 border overflow-hidden cursor-pointer hover:border-zinc-600 transition-colors ${getActiveBorderClass()} ${className}`}
      >
        {/* Zone Image with Overlaid Header */}
        <div className='relative w-full h-20 overflow-hidden bg-zinc-800'>
          {/* Background Image */}
          {zone.image_url ? (
            <img
              src={zone.image_url}
              alt=''
              className='absolute inset-0 w-full h-full object-cover'
              style={{ objectPosition: '70% center' }}
              onError={e => {
                e.currentTarget.style.display = 'none';
              }}
            />
          ) : (
            <div className='absolute inset-0 bg-zinc-800' />
          )}

          {/* Base Overlay - Left to right fade with diagonal highlight */}
          <div
            className='absolute inset-0'
            style={{
              background: `
                linear-gradient(90deg,
                  rgba(0, 0, 0, 0.95) 0%,
                  rgba(0, 0, 0, 0.9) 25%,
                  rgba(0, 0, 0, 0.8) 40%,
                  rgba(0, 0, 0, 0.6) 50%,
                  rgba(0, 0, 0, 0.4) 65%,
                  rgba(0, 0, 0, 0.2) 80%,
                  rgba(0, 0, 0, 0.05) 90%,
                  transparent 100%
                ),
                linear-gradient(135deg,
                  rgba(255, 255, 255, 0.1) 0%,
                  rgba(255, 255, 255, 0.05) 25%,
                  transparent 50%,
                  rgba(255, 255, 255, 0.02) 75%,
                  transparent 100%
                )
              `,
            }}
          ></div>

          {/* Header Content - Overlaid with additional gradient */}
          <div
            className='relative h-full flex flex-col justify-between p-3'
            style={{
              background: `
                linear-gradient(90deg,
                  rgba(0, 0, 0, 0.95) 0%,
                  rgba(0, 0, 0, 0.9) 30%,
                  rgba(0, 0, 0, 0.8) 45%,
                  rgba(0, 0, 0, 0.6) 55%,
                  rgba(0, 0, 0, 0.3) 70%,
                  transparent 100%
                )
              `,
            }}
          >
            {/* Top Section - Title only */}
            <div className='flex items-center space-x-2 min-w-0'>
              <h3 className='text-zinc-300 font-medium text-base truncate drop-shadow-md'>
                {zone.zone_name}
              </h3>
            </div>

            {/* Bottom Section - Player Stats */}
            <div className='flex flex-col gap-2'>
              {/* Zone Metadata and Player Stats Group */}
              <div className='flex items-center justify-between flex-wrap gap-2'>
                <div className='flex items-center flex-wrap gap-2'>
                  {/* Zone Metadata Pills */}
                  {zone.is_town && (
                    <span
                      className={`${getZonePillClasses()} ${getZonePillBaseClasses()} backdrop-blur-sm bg-zinc-900/70 opacity-30 hover:opacity-100 transition-opacity`}
                    >
                      Town
                    </span>
                  )}
                  {zone.zone_name.toLowerCase().includes('hideout') && (
                    <span
                      className={`${getZonePillClasses()} ${getZonePillBaseClasses()} backdrop-blur-sm bg-zinc-900/70 opacity-30 hover:opacity-100 transition-opacity`}
                    >
                      Hideout
                    </span>
                  )}
                  {!zone.is_town &&
                    !zone.zone_name.toLowerCase().includes('hideout') && (
                      <span
                        className={`${getZonePillClasses()} ${getZonePillBaseClasses()} backdrop-blur-sm bg-zinc-900/70 opacity-30 hover:opacity-100 transition-opacity`}
                      >
                        Zone
                      </span>
                    )}
                  {zone.act && (
                    <span
                      className={`${getZonePillClasses()} ${getZonePillBaseClasses()} backdrop-blur-sm bg-zinc-900/70 opacity-30 hover:opacity-100 transition-opacity`}
                    >
                      Act {zone.act}
                    </span>
                  )}
                  {zone.area_level && (
                    <span
                      className={`${getZonePillClasses()} ${getZonePillBaseClasses()} backdrop-blur-sm bg-zinc-900/70 opacity-30 hover:opacity-100 transition-opacity`}
                    >
                      Level {zone.area_level}
                    </span>
                  )}
                  {zone.has_waypoint && (
                    <span
                      className={`${getZonePillClasses()} ${getZonePillBaseClasses()} text-blue-400 backdrop-blur-sm bg-zinc-900/70 opacity-30 hover:opacity-100 transition-opacity`}
                      title='Has Waypoint'
                    >
                      <MapIcon className='w-3 h-3 inline-block mr-1' />
                      Waypoint
                    </span>
                  )}
                  {/* Player Stats Pills */}
                  <span
                    className={`${getZonePillClasses()} ${getZonePillBaseClasses()} backdrop-blur-sm bg-zinc-900/70 font-mono opacity-30 hover:opacity-100 transition-opacity`}
                  >
                    <TimeDisplay seconds={zone.duration} showSeconds={false} />
                  </span>
                  <span
                    className={`${getZonePillClasses()} ${getZonePillBaseClasses()} backdrop-blur-sm bg-zinc-900/70 opacity-30 hover:opacity-100 transition-opacity`}
                  >
                    {zone.visits} visit{zone.visits !== 1 ? 's' : ''}
                  </span>
                  {/* Only show deaths for zones where characters can actually die */}
                  {!zone.is_town &&
                    !zone.zone_name.toLowerCase().includes('hideout') && (
                      <span
                        className={`${getZonePillClasses()} ${getZonePillBaseClasses()} backdrop-blur-sm bg-zinc-900/70 opacity-30 hover:opacity-100 transition-opacity`}
                      >
                        {zone.deaths} death{zone.deaths !== 1 ? 's' : ''}
                      </span>
                    )}
                </div>
                <div className='text-xs text-zinc-500 drop-shadow-md'>
                  {zone.last_visited
                    ? formatTimeAgo(zone.last_visited)
                    : 'Never visited'}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </>
  );
});
