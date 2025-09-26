// Log Recent Log Lines Styles
// Centralized styling utilities for the RecentLogLines component

export const logRecentLogLinesStyles = {
  container: 'bg-zinc-900/50 p-4 border border-zinc-800',
  header: 'flex items-center justify-between mb-3',
  title: 'text-lg font-semibold text-white',
  logLinesContainer: 'space-y-1 max-h-32 overflow-auto',
  logLine: 'text-sm text-zinc-300 font-mono',
  emptyState: 'text-zinc-500 text-sm',
} as const;
