// Form Select Input Styles
// Centralized styling utilities for the SelectInput component

export const formSelectInputStyles = {
  container: 'relative',
  select:
    'w-full px-3 py-2 pr-8 border border-zinc-600 bg-zinc-700/50 text-white shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500/50 appearance-none cursor-pointer transition-colors',
  placeholderOption: 'bg-zinc-700 text-zinc-400',
  option: 'bg-zinc-700 text-white',
} as const;
