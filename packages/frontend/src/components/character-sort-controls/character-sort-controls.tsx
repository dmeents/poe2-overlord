import { memo } from 'react';
import type { SortOption } from '../../hooks/useCharacterFilters';
import { Button } from '../button';
import { SelectInput } from '../form-select-input';

export interface CharacterSortControlsProps {
  sort: SortOption;
  onSortChange: (
    field: SortOption['field'],
    direction?: SortOption['direction']
  ) => void;
  onResetSort: () => void;
  onClearFilters: () => void;
  characterCount: number;
}

const SORT_OPTIONS: { value: SortOption['field']; label: string }[] = [
  { value: 'level', label: 'Level' },
  { value: 'last_played', label: 'Last Played' },
  { value: 'created_at', label: 'Created' },
  { value: 'name', label: 'Name' },
  { value: 'play_time', label: 'Play Time' },
];

export const CharacterSortControls = memo(function CharacterSortControls({
  sort,
  onSortChange,
  onResetSort,
  onClearFilters,
}: CharacterSortControlsProps) {
  const handleFieldChange = (value: string) => {
    onSortChange(value as SortOption['field']);
  };

  const handleDirectionToggle = () => {
    onSortChange(sort.field, sort.direction === 'asc' ? 'desc' : 'asc');
  };

  const getDirectionLabel = () => {
    return sort.direction === 'asc' ? 'Ascending' : 'Descending';
  };

  return (
    <div className='space-y-2'>
      <div className='flex gap-2'>
        <SelectInput
          id='sort-field'
          value={sort.field}
          onChange={handleFieldChange}
          options={SORT_OPTIONS}
          className='flex-1 px-3 py-2 bg-zinc-700/50 border border-zinc-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500/50 transition-colors'
        />

        <button
          onClick={handleDirectionToggle}
          className='px-3 py-2 h-10 bg-zinc-700/50 hover:bg-zinc-700 border border-zinc-600 rounded-lg text-zinc-300 hover:text-white transition-colors flex items-center justify-center'
          title={`Currently ${getDirectionLabel()}. Click to toggle.`}
        >
          <svg
            className={`w-4 h-4 transition-transform ${sort.direction === 'desc' ? 'rotate-180' : ''}`}
            fill='none'
            stroke='currentColor'
            viewBox='0 0 24 24'
          >
            <path
              strokeLinecap='round'
              strokeLinejoin='round'
              strokeWidth={2}
              d='M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4'
            />
          </svg>
        </button>

        <Button
          onClick={() => {
            onResetSort();
            onClearFilters();
          }}
          variant='outline'
          size='sm'
          className='px-3 py-2 h-10 text-sm font-medium text-zinc-400 hover:text-white bg-zinc-700/50 hover:bg-zinc-700 border border-zinc-600 rounded-lg transition-colors flex items-center justify-center'
        >
          Reset
        </Button>
      </div>
    </div>
  );
});
