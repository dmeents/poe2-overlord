import { memo, useEffect, useRef, useState } from 'react';
import type { CharacterFilters as CharacterFiltersType } from '../../hooks/useCharacterFilters';
import type { Ascendency, CharacterClass, League } from '../../types';
import { Button } from '../button';
import { SelectInput } from '../form-select-input';
import {
  activeFilterChipClasses,
  chipRemoveButtonClasses,
  clearButtonClasses,
  filterLabelClasses,
  filterSectionClasses,
  filterSectionTitleClasses,
  selectClasses,
} from './character-filters.styles';

export interface CharacterFiltersProps {
  filters: CharacterFiltersType;
  onFilterChange: <K extends keyof CharacterFiltersType>(
    key: K,
    value: CharacterFiltersType[K]
  ) => void;
  onClearFilters: () => void;
  hasActiveFilters: boolean;
  characterCount: number;
  totalCount: number;
}

const LEAGUE_OPTIONS: { value: League | 'All'; label: string }[] = [
  { value: 'All', label: 'All Leagues' },
  { value: 'Standard', label: 'Standard' },
  { value: 'Third Edict', label: 'Third Edict' },
];

const CHARACTER_CLASSES: CharacterClass[] = [
  'Warrior',
  'Sorceress',
  'Ranger',
  'Huntress',
  'Monk',
  'Mercenary',
  'Witch',
];

const ASCENDENCIES: Ascendency[] = [
  // Warrior ascendencies
  'Titan',
  'Warbringer',
  'Smith of Katava',
  // Sorceress ascendencies
  'Stormweaver',
  'Chronomancer',
  // Ranger ascendencies
  'Deadeye',
  'Pathfinder',
  // Huntress ascendencies
  'Ritualist',
  'Amazon',
  // Monk ascendencies
  'Invoker',
  'Acolyte of Chayula',
  // Mercenary ascendencies
  'Gemling Legionnaire',
  'Tactitian',
  'Witchhunter',
  // Witch ascendencies
  'Blood Mage',
  'Infernalist',
  'Lich',
];

export const CharacterFilters = memo(function CharacterFilters({
  filters,
  onFilterChange,
  onClearFilters,
  hasActiveFilters,
}: CharacterFiltersProps) {
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
        className='w-full flex items-center justify-between px-3 py-2 bg-zinc-700/50 hover:bg-zinc-700 border border-zinc-600 rounded-lg text-zinc-300 hover:text-white transition-colors'
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <span className='text-sm font-medium'>
          {hasActiveFilters ? 'Filters Active' : 'All Filters'}
        </span>
        <svg
          className={`w-4 h-4 text-zinc-400 transition-transform ${isExpanded ? 'rotate-180' : ''}`}
          fill='none'
          stroke='currentColor'
          viewBox='0 0 24 24'
        >
          <path
            strokeLinecap='round'
            strokeLinejoin='round'
            strokeWidth={2}
            d='M19 9l-7 7-7-7'
          />
        </svg>
      </button>

      {/* Filter Content Overlay */}
      {isExpanded && (
        <div className='absolute top-full left-0 right-0 z-50 mt-2 space-y-3 p-4 bg-zinc-800/95 backdrop-blur-sm rounded-lg border border-zinc-700/50 shadow-xl'>
          {/* League Filter */}
          <div className={filterSectionClasses}>
            <label className={filterLabelClasses}>League</label>
            <SelectInput
              id='league-filter'
              value={filters.league}
              onChange={value =>
                onFilterChange('league', value as League | 'All')
              }
              options={LEAGUE_OPTIONS}
              className={selectClasses}
            />
          </div>

          {/* Game Mode Filters */}
          <div className={filterSectionClasses}>
            <label className={filterLabelClasses}>Hardcore</label>
            <SelectInput
              id='hardcore-filter'
              value={
                filters.hardcore === null
                  ? 'all'
                  : filters.hardcore
                    ? 'hardcore'
                    : 'non-hardcore'
              }
              onChange={value => {
                const newValue = value === 'all' ? null : value === 'hardcore';
                onFilterChange('hardcore', newValue);
              }}
              options={[
                { value: 'all', label: 'All' },
                { value: 'hardcore', label: 'Hardcore Only' },
                { value: 'non-hardcore', label: 'Non-Hardcore Only' },
              ]}
              className={selectClasses}
            />
          </div>

          <div className={filterSectionClasses}>
            <label className={filterLabelClasses}>Solo Self Found</label>
            <SelectInput
              id='ssf-filter'
              value={
                filters.soloSelfFound === null
                  ? 'all'
                  : filters.soloSelfFound
                    ? 'ssf'
                    : 'non-ssf'
              }
              onChange={value => {
                const newValue = value === 'all' ? null : value === 'ssf';
                onFilterChange('soloSelfFound', newValue);
              }}
              options={[
                { value: 'all', label: 'All' },
                { value: 'ssf', label: 'SSF Only' },
                { value: 'non-ssf', label: 'Non-SSF Only' },
              ]}
              className={selectClasses}
            />
          </div>

          {/* Class Filter */}
          <div className={filterSectionClasses}>
            <label className={filterLabelClasses}>Character Class</label>
            <SelectInput
              id='class-filter'
              value={filters.classes.length === 0 ? 'all' : filters.classes[0]}
              onChange={value => {
                if (value === 'all') {
                  onFilterChange('classes', []);
                } else {
                  onFilterChange('classes', [value as CharacterClass]);
                }
              }}
              options={[
                { value: 'all', label: 'All Classes' },
                ...CHARACTER_CLASSES.map(cls => ({ value: cls, label: cls })),
              ]}
              className={selectClasses}
            />
          </div>

          {/* Ascendency Filter */}
          <div className={filterSectionClasses}>
            <label className={filterLabelClasses}>Ascendency</label>
            <SelectInput
              id='ascendency-filter'
              value={
                filters.ascendencies.length === 0
                  ? 'all'
                  : filters.ascendencies[0]
              }
              onChange={value => {
                if (value === 'all') {
                  onFilterChange('ascendencies', []);
                } else {
                  onFilterChange('ascendencies', [value as Ascendency]);
                }
              }}
              options={[
                { value: 'all', label: 'All Ascendencies' },
                ...ASCENDENCIES.map(asc => ({ value: asc, label: asc })),
              ]}
              className={selectClasses}
            />
          </div>

          {/* Active Filter Chips */}
          {hasActiveFilters && (
            <div className={filterSectionClasses}>
              <h4 className={filterSectionTitleClasses}>Active Filters</h4>
              <div className='flex flex-wrap gap-2'>
                {filters.league !== 'All' && (
                  <span className={activeFilterChipClasses}>
                    League: {filters.league}
                    <button
                      onClick={() => onFilterChange('league', 'All')}
                      className={chipRemoveButtonClasses}
                    >
                      ×
                    </button>
                  </span>
                )}
                {filters.hardcore !== null && (
                  <span className={activeFilterChipClasses}>
                    {filters.hardcore ? 'Hardcore' : 'Non-Hardcore'}
                    <button
                      onClick={() => onFilterChange('hardcore', null)}
                      className={chipRemoveButtonClasses}
                    >
                      ×
                    </button>
                  </span>
                )}
                {filters.soloSelfFound !== null && (
                  <span className={activeFilterChipClasses}>
                    {filters.soloSelfFound ? 'SSF' : 'Non-SSF'}
                    <button
                      onClick={() => onFilterChange('soloSelfFound', null)}
                      className={chipRemoveButtonClasses}
                    >
                      ×
                    </button>
                  </span>
                )}
                {filters.classes.length > 0 && (
                  <span className={activeFilterChipClasses}>
                    Class: {filters.classes[0]}
                    <button
                      onClick={() => onFilterChange('classes', [])}
                      className={chipRemoveButtonClasses}
                    >
                      ×
                    </button>
                  </span>
                )}
                {filters.ascendencies.length > 0 && (
                  <span className={activeFilterChipClasses}>
                    Ascendency: {filters.ascendencies[0]}
                    <button
                      onClick={() => onFilterChange('ascendencies', [])}
                      className={chipRemoveButtonClasses}
                    >
                      ×
                    </button>
                  </span>
                )}
                {filters.nameSearch.trim() && (
                  <span className={activeFilterChipClasses}>
                    Name: {filters.nameSearch}
                    <button
                      onClick={() => onFilterChange('nameSearch', '')}
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
      )}
    </div>
  );
});
