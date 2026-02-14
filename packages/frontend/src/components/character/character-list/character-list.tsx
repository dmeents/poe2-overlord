import { UserIcon } from '@heroicons/react/24/outline';
import { memo, useMemo } from 'react';
import type { CharacterFilters, SortOption } from '../../../hooks/useCharacterList';
import type { CharacterData } from '../../../types/character';
import { Button } from '../../ui/button/button';
import { EmptyState } from '../../ui/empty-state/empty-state';
import { FilteredEmptyState } from '../../ui/filtered-empty-state/filtered-empty-state';
import { CharacterCard } from '../character-card/character-card';
import { CharacterListControlsForm } from '../character-list-controls-form/character-list-controls-form';
import { getCharacterGridClasses, getListContainerClasses } from './character-list.styles';

interface CharacterListProps {
  characters: CharacterData[];
  activeCharacterId?: string;
  onSelectCharacter: (characterId: string) => void;
  onEditCharacter: (character: CharacterData) => void;
  onDeleteCharacter: (characterId: string) => void;
  onCreateCharacter: () => void;
  filters: CharacterFilters;
  onFilterChange: <K extends keyof CharacterFilters>(key: K, value: CharacterFilters[K]) => void;
  onClearFilters: () => void;
  hasActiveFilters: boolean;
  sort: SortOption;
  onSortChange: (field: SortOption['field'], direction?: SortOption['direction']) => void;
  onResetSort: () => void;
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
  sort,
  onSortChange,
  onResetSort,
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
      <CharacterListControlsForm
        filters={filters}
        onFilterChange={onFilterChange}
        onClearFilters={onClearFilters}
        hasActiveFilters={hasActiveFilters}
        sort={sort}
        onSortChange={onSortChange}
        onResetSort={onResetSort}
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
