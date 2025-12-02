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
        <div className='relative w-full h-32 overflow-hidden bg-zinc-800'>
          {/* Background Image */}
          {zone.image_url ? (
            <img
              src={zone.image_url}
              alt=''
              className='absolute inset-0 w-full h-full object-cover opacity-100'
              onError={e => {
                e.currentTarget.style.display = 'none';
              }}
            />
          ) : (
            <div className='absolute inset-0 bg-zinc-800' />
          )}

          {/* Gradient Overlay - Solid black bottom 25%, gradual transition to transparent */}
          <div className='absolute inset-0 bg-gradient-to-t from-black from-25% via-black/70 via-40% via-black/40 via-55% to-transparent'></div>

          {/* Header Content - Overlaid */}
          <div className='relative h-full flex flex-col justify-between p-3'>
            {/* Top Section - Title only */}
            <div className='flex items-center space-x-2 min-w-0'>
              <h3 className='text-white font-semibold text-lg truncate drop-shadow-lg'>
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
                      className={`${getZonePillClasses()} ${getZonePillBaseClasses()} backdrop-blur-sm bg-zinc-900/70`}
                    >
                      Town
                    </span>
                  )}
                  {zone.zone_name.toLowerCase().includes('hideout') && (
                    <span
                      className={`${getZonePillClasses()} ${getZonePillBaseClasses()} backdrop-blur-sm bg-zinc-900/70`}
                    >
                      Hideout
                    </span>
                  )}
                  {!zone.is_town &&
                    !zone.zone_name.toLowerCase().includes('hideout') && (
                      <span
                        className={`${getZonePillClasses()} ${getZonePillBaseClasses()} backdrop-blur-sm bg-zinc-900/70`}
                      >
                        Zone
                      </span>
                    )}
                  {zone.act && (
                    <span
                      className={`${getZonePillClasses()} ${getZonePillBaseClasses()} backdrop-blur-sm bg-zinc-900/70`}
                    >
                      Act {zone.act}
                    </span>
                  )}
                  {zone.area_level && (
                    <span
                      className={`${getZonePillClasses()} ${getZonePillBaseClasses()} backdrop-blur-sm bg-zinc-900/70`}
                    >
                      Level {zone.area_level}
                    </span>
                  )}
                  {zone.has_waypoint && (
                    <span
                      className={`${getZonePillClasses()} ${getZonePillBaseClasses()} text-blue-400 backdrop-blur-sm bg-zinc-900/70`}
                      title='Has Waypoint'
                    >
                      <MapIcon className='w-3 h-3 inline-block mr-1' />
                      Waypoint
                    </span>
                  )}
                  {/* Player Stats Pills */}
                  <span
                    className={`${getZonePillClasses()} ${getZonePillBaseClasses()} backdrop-blur-sm bg-zinc-900/70 font-mono`}
                  >
                    <TimeDisplay seconds={zone.duration} showSeconds={false} />
                  </span>
                  <span
                    className={`${getZonePillClasses()} ${getZonePillBaseClasses()} backdrop-blur-sm bg-zinc-900/70`}
                  >
                    {zone.visits} visit{zone.visits !== 1 ? 's' : ''}
                  </span>
                  {/* Only show deaths for zones where characters can actually die */}
                  {!zone.is_town &&
                    !zone.zone_name.toLowerCase().includes('hideout') && (
                      <span
                        className={`${getZonePillClasses()} ${
                          zone.deaths > 0 ? 'text-red-400' : 'text-zinc-400'
                        } ${getZonePillBaseClasses()} backdrop-blur-sm bg-zinc-900/70`}
                      >
                        {zone.deaths} death{zone.deaths !== 1 ? 's' : ''}
                      </span>
                    )}
                </div>
                <div className='text-sm text-zinc-300 drop-shadow-lg'>
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
