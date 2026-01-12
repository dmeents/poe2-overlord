import type { ZoneStats } from '@/types/character';
import {
  MapIcon,
  HomeIcon,
  BuildingStorefrontIcon,
} from '@heroicons/react/24/outline';
import { memo } from 'react';
import { TimeDisplay } from '../../ui/time-display/time-display';
import { getDisplayAct } from '@/utils/zone-utils';
import { useZone } from '@/contexts/ZoneContext';
import { Card } from '../../ui/card/card';
import { useElapsedTime } from '@/hooks/useElapsedTime';

interface CurrentZoneCardProps {
  zone: ZoneStats;
}

export const CurrentZoneCard = memo(function CurrentZoneCard({
  zone,
}: CurrentZoneCardProps) {
  const { openZone } = useZone();
  const handleViewDetails = () => openZone(zone.zone_name);

  // Calculate live elapsed time for active zones
  const elapsedSeconds = useElapsedTime({
    entryTimestamp: zone.entry_timestamp,
    baseDuration: zone.duration,
    isActive: zone.is_active,
  });

  return (
    <Card
      title="Active Zone"
      accentColor="emerald"
      showStatusIndicator={true}
      rightAction={{
        label: 'View Details →',
        onClick: handleViewDetails,
      }}
    >
      <div className="p-4">
        <div className="flex items-start justify-between mb-4">
          <div className="flex-1 min-w-0">
            <h3 className="text-xl font-bold text-white mb-1 truncate">
              {zone.zone_name}
            </h3>
            <div className="flex items-center gap-2 flex-wrap">
              {getDisplayAct(zone) && (
                <span className="px-2 py-0.5 bg-purple-500/10 text-purple-400 text-xs rounded">
                  {getDisplayAct(zone)}
                </span>
              )}
              {zone.area_level && (
                <span className="px-2 py-0.5 bg-zinc-800 text-zinc-400 text-xs rounded">
                  Level {zone.area_level}
                </span>
              )}
            </div>
          </div>
          <div className="flex gap-1.5 ml-3">
            {zone.has_waypoint && <MapIcon className="w-4 h-4 text-zinc-400" />}
            {zone.is_town && (
              <BuildingStorefrontIcon className="w-4 h-4 text-zinc-400" />
            )}
            {zone.zone_name.toLowerCase().includes('hideout') && (
              <HomeIcon className="w-4 h-4 text-zinc-400" />
            )}
          </div>
        </div>
        {/* Stat Boxes */}
        <div className="grid grid-cols-3 gap-3">
          <div className="bg-zinc-800/50 rounded px-3 py-2">
            <div className="text-xs text-zinc-500 mb-0.5">Time</div>
            <div className="text-sm font-mono text-zinc-200">
              <TimeDisplay seconds={elapsedSeconds} showSeconds={false} />
            </div>
          </div>
          <div className="bg-zinc-800/50 rounded px-3 py-2">
            <div className="text-xs text-zinc-500 mb-0.5">Visits</div>
            <div className="text-sm text-zinc-200">{zone.visits}</div>
          </div>
          <div className="bg-zinc-800/50 rounded px-3 py-2">
            <div className="text-xs text-zinc-500 mb-0.5">Deaths</div>
            <div className="text-sm text-zinc-200">{zone.deaths}</div>
          </div>
        </div>
      </div>
    </Card>
  );
});
