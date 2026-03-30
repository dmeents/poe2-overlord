export const pinnedNotesCardStyles = {
  grid: 'grid grid-cols-2 gap-4 p-5',
  noteCard:
    'flex flex-col bg-stone-800/40 border border-stone-700/60 rounded p-4 hover:border-stone-600/60 transition-colors min-h-32',
  noteTitle: 'text-sm font-semibold text-stone-100 mb-2 truncate',
  noteContent:
    'flex-1 prose prose-invert prose-xs max-w-none text-stone-300 overflow-hidden [&_h1]:text-stone-200 [&_h1]:text-sm [&_h2]:text-stone-200 [&_h2]:text-sm [&_h3]:text-stone-200 [&_h3]:text-xs [&_a]:text-ember-400 [&_code]:bg-stone-700 [&_code]:text-ember-300 [&_pre]:bg-stone-900 [&_blockquote]:border-ember-500/40 [&_li]:text-stone-300 [&_p]:text-xs [&_p]:leading-relaxed [&_ul]:text-xs [&_ol]:text-xs line-clamp-[12]',
  noteEmpty: 'text-xs text-stone-500 italic',
  empty: 'px-5 py-6 text-center',
  emptyText: 'text-sm text-stone-500',
  emptyHint: 'text-xs text-stone-600 mt-1',
} as const;
