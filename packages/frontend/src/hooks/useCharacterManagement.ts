import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useCallback, useEffect, useRef, useState } from 'react';
import type { CharacterFormData } from '../components/character-modals/character-form-modal';
import type { Character } from '../types';
import type { CharacterTrackingData } from '../types/character-tracking';
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

  // State for characters with tracking data
  const [charactersWithTracking, setCharactersWithTracking] = useState<
    Character[]
  >([]);
  const [isLoadingTracking, setIsLoadingTracking] = useState(false);

  // Event listener management
  const listenerRef = useRef<(() => void) | null>(null);
  const isListeningRef = useRef(false);

  // Fetch tracking data for all characters
  useEffect(() => {
    const fetchTrackingData = async () => {
      if (characters.length === 0) {
        setCharactersWithTracking([]);
        return;
      }

      setIsLoadingTracking(true);
      try {
        const charactersWithData = await Promise.all(
          characters.map(async character => {
            try {
              const trackingData = await invoke<CharacterTrackingData | null>(
                'get_character_tracking_data',
                { characterId: character.id }
              );
              return {
                ...character,
                trackingData: trackingData || undefined,
              };
            } catch (err) {
              console.error(
                `Failed to fetch tracking data for character ${character.id}:`,
                err
              );
              return character; // Return character without tracking data
            }
          })
        );
        setCharactersWithTracking(charactersWithData);
      } catch (err) {
        console.error('Failed to fetch character tracking data:', err);
        setCharactersWithTracking(characters); // Fallback to characters without tracking data
      } finally {
        setIsLoadingTracking(false);
      }
    };

    fetchTrackingData();
  }, [characters]);

  const {
    data: activeCharacter = null,
    isLoading: activeCharacterLoading,
    error: activeCharacterError,
  } = useActiveCharacter();

  // State for active character's real-time tracking data
  const [activeCharacterTrackingData, setActiveCharacterTrackingData] =
    useState<CharacterTrackingData | null>(null);

  // Event handler for character tracking data updates
  const handleTrackingDataUpdate = useCallback(
    (event: { payload: unknown }) => {
      // The backend sends the entire AppEvent enum, so we need to extract the data
      let character_id: string | undefined;
      let eventData: CharacterTrackingData | undefined;

      if (event.payload && typeof event.payload === 'object') {
        // Check if it's the CharacterTrackingDataUpdated variant
        const payload = event.payload as Record<string, unknown>;
        if (payload.CharacterTrackingDataUpdated) {
          const trackingEvent = payload.CharacterTrackingDataUpdated as {
            character_id: string;
            data: CharacterTrackingData;
          };
          character_id = trackingEvent.character_id;
          eventData = trackingEvent.data;
        }
      }

      // Only update if this is for the active character
      if (character_id === activeCharacter?.id && eventData) {
        setActiveCharacterTrackingData(eventData);

        // Also update the character in the characters list
        setCharactersWithTracking(prev =>
          prev.map(char =>
            char.id === character_id
              ? { ...char, trackingData: eventData }
              : char
          )
        );
      }
    },
    [activeCharacter?.id]
  );

  // Set up event listener for active character's tracking data
  useEffect(() => {
    // Clean up existing listener
    if (listenerRef.current) {
      listenerRef.current();
      listenerRef.current = null;
    }

    if (!activeCharacter?.id) {
      setActiveCharacterTrackingData(null);
      return;
    }

    // Prevent multiple listeners
    if (isListeningRef.current) {
      return;
    }

    isListeningRef.current = true;

    listen('character-tracking-data-updated', handleTrackingDataUpdate)
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
  }, [activeCharacter?.id, handleTrackingDataUpdate]);

  // Mutation hooks
  const createCharacterMutation = useCreateCharacter();
  const updateCharacterMutation = useUpdateCharacter();
  const deleteCharacterMutation = useDeleteCharacter();
  const setActiveCharacterMutation = useSetActiveCharacter();

  // Derived state
  const isLoading =
    charactersLoading || activeCharacterLoading || isLoadingTracking;
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

  // Get the active character with real-time tracking data
  const activeCharacterWithTracking = activeCharacter
    ? {
        ...activeCharacter,
        trackingData:
          activeCharacterTrackingData || activeCharacter.trackingData,
      }
    : null;

  return {
    characters: charactersWithTracking,
    activeCharacter: activeCharacterWithTracking,
    activeCharacterTrackingData,
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
