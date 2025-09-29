// Act Distribution Chart Styles
// Centralized styling utilities for the ActDistributionChart component

export const actDistributionChartStyles = {
  container: 'bg-zinc-800/50 border border-zinc-700/50 p-6 shadow-lg',
  title: 'text-lg font-semibold text-white mb-6 flex items-center',

  // Chart section
  chartSection: 'space-y-6',
  donutContainer: 'relative flex items-center justify-center',
  donutSvg: 'w-32 h-32 transform -rotate-90',
  segment: 'transition-all duration-300 hover:opacity-80',
  centerText: 'absolute inset-0 flex flex-col items-center justify-center',
  centerValue: 'text-2xl font-bold text-white',
  centerLabel: 'text-xs text-zinc-400 uppercase tracking-wide',

  // Legend (matching insights card pattern)
  legend: 'space-y-2',
  legendItem:
    'flex items-center justify-between p-3 bg-zinc-900/80 border border-zinc-700/50',
  legendColor: 'w-3 h-3 rounded-sm flex-shrink-0',
  legendInfo: 'flex-1 ml-3',
  legendName: 'text-zinc-300 font-medium',
  legendTime: 'text-zinc-400 text-sm',
  legendPercentage: 'text-zinc-400 text-sm',

  // Total time item (special styling for the total row)
  totalTimeItem:
    'flex items-center justify-between p-3 bg-zinc-900/80 border border-zinc-700/50 font-medium',
  totalTimeInfo: 'flex-1',
  totalTimeName: 'text-zinc-200 font-medium',
  totalTimeValue: 'text-zinc-300 font-semibold',

  // Empty state
  emptyState: 'text-center text-zinc-500 py-8',
  emptyIcon: 'mx-auto h-8 w-8 text-zinc-600 mb-2',
  emptyTitle: 'text-sm font-medium',
  emptySubtitle: 'text-xs text-zinc-600 mt-1',
} as const;
