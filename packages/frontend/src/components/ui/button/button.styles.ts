export const buttonStyles = {
  base: 'cursor-pointer inline-flex items-center justify-center font-medium transition-all duration-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ember-500 focus-visible:ring-offset-2 focus-visible:ring-offset-stone-950 disabled:pointer-events-none disabled:opacity-50',
  variants: {
    primary: 'bg-ember-800 text-ember-100 hover:bg-ember-700 border border-ember-700 glow-ember',
    secondary: 'bg-stone-800 text-stone-200 hover:bg-stone-700 border border-stone-700',
    outline:
      'border border-stone-700 bg-stone-900 text-stone-200 hover:bg-stone-800 hover:border-stone-600',
    ghost: 'hover:bg-stone-800 hover:text-stone-200',
    text: 'bg-transparent hover:text-stone-200',
    icon: 'flex items-center justify-center bg-transparent text-stone-400 hover:text-ember-400',
    danger: 'bg-blood-700 text-blood-100 hover:bg-blood-600 border border-blood-600 glow-blood',
    active: 'bg-stone-800 text-ember-400 border border-ember-700/50 glow-ember',
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
