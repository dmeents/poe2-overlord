// Top Items Card Styles
// Uses stone/verdant/blood color palette from theme

export const topItemsCardStyles = {
  container: 'space-y-2',
  row: 'flex items-start justify-between text-sm p-2 hover:bg-stone-700/30',
  leftSection: 'flex items-center gap-2 flex-1 min-w-0',
  image: 'w-6 h-6 flex-shrink-0',
  nameContainer: 'flex-1 min-w-0',
  name: 'text-white truncate',
  statsRow: 'flex items-center gap-3 text-xs text-stone-400 mt-0.5',
  valueContainer: 'text-right',
  valueRow: 'text-stone-300 font-semibold flex items-center justify-end gap-1',
  valueIcon: 'w-4 h-4',
  changeContainer: 'text-xs text-stone-400 mt-1 flex justify-end',
  changePositive: 'font-semibold opacity-60 text-verdant-400',
  changeNegative: 'font-semibold opacity-60 text-blood-400',
  emptyState: 'text-stone-400 text-sm text-center py-8',
} as const;
