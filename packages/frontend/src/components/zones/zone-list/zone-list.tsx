import type {
  ZoneFilters as ZoneFiltersType,
  ZoneSortOption,
} from '@/hooks/useZoneFilters';
import type { ZoneStats } from '@/types/character';
import { MagnifyingGlassIcon } from '@heroicons/react/24/outline';
import { memo, useCallback } from 'react';
import { EmptyState } from '../../ui/empty-state/empty-state';
import { ZoneCard } from '../zone-card/zone-card';
import { ZoneListControlsForm } from '../zone-list-controls-form/zone-list-controls-form';
import {
  getListContainerClasses,
  getZoneGridClasses,
} from './zone-list.styles';

interface ZoneListProps {
  zones: ZoneStats[];
  filters: ZoneFiltersType;
  onFilterChange: <K extends keyof ZoneFiltersType>(
    key: K,
    value: ZoneFiltersType[K]
  ) => void;
  onClearFilters: () => void;
  hasActiveFilters: boolean;
  sort: ZoneSortOption;
  onSortChange: (
    field: ZoneSortOption['field'],
    direction?: ZoneSortOption['direction']
  ) => void;
  onResetSort: () => void;
  zoneCount: number;
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
  zoneCount,
  totalCount,
}: ZoneListProps) {
  // Memoize event handlers to prevent unnecessary re-renders
  const handleFilterChange = useCallback(
    <K extends keyof ZoneFiltersType>(key: K, value: ZoneFiltersType[K]) => {
      onFilterChange(key, value);
    },
    [onFilterChange]
  );

  const handleSortChange = useCallback(
    (
      field: ZoneSortOption['field'],
      direction?: ZoneSortOption['direction']
    ) => {
      onSortChange(field, direction);
    },
    [onSortChange]
  );

  const handleClearFilters = useCallback(() => {
    onClearFilters();
  }, [onClearFilters]);

  const handleResetSort = useCallback(() => {
    onResetSort();
  }, [onResetSort]);

  // Only show empty state if there are truly no zones in the system
  // (not just filtered results)
  if (totalCount === 0) {
    return (
      <EmptyState
        icon={
          <svg
            className='h-12 w-12'
            fill='none'
            viewBox='0 0 24 24'
            stroke='currentColor'
          >
            <path
              strokeLinecap='round'
              strokeLinejoin='round'
              strokeWidth={1.5}
              d='M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z'
            />
            <path
              strokeLinecap='round'
              strokeLinejoin='round'
              strokeWidth={1.5}
              d='M15 11a3 3 0 11-6 0 3 3 0 016 0z'
            />
          </svg>
        }
        title='No Zone Data Available'
        description='Start playing Path of Exile 2 to begin tracking your time in different locations.'
      />
    );
  }

  return (
    <div className={getListContainerClasses()}>
      <ZoneListControlsForm
        filters={filters}
        onFilterChange={handleFilterChange}
        onClearFilters={handleClearFilters}
        hasActiveFilters={hasActiveFilters}
        sort={sort}
        onSortChange={handleSortChange}
        onResetSort={handleResetSort}
        zoneCount={zoneCount}
        totalCount={totalCount}
      />

      {zones.length > 0 ? (
        <div className={getZoneGridClasses()}>
          {zones.map(zone => {
            return (
              <ZoneCard
                key={zone.zone_name}
                zone={zone}
                // Add any zone-specific props here if needed in the future
              />
            );
          })}
        </div>
      ) : (
        <div className='flex flex-col items-center justify-center py-16 px-6 text-center'>
          <div className='w-16 h-16 bg-zinc-800/50 flex items-center justify-center mb-4'>
            <MagnifyingGlassIcon className='w-8 h-8 text-zinc-500' />
          </div>
          <h3 className='text-lg font-medium text-zinc-300 mb-2'>
            No zones found
          </h3>
          <p className='text-zinc-500 mb-4 max-w-md'>
            No zones match your current search and filter criteria. Try
            adjusting your filters or search terms.
          </p>
          <button
            onClick={handleClearFilters}
            className='px-4 py-2 text-sm font-medium text-blue-400 hover:text-blue-300 bg-blue-500/10 hover:bg-blue-500/20 border border-blue-500/30 transition-colors'
          >
            Clear All Filters
          </button>
        </div>
      )}
    </div>
  );
});
