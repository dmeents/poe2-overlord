// Form Sort Select Styles
// Specialized styling for sort dropdown functionality
// Uses stone/ember color palette from theme

export const formSortSelectStyles = {
  container: 'relative',
  label: 'block text-sm font-medium text-stone-300 uppercase tracking-wide mb-2',

  // Trigger button
  triggerContainer: 'relative',
  trigger:
    'flex items-center justify-between w-full px-4 py-2 bg-stone-800/60 hover:bg-stone-700 border border-stone-700/60 text-stone-300 hover:text-stone-50 transition-colors h-10 focus:outline-none focus:ring-1 focus:ring-ember-500/50 focus:border-ember-500/50 disabled:opacity-50 disabled:cursor-not-allowed',
  triggerText: 'text-sm font-medium',
  triggerIcons: 'flex items-center space-x-2',
  directionIcon: 'text-xs text-stone-400',
  chevron: 'w-4 h-4 text-stone-400 transition-transform',
  chevronOpen: 'rotate-180',

  // Dropdown - z-20: Dropdowns/popovers (see patterns.md for z-index scale)
  dropdown: 'fixed mt-2 bg-stone-800 border border-stone-700 card-shadow z-20 w-64 min-w-max',
  header: 'flex items-center justify-between px-4 pt-4 pb-3 mb-3 border-b border-stone-700',
  headerTitle: 'text-sm font-medium text-stone-300',
  resetButton: 'text-xs text-stone-400 hover:text-stone-50 transition-colors',

  // Options
  optionsList: 'space-y-2 px-4',
  option:
    'flex items-center justify-between px-3 py-2 hover:bg-stone-700/50 cursor-pointer transition-colors',
  optionSelected: 'bg-ember-500/20 text-ember-400',
  optionLabel: 'text-sm text-stone-300',
  optionIcon: 'w-4 h-4 text-ember-400',

  // Direction toggle
  directionToggle: 'mt-4 pt-4 border-t border-stone-700 px-4 pb-4',
  directionButton:
    'w-full flex items-center justify-center space-x-2 px-3 py-2 bg-stone-800/60 hover:bg-stone-700 text-stone-300 hover:text-stone-50 transition-colors',
  directionText: 'text-sm',
  directionIconLarge: 'text-lg',
} as const;
