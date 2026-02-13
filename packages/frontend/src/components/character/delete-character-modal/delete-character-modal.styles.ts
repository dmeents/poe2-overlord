/**
 * Delete Character Modal Styling Functions
 *
 * This file contains styling-related functions and constants
 * for the DeleteCharacterModal component.
 * Uses stone/blood color palette from theme.
 */

/**
 * Get warning icon classes
 */
export function getWarningIconClasses(): string {
  return 'h-6 w-6 text-blood-400';
}

/**
 * Get modal content container classes
 */
export function getModalContentClasses(): string {
  return 'mb-6';
}

/**
 * Get character info display container classes
 */
export function getCharacterInfoClasses(): string {
  return 'bg-stone-900/50 p-4 rounded-lg border border-stone-700';
}

/**
 * Get character info text classes
 */
export function getCharacterInfoTextClasses(): string {
  return 'text-sm text-stone-400 space-y-1';
}

/**
 * Get character info label classes
 */
export function getCharacterInfoLabelClasses(): string {
  return 'text-stone-500';
}

/**
 * Get warning container classes
 */
export function getWarningContainerClasses(): string {
  return 'mt-4 p-3 bg-blood-500/10 border border-blood-500/20 rounded-lg';
}

/**
 * Get warning text classes
 */
export function getWarningTextClasses(): string {
  return 'text-sm text-blood-400';
}

/**
 * Get warning strong text classes
 */
export function getWarningStrongClasses(): string {
  return 'font-semibold';
}

/**
 * Get modal actions container classes
 */
export function getModalActionsClasses(): string {
  return 'flex justify-end gap-3';
}

/**
 * Get delete button classes
 */
export function getDeleteButtonClasses(): string {
  return 'text-blood-400 hover:text-blood-300 hover:border-blood-400';
}
