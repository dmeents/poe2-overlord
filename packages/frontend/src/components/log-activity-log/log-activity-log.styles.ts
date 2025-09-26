// Log Activity Log Styles
// Centralized styling utilities for the ActivityLog component

export const logActivityLogStyles = {
  container: 'bg-zinc-900/50 p-4 border border-zinc-800',
  header: 'flex items-center justify-between mb-4',
  title: 'text-lg font-semibold text-white',
  eventsContainer: 'space-y-2 max-h-[64rem] overflow-y-auto',
  emptyState: 'text-center text-zinc-500 py-8',
} as const;
