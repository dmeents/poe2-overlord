export const starredCurrenciesCardStyles = {
  emptyState: 'text-center py-6 text-stone-500 text-sm',
  row: 'flex items-center justify-between py-2 px-4 border-b border-stone-700/30 last:border-b-0',
  leftSection: 'flex items-center gap-2 flex-1 min-w-0',
  image: 'w-6 h-6 flex-shrink-0',
  name: 'text-stone-300 text-sm truncate',
  volumeRow: 'text-xs text-stone-400 mt-0.5',
  rightSection: 'flex flex-col items-end gap-0.5',
  valueRow: 'flex items-center gap-1.5 text-sm font-semibold text-stone-200',
  valueIcon: 'w-4 h-4',
  changePositive: 'text-xs font-semibold opacity-60 text-verdant-400',
  changeNegative: 'text-xs font-semibold opacity-60 text-blood-400',
} as const;
