export const playtimeInsightsStyles = {
  container: 'bg-zinc-800/50 border border-zinc-700/50 p-6 shadow-lg',
  title: 'text-lg font-semibold text-white mb-6 flex items-center',

  // Grid layout
  grid: 'grid grid-cols-2 gap-4 mb-6',

  // Stat items
  statItem: 'text-center p-4 bg-zinc-900/80 border border-zinc-700/50',
  statValue: 'text-2xl font-bold text-white mb-1',
  statLabel: 'text-sm text-zinc-400 uppercase tracking-wide',

  // Loading states
  loadingContainer: 'animate-pulse',
  loadingItem: 'h-4 bg-zinc-700 rounded mb-2',
  loadingValue: 'h-6 bg-zinc-700 rounded',

  // Empty state
  emptyState: 'text-center py-8',
  emptyStateSubtext: 'text-sm text-zinc-500 mt-2',

  // Efficiency section
  efficiencySection: 'mt-6 space-y-4',
  efficiencyTitle: 'text-sm font-medium text-zinc-300 mb-3 flex items-center',
  efficiencyGrid: 'space-y-2',
  efficiencyItem:
    'flex items-center justify-between p-3 bg-zinc-900/80 border border-zinc-700/50',
  efficiencyLabel: 'text-zinc-300 font-medium',
  efficiencyValue: 'text-zinc-400 text-sm',

  // Location section
  locationSection: 'mt-6 space-y-4',
  locationTitle: 'text-sm font-medium text-zinc-300 mb-3 flex items-center',
  locationItem:
    'flex items-center justify-between p-3 bg-zinc-900/80 border border-zinc-700/50',
  locationName: 'text-zinc-300 font-medium truncate',
  locationTime: 'text-zinc-400 text-sm',
} as const;
