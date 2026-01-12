/* eslint-disable react-refresh/only-export-components */
import type { CharacterData } from '@/types/character';
import { useAppEventListener } from '@/hooks/useAppEventListener';
import { createContext, useContext, useState, useEffect } from 'react';
import { useActiveCharacter, useCharacters } from '@/queries/characters';
import {
  EVENT_KEYS,
  type ExtractPayload,
  type CharacterUpdatedEvent,
} from '@/utils/events/registry';

interface CharacterContextValue {
  characters: CharacterData[];
  activeCharacter: CharacterData | null;
  isLoading: boolean;
  error: string | null;
  isListening: boolean;
}

const CharacterContext = createContext<CharacterContextValue | undefined>(
  undefined
);

export function CharacterProvider({ children }: React.PropsWithChildren) {
  const {
    data: characters = [],
    isLoading: charactersLoading,
    error: charactersError,
  } = useCharacters();

  const {
    data: activeCharacter = null,
    isLoading: activeCharacterLoading,
    error: activeCharacterError,
  } = useActiveCharacter();

  const [charactersWithUpdates, setCharactersWithUpdates] = useState<
    CharacterData[]
  >([]);
  const [activeCharacterWithUpdates, setActiveCharacterWithUpdates] =
    useState<CharacterData | null>(null);

  useEffect(() => {
    setCharactersWithUpdates(characters);
  }, [characters]);

  useEffect(() => {
    setActiveCharacterWithUpdates(activeCharacter);
  }, [activeCharacter]);

  const { isListening } = useAppEventListener(
    [
      {
        eventType: EVENT_KEYS.CharacterUpdated,
        handler: (payload: unknown) => {
          const { character_id, data: characterData } =
            payload as ExtractPayload<CharacterUpdatedEvent>;

          setCharactersWithUpdates(prev =>
            prev.map(char => (char.id === character_id ? characterData : char))
          );

          // Use functional update to avoid stale closure issue
          setActiveCharacterWithUpdates(prev =>
            prev?.id === character_id ? characterData : prev
          );
        },
      },
    ],
    [] // Dependencies removed - using functional updates avoids stale closures
  );

  const isLoading = charactersLoading || activeCharacterLoading;
  const error =
    charactersError?.message || activeCharacterError?.message || null;

  return (
    <CharacterContext.Provider
      value={{
        characters: charactersWithUpdates,
        activeCharacter: activeCharacterWithUpdates,
        isLoading,
        error,
        isListening,
      }}
    >
      {children}
    </CharacterContext.Provider>
  );
}

export function useCharacter() {
  const context = useContext(CharacterContext);

  if (context === undefined) {
    throw new Error('useCharacter must be used within CharacterProvider');
  }

  return context;
}
