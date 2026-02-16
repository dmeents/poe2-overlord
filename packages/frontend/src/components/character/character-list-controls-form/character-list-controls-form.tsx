import { memo, useMemo, useState } from 'react';
import { useCharacterConfig } from '../../../hooks/useCharacterConfig';
import type {
  CharacterFilters as CharacterFiltersType,
  SortOption,
} from '../../../hooks/useCharacterList';
import type { Ascendency, CharacterClass, League } from '../../../types/character';
import { FilterToggle } from '../../forms/form-filter-toggle/form-filter-toggle';
import { Input } from '../../forms/form-input/form-input';
import { Select } from '../../forms/form-select/form-select';
import { SortSelect } from '../../forms/form-sort-select/form-sort-select';
import { Button } from '../../ui/button/button';
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
    value: CharacterFiltersType[K],
  ) => void;
  onClearFilters: () => void;
  hasActiveFilters: boolean;
  sort: SortOption;
  onSortChange: (field: SortOption['field'], direction?: SortOption['direction']) => void;
  onResetSort: () => void;
}

const SORT_OPTIONS = [
  { value: 'level', label: 'Level' },
  { value: 'last_played', label: 'Last Played' },
  { value: 'created_at', label: 'Created' },
  { value: 'name', label: 'Name' },
  { value: 'play_time', label: 'Play Time' },
];

export const CharacterListControlsForm = memo(function CharacterListControlsForm({
  filters,
  onFilterChange,
  onClearFilters,
  hasActiveFilters,
  sort,
  onSortChange,
  onResetSort,
}: CharacterListControlsFormProps) {
  const [isExpanded, setIsExpanded] = useState(false);

  const { leagues, characterClasses, getAscendenciesForClass } = useCharacterConfig();

  // Add "All" options to the dynamic data
  const leagueOptions = [{ value: 'All', label: 'All Leagues' }, ...leagues];

  const characterClassOptions = [{ value: 'all', label: 'All Classes' }, ...characterClasses];

  // Compute available ascendencies based on selected class
  const availableAscendencies = useMemo(() => {
    if (filters.classes.length > 0) {
      return getAscendenciesForClass(filters.classes[0] as CharacterClass);
    }
    return [];
  }, [filters.classes, getAscendenciesForClass]);

  // Build ascendency options based on selected class
  const ascendencyOptions = [{ value: 'all', label: 'All Ascendencies' }, ...availableAscendencies];

  // Calculate active filter count
  const activeFilterCount = [
    filters.league !== 'All',
    filters.hardcore !== null,
    filters.soloSelfFound !== null,
    filters.classes.length > 0,
    filters.ascendencies.length > 0,
    filters.nameSearch.trim() !== '',
  ].filter(Boolean).length;

  const handleSortChange = (field: string, direction?: 'asc' | 'desc') => {
    onSortChange(field as SortOption['field'], direction);
  };

  const handleReset = () => {
    onResetSort();
    onClearFilters();
  };

  return (
    <div className="space-y-4 p-4">
      <div className="mb-4">
        <Input
          id="character-search"
          value={filters.nameSearch}
          onChange={value => onFilterChange('nameSearch', value)}
          type="search"
          placeholder="Enter character name..."
        />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        <div>
          <span className="block text-sm font-medium text-stone-300 uppercase tracking-wide mb-2">
            Filters
          </span>
          <FilterToggle
            isExpanded={isExpanded}
            onToggle={() => setIsExpanded(!isExpanded)}
            label={hasActiveFilters ? 'Filters Active' : 'All Filters'}
            activeCount={activeFilterCount}>
            <div className="space-y-3">
              <div className={filterSectionClasses}>
                <Select
                  id="league-filter"
                  value={filters.league}
                  onChange={(value: string) => onFilterChange('league', value as League | 'All')}
                  options={leagueOptions}
                  variant="dropdown"
                  label="League"
                />
              </div>

              <div className={filterSectionClasses}>
                <Select
                  id="hardcore-filter"
                  value={
                    filters.hardcore === null
                      ? 'all'
                      : filters.hardcore
                        ? 'hardcore'
                        : 'non-hardcore'
                  }
                  onChange={(value: string) => {
                    const newValue = value === 'all' ? null : value === 'hardcore';
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
                  variant="dropdown"
                  label="Hardcore"
                />
              </div>

              <div className={filterSectionClasses}>
                <Select
                  id="ssf-filter"
                  value={
                    filters.soloSelfFound === null
                      ? 'all'
                      : filters.soloSelfFound
                        ? 'ssf'
                        : 'non-ssf'
                  }
                  onChange={(value: string) => {
                    const newValue = value === 'all' ? null : value === 'ssf';
                    onFilterChange('soloSelfFound', newValue);
                  }}
                  options={[
                    { value: 'all', label: 'All' },
                    { value: 'ssf', label: 'SSF Only' },
                    { value: 'non-ssf', label: 'Non-SSF Only' },
                  ]}
                  variant="dropdown"
                  label="Solo Self Found"
                />
              </div>

              <div className={filterSectionClasses}>
                <Select
                  id="class-filter"
                  value={filters.classes.length === 0 ? 'all' : filters.classes[0]}
                  onChange={(value: string) => {
                    if (value === 'all') {
                      onFilterChange('classes', []);
                      // Clear ascendency when class is deselected
                      if (filters.ascendencies.length > 0) {
                        onFilterChange('ascendencies', []);
                      }
                    } else {
                      onFilterChange('classes', [value as CharacterClass]);
                      // Clear ascendency if it's invalid for the new class
                      if (filters.ascendencies.length > 0) {
                        const newAscendencies = getAscendenciesForClass(value as CharacterClass);
                        const isValid = newAscendencies.some(
                          a => a.value === filters.ascendencies[0],
                        );
                        if (!isValid) {
                          onFilterChange('ascendencies', []);
                        }
                      }
                    }
                  }}
                  options={characterClassOptions}
                  variant="dropdown"
                  label="Character Class"
                />
              </div>

              <div className={filterSectionClasses}>
                <Select
                  id="ascendency-filter"
                  value={filters.ascendencies.length === 0 ? 'all' : filters.ascendencies[0]}
                  onChange={(value: string) => {
                    if (value === 'all') {
                      onFilterChange('ascendencies', []);
                    } else {
                      onFilterChange('ascendencies', [value as Ascendency]);
                    }
                  }}
                  options={ascendencyOptions}
                  variant="dropdown"
                  label="Ascendency"
                  disabled={filters.classes.length === 0}
                  placeholder={
                    filters.classes.length === 0 ? 'Select a class first' : 'Select ascendency'
                  }
                />
              </div>

              {hasActiveFilters && (
                <div className={filterSectionClasses}>
                  <h4 className={filterSectionTitleClasses}>Active Filters</h4>
                  <div className="flex flex-wrap gap-2">
                    {filters.league !== 'All' && (
                      <span className={activeFilterChipClasses}>
                        League: {filters.league}
                        <button
                          type="button"
                          onClick={() => onFilterChange('league', 'All')}
                          className={chipRemoveButtonClasses}>
                          ×
                        </button>
                      </span>
                    )}
                    {filters.hardcore !== null && (
                      <span className={activeFilterChipClasses}>
                        {filters.hardcore ? 'Hardcore' : 'Non-Hardcore'}
                        <button
                          type="button"
                          onClick={() => onFilterChange('hardcore', null)}
                          className={chipRemoveButtonClasses}>
                          ×
                        </button>
                      </span>
                    )}
                    {filters.soloSelfFound !== null && (
                      <span className={activeFilterChipClasses}>
                        {filters.soloSelfFound ? 'SSF' : 'Non-SSF'}
                        <button
                          type="button"
                          onClick={() => onFilterChange('soloSelfFound', null)}
                          className={chipRemoveButtonClasses}>
                          ×
                        </button>
                      </span>
                    )}
                    {filters.classes.length > 0 && (
                      <span className={activeFilterChipClasses}>
                        Class: {filters.classes[0]}
                        <button
                          type="button"
                          onClick={() => onFilterChange('classes', [])}
                          className={chipRemoveButtonClasses}>
                          ×
                        </button>
                      </span>
                    )}
                    {filters.ascendencies.length > 0 && (
                      <span className={activeFilterChipClasses}>
                        Ascendency: {filters.ascendencies[0]}
                        <button
                          type="button"
                          onClick={() => onFilterChange('ascendencies', [])}
                          className={chipRemoveButtonClasses}>
                          ×
                        </button>
                      </span>
                    )}
                    {filters.nameSearch.trim() && (
                      <span className={activeFilterChipClasses}>
                        Name: {filters.nameSearch}
                        <button
                          type="button"
                          onClick={() => onFilterChange('nameSearch', '')}
                          className={chipRemoveButtonClasses}>
                          ×
                        </button>
                      </span>
                    )}
                  </div>
                </div>
              )}

              {hasActiveFilters && (
                <div className="flex justify-end">
                  <Button
                    onClick={onClearFilters}
                    variant="outline"
                    size="sm"
                    className={clearButtonClasses}>
                    Clear All Filters
                  </Button>
                </div>
              )}
            </div>
          </FilterToggle>
        </div>

        <div>
          <span className="block text-sm font-medium text-stone-300 uppercase tracking-wide mb-2">
            Sort
          </span>
          <div className="flex gap-2">
            <div className="flex-1">
              <SortSelect
                id="character-sort"
                value={sort.field}
                direction={sort.direction}
                onChange={handleSortChange}
                onReset={onResetSort}
                options={SORT_OPTIONS}
              />
            </div>
          </div>
        </div>

        <div className="flex flex-col justify-end">
          <div className="h-6"></div>
          <Button onClick={handleReset} variant="outline" className="h-10 px-4 text-sm">
            Reset All
          </Button>
        </div>
      </div>
    </div>
  );
});
