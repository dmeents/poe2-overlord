// Loading Spinner Styles
// Centralized styling utilities for the LoadingSpinner component
// Uses stone/ember color palette from theme

export const loadingSpinnerStyles = {
  container: 'flex items-center justify-center p-8',
  spinner: 'animate-spin h-8 w-8 border-4 border-stone-700 border-t-ember-500 rounded-full',
  message: 'ml-2 text-stone-400',
} as const;
