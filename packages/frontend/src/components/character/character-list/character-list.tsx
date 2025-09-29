import { memo, useCallback, useMemo } from 'react';
import type {
  CharacterFilters,
  SortOption,
} from '../../hooks/useCharacterFilters';
import type { CharacterData } from '../../../types';
import { CharacterCard } from '../character-card';
import { CharacterListControls } from './character-list-controls';
import {
  getCharacterGridClasses,
  getListContainerClasses,
} from './character-list.styles';
import { EmptyCharacterList } from './empty-character-list';

interface CharacterListProps {
  characters: CharacterData[];
  activeCharacterId?: string;
  onSelectCharacter: (characterId: string) => void;
  onEditCharacter: (character: CharacterData) => void;
  onDeleteCharacter: (characterId: string) => void;
  onCreateCharacter: () => void;
  filters: CharacterFilters;
  onFilterChange: <K extends keyof CharacterFilters>(
    key: K,
    value: CharacterFilters[K]
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
  characterCount,
  totalCount,
}: CharacterListProps) {
  // Memoize event handlers to prevent unnecessary re-renders
  const handleSelectCharacter = useCallback(
    (characterId: string) => {
      onSelectCharacter(characterId);
    },
    [onSelectCharacter]
  );

  const handleEditCharacter = useCallback(
    (character: CharacterData) => {
      onEditCharacter(character);
    },
    [onEditCharacter]
  );

  const handleDeleteCharacter = useCallback(
    (characterId: string) => {
      onDeleteCharacter(characterId);
    },
    [onDeleteCharacter]
  );

  // Create stable character handlers to prevent unnecessary re-renders
  const characterHandlers = useMemo(() => {
    const handlers = new Map();
    characters.forEach(character => {
      handlers.set(character.id, {
        onSelect: () => handleSelectCharacter(character.id),
        onEdit: () => handleEditCharacter(character),
        onDelete: () => handleDeleteCharacter(character.id),
      });
    });
    return handlers;
  }, [
    characters,
    handleSelectCharacter,
    handleEditCharacter,
    handleDeleteCharacter,
  ]);

  // Only show empty state if there are truly no characters in the system
  // (not just filtered results)
  if (totalCount === 0) {
    return <EmptyCharacterList onCreateCharacter={onCreateCharacter} />;
  }

  return (
    <div className={getListContainerClasses()}>
      <CharacterListControls
        filters={filters}
        onFilterChange={onFilterChange}
        onClearFilters={onClearFilters}
        hasActiveFilters={hasActiveFilters}
        sort={sort}
        onSortChange={onSortChange}
        onResetSort={onResetSort}
        characterCount={characterCount}
        totalCount={totalCount}
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
                showDetails={true}
              />
            );
          })}
        </div>
      ) : (
        <div className='flex flex-col items-center justify-center py-16 px-6 text-center'>
          <div className='w-16 h-16 bg-zinc-800/50 flex items-center justify-center mb-4'>
            <svg
              className='w-8 h-8 text-zinc-500'
              fill='none'
              stroke='currentColor'
              viewBox='0 0 24 24'
            >
              <path
                strokeLinecap='round'
                strokeLinejoin='round'
                strokeWidth={2}
                d='M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z'
              />
            </svg>
          </div>
          <h3 className='text-lg font-medium text-zinc-300 mb-2'>
            No characters found
          </h3>
          <p className='text-zinc-500 mb-4 max-w-md'>
            No characters match your current search and filter criteria. Try
            adjusting your filters or search terms.
          </p>
          <button
            onClick={onClearFilters}
            className='px-4 py-2 text-sm font-medium text-blue-400 hover:text-blue-300 bg-blue-500/10 hover:bg-blue-500/20 border border-blue-500/30 transition-colors'
          >
            Clear All Filters
          </button>
        </div>
      )}
    </div>
  );
});
