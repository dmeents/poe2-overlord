// Status Indicator Styles
// Centralized styling utilities for the StatusIndicator component
// Uses theme color palette

export const statusIndicatorStyles = {
  container: '', // Base container styles
  sizeClasses: {
    sm: 'w-4 h-4',
    md: 'w-5 h-5',
    lg: 'w-6 h-6',
  },
  statusClasses: {
    success: 'text-verdant-500',
    warning: 'text-molten-400',
    error: 'text-blood-500',
    info: 'text-stone-500',
  },
  animated: 'animate-pulse',
} as const;
