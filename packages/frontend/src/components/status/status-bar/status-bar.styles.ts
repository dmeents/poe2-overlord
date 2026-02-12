// Status Bar Styles
// Centralized styling utilities for the StatusBar component

export const statusBarStyles = {
  // z-30: Fixed UI chrome (see patterns.md for z-index scale)
  container:
    'fixed bottom-0 w-full py-1 px-4 border-t bg-stone-950/95 backdrop-blur-sm border-stone-800/50 flex justify-between gap-2 shadow-top z-30',
  leftSection: 'text-xs text-stone-400 flex items-center gap-2',
  rightSection: 'flex items-center gap-2',
  tooltip: '', // Tooltip styles are handled by title attribute
} as const;
