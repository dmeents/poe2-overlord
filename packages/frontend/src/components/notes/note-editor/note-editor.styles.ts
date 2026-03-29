export const noteEditorStyles = {
  container: 'flex flex-col h-full',
  titleInput:
    'w-full bg-transparent text-lg font-semibold text-stone-100 placeholder-stone-500 border-0 border-b border-stone-700/60 pb-3 mb-4 focus:outline-none focus:border-ember-500/60 transition-colors',
  tabBar: 'flex items-center gap-1 mb-3',
  tab: 'px-3 py-1 text-xs font-medium rounded transition-colors',
  tabActive: 'bg-stone-700 text-stone-100',
  tabInactive: 'text-stone-400 hover:text-stone-200 hover:bg-stone-800',
  textarea:
    'flex-1 w-full min-h-48 bg-stone-800/40 border border-stone-700/60 rounded text-sm text-stone-200 placeholder-stone-500 p-3 font-mono resize-none focus:outline-none focus:border-ember-500/40 transition-colors',
  preview:
    'flex-1 min-h-48 prose prose-invert prose-sm max-w-none p-3 bg-stone-800/40 border border-stone-700/60 rounded text-stone-200 overflow-auto [&_h1]:text-stone-100 [&_h2]:text-stone-100 [&_h3]:text-stone-100 [&_a]:text-ember-400 [&_code]:bg-stone-700 [&_code]:text-ember-300 [&_pre]:bg-stone-900 [&_blockquote]:border-ember-500/40 [&_li]:text-stone-300',
  emptyPreview: 'text-stone-500 italic text-sm',
  footer: 'flex items-center gap-2 mt-4 pt-3 border-t border-stone-800',
  characterSelect: 'flex-1',
  actions: 'flex items-center gap-2 ml-auto',
  charCount: 'text-xs text-stone-500',
} as const;
