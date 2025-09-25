import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useCallback, useEffect, useState } from 'react';
import type { CharacterFormData } from '../components/character-management/character-form-modal';
import type { Character, LocationSession } from '../types';

export function useCharacterManagement() {
  const [characters, setCharacters] = useState<Character[]>([]);
  const [activeCharacter, setActiveCharacter] = useState<Character | null>(
    null
  );
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Load all characters with their last known locations
  const loadCharacters = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      const allCharacters = await invoke<Character[]>('get_all_characters');

      // Fetch last known location for each character
      const charactersWithLocations = await Promise.all(
        allCharacters.map(async character => {
          try {
            const lastLocation = await invoke<LocationSession | null>(
              'get_character_last_known_location',
              { characterId: character.id }
            );
            return {
              ...character,
              last_known_location: lastLocation || undefined,
            };
          } catch (err) {
            console.warn(
              `Failed to load location for character ${character.name}:`,
              err
            );
            return character;
          }
        })
      );

      setCharacters(charactersWithLocations);
    } catch (err) {
      setError(
        err instanceof Error ? err.message : 'Failed to load characters'
      );
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Load active character
  const loadActiveCharacter = useCallback(async () => {
    try {
      const active = await invoke<Character | null>('get_active_character');
      setActiveCharacter(active);
    } catch (err) {
      console.error('Failed to load active character:', err);
    }
  }, []);

  // Create a new character
  const createCharacter = useCallback(
    async (data: CharacterFormData) => {
      try {
        setError(null);
        const newCharacter = await invoke<Character>('create_character', {
          name: data.name,
          class: data.class,
          ascendency: data.ascendency,
          league: data.league,
          hardcore: data.hardcore,
          soloSelfFound: data.solo_self_found,
        });

        // Refresh characters list
        await loadCharacters();
        await loadActiveCharacter();

        return newCharacter;
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to create character';
        setError(errorMessage);
        throw new Error(errorMessage);
      }
    },
    [loadCharacters, loadActiveCharacter]
  );

  // Update an existing character
  const updateCharacter = useCallback(
    async (characterId: string, data: CharacterFormData) => {
      try {
        setError(null);
        const updatedCharacter = await invoke<Character>('update_character', {
          characterId,
          name: data.name,
          class: data.class,
          ascendency: data.ascendency,
          league: data.league,
          hardcore: data.hardcore,
          soloSelfFound: data.solo_self_found,
        });

        // Refresh characters list
        await loadCharacters();
        await loadActiveCharacter();

        return updatedCharacter;
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to update character';
        setError(errorMessage);
        throw new Error(errorMessage);
      }
    },
    [loadCharacters, loadActiveCharacter]
  );

  // Set active character
  const setActiveCharacterId = useCallback(
    async (characterId: string) => {
      try {
        setError(null);
        await invoke('set_active_character', { characterId });
        await loadActiveCharacter();
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to set active character';
        setError(errorMessage);
        throw new Error(errorMessage);
      }
    },
    [loadActiveCharacter]
  );

  // Delete a character
  const deleteCharacter = useCallback(
    async (characterId: string) => {
      try {
        setError(null);
        await invoke('delete_character', { characterId });

        // Refresh characters list and active character
        await loadCharacters();
        await loadActiveCharacter();
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to delete character';
        setError(errorMessage);
        throw new Error(errorMessage);
      }
    },
    [loadCharacters, loadActiveCharacter]
  );

  // Check if character name is available
  const isNameAvailable = useCallback(
    async (name: string, excludeId?: string): Promise<boolean> => {
      try {
        const isAvailable = await invoke<boolean>(
          'is_character_name_available',
          { name }
        );

        // If we're editing an existing character, check if the name is the same as the current name
        if (excludeId) {
          const currentCharacter = characters.find(c => c.id === excludeId);
          if (currentCharacter && currentCharacter.name === name) {
            return true; // Name hasn't changed, so it's available
          }
        }

        return isAvailable;
      } catch (err) {
        console.error('Failed to check name availability:', err);
        return false;
      }
    },
    [characters]
  );

  // Load data on mount
  useEffect(() => {
    loadCharacters();
    loadActiveCharacter();
  }, [loadCharacters, loadActiveCharacter]);

  // Set up real-time event listeners for character updates
  useEffect(() => {
    const unlistenFns: (() => void)[] = [];

    const setupListeners = async () => {
      try {
        // Listen for character level-up events
        const unlistenLevelUp = await listen(
          'character-level-updated',
          event => {
            console.log('Character level-up event received:', event.payload);
            const { character_name, new_level } = event.payload as {
              character_name: string;
              new_level: number;
            };

            // Update the character in the characters list by name
            setCharacters(prevCharacters =>
              prevCharacters.map(char =>
                char.name === character_name
                  ? { ...char, level: new_level }
                  : char
              )
            );

            // Update active character if it's the same character
            setActiveCharacter(prevActive =>
              prevActive && prevActive.name === character_name
                ? { ...prevActive, level: new_level }
                : prevActive
            );
          }
        );
        unlistenFns.push(unlistenLevelUp);

        // Listen for character death events
        const unlistenDeath = await listen('character-death-updated', event => {
          console.log('Character death event received:', event.payload);
          const { character_name } = event.payload as {
            character_name: string;
          };

          // Update the character in the characters list by name (increment death count)
          setCharacters(prevCharacters =>
            prevCharacters.map(char =>
              char.name === character_name
                ? { ...char, death_count: char.death_count + 1 }
                : char
            )
          );

          // Update active character if it's the same character
          setActiveCharacter(prevActive =>
            prevActive && prevActive.name === character_name
              ? { ...prevActive, death_count: prevActive.death_count + 1 }
              : prevActive
          );
        });
        unlistenFns.push(unlistenDeath);
      } catch (err) {
        console.error('Failed to set up character event listeners:', err);
      }
    };

    setupListeners();

    // Cleanup listeners
    return () => {
      unlistenFns.forEach(unlisten => unlisten());
    };
  }, []);

  return {
    characters,
    activeCharacter,
    isLoading,
    error,
    createCharacter,
    updateCharacter,
    setActiveCharacterId,
    deleteCharacter,
    isNameAvailable,
    refreshCharacters: loadCharacters,
    refreshActiveCharacter: loadActiveCharacter,
  };
}
