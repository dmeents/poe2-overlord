// Accordion Styles
// Centralized styling utilities for the Accordion component
// Uses stone color palette from theme

export const accordionStyles = {
  container: 'bg-stone-900/80 border border-stone-700/50',
  header: 'bg-stone-700/50 border-b border-stone-700/50',
  button:
    'flex items-center justify-between w-full text-left hover:text-white transition-colors cursor-pointer p-3',
  title: 'text-base font-semibold text-white',
  subtitle: 'text-xs text-stone-400',
  icon: 'w-4 h-4 text-stone-400',
  content: 'p-3',
} as const;
