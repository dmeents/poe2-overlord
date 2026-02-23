import { UserIcon } from '@heroicons/react/24/outline';
import { memo, useMemo } from 'react';
import type {
  CharacterFilters,
  CharacterSortField,
} from '../../../hooks/configs/character-list.config';
import type { ActiveChip } from '../../../hooks/useListControls';
import type { CharacterSummaryData } from '../../../types/character';
import { ListControlBar } from '../../forms/list-control-bar/list-control-bar';
import { Button } from '../../ui/button/button';
import { EmptyState } from '../../ui/empty-state/empty-state';
import { FilteredEmptyState } from '../../ui/filtered-empty-state/filtered-empty-state';
import { CharacterCard } from '../character-card/character-card';
import { CharacterFilterContent } from '../character-filter-content/character-filter-content';
import { getCharacterGridClasses, getListContainerClasses } from './character-list.styles';

const SORT_OPTIONS = [
  { value: 'level', label: 'Level' },
  { value: 'last_played', label: 'Last Played' },
  { value: 'created_at', label: 'Created' },
  { value: 'name', label: 'Name' },
  { value: 'play_time', label: 'Play Time' },
];

interface CharacterListProps {
  characters: CharacterSummaryData[];
  activeCharacterId?: string;
  onSelectCharacter: (characterId: string) => void;
  onEditCharacter: (character: CharacterSummaryData) => void;
  onDeleteCharacter: (characterId: string) => void;
  onCreateCharacter: () => void;
  filters: CharacterFilters;
  onFilterChange: <K extends keyof CharacterFilters>(key: K, value: CharacterFilters[K]) => void;
  onClearFilters: () => void;
  hasActiveFilters: boolean;
  activeFilterCount: number;
  activeChips: ActiveChip[];
  sort: { field: CharacterSortField; direction: 'asc' | 'desc' };
  onSortChange: (field: CharacterSortField, direction?: 'asc' | 'desc') => void;
  onResetSort: () => void;
  onResetAll: () => void;
  filteredCount: number;
  totalCount: number;
}

export const CharacterList = memo(function CharacterList({
  characters,
  activeCharacterId,
  onSelectCharacter,
  onEditCharacter,
  onDeleteCharacter,
  onCreateCharacter,
  filters,
  onFilterChange,
  onClearFilters,
  hasActiveFilters,
  activeFilterCount,
  activeChips,
  sort,
  onSortChange,
  onResetSort,
  onResetAll,
  filteredCount,
  totalCount,
}: CharacterListProps) {
  const characterHandlers = useMemo(() => {
    const handlers = new Map();
    characters.forEach(character => {
      handlers.set(character.id, {
        onSelect: () => onSelectCharacter(character.id),
        onEdit: () => onEditCharacter(character),
        onDelete: () => onDeleteCharacter(character.id),
      });
    });
    return handlers;
  }, [characters, onSelectCharacter, onEditCharacter, onDeleteCharacter]);

  // Only show empty state if there are truly no characters in the system
  // (not just filtered results)
  if (totalCount === 0) {
    return (
      <EmptyState
        icon={<UserIcon className="h-12 w-12" />}
        title="No Characters"
        description="Create your first character to start tracking your adventures."
        action={
          <Button onClick={onCreateCharacter} variant="primary">
            Create Character
          </Button>
        }
      />
    );
  }

  return (
    <div className={getListContainerClasses()}>
      <ListControlBar
        searchValue={filters.nameSearch}
        onSearchChange={value => onFilterChange('nameSearch', value)}
        searchPlaceholder="Search characters..."
        filterContent={<CharacterFilterContent filters={filters} onFilterChange={onFilterChange} />}
        activeFilterCount={activeFilterCount}
        hasActiveFilters={hasActiveFilters}
        onClearFilters={onClearFilters}
        sortField={sort.field}
        sortDirection={sort.direction}
        sortOptions={SORT_OPTIONS}
        onSortChange={(field, direction) => onSortChange(field as CharacterSortField, direction)}
        onResetSort={onResetSort}
        filteredCount={filteredCount}
        totalCount={totalCount}
        countLabel="characters"
        activeChips={activeChips}
        onResetAll={onResetAll}
      />

      {characters.length > 0 ? (
        <div className={getCharacterGridClasses()}>
          {characters.map(character => {
            const handlers = characterHandlers.get(character.id);
            return (
              <CharacterCard
                key={character.id}
                character={character}
                isActive={character.id === activeCharacterId}
                onSelect={handlers?.onSelect || (() => {})}
                onEdit={handlers?.onEdit || (() => {})}
                onDelete={handlers?.onDelete || (() => {})}
              />
            );
          })}
        </div>
      ) : (
        <FilteredEmptyState itemType="characters" onClearFilters={onClearFilters} />
      )}
    </div>
  );
});
