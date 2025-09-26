// Time Session History Styles
// Centralized styling utilities for the SessionHistory component

export const timeSessionHistoryStyles = {
  container: 'bg-zinc-900/50 p-4 rounded-lg border border-zinc-800',
  title: 'text-lg font-semibold text-white mb-2',
  titleWithMargin: 'text-lg font-semibold text-white mb-4',
  emptyState: 'text-center text-zinc-500 py-4',
  sessionsContainer: 'space-y-3 max-h-96 overflow-y-auto',
  sessionItem: 'p-3 bg-zinc-800/50 rounded-lg border border-zinc-700',
  sessionHeader: 'flex items-center justify-between mb-2',
  sessionInfo: 'flex items-center gap-2',
  sessionType: 'text-sm font-medium text-zinc-300',
  sessionName: 'text-white font-semibold',
  sessionDuration: 'text-sm text-emerald-400 font-mono',
  sessionGrid: 'grid grid-cols-2 gap-4 text-sm text-zinc-400',
  sessionLabel: 'block text-zinc-500 text-xs uppercase tracking-wide',
  sessionValue: 'text-white font-medium',
} as const;
