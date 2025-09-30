import type { CharacterFormData } from '../components/character/character-form-modal/character-form-modal';
import type { CharacterData } from '../types';

/**
 * Validation rules for character form fields
 */
export const CHARACTER_FORM_VALIDATION = {
  name: {
    required: true,
    minLength: 2,
    maxLength: 50,
    messages: {
      required: 'Character name is required',
      minLength: 'Character name must be at least 2 characters',
      maxLength: 'Character name must be less than 50 characters',
    },
  },
} as const;
/**
 * Get default form data, optionally populated from an existing character
 * Note: This function is now deprecated in favor of useCharacterConfig.getDefaultFormData()
 * which uses dynamic data from the backend
 */
export function getDefaultFormData(
  character?: CharacterData
): CharacterFormData {
  return {
    name: character?.name || '',
    class: character?.class || 'Warrior',
    ascendency: character?.ascendency || 'Titan',
    league: character?.league || 'Standard',
    hardcore: character?.hardcore || false,
    solo_self_found: character?.solo_self_found || false,
  };
}
