// Status Bar Styles
// Centralized styling utilities for the StatusBar component
// Uses stone color palette from theme

export const statusBarStyles = {
  // shadow: effects.shadow.top
  container:
    'fixed bottom-0 w-full py-1 px-4 border-t bg-stone-950/95 backdrop-blur-sm border-stone-800/50 flex justify-between gap-2 shadow-[0_-4px_6px_rgba(0,0,0,0.7)]',
  leftSection: 'text-xs text-stone-400 flex items-center gap-2',
  rightSection: 'flex items-center gap-2',
  tooltip: '', // Tooltip styles are handled by title attribute
} as const;
