// Window Title Styles
// Centralized styling utilities for the WindowTitle component
// Uses stone/molten color palette from theme

export const windowTitleStyles = {
  // shadow: effects.shadow.md
  container:
    'px-4 py-1 bg-stone-950/95 backdrop-blur-sm border-b border-stone-800/50 select-none grid grid-cols-[auto_max-content] fixed top-0 left-0 right-0 z-50 shadow-[0_4px_6px_rgba(0,0,0,0.7)]',
  title: 'flex items-center gap-2 text-sm text-ember-400 font-cursive',
  controls: 'flex items-center gap-2',
  controlButton: 'w-5 h-5 p-0 text-stone-500 hover:text-stone-300 transition-colors',
} as const;
