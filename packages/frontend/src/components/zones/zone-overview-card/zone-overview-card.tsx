import { MapIcon } from '@heroicons/react/24/outline';
import { useNavigate } from '@tanstack/react-router';
import { Card } from '@/components/ui/card/card';
import { DataItem } from '@/components/ui/data-item/data-item';
import { EmptyState } from '@/components/ui/empty-state/empty-state';
import { useCharacter } from '@/contexts/CharacterContext';
import type { ZoneStats } from '@/types/character';
import { formatDurationMinutes } from '@/utils/format-duration';

interface ZoneOverviewCardProps {
  zones: ZoneStats[];
}

export function ZoneOverviewCard({ zones }: ZoneOverviewCardProps) {
  const { activeCharacter } = useCharacter();
  const navigate = useNavigate();
  const summary = activeCharacter?.summary;

  const handleViewAll = () => {
    navigate({ to: '/playtime' });
  };

  if (!summary) {
    return (
      <Card
        title="Zones"
        icon={<MapIcon />}
        rightAction={{ label: 'View all', onClick: handleViewAll }}>
        <EmptyState
          icon={<MapIcon className="w-8 h-8" />}
          title="No Zone Data"
          description="Zone history will appear here as you play"
        />
      </Card>
    );
  }

  const nonTownHideoutZones = zones.filter(
    zone => !zone.is_town && !zone.zone_name.includes('Hideout'),
  );

  const totalPlayTime = summary.total_play_time || 0;
  const totalHideoutTime = summary.total_hideout_time || 0;
  const totalTownTime = summary.total_town_time || 0;
  const activePlayTime = totalPlayTime - totalHideoutTime - totalTownTime;
  const activePlayPercentage =
    totalPlayTime > 0 ? ((activePlayTime / totalPlayTime) * 100).toFixed(1) : '0';

  const mostTimeSpent = nonTownHideoutZones.length > 0 ? nonTownHideoutZones[0] : null;
  const mostDeaths =
    nonTownHideoutZones.length > 0
      ? nonTownHideoutZones.reduce(
          (max, zone) => (zone.deaths > max.deaths ? zone : max),
          nonTownHideoutZones[0],
        )
      : null;

  return (
    <Card
      title="Zones"
      icon={<MapIcon />}
      className="py-0"
      rightAction={{ label: 'View all', onClick: handleViewAll }}>
      <DataItem label="Total Play Time" value={formatDurationMinutes(totalPlayTime)} />
      <DataItem
        label="Active Play"
        value={formatDurationMinutes(activePlayTime)}
        subValue={`${activePlayPercentage}%`}
      />
      <DataItem
        label="Zones Visited"
        value={nonTownHideoutZones.length}
        subValue={`${summary.total_zones_visited} total`}
      />
      {mostTimeSpent && (
        <DataItem
          label="Most Time Spent"
          value={mostTimeSpent.zone_name}
          subValue={formatDurationMinutes(mostTimeSpent.duration)}
        />
      )}
      {mostDeaths && mostDeaths.deaths > 0 && (
        <DataItem
          label="Most Deaths"
          value={mostDeaths.zone_name}
          subValue={`${mostDeaths.deaths} deaths`}
        />
      )}
    </Card>
  );
}
