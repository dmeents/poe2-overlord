// Form Text Input Styles
// Centralized styling utilities for the TextInput component

export const formTextInputStyles = {
  container: 'space-y-2',
  baseInput:
    'w-full px-3 py-2 border bg-zinc-900 text-zinc-100 placeholder-zinc-600 shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500',
  validInput: 'border-zinc-700',
  invalidInput: 'border-amber-500 bg-amber-950/20',
  warningMessage: 'text-sm text-amber-500',
} as const;
