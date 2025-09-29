// Status Indicator Styles
// Centralized styling utilities for the StatusIndicator component

export const statusIndicatorStyles = {
  container: '', // Base container styles
  sizeClasses: {
    sm: 'w-4 h-4',
    md: 'w-5 h-5',
    lg: 'w-6 h-6',
  },
  statusClasses: {
    success: 'text-green-800',
    warning: 'text-yellow-600',
    error: 'text-red-800',
    info: 'text-zinc-500',
  },
  animated: 'animate-pulse',
} as const;
