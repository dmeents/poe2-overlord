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
