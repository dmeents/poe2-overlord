// Form Sort Select Styles
// Specialized styling for sort dropdown functionality

export const formSortSelectStyles = {
  container: 'relative',
  label: 'block text-sm font-medium text-zinc-300 uppercase tracking-wide mb-2',

  // Trigger button
  triggerContainer: 'relative',
  trigger:
    'flex items-center justify-between w-full px-4 py-2 bg-zinc-700/50 hover:bg-zinc-700 border border-zinc-600 text-zinc-300 hover:text-white transition-colors h-10 focus:outline-none focus:ring-2 focus:ring-emerald-500/50 focus:border-emerald-500/50 disabled:opacity-50 disabled:cursor-not-allowed',
  triggerText: 'text-sm font-medium',
  triggerIcons: 'flex items-center space-x-2',
  directionIcon: 'text-xs text-zinc-400',
  chevron: 'w-4 h-4 text-zinc-400 transition-transform',
  chevronOpen: 'rotate-180',

  // Dropdown
  dropdown:
    'fixed mt-2 bg-zinc-800 border border-zinc-700 shadow-2xl z-50 p-4 w-64 min-w-max',
  header: 'flex items-center justify-between mb-3',
  headerTitle: 'text-sm font-medium text-zinc-300',
  resetButton: 'text-xs text-zinc-400 hover:text-white transition-colors',

  // Options
  optionsList: 'space-y-2',
  option:
    'flex items-center justify-between px-3 py-2 hover:bg-zinc-700/50 cursor-pointer transition-colors',
  optionSelected: 'bg-emerald-500/20 text-emerald-400',
  optionLabel: 'text-sm text-zinc-300',
  optionIcon: 'w-4 h-4 text-emerald-400',

  // Direction toggle
  directionToggle: 'mt-4 pt-4 border-t border-zinc-700',
  directionButton:
    'w-full flex items-center justify-center space-x-2 px-3 py-2 bg-zinc-700/50 hover:bg-zinc-700 text-zinc-300 hover:text-white transition-colors',
  directionText: 'text-sm',
  directionIconLarge: 'text-lg',
} as const;
