import { MagnifyingGlassIcon, MapPinIcon } from '@heroicons/react/24/outline';
import { memo, useCallback } from 'react';
import type { ZoneFilters as ZoneFiltersType, ZoneSortOption } from '@/hooks/useZoneList';
import type { ZoneStats } from '@/types/character';
import { EmptyState } from '../../ui/empty-state/empty-state';
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
  const handleFilterChange = useCallback(
    <K extends keyof ZoneFiltersType>(key: K, value: ZoneFiltersType[K]) => {
      onFilterChange(key, value);
    },
    [onFilterChange],
  );

  const handleSortChange = useCallback(
    (field: ZoneSortOption['field'], direction?: ZoneSortOption['direction']) => {
      onSortChange(field, direction);
    },
    [onSortChange],
  );

  const handleClearFilters = useCallback(() => {
    onClearFilters();
  }, [onClearFilters]);

  const handleResetSort = useCallback(() => {
    onResetSort();
  }, [onResetSort]);

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
        onFilterChange={handleFilterChange}
        onClearFilters={handleClearFilters}
        hasActiveFilters={hasActiveFilters}
        sort={sort}
        onSortChange={handleSortChange}
        onResetSort={handleResetSort}
      />
      {zones.length > 0 ? (
        <div className="bg-zinc-900/50 border border-zinc-700/50 overflow-hidden">
          <div
            className="grid gap-2 px-4 py-2 bg-zinc-800/50 border-b border-zinc-700/50 text-xs font-medium text-zinc-400 uppercase tracking-wider"
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
        <div className="flex flex-col items-center justify-center py-16 px-6 text-center">
          <div className="w-16 h-16 bg-zinc-800/50 flex items-center justify-center mb-4">
            <MagnifyingGlassIcon className="w-8 h-8 text-zinc-500" />
          </div>
          <h3 className="text-lg font-medium text-zinc-300 mb-2">No zones found</h3>
          <p className="text-zinc-500 mb-4 max-w-md">
            No zones match your current search and filter criteria. Try adjusting your filters or
            search terms.
          </p>
          <button
            onClick={handleClearFilters}
            className="px-4 py-2 text-sm font-medium text-blue-400 hover:text-blue-300 bg-blue-500/10 hover:bg-blue-500/20 border border-blue-500/30 transition-colors">
            Clear All Filters
          </button>
        </div>
      )}
    </div>
  );
});
