// Form Field Styles
// Centralized styling utilities for the FormField component
// Uses stone/bone color palette from theme

export const formFieldStyles = {
  container: 'min-h-[60px] flex flex-col justify-center',
  fieldContainer: 'flex items-center justify-between gap-8',
  label:
    'text-sm font-medium text-bone-300 flex-shrink-0 min-w-[var(--form-label-width)] flex items-center',
  childrenContainer: 'flex-1 min-w-0 max-w-sm',
  description: 'text-sm text-stone-500 ml-[calc(var(--form-label-width)+2rem)] mt-1',
  divider: 'border-b border-stone-800/50 my-2',
} as const;
