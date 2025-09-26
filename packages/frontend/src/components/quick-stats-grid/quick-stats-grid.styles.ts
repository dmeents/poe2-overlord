// Quick Stats Grid Styles
// Centralized styling utilities for the QuickStatsGrid component

export const quickStatsGridStyles = {
  container: 'bg-zinc-900/50 p-6 rounded-lg border border-zinc-800',
  title: 'text-lg font-semibold text-white mb-4',
  grid: 'grid grid-cols-2 md:grid-cols-4 gap-4',
  loadingContainer: 'animate-pulse',
  loadingItem: 'h-4 bg-zinc-700 rounded mb-2',
  loadingValue: 'h-6 bg-zinc-700 rounded',
  statItem: 'text-center',
  statLabel: 'text-zinc-400 text-xs mb-1',
  statValue: 'text-white text-lg font-semibold',
  statValueSmall: 'text-white text-sm font-medium truncate',
  statSubtext: 'text-zinc-400 text-xs',
} as const;
