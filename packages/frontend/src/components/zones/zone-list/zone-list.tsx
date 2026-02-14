import { MapPinIcon } from '@heroicons/react/24/outline';
import { memo } from 'react';
import type { ZoneFilters as ZoneFiltersType, ZoneSortOption } from '@/hooks/useZoneList';
import type { ZoneStats } from '@/types/character';
import { EmptyState } from '../../ui/empty-state/empty-state';
import { FilteredEmptyState } from '../../ui/filtered-empty-state/filtered-empty-state';
import { ZoneCard } from '../zone-card/zone-card';
import { ZoneListControlsForm } from '../zone-list-controls-form/zone-list-controls-form';

interface ZoneListProps {
  zones: ZoneStats[];
  filters: ZoneFiltersType;
  onFilterChange: <K extends keyof ZoneFiltersType>(key: K, value: ZoneFiltersType[K]) => void;
  onClearFilters: () => void;
  hasActiveFilters: boolean;
  sort: ZoneSortOption;
  onSortChange: (field: ZoneSortOption['field'], direction?: ZoneSortOption['direction']) => void;
  onResetSort: () => void;
  totalCount: number;
}

export const ZoneList = memo(function ZoneList({
  zones,
  filters,
  onFilterChange,
  onClearFilters,
  hasActiveFilters,
  sort,
  onSortChange,
  onResetSort,
  totalCount,
}: ZoneListProps) {
  if (totalCount === 0) {
    return (
      <EmptyState
        icon={<MapPinIcon className="h-12 w-12" />}
        title="No Zone Data Available"
        description="Start playing Path of Exile 2 to begin tracking your time in different locations."
      />
    );
  }

  return (
    <div className="space-y-4">
      <ZoneListControlsForm
        filters={filters}
        onFilterChange={onFilterChange}
        onClearFilters={onClearFilters}
        hasActiveFilters={hasActiveFilters}
        sort={sort}
        onSortChange={onSortChange}
        onResetSort={onResetSort}
      />
      {zones.length > 0 ? (
        <div className="bg-stone-900/50 border border-stone-700/50 overflow-hidden">
          <div
            className="grid gap-2 px-4 py-2 bg-stone-800/50 border-b border-stone-700/50 text-xs font-medium text-stone-400 uppercase tracking-wider"
            style={{
              gridTemplateColumns: '5fr 1fr 1fr 1fr 1fr 1fr',
            }}>
            <div>Zone</div>
            <div className="text-right">Act</div>
            <div className="text-right">Level</div>
            <div className="text-right">Visits</div>
            <div className="text-right">Deaths</div>
            <div className="text-right">Duration</div>
          </div>
          {zones.map((zone, index) => {
            return <ZoneCard key={zone.zone_name} zone={zone} isEven={index % 2 === 0} />;
          })}
        </div>
      ) : (
        <FilteredEmptyState itemType="zones" onClearFilters={onClearFilters} />
      )}
    </div>
  );
});
