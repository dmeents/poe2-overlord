import { useZoneFiltering } from '@/hooks/useZoneFiltering';
import { useZoneFilters } from '@/hooks/useZoneFilters';
import type { ZoneStats } from '@/types';
import { MagnifyingGlassIcon, MapPinIcon } from '@heroicons/react/24/outline';
import { ZoneCard } from './zone-card';
import { ZoneFilters } from './zone-filters';
import { ZoneSort } from './zone-sort';
import { zoneTrackerStyles } from './zone-tracker.styles';

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
  } = useZoneFilters();

  const { filteredZones, zoneCount, totalCount } = useZoneFiltering(
    zones,
    filters,
    sort
  );

  const handleResetAll = () => {
    clearFilters();
    resetSort();
  };

  if (zones.length === 0) {
    return (
      <div className={`${zoneTrackerStyles.container} ${className}`}>
        <h3 className={zoneTrackerStyles.title}>Zone Tracker</h3>
        <div className={zoneTrackerStyles.emptyState}>
          <div className={zoneTrackerStyles.emptyIcon}>
            <MapPinIcon className='mx-auto h-16 w-16' />
          </div>
          <h3 className={zoneTrackerStyles.emptyTitle}>
            No Zone Data Available
          </h3>
          <p className={zoneTrackerStyles.emptyDescription}>
            Start playing Path of Exile 2 to begin tracking your time in
            different locations.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className={`${zoneTrackerStyles.container} ${className}`}>
      {/* Header */}
      <div className={zoneTrackerStyles.header}>
        <h3 className={zoneTrackerStyles.title}>
          <MapPinIcon className='w-5 h-5 mr-2 text-zinc-400' />
          Zone Tracker
        </h3>
        <div className='flex items-center space-x-2'>
          <span className='text-sm text-zinc-400'>
            {zoneCount} of {totalCount} zones
          </span>
        </div>
      </div>

      {/* Controls */}
      <div className={zoneTrackerStyles.controls}>
        {/* Search */}
        <div className={zoneTrackerStyles.searchContainer}>
          <input
            type='text'
            value={filters.search}
            onChange={e => updateFilter('search', e.target.value)}
            placeholder='Search zones, acts, or location types...'
            className={zoneTrackerStyles.searchInput}
          />
        </div>

        {/* Filters, Sort, and Reset */}
        <div className={zoneTrackerStyles.filterSortContainer}>
          <ZoneFilters
            filters={filters}
            onFilterChange={updateFilter}
            onClearFilters={clearFilters}
            hasActiveFilters={hasActiveFilters}
            zoneCount={zoneCount}
            totalCount={totalCount}
          />
          <ZoneSort
            sort={sort}
            onSortChange={updateSort}
            onResetSort={resetSort}
          />
          {(hasActiveFilters ||
            sort.field !== 'last_visited' ||
            sort.direction !== 'desc') && (
            <button
              onClick={handleResetAll}
              className={zoneTrackerStyles.resetButton}
            >
              Reset All
            </button>
          )}
        </div>
      </div>

      {/* Zone List */}
      {filteredZones.length === 0 ? (
        <div className={zoneTrackerStyles.emptyState}>
          <div className={zoneTrackerStyles.emptyIcon}>
            <MagnifyingGlassIcon className='mx-auto h-12 w-12' />
          </div>
          <h3 className={zoneTrackerStyles.emptyTitle}>
            No Zones Match Your Filters
          </h3>
          <p className={zoneTrackerStyles.emptyDescription}>
            Try adjusting your search or filter criteria to see more results.
          </p>
        </div>
      ) : (
        <div className={zoneTrackerStyles.zonesContainer}>
          {filteredZones.map(zone => (
            <ZoneCard key={zone.location_id} zone={zone} />
          ))}
        </div>
      )}
    </div>
  );
}
