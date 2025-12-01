// Modal Styles
// Centralized styling utilities for the Modal component

export const modalStyles = {
  overlay: 'fixed inset-0 z-50 overflow-y-auto',
  container: 'flex min-h-full items-center justify-center p-4',
  backdrop: 'fixed inset-0 bg-black/50 transition-opacity',
  modal:
    'relative w-full bg-zinc-800 rounded-lg shadow-xl border border-zinc-700',
  content: 'p-6',
  header: 'flex items-center justify-between mb-6',
  headerContent: 'flex items-center gap-3',
  icon: 'flex-shrink-0',
  title: 'text-2xl font-bold text-white',
  closeButton:
    'text-zinc-400 hover:text-white transition-colors cursor-pointer',
  closeIcon: 'h-6 w-6',
  sizeClasses: {
    sm: 'max-w-sm',
    md: 'max-w-md',
    lg: 'max-w-lg',
    xl: 'max-w-xl',
    '2xl': 'max-w-2xl',
  },
} as const;
