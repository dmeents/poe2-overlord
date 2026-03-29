export const noteListStyles = {
  container: 'flex flex-col',
  header: 'flex items-center justify-between px-4 py-3 border-b border-stone-800',
  headerTitle: 'text-xs text-stone-400 uppercase tracking-wider',
  headerCount: 'text-xs text-stone-500',
  scrollArea: 'overflow-y-auto max-h-[calc(100vh-24rem)]',
  emptyContainer: 'px-4 py-8 text-center',
  emptyText: 'text-sm text-stone-500',
} as const;
