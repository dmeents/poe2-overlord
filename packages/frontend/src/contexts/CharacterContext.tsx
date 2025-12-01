/* eslint-disable react-refresh/only-export-components */
import type { CharacterData } from '@/types/character';
import { useAppEventListener } from '@/hooks/useAppEventListener';
import { createContext, useContext, useState, useEffect } from 'react';
import { useActiveCharacter, useCharacters } from '@/hooks/useCharacterQueries';
import type {
  ExtractPayload,
  CharacterTrackingDataUpdatedEvent,
} from '@/utils/events/registry';

interface CharacterContextValue {
  // Character data
  characters: CharacterData[];
  activeCharacter: CharacterData | null;

  // Loading & status states
  isLoading: boolean;
  error: string | null;
  isListening: boolean;
}

const CharacterContext = createContext<CharacterContextValue | undefined>(
  undefined
);

export function CharacterProvider({ children }: React.PropsWithChildren) {
  // React Query hooks for data fetching
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

  // State for real-time character data updates
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
        eventType: 'CharacterTrackingDataUpdated',
        handler: (payload: unknown) => {
          const { character_id, data: characterData } =
            payload as ExtractPayload<CharacterTrackingDataUpdatedEvent>;

          setCharactersWithUpdates(prev =>
            prev.map(char => (char.id === character_id ? characterData : char))
          );

          if (character_id === activeCharacterWithUpdates?.id) {
            setActiveCharacterWithUpdates(characterData);
          }
        },
      },
    ],
    [activeCharacterWithUpdates?.id]
  );

  // Derived state
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
