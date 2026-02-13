// DataItem Styles
// Centralized styling utilities for the DataItem component
// Uses stone color palette from theme

export const dataItemStyles = {
  container:
    'flex items-center justify-between px-4 h-12 border-l-2 border-transparent transition-all hover:bg-stone-800/70 odd:bg-stone-900/60 even:bg-stone-900/30',
  labelContainer: 'flex items-center gap-2 min-w-0',
  icon: 'flex-shrink-0 w-3.5 h-3.5 text-stone-400',
  label: 'text-stone-200 text-sm truncate',
  valueContainer: 'text-right flex-shrink-0 pl-3',
  value: 'text-stone-200 text-sm font-semibold',
  subValue: 'text-xs text-stone-400 leading-tight font-mono',
} as const;
