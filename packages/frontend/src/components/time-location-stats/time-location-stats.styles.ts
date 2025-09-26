// Time Location Stats Styles
// Centralized styling utilities for the LocationStats component

export const timeLocationStatsStyles = {
  container: 'bg-zinc-900/50 p-4 rounded-lg border border-zinc-800',
  title: 'text-lg font-semibold text-white mb-2',
  titleWithMargin: 'text-lg font-semibold text-white mb-4',
  emptyState: 'text-center text-zinc-500 py-4',
  statsContainer: 'space-y-3',
  statItem: 'p-3 bg-zinc-800/50 rounded-lg border border-zinc-700',
  statHeader: 'flex items-center justify-between mb-2',
  statInfo: 'flex items-center gap-2',
  statType: 'text-sm font-medium text-zinc-300',
  statName: 'text-white font-semibold',
  statTime: 'text-sm text-emerald-400 font-mono',
  statGrid: 'grid grid-cols-3 gap-4 text-sm text-zinc-400',
  statLabel: 'block text-zinc-500 text-xs uppercase tracking-wide',
  statValue: 'text-white font-medium',
} as const;
