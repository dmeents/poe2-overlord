import type { ZoneFilters as ZoneFiltersType } from '@/hooks/useZoneFilters';
import type { LocationType } from '@/types';
import { ChevronDownIcon } from '@heroicons/react/24/outline';
import { memo, useEffect, useRef, useState } from 'react';
import { zoneTrackerStyles } from './zone-tracker.styles';

interface ZoneFiltersProps {
  filters: ZoneFiltersType;
  onFilterChange: <K extends keyof ZoneFiltersType>(
    key: K,
    value: ZoneFiltersType[K]
  ) => void;
  onClearFilters: () => void;
  hasActiveFilters: boolean;
  zoneCount: number;
  totalCount: number;
}

const LOCATION_TYPES: { value: LocationType | 'All'; label: string }[] = [
  { value: 'All', label: 'All Types' },
  { value: 'Zone', label: 'Zones' },
  { value: 'Act', label: 'Acts' },
  { value: 'Hideout', label: 'Hideouts' },
];

const ACTS = [
  'All',
  'Act 1',
  'Act 2',
  'Act 3',
  'Act 4',
  'Interlude',
  'Endgame',
];

export const ZoneFilters = memo(function ZoneFilters({
  filters,
  onFilterChange,
  onClearFilters,
  hasActiveFilters,
  zoneCount,
  totalCount,
}: ZoneFiltersProps) {
  const [isExpanded, setIsExpanded] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node)
      ) {
        setIsExpanded(false);
      }
    };

    if (isExpanded) {
      document.addEventListener('mousedown', handleClickOutside);
    }

    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [isExpanded]);

  return (
    <div className='relative' ref={dropdownRef}>
      {/* Filter Toggle Button */}
      <button
        className={zoneTrackerStyles.filterButton}
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <span className='text-sm font-medium'>
          {hasActiveFilters ? 'Filters Active' : 'All Filters'}
        </span>
        <ChevronDownIcon
          className={`w-4 h-4 text-zinc-400 transition-transform ${isExpanded ? 'rotate-180' : ''}`}
        />
      </button>

      {/* Filter Content Overlay */}
      {isExpanded && (
        <div className={zoneTrackerStyles.filterDropdown}>
          <div className={zoneTrackerStyles.filterGrid}>
            {/* Location Type Filter */}
            <div className={zoneTrackerStyles.filterGroup}>
              <label className={zoneTrackerStyles.filterLabel}>
                Location Type
              </label>
              <select
                value={filters.locationType}
                onChange={e =>
                  onFilterChange(
                    'locationType',
                    e.target.value as LocationType | 'All'
                  )
                }
                className={zoneTrackerStyles.filterSelect}
              >
                {LOCATION_TYPES.map(type => (
                  <option key={type.value} value={type.value}>
                    {type.label}
                  </option>
                ))}
              </select>
            </div>

            {/* Act Filter */}
            <div className={zoneTrackerStyles.filterGroup}>
              <label className={zoneTrackerStyles.filterLabel}>Act</label>
              <select
                value={filters.act}
                onChange={e => onFilterChange('act', e.target.value)}
                className={zoneTrackerStyles.filterSelect}
              >
                {ACTS.map(act => (
                  <option key={act} value={act}>
                    {act}
                  </option>
                ))}
              </select>
            </div>

            {/* Town Filter */}
            <div className={zoneTrackerStyles.filterGroup}>
              <label className={zoneTrackerStyles.filterLabel}>
                Town Status
              </label>
              <select
                value={
                  filters.isTown === null
                    ? 'All'
                    : filters.isTown
                      ? 'Town'
                      : 'Non-Town'
                }
                onChange={e => {
                  const value =
                    e.target.value === 'All' ? null : e.target.value === 'Town';
                  onFilterChange('isTown', value);
                }}
                className={zoneTrackerStyles.filterSelect}
              >
                <option value='All'>All</option>
                <option value='Town'>Towns Only</option>
                <option value='Non-Town'>Non-Towns Only</option>
              </select>
            </div>

            {/* Active Filter */}
            <div className={zoneTrackerStyles.filterGroup}>
              <label className={zoneTrackerStyles.filterLabel}>Status</label>
              <select
                value={
                  filters.isActive === null
                    ? 'All'
                    : filters.isActive
                      ? 'Active'
                      : 'Inactive'
                }
                onChange={e => {
                  const value =
                    e.target.value === 'All'
                      ? null
                      : e.target.value === 'Active';
                  onFilterChange('isActive', value);
                }}
                className={zoneTrackerStyles.filterSelect}
              >
                <option value='All'>All</option>
                <option value='Active'>Active Only</option>
                <option value='Inactive'>Inactive Only</option>
              </select>
            </div>

            {/* Visit Count Filters */}
            <div className={zoneTrackerStyles.filterGroup}>
              <label className={zoneTrackerStyles.filterLabel}>
                Min Visits
              </label>
              <input
                type='number'
                min='0'
                value={filters.minVisits || ''}
                onChange={e =>
                  onFilterChange(
                    'minVisits',
                    e.target.value ? parseInt(e.target.value) : null
                  )
                }
                className={zoneTrackerStyles.filterSelect}
                placeholder='Any'
              />
            </div>

            <div className={zoneTrackerStyles.filterGroup}>
              <label className={zoneTrackerStyles.filterLabel}>
                Max Visits
              </label>
              <input
                type='number'
                min='0'
                value={filters.maxVisits || ''}
                onChange={e =>
                  onFilterChange(
                    'maxVisits',
                    e.target.value ? parseInt(e.target.value) : null
                  )
                }
                className={zoneTrackerStyles.filterSelect}
                placeholder='Any'
              />
            </div>

            {/* Death Count Filters */}
            <div className={zoneTrackerStyles.filterGroup}>
              <label className={zoneTrackerStyles.filterLabel}>
                Min Deaths
              </label>
              <input
                type='number'
                min='0'
                value={filters.minDeaths || ''}
                onChange={e =>
                  onFilterChange(
                    'minDeaths',
                    e.target.value ? parseInt(e.target.value) : null
                  )
                }
                className={zoneTrackerStyles.filterSelect}
                placeholder='Any'
              />
            </div>

            <div className={zoneTrackerStyles.filterGroup}>
              <label className={zoneTrackerStyles.filterLabel}>
                Max Deaths
              </label>
              <input
                type='number'
                min='0'
                value={filters.maxDeaths || ''}
                onChange={e =>
                  onFilterChange(
                    'maxDeaths',
                    e.target.value ? parseInt(e.target.value) : null
                  )
                }
                className={zoneTrackerStyles.filterSelect}
                placeholder='Any'
              />
            </div>
          </div>

          {/* Filter Actions */}
          <div className='flex items-center justify-between mt-4 pt-4 border-t border-zinc-700'>
            <div className='text-sm text-zinc-400'>
              Showing {zoneCount} of {totalCount} zones
            </div>
            <button
              onClick={() => {
                onClearFilters();
                setIsExpanded(false);
              }}
              className='px-3 py-1.5 text-sm bg-zinc-600/50 hover:bg-zinc-600 text-zinc-300 hover:text-white transition-colors'
            >
              Clear All
            </button>
          </div>
        </div>
      )}
    </div>
  );
});
