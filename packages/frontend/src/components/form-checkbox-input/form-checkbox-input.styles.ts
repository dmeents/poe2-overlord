// Form Checkbox Input Styles
// Centralized styling utilities for the CheckboxInput component

export const formCheckboxInputStyles = {
  container: 'min-h-[60px] flex flex-col justify-center',
  fieldContainer: 'flex items-center justify-between gap-8',
  label:
    'text-sm font-medium text-zinc-300 flex-shrink-0 min-w-[220px] flex items-center',
  inputContainer: 'flex-1 min-w-0 max-w-sm flex justify-end items-center',
  checkbox:
    'h-4 w-4 text-blue-500 focus:ring-blue-500 border-zinc-600 bg-zinc-900 rounded',
  description: 'text-sm text-zinc-500 ml-[calc(220px+2rem)] mt-1',
  divider: 'border-b border-zinc-800/50 my-2',
} as const;
