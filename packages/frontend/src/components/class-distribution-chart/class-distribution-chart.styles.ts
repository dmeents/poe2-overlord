// Class Distribution Chart Styles
// Centralized styling utilities for the ClassDistributionChart component

export const classDistributionChartStyles = {
  container: 'mt-6 space-y-4',
  title: 'text-sm font-medium text-zinc-300 mb-3 flex items-center',

  // Chart section
  chartSection: 'space-y-4',
  donutContainer: 'relative flex items-center justify-center h-40',
  centerText:
    'absolute inset-0 flex flex-col items-center justify-center pointer-events-none',
  centerValue: 'text-xl font-bold text-white',
  centerLabel: 'text-xs text-zinc-400 uppercase tracking-wide',

  // Legend
  legend: 'space-y-2',
  legendItem:
    'flex items-center justify-between p-2 bg-zinc-900/80 border border-zinc-700/50',
  legendColor: 'w-3 h-3 rounded-sm flex-shrink-0',
  legendInfo: 'flex-1 ml-3',
  legendName: 'text-zinc-300 font-medium text-sm',
  legendCount: 'text-zinc-400 text-xs',
  legendPercentage: 'text-zinc-400 text-xs',

  // Empty state
  emptyState: 'text-center text-zinc-500 py-6',
  emptyIcon: 'mx-auto h-8 w-8 text-zinc-600 mb-2',
  emptyTitle: 'text-sm font-medium',
  emptySubtitle: 'text-xs text-zinc-600 mt-1',
} as const;
