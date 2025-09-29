import type { ZoneFilters as ZoneFiltersType } from '@/hooks/useZoneFilters';
import type { LocationType } from '@/types';
import { ChevronDownIcon } from '@heroicons/react/24/outline';
import { memo, useEffect, useRef, useState } from 'react';
import { Input, Select } from '../';
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

const LOCATION_TYPES = [
  { value: 'All', label: 'All Types' },
  { value: 'Zone', label: 'Zones' },
  { value: 'Hideout', label: 'Hideouts' },
];

const ACTS = [
  { value: 'All', label: 'All' },
  { value: 'Act 1', label: 'Act 1' },
  { value: 'Act 2', label: 'Act 2' },
  { value: 'Act 3', label: 'Act 3' },
  { value: 'Act 4', label: 'Act 4' },
  { value: 'Interlude', label: 'Interlude' },
  { value: 'Endgame', label: 'Endgame' },
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
              <Select
                id='location-type-filter'
                value={filters.locationType}
                onChange={value =>
                  onFilterChange('locationType', value as LocationType | 'All')
                }
                options={LOCATION_TYPES}
                variant='basic'
                label='Location Type'
              />
            </div>

            {/* Act Filter */}
            <div className={zoneTrackerStyles.filterGroup}>
              <Select
                id='act-filter'
                value={filters.act}
                onChange={value => onFilterChange('act', value)}
                options={ACTS}
                variant='basic'
                label='Act'
              />
            </div>

            {/* Town Filter */}
            <div className={zoneTrackerStyles.filterGroup}>
              <Select
                id='town-filter'
                value={
                  filters.isTown === null
                    ? 'All'
                    : filters.isTown
                      ? 'Town'
                      : 'Non-Town'
                }
                onChange={value => {
                  const newValue = value === 'All' ? null : value === 'Town';
                  onFilterChange('isTown', newValue);
                }}
                options={[
                  { value: 'All', label: 'All' },
                  { value: 'Town', label: 'Towns Only' },
                  { value: 'Non-Town', label: 'Non-Towns Only' },
                ]}
                variant='basic'
                label='Town Status'
              />
            </div>

            {/* Active Filter */}
            <div className={zoneTrackerStyles.filterGroup}>
              <Select
                id='active-filter'
                value={
                  filters.isActive === null
                    ? 'All'
                    : filters.isActive
                      ? 'Active'
                      : 'Inactive'
                }
                onChange={value => {
                  const newValue = value === 'All' ? null : value === 'Active';
                  onFilterChange('isActive', newValue);
                }}
                options={[
                  { value: 'All', label: 'All' },
                  { value: 'Active', label: 'Active Only' },
                  { value: 'Inactive', label: 'Inactive Only' },
                ]}
                variant='basic'
                label='Status'
              />
            </div>

            {/* Visit Count Filters */}
            <div className={zoneTrackerStyles.filterGroup}>
              <Input
                id='min-visits-filter'
                value={filters.minVisits}
                onChange={value =>
                  onFilterChange('minVisits', value as number | null)
                }
                type='number'
                label='Min Visits'
                placeholder='Any'
                min={0}
              />
            </div>

            <div className={zoneTrackerStyles.filterGroup}>
              <Input
                id='max-visits-filter'
                value={filters.maxVisits}
                onChange={value =>
                  onFilterChange('maxVisits', value as number | null)
                }
                type='number'
                label='Max Visits'
                placeholder='Any'
                min={0}
              />
            </div>

            {/* Death Count Filters */}
            <div className={zoneTrackerStyles.filterGroup}>
              <Input
                id='min-deaths-filter'
                value={filters.minDeaths}
                onChange={value =>
                  onFilterChange('minDeaths', value as number | null)
                }
                type='number'
                label='Min Deaths'
                placeholder='Any'
                min={0}
              />
            </div>

            <div className={zoneTrackerStyles.filterGroup}>
              <Input
                id='max-deaths-filter'
                value={filters.maxDeaths}
                onChange={value =>
                  onFilterChange('maxDeaths', value as number | null)
                }
                type='number'
                label='Max Deaths'
                placeholder='Any'
                min={0}
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
