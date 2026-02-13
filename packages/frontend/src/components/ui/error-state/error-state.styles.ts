// ErrorState Styles
// Centralized styling utilities for the ErrorState component
// Uses blood (danger) and stone color palette from theme

export const errorStateStyles = {
  container: 'text-center py-8',
  iconContainer: 'text-blood-400 mb-4',
  icon: 'mx-auto h-12 w-12',
  title: 'text-lg font-semibold text-blood-400 mb-2',
  message: 'text-sm text-stone-400',
  actionContainer: 'mt-4',
} as const;
