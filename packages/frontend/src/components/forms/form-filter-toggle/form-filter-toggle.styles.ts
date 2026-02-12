// Form Filter Toggle Styles
// Updated for overlay dropdown positioning

export const formFilterToggleStyles = {
  container: 'relative',
  toggleButton:
    'flex items-center justify-between w-full px-4 py-2 bg-stone-700/50 hover:bg-stone-700 border border-stone-600 text-stone-300 hover:text-white transition-colors h-10 focus:outline-none focus:ring-2 focus:ring-ember-500/50 focus:border-ember-500/50 disabled:opacity-50 disabled:cursor-not-allowed',
  toggleText: 'text-sm font-medium',
  chevron: 'w-4 h-4 text-stone-400 transition-transform',
  // z-20: Dropdowns/popovers (see patterns.md for z-index scale)
  content:
    'fixed z-20 mt-2 space-y-3 p-4 bg-stone-800 border border-stone-700/50 shadow-xl min-w-[300px] max-w-[400px]',
} as const;
