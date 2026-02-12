/**
 * Character List Styling Functions
 *
 * This file contains styling-related functions and constants
 * for the CharacterList components.
 */

/**
 * Get main list container classes
 */
export function getListContainerClasses(): string {
  return 'space-y-4';
}

/**
 * Get character grid classes
 */
export function getCharacterGridClasses(): string {
  return 'grid grid-cols-1 gap-3';
}

/**
 * Get list header container classes
 */
export function getListHeaderClasses(): string {
  return 'flex justify-between items-center';
}

/**
 * Get list header title classes
 */
export function getListHeaderTitleClasses(): string {
  return 'text-xl font-semibold text-white';
}
