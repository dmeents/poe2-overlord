// Form Field Styles
// Centralized styling utilities for the FormField component

export const formFieldStyles = {
  container: 'min-h-[60px] flex flex-col justify-center',
  fieldContainer: 'flex items-center justify-between gap-8',
  label: 'text-sm font-medium text-zinc-300 flex-shrink-0 min-w-[220px] flex items-center',
  childrenContainer: 'flex-1 min-w-0 max-w-sm',
  description: 'text-sm text-zinc-500 ml-[calc(220px+2rem)] mt-1',
  divider: 'border-b border-zinc-800/50 my-2',
} as const;
