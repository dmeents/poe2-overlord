import type { Character } from '../../types';
import { CharacterCard } from './character-card';
import { CharacterListHeader } from './character-list-header';
import { EmptyCharacterList } from './empty-character-list';

interface CharacterListProps {
  characters: Character[];
  activeCharacterId?: string;
  onSelectCharacter: (characterId: string) => void;
  onEditCharacter: (character: Character) => void;
  onDeleteCharacter: (characterId: string) => void;
  onCreateCharacter: () => void;
  getPlayTime: (characterId: string) => number;
}

export function CharacterList({
  characters,
  activeCharacterId,
  onSelectCharacter,
  onEditCharacter,
  onDeleteCharacter,
  onCreateCharacter,
  getPlayTime,
}: CharacterListProps) {
  if (characters.length === 0) {
    return <EmptyCharacterList onCreateCharacter={onCreateCharacter} />;
  }

  return (
    <div className='space-y-4'>
      <CharacterListHeader onCreateCharacter={onCreateCharacter} />

      <div className='grid gap-3'>
        {characters.map(character => (
          <CharacterCard
            key={character.id}
            character={character}
            isActive={character.id === activeCharacterId}
            onSelect={() => onSelectCharacter(character.id)}
            onEdit={() => onEditCharacter(character)}
            onDelete={() => onDeleteCharacter(character.id)}
            totalPlayTime={getPlayTime(character.id)}
          />
        ))}
      </div>
    </div>
  );
}

