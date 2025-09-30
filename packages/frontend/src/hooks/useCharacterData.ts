import { useCallback, useEffect, useState } from 'react';
import type { CharacterData } from '../types';
import type { CharacterTrackingData } from '../types/character';
import {
  useActiveCharacter,
  useCharacters,
} from './useCharacterQueries';

/**
 * Hook for managing character data fetching and state.
 * Handles data loading, real-time updates, and derived state.
 */
export function useCharacterData() {
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

  // Derived state
  const isLoading = charactersLoading || activeCharacterLoading;
  const error = charactersError?.message || activeCharacterError?.message || null;

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

  // Legacy functions for backward compatibility (no-op since React Query handles caching)
  const loadCharacters = useCallback(() => {
    // No-op: React Query handles this automatically
  }, []);

  const loadActiveCharacter = useCallback(() => {
    // No-op: React Query handles this automatically
  }, []);

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
    // Legacy functions for backward compatibility
    loadCharacters,
    loadActiveCharacter,
    // Helper functions
    getCharacterTrackingData,
    // State setters for real-time updates (used by event hook)
    setCharactersWithUpdates,
    setActiveCharacterWithUpdates,
  };
}
