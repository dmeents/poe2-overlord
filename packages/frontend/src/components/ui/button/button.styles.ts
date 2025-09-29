// Button Styles
// Centralized styling utilities for the Button component

export const buttonStyles = {
  base: 'cursor-pointer inline-flex items-center justify-center font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50',
  variants: {
    primary:
      'bg-emerald-800 text-white hover:bg-emerald-900 border border-emerald-900',
    secondary:
      'bg-zinc-800 text-zinc-200 hover:bg-zinc-700 border border-zinc-700',
    outline:
      'border border-zinc-700 bg-zinc-900 text-zinc-200 hover:bg-zinc-800',
    ghost: 'hover:bg-zinc-800 hover:text-zinc-200 cursor-default',
    icon: 'flex items-center justify-center bg-transparent text-zinc-400 hover:text-zinc-200',
  },
  sizes: {
    xs: 'h-6 px-2 text-xs',
    sm: 'h-8 px-3 text-sm',
    md: 'h-10 px-4 py-2',
    lg: 'h-11 px-8',
  },
  iconSizes: {
    xs: 'w-4 h-4',
    sm: 'w-5 h-5',
    md: 'w-6 h-6',
    lg: 'h-8 w-8 p-0',
  },
} as const;
