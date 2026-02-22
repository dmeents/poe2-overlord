export const toggleStyles = {
  base: 'relative inline-flex flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ember-500 focus-visible:ring-offset-2 focus-visible:ring-offset-stone-950 disabled:cursor-not-allowed disabled:opacity-50',
  track: {
    off: 'bg-stone-700',
    on: 'bg-ember-600',
  },
  sizes: {
    sm: {
      track: 'w-8 h-4',
      thumb: 'w-3 h-3',
      translate: 'translate-x-4',
    },
    md: {
      track: 'w-10 h-5',
      thumb: 'w-4 h-4',
      translate: 'translate-x-5',
    },
  },
  thumb:
    'pointer-events-none inline-block rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out',
} as const;
