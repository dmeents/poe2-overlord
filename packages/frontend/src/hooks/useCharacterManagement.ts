import { useCharacterData } from './useCharacterData';
import { useCharacterEvents } from './useCharacterEvents';
import { useCharacterMutations } from './useCharacterMutations';

/**
 * Main character management hook that composes focused hooks.
 * Provides a unified interface for character data, mutations, and events.
 */
export function useCharacterManagement() {
  // Use the focused hooks
  const characterData = useCharacterData();
  const characterMutations = useCharacterMutations();
  
  // Use character events with the data hook's state setters
  const characterEvents = useCharacterEvents(
    characterData.activeCharacter?.id || null,
    characterData.setCharactersWithUpdates,
    characterData.setActiveCharacterWithUpdates
  );

  // Combine error states
  const error = characterData.error || characterEvents.eventError || null;

  return {
    // Character data
    characters: characterData.characters,
    activeCharacter: characterData.activeCharacter,
    activeCharacterTrackingData: characterData.activeCharacterTrackingData,
    isLoading: characterData.isLoading,
    error,
    
    // Event listening status
    isListeningToEvents: characterEvents.isListeningToEvents,
    
    // Legacy functions for backward compatibility
    loadCharacters: characterData.loadCharacters,
    loadActiveCharacter: characterData.loadActiveCharacter,
    
    // CRUD operations
    createCharacter: characterMutations.createCharacter,
    updateCharacter: characterMutations.updateCharacter,
    setActiveCharacterId: characterMutations.setActiveCharacterId,
    deleteCharacter: characterMutations.deleteCharacter,
  };
}
