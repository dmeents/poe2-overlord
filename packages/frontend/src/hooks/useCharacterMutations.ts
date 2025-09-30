import { useCallback } from 'react';
import type { CharacterFormData } from '../components/character/character-form-modal/character-form-modal';
import {
  useCreateCharacter,
  useDeleteCharacter,
  useSetActiveCharacter,
  useUpdateCharacter,
} from './useCharacterQueries';

/**
 * Hook for managing character CRUD operations.
 * Provides mutation functions with proper error handling and optimistic updates.
 */
export function useCharacterMutations() {
  // Mutation hooks
  const createCharacterMutation = useCreateCharacter();
  const updateCharacterMutation = useUpdateCharacter();
  const deleteCharacterMutation = useDeleteCharacter();
  const setActiveCharacterMutation = useSetActiveCharacter();

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

  return {
    // CRUD operations
    createCharacter,
    updateCharacter,
    setActiveCharacterId,
    deleteCharacter,
    // Mutation states for advanced usage
    createCharacterMutation,
    updateCharacterMutation,
    deleteCharacterMutation,
    setActiveCharacterMutation,
  };
}
