/**
 * Shared Modal Styling Functions
 *
 * Styling functions shared between character modals.
 * Uses stone/blood color palette from theme.
 */

export function getCharacterInfoClasses(): string {
  return 'bg-stone-900/50 p-4 rounded-lg border border-stone-700';
}

export function getCharacterInfoTextClasses(): string {
  return 'text-sm text-stone-400 space-y-1';
}

export function getCharacterInfoLabelClasses(): string {
  return 'text-stone-500';
}

export function getWarningContainerClasses(): string {
  return 'mt-4 p-3 bg-blood-500/10 border border-blood-500/20 rounded-lg';
}

export function getWarningTextClasses(): string {
  return 'text-sm text-blood-400';
}

export function getWarningStrongClasses(): string {
  return 'font-semibold';
}
