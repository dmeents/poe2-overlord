import { BuildingStorefrontIcon, HomeIcon, MapIcon } from '@heroicons/react/24/outline';
import { memo } from 'react';
import { useGameProcess } from '@/contexts/GameProcessContext';
import { useZone } from '@/contexts/ZoneContext';
import { useElapsedTime } from '@/hooks/useElapsedTime';
import type { ZoneStats } from '@/types/character';
import { getDisplayAct } from '@/utils/zone-utils';
import { Card } from '../../ui/card/card';
import { TimeDisplay } from '../../ui/time-display/time-display';
import { currentZoneCardStyles as s } from './current-zone-card.styles';

interface CurrentZoneCardProps {
  zone: ZoneStats;
}

export const CurrentZoneCard = memo(function CurrentZoneCard({ zone }: CurrentZoneCardProps) {
  const { openZone } = useZone();
  const { gameRunning } = useGameProcess();
  const handleViewDetails = () => openZone(zone.zone_name);

  // Calculate live elapsed time for active zones; stop ticking when game process ends
  const elapsedSeconds = useElapsedTime({
    entryTimestamp: zone.entry_timestamp,
    baseDuration: zone.duration,
    isActive: zone.is_active && gameRunning,
  });

  return (
    <Card
      title="Active Zone"
      accentColor="ember"
      showStatusIndicator={true}
      rightAction={{
        label: 'View Details →',
        onClick: handleViewDetails,
      }}>
      <div className={s.body}>
        <div className={s.header}>
          <div className={s.nameSection}>
            <h3 className={s.zoneName}>{zone.zone_name}</h3>
            <div className={s.badgeRow}>
              {getDisplayAct(zone) && <span className={s.actBadge}>{getDisplayAct(zone)}</span>}
              {zone.area_level && <span className={s.levelBadge}>Level {zone.area_level}</span>}
            </div>
          </div>
          <div className={s.iconRow}>
            {zone.has_waypoint && <MapIcon className={s.icon} />}
            {zone.is_town && <BuildingStorefrontIcon className={s.icon} />}
            {zone.zone_name.toLowerCase().includes('hideout') && <HomeIcon className={s.icon} />}
          </div>
        </div>
        <div className={s.statsGrid}>
          <div className={s.statBox}>
            <div className={s.statLabel}>Time</div>
            <div className={s.statValueMono}>
              <TimeDisplay seconds={elapsedSeconds} showSeconds={false} />
            </div>
          </div>
          <div className={s.statBox}>
            <div className={s.statLabel}>Visits</div>
            <div className={s.statValue}>{zone.visits}</div>
          </div>
          <div className={s.statBox}>
            <div className={s.statLabel}>Deaths</div>
            <div className={s.statValue}>{zone.deaths}</div>
          </div>
        </div>
      </div>
    </Card>
  );
});
