export const noteListItemStyles = {
  container:
    'w-full text-left flex flex-col gap-1 px-4 py-3 cursor-pointer transition-colors border-b border-stone-800/60 hover:bg-stone-800/40',
  containerActive: 'bg-stone-800/60 border-l-2 border-l-ember-500',
  containerInactive: 'border-l-2 border-l-transparent',
  header: 'flex items-center justify-between gap-2',
  title: 'text-sm font-medium text-stone-100 truncate flex-1',
  pinIcon: 'text-ember-400 flex-shrink-0 text-xs',
  preview: 'text-xs text-stone-400 line-clamp-2 leading-relaxed',
  meta: 'flex items-center gap-2 mt-1',
  timestamp: 'text-xs text-stone-500',
  characterBadge: 'text-xs bg-stone-700/60 text-stone-400 px-1.5 py-0.5 rounded truncate max-w-24',
} as const;
