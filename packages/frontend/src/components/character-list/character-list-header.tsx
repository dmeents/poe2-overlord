import { CharacterFilters, CharacterSortControls } from '../';
import type {
  CharacterFilters as CharacterFiltersType,
  SortOption,
} from '../../hooks/useCharacterFilters';
import { TextInput } from '../form-text-input';

interface CharacterListHeaderProps {
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
    return `Showing ${characterCount} of ${totalCount} characters`;
  };

  return (
    <div className='space-y-4'>
      {/* Inline Search, Filters, and Sort */}
      <div className='p-4 bg-zinc-800/50 border border-zinc-700/50'>
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
              className='w-full h-10 px-3 py-2 bg-zinc-700/50 border border-zinc-600 text-white placeholder-zinc-400 focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500/50 transition-colors'
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
