// Modal Styles
// Centralized styling utilities for the Modal component
// Uses stone color palette from theme

export const modalStyles = {
  overlay: 'fixed inset-0 z-50 overflow-y-auto',
  container: 'flex min-h-full items-center justify-center p-4',
  backdrop: 'fixed inset-0 bg-stone-950/80 backdrop-blur-sm transition-opacity',
  modal: 'relative w-full bg-stone-900 card-shadow border border-stone-700/50 rounded-lg',
  content: 'p-6',
  header: 'flex items-center justify-between mb-6',
  headerContent: 'flex items-center gap-3',
  icon: 'flex-shrink-0 text-stone-100',
  title: 'text-2xl font-bold text-stone-100',
  closeButton: 'text-stone-400 hover:text-ember-400 transition-colors cursor-pointer',
  closeIcon: 'h-6 w-6',
  sizeClasses: {
    sm: 'max-w-sm',
    md: 'max-w-md',
    lg: 'max-w-lg',
    xl: 'max-w-xl',
    '2xl': 'max-w-2xl',
  },
} as const;
