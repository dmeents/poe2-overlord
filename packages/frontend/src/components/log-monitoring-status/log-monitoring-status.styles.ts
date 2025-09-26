// Log Monitoring Status Styles
// Centralized styling utilities for the MonitoringStatus component

export const logMonitoringStatusStyles = {
  container: 'bg-zinc-900/50 p-4 border border-zinc-800',
  title: 'text-lg font-semibold text-white mb-3',
  statusList: 'space-y-2',
  statusItem: 'flex items-center justify-between',
  label: 'text-zinc-300',
  statusBadge: 'px-2 py-1 text-sm font-medium',
  statusActive: 'bg-green-900/50 text-green-300',
  statusInactive: 'bg-red-900/50 text-red-300',
  value: 'text-white font-mono',
} as const;
