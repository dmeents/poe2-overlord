import { memo } from 'react';
import { Button, SortSelect } from '../';
import type { SortOption } from '../../hooks/useCharacterFilters';

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

const SORT_OPTIONS = [
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
  const handleChange = (field: string, direction?: 'asc' | 'desc') => {
    onSortChange(field as SortOption['field'], direction);
  };

  const handleReset = () => {
    onResetSort();
    onClearFilters();
  };

  return (
    <div className='space-y-2'>
      <div className='flex gap-2'>
        <div className='flex-1'>
          <SortSelect
            id='character-sort'
            value={sort.field}
            direction={sort.direction}
            onChange={handleChange}
            onReset={onResetSort}
            options={SORT_OPTIONS}
          />
        </div>

        <Button
          onClick={handleReset}
          variant='outline'
          size='sm'
          className='h-10 text-sm font-medium text-zinc-400 hover:text-white bg-zinc-700/50 hover:bg-zinc-700 border border-zinc-600 transition-colors flex items-center justify-center'
        >
          Reset All
        </Button>
      </div>
    </div>
  );
});
