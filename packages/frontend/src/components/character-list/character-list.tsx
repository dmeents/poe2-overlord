import type { CharacterData } from '../../types';
import { CharacterCard } from '../character-card';
import { CharacterListHeader } from './character-list-header';
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
}

export function CharacterList({
  characters,
  activeCharacterId,
  onSelectCharacter,
  onEditCharacter,
  onDeleteCharacter,
  onCreateCharacter,
}: CharacterListProps) {
  if (characters.length === 0) {
    return <EmptyCharacterList onCreateCharacter={onCreateCharacter} />;
  }

  return (
    <div className={getListContainerClasses()}>
      <CharacterListHeader onCreateCharacter={onCreateCharacter} />

      <div className={getCharacterGridClasses()}>
        {characters.map(character => (
          <CharacterCard
            key={character.id}
            character={character}
            isActive={character.id === activeCharacterId}
            onSelect={() => onSelectCharacter(character.id)}
            onEdit={() => onEditCharacter(character)}
            onDelete={() => onDeleteCharacter(character.id)}
          />
        ))}
      </div>
    </div>
  );
}
