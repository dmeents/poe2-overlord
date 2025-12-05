import { useZoneList } from '@/hooks/useZoneList';
import type { ZoneStats } from '@/types/character';
import { MapPinIcon } from '@heroicons/react/24/outline';
import { Card } from '../../ui/card/card';
import { ZoneList } from '../zone-list/zone-list';

interface ZoneTrackerProps {
  zones: ZoneStats[];
  className?: string;
}

export function ZoneTracker({ zones, className = '' }: ZoneTrackerProps) {
  const {
    filters,
    sort,
    updateFilter,
    updateSort,
    clearFilters,
    resetSort,
    hasActiveFilters,
    filteredZones,
    zoneCount,
    totalCount,
  } = useZoneList(zones);

  return (
    <Card
      title='Zones'
      subtitle={`${zoneCount} of ${totalCount} zones`}
      icon={<MapPinIcon className='w-5 h-5' />}
      className={className}
    >
      <ZoneList
        zones={filteredZones}
        allZones={zones}
        filters={filters}
        onFilterChange={updateFilter}
        onClearFilters={clearFilters}
        hasActiveFilters={hasActiveFilters}
        sort={sort}
        onSortChange={updateSort}
        onResetSort={resetSort}
        zoneCount={zoneCount}
        totalCount={totalCount}
      />
    </Card>
  );
}
