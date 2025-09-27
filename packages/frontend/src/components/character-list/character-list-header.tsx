import { CharacterFilters, CharacterSortControls } from '../';
import type {
  CharacterFilters as CharacterFiltersType,
  SortOption,
} from '../../hooks/useCharacterFilters';
import { Button } from '../button';
import { TextInput } from '../form-text-input';
import {
  getListHeaderClasses,
  getListHeaderTitleClasses,
} from './character-list.styles';

interface CharacterListHeaderProps {
  onCreateCharacter: () => void;
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

export function CharacterListHeader({
  onCreateCharacter,
  filters,
  onFilterChange,
  onClearFilters,
  hasActiveFilters,
  sort,
  onSortChange,
  onResetSort,
  characterCount,
  totalCount,
}: CharacterListHeaderProps) {
  const getCharacterCountText = () => {
    const parts = [];

    // Base count
    parts.push(`Showing ${characterCount} of ${totalCount} characters`);

    // Add search context
    if (filters.nameSearch.trim()) {
      parts.push(`matching "${filters.nameSearch}"`);
    }

    // Add filter context
    const activeFilters = [];
    if (filters.league !== 'All') {
      activeFilters.push(filters.league);
    }
    if (filters.hardcore !== null) {
      activeFilters.push(filters.hardcore ? 'Hardcore' : 'Non-Hardcore');
    }
    if (filters.soloSelfFound !== null) {
      activeFilters.push(filters.soloSelfFound ? 'SSF' : 'Non-SSF');
    }
    if (filters.classes.length > 0) {
      activeFilters.push(`${filters.classes[0]} class`);
    }
    if (filters.ascendencies.length > 0) {
      activeFilters.push(`${filters.ascendencies[0]} ascendency`);
    }

    if (activeFilters.length > 0) {
      parts.push(`filtered by ${activeFilters.join(', ')}`);
    }

    // Add sort context
    const sortField =
      sort.field === 'last_played'
        ? 'last played'
        : sort.field === 'created_at'
          ? 'created'
          : sort.field === 'play_time'
            ? 'play time'
            : sort.field;
    const sortDirection =
      sort.direction === 'desc' ? 'newest first' : 'oldest first';
    parts.push(`sorted by ${sortField} (${sortDirection})`);

    return parts.join(' • ');
  };

  return (
    <div className='space-y-4'>
      <div className={getListHeaderClasses()}>
        <h2 className={getListHeaderTitleClasses()}>Your Characters</h2>
        <Button onClick={onCreateCharacter} variant='primary' size='sm'>
          Create Character
        </Button>
      </div>

      {/* Inline Search, Filters, and Sort */}
      <div className='p-4 bg-zinc-800/30 rounded-lg border border-zinc-700/50'>
        <div className='grid grid-cols-1 lg:grid-cols-3 gap-4'>
          {/* Search Field */}
          <div>
            <label className='block text-sm font-medium text-zinc-300 uppercase tracking-wide mb-2'>
              Search
            </label>
            <TextInput
              id='character-search'
              value={filters.nameSearch}
              onChange={value => onFilterChange('nameSearch', value)}
              placeholder='Enter character name...'
              className='w-full px-3 py-2 bg-zinc-700/50 border border-zinc-600 rounded-lg text-white placeholder-zinc-400 focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500/50 transition-colors'
            />
          </div>

          {/* Filters */}
          <div>
            <label className='block text-sm font-medium text-zinc-300 uppercase tracking-wide mb-2'>
              Filters
            </label>
            <CharacterFilters
              filters={filters}
              onFilterChange={onFilterChange}
              onClearFilters={onClearFilters}
              hasActiveFilters={hasActiveFilters}
              characterCount={characterCount}
              totalCount={totalCount}
            />
          </div>

          {/* Sort */}
          <div>
            <label className='block text-sm font-medium text-zinc-300 uppercase tracking-wide mb-2'>
              Sort
            </label>
            <CharacterSortControls
              sort={sort}
              onSortChange={onSortChange}
              onResetSort={onResetSort}
              onClearFilters={onClearFilters}
              characterCount={characterCount}
            />
          </div>
        </div>

        {/* Character Count with Context */}
        <div className='mt-4 text-sm text-zinc-400 text-center'>
          {getCharacterCountText()}
        </div>
      </div>
    </div>
  );
}
