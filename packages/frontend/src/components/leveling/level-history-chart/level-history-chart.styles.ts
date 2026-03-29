export const levelHistoryChartStyles = {
  container: 'relative px-5 pt-6 pb-4',
  header: 'flex items-center justify-end pt-4 px-5 mb-1',
  title: 'text-sm font-medium text-stone-300',
  toggleGroup: 'flex gap-1',
  toggleButton: 'px-2 py-0.5 text-xs rounded border transition-colors',
  toggleActive: 'border-ember-600 bg-ember-900/40 text-ember-300',
  toggleInactive: 'border-stone-700 bg-transparent text-stone-500 hover:text-stone-400',
} as const;
