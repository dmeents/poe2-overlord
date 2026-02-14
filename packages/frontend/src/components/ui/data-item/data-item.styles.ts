// DataItem Styles
// Centralized styling utilities for the DataItem component
// Uses stone color palette from theme

export const dataItemStyles = {
  container:
    'flex items-center justify-between px-4 py-2.5 transition-colors hover:bg-stone-800/20',
  containerColored: 'border-l-2 pl-3.5',
  labelContainer: 'flex items-center gap-2 min-w-0',
  icon: 'flex-shrink-0 w-3.5 h-3.5 text-stone-400',
  label: 'text-stone-400 text-sm truncate',
  valueContainer: 'text-right flex-shrink-0 pl-3',
  value: 'text-stone-100 text-sm font-semibold',
  subValue: 'text-xs text-stone-500 leading-tight font-mono',
} as const;
