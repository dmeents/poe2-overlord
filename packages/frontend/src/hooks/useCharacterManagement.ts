import { invoke } from '@tauri-apps/api/core';
import { useCallback, useEffect, useState } from 'react';
import type { CharacterFormData } from '../components/character-modals/character-form-modal';
import type { Character } from '../types';

export function useCharacterManagement() {
  const [characters, setCharacters] = useState<Character[]>([]);
  const [activeCharacter, setActiveCharacter] = useState<Character | null>(
    null
  );
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Load all characters
  const loadCharacters = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      const allCharacters = await invoke<Character[]>('get_all_characters');
      setCharacters(allCharacters);
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Failed to load characters';
      setError(errorMessage);
      console.error('Failed to load characters:', err);
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
    async (data: CharacterFormData): Promise<Character> => {
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

        // Refresh characters list and active character
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
    async (
      characterId: string,
      data: CharacterFormData
    ): Promise<Character> => {
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

        // Refresh characters list and active character
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
    async (characterId: string): Promise<void> => {
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
    async (characterId: string): Promise<void> => {
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

  // Load data on mount
  useEffect(() => {
    loadCharacters();
    loadActiveCharacter();
  }, [loadCharacters, loadActiveCharacter]);

  return {
    characters,
    activeCharacter,
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
