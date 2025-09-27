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

/**
 * Get empty state container classes
 */
export function getEmptyStateClasses(): string {
  return 'text-center py-12';
}

/**
 * Get empty state icon container classes
 */
export function getEmptyStateIconClasses(): string {
  return 'text-zinc-400 mb-4';
}

/**
 * Get empty state icon classes
 */
export function getEmptyStateIconSvgClasses(): string {
  return 'mx-auto h-12 w-12';
}

/**
 * Get empty state title classes
 */
export function getEmptyStateTitleClasses(): string {
  return 'text-lg font-medium text-white mb-2';
}

/**
 * Get empty state description classes
 */
export function getEmptyStateDescriptionClasses(): string {
  return 'text-zinc-400 mb-6';
}
