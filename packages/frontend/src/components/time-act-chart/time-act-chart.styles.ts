// Time Act Chart Styles
// Centralized styling utilities for the ActTimeChart component

export const timeActChartStyles = {
  container: 'bg-zinc-900/50 p-4 rounded-lg border border-zinc-800',
  title: 'text-lg font-semibold text-white mb-2',
  header: 'flex items-center justify-between mb-4',
  totalTime: 'text-sm text-zinc-400',
  chartContainer: 'space-y-3',
  actItem: 'group',
  actHeader: 'flex items-center justify-between mb-2',
  actInfo: 'flex items-center gap-2',
  actColor: 'w-3 h-3 rounded-sm',
  actName: 'text-white font-medium',
  actTime: 'flex items-center gap-3 text-sm',
  actTimeValue: 'text-zinc-300 font-mono',
  progressContainer: 'relative',
  progressBar: 'w-full h-2 bg-zinc-800 rounded-sm overflow-hidden',
  progressFill: 'h-full transition-all duration-500 ease-out',
  tooltip:
    'absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-200 pointer-events-none',
  tooltipContent:
    'absolute top-0 left-1/2 transform -translate-y-full -translate-x-1/2 bg-zinc-800 border border-zinc-700 rounded px-2 py-1 text-xs text-white whitespace-nowrap z-10',
  tooltipTitle: 'font-medium',
  tooltipTime: 'text-zinc-300',
  tooltipDetails: 'text-zinc-400',
  emptyState: 'text-center text-zinc-500 py-8',
  emptyIcon: 'mx-auto h-8 w-8 text-zinc-600',
  emptyTitle: 'text-sm',
  emptySubtitle: 'text-xs text-zinc-600 mt-1',
  actColors: [
    'bg-emerald-500',
    'bg-blue-500',
    'bg-purple-500',
    'bg-amber-500',
    'bg-red-500',
    'bg-cyan-500',
    'bg-pink-500',
    'bg-indigo-500',
    'bg-orange-500',
    'bg-teal-500',
  ],
} as const;
