export const levelHistoryChartStyles = {
  container: 'relative',
  header: 'flex items-center justify-between mb-3',
  title: 'text-sm font-medium text-stone-400',
  toggleGroup: 'flex gap-1',
  toggleButton: 'px-2 py-0.5 text-xs rounded border transition-colors',
  toggleActive: 'border-ember-600 bg-ember-900/40 text-ember-300',
  toggleInactive: 'border-stone-700 bg-transparent text-stone-500 hover:text-stone-400',
} as const;
