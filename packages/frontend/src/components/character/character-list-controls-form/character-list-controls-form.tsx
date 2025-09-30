import { memo, useEffect, useState } from 'react';
import {
  Accordion,
  Button,
  FilterToggle,
  Input,
  Select,
  SortSelect,
} from '../';
import { useCharacterConfig } from '../../../hooks/useCharacterConfig';
import type {
  CharacterFilters as CharacterFiltersType,
  SortOption,
} from '../../../hooks/useCharacterFilters';
import type { Ascendency, CharacterClass, League } from '../../../types';
import {
  activeFilterChipClasses,
  chipRemoveButtonClasses,
  clearButtonClasses,
  filterSectionClasses,
  filterSectionTitleClasses,
} from './character-list-controls-form.styles';

interface CharacterListControlsFormProps {
  filters: CharacterFiltersType;
  onFilterChange: <K extends keyof CharacterFiltersType>(
    key: K,
    value: CharacterFiltersType[K]
  ) => void;
  onClearFilters: () => void;
  hasActiveFilters: boolean;
  sort: SortOption;
  onSortChange: (
    field: SortOption['field'],
    direction?: SortOption['direction']
  ) => void;
  onResetSort: () => void;
  characterCount: number;
  totalCount: number;
}

const SORT_OPTIONS = [
  { value: 'level', label: 'Level' },
  { value: 'last_played', label: 'Last Played' },
  { value: 'created_at', label: 'Created' },
  { value: 'name', label: 'Name' },
  { value: 'play_time', label: 'Play Time' },
];

export const CharacterListControlsForm = memo(
  function CharacterListControlsForm({
    filters,
    onFilterChange,
    onClearFilters,
    hasActiveFilters,
    sort,
    onSortChange,
    onResetSort,
    characterCount,
    totalCount,
  }: CharacterListControlsFormProps) {
    const [isExpanded, setIsExpanded] = useState(false);
    const [isFormCollapsed, setIsFormCollapsed] = useState(true);
    const [availableAscendencies, setAvailableAscendencies] = useState<
      { value: Ascendency; label: string }[]
    >([]);
    const [isLoadingAscendencies, setIsLoadingAscendencies] = useState(false);

    const {
      leagues,
      characterClasses,
      getAscendenciesForClass,
      isLoading: configLoading,
    } = useCharacterConfig();

    // Add "All" options to the dynamic data
    const leagueOptions = [{ value: 'All', label: 'All Leagues' }, ...leagues];

    const characterClassOptions = [
      { value: 'all', label: 'All Classes' },
      ...characterClasses,
    ];

    // Load ascendencies when a class is selected
    useEffect(() => {
      const loadAscendencies = async () => {
        if (filters.classes.length > 0) {
          setIsLoadingAscendencies(true);
          try {
            const ascendencies = await getAscendenciesForClass(
              filters.classes[0] as CharacterClass
            );
            setAvailableAscendencies(ascendencies);

            // Clear ascendency filter if current selection is not valid for the new class
            if (filters.ascendencies.length > 0) {
              const currentAscendency = filters.ascendencies[0];
              const isValidAscendency = ascendencies.some(
                asc => asc.value === currentAscendency
              );
              if (!isValidAscendency) {
                onFilterChange('ascendencies', []);
              }
            }
          } catch (error) {
            console.error('Failed to load ascendencies:', error);
            setAvailableAscendencies([]);
            // Clear ascendency filter on error
            if (filters.ascendencies.length > 0) {
              onFilterChange('ascendencies', []);
            }
          } finally {
            setIsLoadingAscendencies(false);
          }
        } else {
          setAvailableAscendencies([]);
          // Clear ascendency filter when no class is selected
          if (filters.ascendencies.length > 0) {
            onFilterChange('ascendencies', []);
          }
        }
      };

      loadAscendencies();
    }, [
      filters.classes,
      getAscendenciesForClass,
      filters.ascendencies,
      onFilterChange,
    ]);

    // Build ascendency options based on selected class
    const ascendencyOptions = [
      { value: 'all', label: 'All Ascendencies' },
      ...availableAscendencies,
    ];

    // Calculate active filter count
    const activeFilterCount = [
      filters.league !== 'All',
      filters.hardcore !== null,
      filters.soloSelfFound !== null,
      filters.classes.length > 0,
      filters.ascendencies.length > 0,
      filters.nameSearch.trim() !== '',
    ].filter(Boolean).length;

    const getCharacterCountText = () => {
      return `Showing ${characterCount} of ${totalCount} characters`;
    };

    const handleSortChange = (field: string, direction?: 'asc' | 'desc') => {
      onSortChange(field as SortOption['field'], direction);
    };

    const handleReset = () => {
      onResetSort();
      onClearFilters();
    };

    return (
      <div className='space-y-4'>
        <Accordion
          title='Character Controls'
          subtitle={getCharacterCountText()}
          isExpanded={!isFormCollapsed}
          onToggle={() => setIsFormCollapsed(!isFormCollapsed)}
        >
          {/* First Row: Search Bar */}
          <div className='mb-4'>
            <Input
              id='character-search'
              value={filters.nameSearch}
              onChange={(value: string | number | null) =>
                onFilterChange('nameSearch', value as string)
              }
              type='search'
              placeholder='Enter character name...'
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
                  {/* League Filter */}
                  <div className={filterSectionClasses}>
                    <Select
                      id='league-filter'
                      value={filters.league}
                      onChange={(value: string) =>
                        onFilterChange('league', value as League | 'All')
                      }
                      options={leagueOptions}
                      variant='dropdown'
                      label='League'
                      disabled={configLoading}
                    />
                  </div>

                  {/* Game Mode Filters */}
                  <div className={filterSectionClasses}>
                    <Select
                      id='hardcore-filter'
                      value={
                        filters.hardcore === null
                          ? 'all'
                          : filters.hardcore
                            ? 'hardcore'
                            : 'non-hardcore'
                      }
                      onChange={(value: string) => {
                        const newValue =
                          value === 'all' ? null : value === 'hardcore';
                        onFilterChange('hardcore', newValue);
                      }}
                      options={[
                        { value: 'all', label: 'All' },
                        { value: 'hardcore', label: 'Hardcore Only' },
                        {
                          value: 'non-hardcore',
                          label: 'Non-Hardcore Only',
                        },
                      ]}
                      variant='dropdown'
                      label='Hardcore'
                    />
                  </div>

                  <div className={filterSectionClasses}>
                    <Select
                      id='ssf-filter'
                      value={
                        filters.soloSelfFound === null
                          ? 'all'
                          : filters.soloSelfFound
                            ? 'ssf'
                            : 'non-ssf'
                      }
                      onChange={(value: string) => {
                        const newValue =
                          value === 'all' ? null : value === 'ssf';
                        onFilterChange('soloSelfFound', newValue);
                      }}
                      options={[
                        { value: 'all', label: 'All' },
                        { value: 'ssf', label: 'SSF Only' },
                        { value: 'non-ssf', label: 'Non-SSF Only' },
                      ]}
                      variant='dropdown'
                      label='Solo Self Found'
                    />
                  </div>

                  {/* Class Filter */}
                  <div className={filterSectionClasses}>
                    <Select
                      id='class-filter'
                      value={
                        filters.classes.length === 0
                          ? 'all'
                          : filters.classes[0]
                      }
                      onChange={(value: string) => {
                        if (value === 'all') {
                          onFilterChange('classes', []);
                        } else {
                          onFilterChange('classes', [value as CharacterClass]);
                        }
                      }}
                      options={characterClassOptions}
                      variant='dropdown'
                      label='Character Class'
                      disabled={configLoading}
                    />
                  </div>

                  {/* Ascendency Filter */}
                  <div className={filterSectionClasses}>
                    <Select
                      id='ascendency-filter'
                      value={
                        filters.ascendencies.length === 0
                          ? 'all'
                          : filters.ascendencies[0]
                      }
                      onChange={(value: string) => {
                        if (value === 'all') {
                          onFilterChange('ascendencies', []);
                        } else {
                          onFilterChange('ascendencies', [value as Ascendency]);
                        }
                      }}
                      options={ascendencyOptions}
                      variant='dropdown'
                      label='Ascendency'
                      disabled={
                        configLoading ||
                        isLoadingAscendencies ||
                        filters.classes.length === 0
                      }
                      placeholder={
                        filters.classes.length === 0
                          ? 'Select a class first'
                          : isLoadingAscendencies
                            ? 'Loading ascendencies...'
                            : 'Select ascendency'
                      }
                    />
                  </div>

                  {/* Active Filter Chips */}
                  {hasActiveFilters && (
                    <div className={filterSectionClasses}>
                      <h4 className={filterSectionTitleClasses}>
                        Active Filters
                      </h4>
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
                              onClick={() =>
                                onFilterChange('soloSelfFound', null)
                              }
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
                    id='character-sort'
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
  }
);
