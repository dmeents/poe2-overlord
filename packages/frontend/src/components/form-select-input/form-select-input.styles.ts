// Form Select Input Styles
// Centralized styling utilities for the SelectInput component

export const formSelectInputStyles = {
  container: 'relative',
  select:
    'w-full px-3 py-2 pr-8 border border-zinc-700 bg-zinc-900 text-white shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 appearance-none cursor-pointer',
  placeholderOption: 'bg-zinc-900 text-zinc-400',
  option: 'bg-zinc-900 text-white',
} as const;
