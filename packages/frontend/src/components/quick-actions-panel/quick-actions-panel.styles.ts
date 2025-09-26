// Quick Actions Panel Styles
// Centralized styling utilities for the QuickActionsPanel component

export const quickActionsPanelStyles = {
  container: 'bg-zinc-900/50 p-6 rounded-lg border border-zinc-800',
  title: 'text-lg font-semibold text-white mb-6',
  actionsContainer: 'space-y-4',
  primaryActions: 'space-y-3',
  actionButton: 'w-full justify-start gap-3',
  secondaryActions: 'pt-4 border-t border-zinc-700',
  secondaryButton: 'w-full justify-start gap-2',
} as const;
