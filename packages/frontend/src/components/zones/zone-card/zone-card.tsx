import { BuildingStorefrontIcon, HomeIcon, MapIcon } from '@heroicons/react/24/outline';
import { memo } from 'react';
import { useZone } from '@/contexts/ZoneContext';
import type { ZoneStats } from '@/types/character';
import { getDisplayAct } from '@/utils/zone-utils';
import { TimeDisplay } from '../../ui/time-display/time-display';

interface ZoneCardProps {
  zone: ZoneStats;
  className?: string;
  isEven?: boolean;
}

export const ZoneCard = memo(function ZoneCard({
  zone,
  className = '',
  isEven = false,
}: ZoneCardProps) {
  const { openZone } = useZone();

  const handleRowClick = () => {
    openZone(zone.zone_name);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      openZone(zone.zone_name);
    }
  };

  return (
    // biome-ignore lint/a11y/useSemanticElements: Grid layout requires div; button cannot contain child divs
    <div
      onClick={handleRowClick}
      onKeyDown={handleKeyDown}
      role="button"
      tabIndex={0}
      className={`
        grid gap-2 px-4 py-3
        cursor-pointer transition-all
        hover:bg-stone-800/70
        ${
          zone.is_active
            ? 'bg-verdant-900/10 border-l-2 border-l-verdant-500'
            : isEven
              ? 'bg-stone-900/30'
              : 'bg-stone-900/60'
        }
        ${className}
      `}
      style={{
        gridTemplateColumns: '5fr 1fr 1fr 1fr 1fr 1fr',
      }}>
      {/* Zone Name with Icon */}
      <div className="flex items-center gap-2 min-w-0">
        {zone.has_waypoint && <MapIcon className="w-3.5 h-3.5 text-stone-400 flex-shrink-0" />}
        <span className="text-stone-200 truncate">{zone.zone_name}</span>
        {zone.is_town && (
          <BuildingStorefrontIcon className="w-3.5 h-3.5 text-stone-400 flex-shrink-0" />
        )}
        {zone.zone_name.toLowerCase().includes('hideout') && (
          <HomeIcon className="w-3.5 h-3.5 text-stone-400 flex-shrink-0" />
        )}
      </div>

      {/* Act */}
      <div className="flex items-center justify-end text-xs">
        {getDisplayAct(zone) && <span className="text-stone-400">{getDisplayAct(zone)}</span>}
      </div>

      {/* Level */}
      <div className="flex items-center justify-end text-xs">
        {zone.area_level && <span className="text-stone-400">{zone.area_level}</span>}
      </div>

      {/* Visits */}
      <div className="flex items-center justify-end text-xs">
        <span className="text-stone-400">{zone.visits}</span>
      </div>

      {/* Deaths */}
      <div className="flex items-center justify-end text-xs">
        <span className="text-stone-400">{zone.deaths}</span>
      </div>

      {/* Duration */}
      <div className="flex items-center justify-end text-xs">
        <span className="font-mono text-stone-400">
          <TimeDisplay seconds={zone.duration} showSeconds={false} />
        </span>
      </div>
    </div>
  );
});
