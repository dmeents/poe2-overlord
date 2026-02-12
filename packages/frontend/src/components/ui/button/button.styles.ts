// Button Styles
// Centralized styling utilities for the Button component
// Uses ember/stone color palette from theme

export const buttonStyles = {
  base: 'cursor-pointer inline-flex items-center justify-center font-medium transition-all duration-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ember-500 focus-visible:ring-offset-2 focus-visible:ring-offset-stone-950 disabled:pointer-events-none disabled:opacity-50',
  variants: {
    primary:
      'bg-ember-600 text-white hover:bg-ember-700 border border-ember-700 shadow-[0_0_10px_rgba(249,115,22,0.15)] hover:shadow-[0_0_15px_rgba(249,115,22,0.25)]',
    secondary: 'bg-stone-800 text-stone-200 hover:bg-stone-700 border border-stone-700',
    outline:
      'border border-stone-700 bg-stone-900 text-stone-200 hover:bg-stone-800 hover:border-stone-600',
    ghost: 'hover:bg-stone-800 hover:text-stone-200',
    text: 'bg-transparent hover:text-stone-200',
    icon: 'flex items-center justify-center bg-transparent text-stone-400 hover:text-ember-400',
    danger:
      'bg-blood-600 text-white hover:bg-blood-700 border border-blood-700 shadow-[0_0_10px_rgba(220,38,38,0.15)] hover:shadow-[0_0_15px_rgba(220,38,38,0.25)]',
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
