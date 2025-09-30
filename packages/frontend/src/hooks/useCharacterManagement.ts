import { useCallback, useEffect, useState } from 'react';
import type { CharacterFormData } from '../components/character/character-form-modal/character-form-modal';
import type { CharacterData } from '../types';
import type { CharacterTrackingData } from '../types/character';
import {
  useActiveCharacter,
  useCharacters,
  useCreateCharacter,
  useDeleteCharacter,
  useSetActiveCharacter,
  useUpdateCharacter,
} from './useCharacterQueries';
import { useTauriEventListener } from './useTauriEventListener';

export function useCharacterManagement() {
  // Use React Query hooks for data fetching
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
  >(characters);
  const [activeCharacterWithUpdates, setActiveCharacterWithUpdates] =
    useState<CharacterData | null>(activeCharacter);

  // Initialize characters with updates when data changes
  useEffect(() => {
    setCharactersWithUpdates(characters);
  }, [characters]);

  // Initialize active character with updates when data changes
  useEffect(() => {
    setActiveCharacterWithUpdates(activeCharacter);
  }, [activeCharacter]);

  // Event handler for character tracking data updates
  const handleCharacterDataUpdate = useCallback(
    (event: { payload: unknown }) => {
      // The backend sends the entire AppEvent enum, so we need to extract the data
      let character_id: string | undefined;
      let characterData: CharacterData | undefined;

      if (event.payload && typeof event.payload === 'object') {
        // Check if it's the CharacterTrackingDataUpdated variant
        const payload = event.payload as Record<string, unknown>;
        if (payload.CharacterTrackingDataUpdated) {
          const trackingEvent = payload.CharacterTrackingDataUpdated as {
            character_id: string;
            data: CharacterData;
          };
          character_id = trackingEvent.character_id;
          characterData = trackingEvent.data;
        }
      }

      if (character_id && characterData) {
        // Update the character in the characters list
        setCharactersWithUpdates(prev =>
          prev.map(char => (char.id === character_id ? characterData! : char))
        );

        // Update active character if it's the same character
        if (character_id === activeCharacter?.id) {
          setActiveCharacterWithUpdates(characterData);
        }
      }
    },
    [activeCharacter?.id]
  );

  // Use the generic Tauri event listener for character data updates
  const { isListening: isListeningToEvents, error: eventError } = useTauriEventListener({
    eventName: 'character-tracking-data-updated',
    handler: handleCharacterDataUpdate,
  });

  // Mutation hooks
  const createCharacterMutation = useCreateCharacter();
  const updateCharacterMutation = useUpdateCharacter();
  const deleteCharacterMutation = useDeleteCharacter();
  const setActiveCharacterMutation = useSetActiveCharacter();

  // Derived state
  const isLoading = charactersLoading || activeCharacterLoading;
  const error =
    charactersError?.message || activeCharacterError?.message || eventError || null;

  // Wrapper functions that use mutations
  const createCharacter = useCallback(
    async (data: CharacterFormData) => {
      return await createCharacterMutation.mutateAsync(data);
    },
    [createCharacterMutation]
  );

  const updateCharacter = useCallback(
    async (characterId: string, data: CharacterFormData) => {
      return await updateCharacterMutation.mutateAsync({ characterId, data });
    },
    [updateCharacterMutation]
  );

  const deleteCharacter = useCallback(
    async (characterId: string) => {
      await deleteCharacterMutation.mutateAsync(characterId);
    },
    [deleteCharacterMutation]
  );

  const setActiveCharacterId = useCallback(
    async (characterId: string) => {
      await setActiveCharacterMutation.mutateAsync(characterId);
    },
    [setActiveCharacterMutation]
  );

  // Legacy functions for backward compatibility (no-op since React Query handles caching)
  const loadCharacters = useCallback(() => {
    // No-op: React Query handles this automatically
  }, []);

  const loadActiveCharacter = useCallback(() => {
    // No-op: React Query handles this automatically
  }, []);

  // Extract tracking data from the unified character data
  const getCharacterTrackingData = useCallback(
    (character: CharacterData): CharacterTrackingData => {
      return {
        character_id: character.id,
        current_location: character.current_location,
        summary: character.summary,
        zones: character.zones,
        last_updated: character.last_updated,
      };
    },
    []
  );

  return {
    // Return characters with real-time updates
    characters: charactersWithUpdates,
    // Return active character with real-time updates
    activeCharacter: activeCharacterWithUpdates,
    // Extract tracking data from active character for backward compatibility
    activeCharacterTrackingData: activeCharacterWithUpdates
      ? getCharacterTrackingData(activeCharacterWithUpdates)
      : null,
    isLoading,
    error,
    // Event listening status
    isListeningToEvents,
    // Legacy functions for backward compatibility (no-op since React Query handles caching)
    loadCharacters,
    loadActiveCharacter,
    // CRUD operations
    createCharacter,
    updateCharacter,
    setActiveCharacterId,
    deleteCharacter,
  };
}
