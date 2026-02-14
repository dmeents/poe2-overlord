// Unified Form Select Styles
// Shared styling for both basic and dropdown select variants
// Uses stone/ember color palette from theme

export const formSelectStyles = {
  container: 'relative',
  label: 'block text-sm font-medium text-stone-300 uppercase tracking-wide mb-2',

  // Basic select
  basicSelect:
    'w-full h-10 px-3 py-2 pr-8 bg-stone-800/50 border border-stone-600 text-white text-sm focus:outline-none focus:ring-1 focus:ring-ember-500/50 focus:border-ember-500/50 transition-colors appearance-none cursor-pointer',
  validSelect: 'border-stone-600',
  invalidSelect: 'border-blood-500 focus:ring-blood-500/50 focus:border-blood-500',
  placeholderOption: 'bg-stone-800 text-stone-400',
  option: 'bg-stone-800 text-white',

  // Dropdown select
  triggerContainer: 'relative',
  trigger:
    'flex items-center justify-between w-full px-4 py-2 bg-stone-800/50 hover:bg-stone-700 border border-stone-600 text-white transition-colors h-10 focus:outline-none focus:ring-1 focus:ring-ember-500/50 focus:border-ember-500/50 disabled:opacity-50 disabled:cursor-not-allowed',
  triggerText: 'text-sm font-medium truncate',
  triggerIcons: 'flex items-center space-x-2',
  clearButton: 'text-stone-400 hover:text-white text-lg leading-none px-1 transition-colors',
  chevron: 'w-4 h-4 text-white transition-transform',
  chevronOpen: 'rotate-180',

  // Dropdown - z-20: Dropdowns/popovers (see patterns.md for z-index scale)
  dropdown: 'fixed mt-2 bg-stone-800 border border-stone-700 card-shadow z-20 w-64 min-w-max',
  optionsList: 'py-2',

  // Options
  optionSelected: 'bg-ember-500/20 text-ember-400',
  optionDisabled: 'opacity-50 cursor-not-allowed hover:bg-transparent',
  optionLabel: 'text-sm text-stone-300',

  // Empty state
  emptyState: 'px-4 py-2 text-sm text-stone-500 text-center',
  warningMessage: 'mt-1 text-xs text-blood-400',
} as const;
