import type { CharacterFormData } from '../components/character-management/character-form-modal';
import type { Character } from '../types';

/**
 * Default values for character form fields
 */
export const DEFAULT_CHARACTER_FORM_VALUES: CharacterFormData = {
  name: '',
  class: 'Warrior',
  ascendency: 'Titan',
  league: 'Standard',
  hardcore: false,
  solo_self_found: false,
};

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
 */
export function getDefaultFormData(character?: Character): CharacterFormData {
  return {
    name: character?.name || DEFAULT_CHARACTER_FORM_VALUES.name,
    class: character?.class || DEFAULT_CHARACTER_FORM_VALUES.class,
    ascendency:
      character?.ascendency || DEFAULT_CHARACTER_FORM_VALUES.ascendency,
    league: character?.league || DEFAULT_CHARACTER_FORM_VALUES.league,
    hardcore: character?.hardcore || DEFAULT_CHARACTER_FORM_VALUES.hardcore,
    solo_self_found:
      character?.solo_self_found ||
      DEFAULT_CHARACTER_FORM_VALUES.solo_self_found,
  };
}
