// Window Title Styles
// Centralized styling utilities for the WindowTitle component

export const windowTitleStyles = {
  // z-30: Fixed UI chrome (see patterns.md for z-index scale)
  // Uses chrome-shadow-bottom (filter-based) for WebKitGTK compatibility
  container:
    'px-4 py-1 bg-stone-950 border-b border-stone-800/50 select-none grid grid-cols-[auto_max-content] fixed top-0 left-0 right-0 z-30 chrome-shadow-bottom',
  title: 'flex items-center gap-2 text-sm text-ember-400 font-cursive',
  controls: 'flex items-center gap-2',
  controlButton: 'w-5 h-5 p-0 text-stone-500 hover:text-stone-300 transition-colors',
} as const;
