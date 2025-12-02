// Tooltip Styles
// Centralized styling utilities for the Tooltip component

export const tooltipStyles = {
  container: 'relative inline-block',
  trigger: 'inline-flex items-center gap-1 cursor-pointer',
  icon: 'w-4 h-4 text-zinc-400 hover:text-zinc-300 transition-colors',
  tooltip:
    'absolute z-10 w-80 p-3 bg-zinc-800 border border-zinc-700 text-zinc-200 text-sm shadow-lg -top-2 left-1/2 transform -translate-x-1/2 -translate-y-full',
  content: 'relative',
  arrow:
    'absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-zinc-800',
} as const;
