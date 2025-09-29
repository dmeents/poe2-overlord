// Form Filter Toggle Styles
// Updated for overlay dropdown positioning

export const formFilterToggleStyles = {
  container: 'relative',
  toggleButton:
    'flex items-center justify-between w-full px-4 py-2 bg-zinc-700/50 hover:bg-zinc-700 border border-zinc-600 text-zinc-300 hover:text-white transition-colors h-10 focus:outline-none focus:ring-2 focus:ring-emerald-500/50 focus:border-emerald-500/50 disabled:opacity-50 disabled:cursor-not-allowed',
  toggleText: 'text-sm font-medium',
  chevron: 'w-4 h-4 text-zinc-400 transition-transform',
  content:
    'absolute top-full left-0 right-0 z-50 mt-2 space-y-3 p-4 bg-zinc-800 border border-zinc-700/50 shadow-xl',
} as const;
