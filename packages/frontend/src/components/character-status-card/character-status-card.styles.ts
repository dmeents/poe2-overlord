// Character Status Card Styles
// Centralized styling utilities for the CharacterStatusCard component

export const characterStatusCardStyles = {
  container: 'bg-zinc-900/50 p-6 rounded-lg border border-zinc-800',
  loadingContainer: 'animate-pulse',
  loadingTitle: 'h-6 bg-zinc-700 rounded mb-3 w-3/4',
  loadingSubtitle: 'h-4 bg-zinc-700 rounded mb-2 w-1/2',
  loadingText: 'h-4 bg-zinc-700 rounded w-2/3',
  title: 'text-lg font-semibold text-white mb-3',
  emptyState: 'text-zinc-400 text-sm',
  emptyStateSubtext: 'mt-2 text-xs mb-4',
} as const;
