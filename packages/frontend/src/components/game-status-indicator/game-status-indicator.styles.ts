// Game Status Indicator Styles
// Centralized styling utilities for the GameStatusIndicator component

export const gameStatusIndicatorStyles = {
  container: 'bg-zinc-900/50 p-6 rounded-lg border border-zinc-800',
  title: 'text-lg font-semibold text-white mb-4',
  statusContainer: 'space-y-4',
  statusItem: 'flex items-center gap-2 mb-2',
  statusDot: 'w-2 h-2 rounded-full',
  statusDotOnline: 'bg-green-500',
  statusDotOffline: 'bg-red-500',
  statusText: 'text-white text-sm font-medium',
  statusDetails: 'text-zinc-400 text-xs space-y-1',
  loadingText: 'text-zinc-400 text-xs',
  divider: 'pt-2 border-t border-zinc-700',
} as const;
