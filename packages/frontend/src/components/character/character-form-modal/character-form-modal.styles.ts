/**
 * Character Form Modal Styling Functions
 *
 * This file contains styling-related functions and constants
 * for the CharacterFormModal component.
 */

/**
 * Get form field container classes
 */
export function getFormFieldClasses(): string {
  return 'space-y-4';
}

/**
 * Get form actions container classes
 */
export function getFormActionsClasses(): string {
  return 'flex justify-end gap-3 pt-4 border-t border-zinc-700';
}

/**
 * Get character info display container classes
 */
export function getCharacterInfoClasses(): string {
  return 'bg-zinc-900/50 p-4 rounded-lg border border-zinc-700';
}

/**
 * Get character info text classes
 */
export function getCharacterInfoTextClasses(): string {
  return 'text-sm text-zinc-400 space-y-1';
}

/**
 * Get character info label classes
 */
export function getCharacterInfoLabelClasses(): string {
  return 'text-zinc-500';
}

/**
 * Get warning container classes
 */
export function getWarningContainerClasses(): string {
  return 'mt-4 p-3 bg-red-500/10 border border-red-500/20 rounded-lg';
}

/**
 * Get warning text classes
 */
export function getWarningTextClasses(): string {
  return 'text-sm text-red-400';
}

/**
 * Get warning strong text classes
 */
export function getWarningStrongClasses(): string {
  return 'font-semibold';
}
