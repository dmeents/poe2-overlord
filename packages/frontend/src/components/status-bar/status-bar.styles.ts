// Status Bar Styles
// Centralized styling utilities for the StatusBar component

export const statusBarStyles = {
  container:
    'fixed bottom-0 w-full py-1 px-4 border-b bg-zinc-950 border-zinc-950 flex justify-between gap-2',
  leftSection: 'text-xs text-zinc-400 flex items-center gap-2',
  rightSection: 'flex items-center gap-2',
  tooltip: '', // Tooltip styles are handled by title attribute
} as const;
