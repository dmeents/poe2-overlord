// Window Title Styles
// Centralized styling utilities for the WindowTitle component

export const windowTitleStyles = {
  container:
    'px-4 py-1 bg-zinc-950 border-b border-zinc-900 select-none grid grid-cols-[auto_max-content] fixed top-0 left-0 right-0',
  title: 'flex items-center gap-2 text-sm text-zinc-400 font-cursive',
  controls: 'flex items-center gap-2',
  controlButton: 'w-5 h-5 p-0 text-zinc-500',
} as const;
