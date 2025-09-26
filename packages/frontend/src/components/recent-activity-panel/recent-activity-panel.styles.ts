// Recent Activity Panel Styles
// Centralized styling utilities for the RecentActivityPanel component

export const recentActivityPanelStyles = {
  container: 'bg-zinc-900/50 p-6 rounded-lg border border-zinc-800',
  title: 'text-lg font-semibold text-white mb-4',
  contentContainer: 'space-y-4',
  loadingContainer: 'animate-pulse space-y-3',
  loadingItem: 'h-4 bg-zinc-700 rounded',
  loadingItemWide: 'h-4 bg-zinc-700 rounded w-3/4',
  loadingItemMedium: 'h-4 bg-zinc-700 rounded w-1/2',
  loadingItemNarrow: 'h-4 bg-zinc-700 rounded w-2/3',
  currentStatus: 'text-zinc-400 text-xs mb-2',
  currentStatusItem: 'space-y-1',
  currentStatusText: 'text-white text-sm',
  currentStatusValue: 'font-medium',
  currentStatusTime: 'text-zinc-400 text-xs ml-2',
  recentSessions: 'text-zinc-400 text-xs mb-2',
  sessionsList: 'space-y-2',
  sessionItem: 'text-xs',
  sessionLocation: 'text-white',
  sessionDuration: 'text-zinc-400 ml-2',
  sessionTime: 'text-zinc-500',
  emptyState: 'text-zinc-400 text-sm text-center py-4',
  emptyStateSubtext: 'text-xs mt-1',
} as const;
