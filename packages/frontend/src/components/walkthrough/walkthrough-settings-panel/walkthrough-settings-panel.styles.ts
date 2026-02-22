export const walkthroughSettingsPanelStyles = {
  card: {
    row: 'flex items-center justify-between py-3 border-b border-stone-800 last:border-0',
    label: 'flex-1 min-w-0 mr-4',
    labelText: 'text-sm font-medium text-stone-200',
    descriptionText: 'text-xs text-stone-400 mt-0.5',
  },
  inline: {
    container: 'flex items-center gap-3 flex-wrap px-4 py-2 text-xs text-stone-400',
    label: 'font-medium text-stone-300',
    item: 'flex items-center gap-1.5',
    itemLabel: 'text-stone-400',
  },
} as const;
