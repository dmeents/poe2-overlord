import { memo, useMemo } from 'react';
import type { CharacterFilters } from '../../../hooks/configs/character-list.config';
import { useCharacterConfig } from '../../../hooks/useCharacterConfig';
import type { Ascendency, CharacterClass, League } from '../../../types/character';
import { Select } from '../../forms/form-select/form-select';

interface CharacterFilterContentProps {
  filters: CharacterFilters;
  onFilterChange: <K extends keyof CharacterFilters>(key: K, value: CharacterFilters[K]) => void;
}

/**
 * Character-specific filter controls for use in ListControlBar popover
 */
export const CharacterFilterContent = memo(function CharacterFilterContent({
  filters,
  onFilterChange,
}: CharacterFilterContentProps) {
  const { leagues, characterClasses, getAscendenciesForClass } = useCharacterConfig();

  // Add "All" options
  const leagueOptions = [{ value: 'All', label: 'All Leagues' }, ...leagues];
  const characterClassOptions = [{ value: 'all', label: 'All Classes' }, ...characterClasses];

  // Compute available ascendencies based on selected class
  const availableAscendencies = useMemo(() => {
    if (filters.classes.length > 0) {
      return getAscendenciesForClass(filters.classes[0] as CharacterClass);
    }
    return [];
  }, [filters.classes, getAscendenciesForClass]);

  const ascendencyOptions = [{ value: 'all', label: 'All Ascendencies' }, ...availableAscendencies];

  return (
    <div className="space-y-2.5 min-w-[280px]">
      <Select
        id="league-filter"
        value={filters.league}
        onChange={(value: string) => onFilterChange('league', value as League | 'All')}
        options={leagueOptions}
        variant="dropdown"
        label="League"
      />

      <Select
        id="hardcore-filter"
        value={filters.hardcore === null ? 'all' : filters.hardcore ? 'hardcore' : 'non-hardcore'}
        onChange={(value: string) => {
          const newValue = value === 'all' ? null : value === 'hardcore';
          onFilterChange('hardcore', newValue);
        }}
        options={[
          { value: 'all', label: 'All' },
          { value: 'hardcore', label: 'Hardcore Only' },
          { value: 'non-hardcore', label: 'Non-Hardcore Only' },
        ]}
        variant="dropdown"
        label="Hardcore"
      />

      <Select
        id="ssf-filter"
        value={filters.soloSelfFound === null ? 'all' : filters.soloSelfFound ? 'ssf' : 'non-ssf'}
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
              const isValid = newAscendencies.some(a => a.value === filters.ascendencies[0]);
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
        placeholder={filters.classes.length === 0 ? 'Select a class first' : 'Select ascendency'}
      />
    </div>
  );
});
