import type {
  ZoneFilters as ZoneFiltersType,
  ZoneSortOption,
} from '@/hooks/useZoneFilters';
import { memo, useEffect, useRef, useState } from 'react';
import { FilterToggle } from '../../forms/form-filter-toggle/form-filter-toggle';
import { Input } from '../../forms/form-input/form-input';
import { Select } from '../../forms/form-select/form-select';
import { SortSelect } from '../../forms/form-sort-select/form-sort-select';
import { Accordion } from '../../ui/accordion/accordion';
import { Button } from '../../ui/button/button';
import {
  activeFilterChipClasses,
  chipRemoveButtonClasses,
  clearButtonClasses,
  filterSectionClasses,
  filterSectionTitleClasses,
} from './zone-list-controls-form.styles';

interface ZoneListControlsFormProps {
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

const ACTS = [
  { value: 'All', label: 'All' },
  { value: 'Act 1', label: 'Act 1' },
  { value: 'Act 2', label: 'Act 2' },
  { value: 'Act 3', label: 'Act 3' },
  { value: 'Act 4', label: 'Act 4' },
  { value: 'Interlude', label: 'Interlude' },
  { value: 'Endgame', label: 'Endgame' },
];

const SORT_OPTIONS = [
  { value: 'last_visited', label: 'Last Visited' },
  { value: 'duration', label: 'Time Spent' },
  { value: 'visits', label: 'Visit Count' },
  { value: 'deaths', label: 'Death Count' },
  { value: 'area_level', label: 'Zone Level' },
  { value: 'zone_name', label: 'Zone Name' },
  { value: 'first_visited', label: 'First Visited' },
];

export const ZoneListControlsForm = memo(function ZoneListControlsForm({
  filters,
  onFilterChange,
  onClearFilters,
  hasActiveFilters,
  sort,
  onSortChange,
  onResetSort,
  zoneCount,
  totalCount,
}: ZoneListControlsFormProps) {
  const [isExpanded, setIsExpanded] = useState(false);
  const [isFormCollapsed, setIsFormCollapsed] = useState(true);
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

  // Calculate active filter count
  const activeFilterCount = [
    filters.act !== 'All',
    filters.isTown !== null,
    filters.isActive !== null,
    filters.minVisits !== null,
    filters.maxVisits !== null,
    filters.minDeaths !== null,
    filters.maxDeaths !== null,
    filters.hasBosses !== null,
    filters.hasWaypoint !== null,
    filters.hasNpcs !== null,
    filters.search.trim() !== '',
  ].filter(Boolean).length;

  const getZoneCountText = () => {
    return `Showing ${zoneCount} of ${totalCount} zones`;
  };

  const handleSortChange = (field: string, direction?: 'asc' | 'desc') => {
    onSortChange(field as ZoneSortOption['field'], direction);
  };

  const handleReset = () => {
    onResetSort();
    onClearFilters();
  };

  return (
    <div className='space-y-4'>
      <Accordion
        title='Zone Controls'
        subtitle={getZoneCountText()}
        isExpanded={!isFormCollapsed}
        onToggle={() => setIsFormCollapsed(!isFormCollapsed)}
      >
        {/* First Row: Search Bar */}
        <div className='mb-4'>
          <Input
            id='zone-search'
            value={filters.search}
            onChange={(value: string | number | null) =>
              onFilterChange('search', value as string)
            }
            type='search'
            placeholder='Search zones, acts, or location types...'
            label='Search'
          />
        </div>

        {/* Second Row: Sorts, Filters, and Reset */}
        <div className='grid grid-cols-1 lg:grid-cols-3 gap-4'>
          {/* Filters */}
          <div>
            <label className='block text-sm font-medium text-zinc-300 uppercase tracking-wide mb-2'>
              Filters
            </label>
            <FilterToggle
              isExpanded={isExpanded}
              onToggle={() => setIsExpanded(!isExpanded)}
              label={hasActiveFilters ? 'Filters Active' : 'All Filters'}
              activeCount={activeFilterCount}
            >
              <div className='space-y-3'>
                {/* Act Filter */}
                <div className={filterSectionClasses}>
                  <Select
                    id='act-filter'
                    value={filters.act}
                    onChange={value => onFilterChange('act', value)}
                    options={ACTS}
                    variant='dropdown'
                    label='Act'
                  />
                </div>

                {/* Town Filter */}
                <div className={filterSectionClasses}>
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
                      const newValue =
                        value === 'All' ? null : value === 'Town';
                      onFilterChange('isTown', newValue);
                    }}
                    options={[
                      { value: 'All', label: 'All' },
                      { value: 'Town', label: 'Towns Only' },
                      { value: 'Non-Town', label: 'Non-Towns Only' },
                    ]}
                    variant='dropdown'
                    label='Town Status'
                  />
                </div>

                {/* Active Filter */}
                <div className={filterSectionClasses}>
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
                      const newValue =
                        value === 'All' ? null : value === 'Active';
                      onFilterChange('isActive', newValue);
                    }}
                    options={[
                      { value: 'All', label: 'All' },
                      { value: 'Active', label: 'Active Only' },
                      { value: 'Inactive', label: 'Inactive Only' },
                    ]}
                    variant='dropdown'
                    label='Status'
                  />
                </div>

                {/* Bosses Filter */}
                <div className={filterSectionClasses}>
                  <Select
                    id='bosses-filter'
                    value={
                      filters.hasBosses === null
                        ? 'All'
                        : filters.hasBosses
                          ? 'HasBosses'
                          : 'NoBosses'
                    }
                    onChange={value => {
                      const newValue =
                        value === 'All' ? null : value === 'HasBosses';
                      onFilterChange('hasBosses', newValue);
                    }}
                    options={[
                      { value: 'All', label: 'All' },
                      { value: 'HasBosses', label: 'Has Bosses' },
                      { value: 'NoBosses', label: 'No Bosses' },
                    ]}
                    variant='dropdown'
                    label='Bosses'
                  />
                </div>

                {/* Waypoint Filter */}
                <div className={filterSectionClasses}>
                  <Select
                    id='waypoint-filter'
                    value={
                      filters.hasWaypoint === null
                        ? 'All'
                        : filters.hasWaypoint
                          ? 'HasWaypoint'
                          : 'NoWaypoint'
                    }
                    onChange={value => {
                      const newValue =
                        value === 'All' ? null : value === 'HasWaypoint';
                      onFilterChange('hasWaypoint', newValue);
                    }}
                    options={[
                      { value: 'All', label: 'All' },
                      { value: 'HasWaypoint', label: 'Has Waypoint' },
                      { value: 'NoWaypoint', label: 'No Waypoint' },
                    ]}
                    variant='dropdown'
                    label='Waypoint'
                  />
                </div>

                {/* NPCs Filter */}
                <div className={filterSectionClasses}>
                  <Select
                    id='npcs-filter'
                    value={
                      filters.hasNpcs === null
                        ? 'All'
                        : filters.hasNpcs
                          ? 'HasNpcs'
                          : 'NoNpcs'
                    }
                    onChange={value => {
                      const newValue =
                        value === 'All' ? null : value === 'HasNpcs';
                      onFilterChange('hasNpcs', newValue);
                    }}
                    options={[
                      { value: 'All', label: 'All' },
                      { value: 'HasNpcs', label: 'Has NPCs' },
                      { value: 'NoNpcs', label: 'No NPCs' },
                    ]}
                    variant='dropdown'
                    label='NPCs'
                  />
                </div>

                {/* Visit Count Filters */}
                <div className={filterSectionClasses}>
                  <div className='grid grid-cols-2 gap-2'>
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
                </div>

                {/* Death Count Filters */}
                <div className={filterSectionClasses}>
                  <div className='grid grid-cols-2 gap-2'>
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

                {/* Active Filter Chips */}
                {hasActiveFilters && (
                  <div className={filterSectionClasses}>
                    <h4 className={filterSectionTitleClasses}>
                      Active Filters
                    </h4>
                    <div className='flex flex-wrap gap-2'>
                      {filters.act !== 'All' && (
                        <span className={activeFilterChipClasses}>
                          Act: {filters.act}
                          <button
                            onClick={() => onFilterChange('act', 'All')}
                            className={chipRemoveButtonClasses}
                          >
                            ×
                          </button>
                        </span>
                      )}
                      {filters.isTown !== null && (
                        <span className={activeFilterChipClasses}>
                          {filters.isTown ? 'Towns' : 'Non-Towns'}
                          <button
                            onClick={() => onFilterChange('isTown', null)}
                            className={chipRemoveButtonClasses}
                          >
                            ×
                          </button>
                        </span>
                      )}
                      {filters.isActive !== null && (
                        <span className={activeFilterChipClasses}>
                          {filters.isActive ? 'Active' : 'Inactive'}
                          <button
                            onClick={() => onFilterChange('isActive', null)}
                            className={chipRemoveButtonClasses}
                          >
                            ×
                          </button>
                        </span>
                      )}
                      {filters.minVisits !== null && (
                        <span className={activeFilterChipClasses}>
                          Min Visits: {filters.minVisits}
                          <button
                            onClick={() => onFilterChange('minVisits', null)}
                            className={chipRemoveButtonClasses}
                          >
                            ×
                          </button>
                        </span>
                      )}
                      {filters.maxVisits !== null && (
                        <span className={activeFilterChipClasses}>
                          Max Visits: {filters.maxVisits}
                          <button
                            onClick={() => onFilterChange('maxVisits', null)}
                            className={chipRemoveButtonClasses}
                          >
                            ×
                          </button>
                        </span>
                      )}
                      {filters.minDeaths !== null && (
                        <span className={activeFilterChipClasses}>
                          Min Deaths: {filters.minDeaths}
                          <button
                            onClick={() => onFilterChange('minDeaths', null)}
                            className={chipRemoveButtonClasses}
                          >
                            ×
                          </button>
                        </span>
                      )}
                      {filters.maxDeaths !== null && (
                        <span className={activeFilterChipClasses}>
                          Max Deaths: {filters.maxDeaths}
                          <button
                            onClick={() => onFilterChange('maxDeaths', null)}
                            className={chipRemoveButtonClasses}
                          >
                            ×
                          </button>
                        </span>
                      )}
                      {filters.search.trim() && (
                        <span className={activeFilterChipClasses}>
                          Search: {filters.search}
                          <button
                            onClick={() => onFilterChange('search', '')}
                            className={chipRemoveButtonClasses}
                          >
                            ×
                          </button>
                        </span>
                      )}
                      {filters.hasBosses !== null && (
                        <span className={activeFilterChipClasses}>
                          {filters.hasBosses ? 'Has Bosses' : 'No Bosses'}
                          <button
                            onClick={() => onFilterChange('hasBosses', null)}
                            className={chipRemoveButtonClasses}
                          >
                            ×
                          </button>
                        </span>
                      )}
                      {filters.hasWaypoint !== null && (
                        <span className={activeFilterChipClasses}>
                          {filters.hasWaypoint ? 'Has Waypoint' : 'No Waypoint'}
                          <button
                            onClick={() => onFilterChange('hasWaypoint', null)}
                            className={chipRemoveButtonClasses}
                          >
                            ×
                          </button>
                        </span>
                      )}
                      {filters.hasNpcs !== null && (
                        <span className={activeFilterChipClasses}>
                          {filters.hasNpcs ? 'Has NPCs' : 'No NPCs'}
                          <button
                            onClick={() => onFilterChange('hasNpcs', null)}
                            className={chipRemoveButtonClasses}
                          >
                            ×
                          </button>
                        </span>
                      )}
                    </div>
                  </div>
                )}

                {/* Clear Filters Button */}
                {hasActiveFilters && (
                  <div className='flex justify-end'>
                    <Button
                      onClick={onClearFilters}
                      variant='outline'
                      size='sm'
                      className={clearButtonClasses}
                    >
                      Clear All Filters
                    </Button>
                  </div>
                )}
              </div>
            </FilterToggle>
          </div>

          {/* Sort */}
          <div>
            <label className='block text-sm font-medium text-zinc-300 uppercase tracking-wide mb-2'>
              Sort
            </label>
            <div className='flex gap-2'>
              <div className='flex-1'>
                <SortSelect
                  id='zone-sort'
                  value={sort.field}
                  direction={sort.direction}
                  onChange={handleSortChange}
                  onReset={onResetSort}
                  options={SORT_OPTIONS}
                />
              </div>
            </div>
          </div>

          {/* Reset All Button */}
          <div className='flex flex-col justify-end'>
            <div className='h-6'></div>
            <Button
              onClick={handleReset}
              variant='outline'
              className='h-10 px-4 text-sm'
            >
              Reset All
            </Button>
          </div>
        </div>
      </Accordion>
    </div>
  );
});
