// Tooltip Styles
// Centralized styling utilities for the Tooltip component
// Uses stone color palette from theme

export const tooltipStyles = {
  container: 'relative inline-block',
  trigger: 'inline-flex items-center gap-1 cursor-pointer',
  icon: 'w-4 h-4 text-stone-400 hover:text-stone-300 transition-colors',
  // z-20: Tooltips (see patterns.md for z-index scale)
  tooltip:
    'z-20 w-80 p-3 bg-stone-800 border border-stone-700 text-stone-200 text-sm card-shadow pointer-events-none rounded',
  content: 'relative',
  arrow:
    'absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-stone-800',
} as const;
