import { MapPinIcon } from '@heroicons/react/24/outline';
import { memo } from 'react';
import type { ZoneFilters, ZoneSortField } from '@/hooks/configs/zone-list.config';
import type { ActiveChip } from '@/hooks/useListControls';
import type { ZoneStats } from '@/types/character';
import { ListControlBar } from '../../forms/list-control-bar/list-control-bar';
import { EmptyState } from '../../ui/empty-state/empty-state';
import { FilteredEmptyState } from '../../ui/filtered-empty-state/filtered-empty-state';
import { ZoneCard } from '../zone-card/zone-card';
import { ZoneFilterContent } from '../zone-filter-content/zone-filter-content';

const SORT_OPTIONS = [
  { value: 'last_visited', label: 'Last Visited' },
  { value: 'first_visited', label: 'First Visited' },
  { value: 'duration', label: 'Duration' },
  { value: 'visits', label: 'Visits' },
  { value: 'deaths', label: 'Deaths' },
  { value: 'area_level', label: 'Level' },
  { value: 'zone_name', label: 'Name' },
];

interface ZoneListProps {
  zones: ZoneStats[];
  filters: ZoneFilters;
  onFilterChange: <K extends keyof ZoneFilters>(key: K, value: ZoneFilters[K]) => void;
  onClearFilters: () => void;
  hasActiveFilters: boolean;
  activeFilterCount: number;
  activeChips: ActiveChip[];
  sort: { field: ZoneSortField; direction: 'asc' | 'desc' };
  onSortChange: (field: ZoneSortField, direction?: 'asc' | 'desc') => void;
  onResetSort: () => void;
  onResetAll: () => void;
  filteredCount: number;
  totalCount: number;
}

export const ZoneList = memo(function ZoneList({
  zones,
  filters,
  onFilterChange,
  onClearFilters,
  hasActiveFilters,
  activeFilterCount,
  activeChips,
  sort,
  onSortChange,
  onResetSort,
  onResetAll,
  filteredCount,
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
      <ListControlBar
        searchValue={filters.search}
        onSearchChange={value => onFilterChange('search', value)}
        searchPlaceholder="Search zones..."
        filterContent={<ZoneFilterContent filters={filters} onFilterChange={onFilterChange} />}
        activeFilterCount={activeFilterCount}
        hasActiveFilters={hasActiveFilters}
        onClearFilters={onClearFilters}
        sortField={sort.field}
        sortDirection={sort.direction}
        sortOptions={SORT_OPTIONS}
        onSortChange={(field, direction) => onSortChange(field as ZoneSortField, direction)}
        onResetSort={onResetSort}
        filteredCount={filteredCount}
        totalCount={totalCount}
        countLabel="zones"
        activeChips={activeChips}
        onResetAll={onResetAll}
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
