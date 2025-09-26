// Page Header Styles
// Centralized styling utilities for the PageHeader component

export const pageHeaderStyles = {
  container:
    'w-full bg-gradient-to-r from-zinc-900 via-zinc-800/50 to-zinc-900 border-b border-zinc-800/50 mb-8',
  content: 'max-w-7xl mx-auto px-6 py-8',
  header: 'flex items-start justify-between',
  titleSection: 'flex-1 min-w-0',
  title: 'text-3xl font-bold text-white font-cursive tracking-tight mb-3',
  subtitle: 'text-zinc-300 text-lg leading-relaxed max-w-3xl',
  actions: 'ml-6 flex items-center gap-3 shrink-0',
  backButton: 'flex items-center gap-2',
} as const;
