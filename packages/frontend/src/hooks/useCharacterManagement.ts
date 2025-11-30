import { listen } from '@tauri-apps/api/event';
import { useCallback, useEffect, useRef, useState } from 'react';
import type { CharacterFormData } from '../components/character/character-form-modal/character-form-modal';
import type { CharacterData } from '../types/character';
import type { CharacterTrackingData } from '../types/character';
import {
  useActiveCharacter,
  useCharacters,
  useCreateCharacter,
  useDeleteCharacter,
  useSetActiveCharacter,
  useUpdateCharacter,
} from './useCharacterQueries';

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
  >([]);
  const [activeCharacterWithUpdates, setActiveCharacterWithUpdates] =
    useState<CharacterData | null>(null);

  // Event listener management
  const listenerRef = useRef<(() => void) | null>(null);
  const isListeningRef = useRef(false);

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

  // Set up event listener for character data updates
  useEffect(() => {
    // Clean up existing listener
    if (listenerRef.current) {
      listenerRef.current();
      listenerRef.current = null;
    }

    // Prevent multiple listeners
    if (isListeningRef.current) {
      return;
    }

    isListeningRef.current = true;

    listen('character-tracking-data-updated', handleCharacterDataUpdate)
      .then(unlisten => {
        listenerRef.current = unlisten;
      })
      .catch(error => {
        console.error('Failed to set up event listener:', error);
        isListeningRef.current = false;
      });

    return () => {
      if (listenerRef.current) {
        listenerRef.current();
        listenerRef.current = null;
      }
      isListeningRef.current = false;
    };
  }, [handleCharacterDataUpdate]);

  // Mutation hooks
  const createCharacterMutation = useCreateCharacter();
  const updateCharacterMutation = useUpdateCharacter();
  const deleteCharacterMutation = useDeleteCharacter();
  const setActiveCharacterMutation = useSetActiveCharacter();

  // Derived state
  const isLoading = charactersLoading || activeCharacterLoading;
  const error =
    charactersError?.message || activeCharacterError?.message || null;

  // Wrapper functions that use mutations
  const createCharacter = useCallback(
    async (data: CharacterFormData) => {
      try {
        return await createCharacterMutation.mutateAsync(data);
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to create character';
        throw new Error(errorMessage);
      }
    },
    [createCharacterMutation]
  );

  const updateCharacter = useCallback(
    async (characterId: string, data: CharacterFormData) => {
      try {
        return await updateCharacterMutation.mutateAsync({ characterId, data });
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to update character';
        throw new Error(errorMessage);
      }
    },
    [updateCharacterMutation]
  );

  const deleteCharacter = useCallback(
    async (characterId: string) => {
      try {
        await deleteCharacterMutation.mutateAsync(characterId);
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to delete character';
        throw new Error(errorMessage);
      }
    },
    [deleteCharacterMutation]
  );

  const setActiveCharacterId = useCallback(
    async (characterId: string) => {
      try {
        await setActiveCharacterMutation.mutateAsync(characterId);
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to set active character';
        throw new Error(errorMessage);
      }
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
    loadCharacters,
    loadActiveCharacter,
    createCharacter,
    updateCharacter,
    setActiveCharacterId,
    deleteCharacter,
  };
}
