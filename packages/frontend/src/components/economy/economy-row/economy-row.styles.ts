// Economy Row Styles
// Uses stone/verdant/blood color palette from theme

export const economyRowStyles = {
  container:
    'flex items-center justify-between py-3 px-6 border-b border-stone-700/50 transition-colors',
  containerClickable: 'cursor-pointer hover:bg-stone-800/30',
  leftSection: 'flex items-center gap-2 flex-1 min-w-0',
  starButton: 'flex-shrink-0 p-0.5 rounded transition-colors focus:outline-none',
  starActive: 'text-amber-400 hover:text-amber-300',
  starInactive: 'text-stone-600 hover:text-stone-400',
  image: 'w-8 h-8 flex-shrink-0',
  nameContainer: 'flex-1 min-w-0',
  name: 'text-white font-medium truncate',
  statsRow: 'flex items-center gap-3 text-xs text-stone-400 mt-1',
  valueContainer: 'text-right',
  valueRow: 'flex items-center gap-2 justify-end text-base font-semibold text-stone-200',
  valueIcon: 'w-5 h-5',
  exchangeIcon: 'w-4 h-4 text-stone-500',
  changeContainer: 'text-xs text-stone-400 mt-1 flex justify-end',
  changePositive: 'font-semibold opacity-60 text-verdant-400',
  changeNegative: 'font-semibold opacity-60 text-blood-400',
  // Tooltip styles
  tooltipContainer: 'space-y-1 text-xs',
  tooltipHeader: 'font-semibold border-b border-stone-600 pb-1 mb-2',
  tooltipRow: 'flex justify-between gap-4',
  tooltipLabel: 'text-stone-400',
  tooltipValue: 'text-white font-mono',
} as const;
