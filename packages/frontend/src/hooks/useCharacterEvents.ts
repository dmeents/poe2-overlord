import { useCallback } from 'react';
import type { CharacterData } from '../types';
import { useCacheInvalidation } from './useCacheInvalidation';
import { characterQueryKeys } from './useCharacterQueries';
import { useTauriEventListener } from './useTauriEventListener';

/**
 * Hook for managing character-related events.
 * Handles real-time character data updates and event listening.
 */
export function useCharacterEvents(
  activeCharacterId: string | null,
  setCharactersWithUpdates: (
    updater: (prev: CharacterData[]) => CharacterData[]
  ) => void,
  setActiveCharacterWithUpdates: (character: CharacterData | null) => void
) {
  const { invalidateCharacterQueries, setQueryData } = useCacheInvalidation();

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
        if (character_id === activeCharacterId) {
          setActiveCharacterWithUpdates(characterData);
        }

        // Invalidate React Query cache to ensure consistency
        // Update the specific character in cache with the new data
        setQueryData(characterQueryKeys.detail(character_id) as unknown as string[], characterData);

        // Invalidate character queries to ensure all components see the update
        invalidateCharacterQueries(character_id);
      }
    },
    [
      activeCharacterId,
      setCharactersWithUpdates,
      setActiveCharacterWithUpdates,
      invalidateCharacterQueries,
      setQueryData,
    ]
  );

  // Use the generic Tauri event listener for character data updates
  const { isListening: isListeningToEvents, error: eventError } =
    useTauriEventListener({
      eventName: 'character-tracking-data-updated',
      handler: handleCharacterDataUpdate,
    });

  return {
    isListeningToEvents,
    eventError,
  };
}
