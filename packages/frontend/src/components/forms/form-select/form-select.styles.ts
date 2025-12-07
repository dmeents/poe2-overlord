// Unified Form Select Styles
// Shared styling for both basic and dropdown select variants

export const formSelectStyles = {
  container: 'relative',
  label: 'block text-sm font-medium text-zinc-300 uppercase tracking-wide mb-2',

  // Basic select
  basicSelect:
    'w-full h-10 px-3 py-2 pr-8 bg-zinc-700/50 border border-zinc-600 text-white text-sm focus:outline-none focus:ring-1 focus:ring-emerald-500/50 focus:border-emerald-500/50 transition-colors appearance-none cursor-pointer',
  validSelect: 'border-zinc-600',
  invalidSelect: 'border-red-500 focus:ring-red-500/50 focus:border-red-500',
  placeholderOption: 'bg-zinc-700 text-zinc-400',
  option: 'bg-zinc-700 text-white',

  // Dropdown select
  triggerContainer: 'relative',
  trigger:
    'flex items-center justify-between w-full px-4 py-2 bg-zinc-700/50 hover:bg-zinc-700 border border-zinc-600 text-white transition-colors h-10 focus:outline-none focus:ring-1 focus:ring-emerald-500/50 focus:border-emerald-500/50 disabled:opacity-50 disabled:cursor-not-allowed',
  triggerText: 'text-sm font-medium truncate',
  triggerIcons: 'flex items-center space-x-2',
  clearButton:
    'text-zinc-400 hover:text-white text-lg leading-none px-1 transition-colors',
  chevron: 'w-4 h-4 text-white transition-transform',
  chevronOpen: 'rotate-180',

  // Dropdown
  dropdown:
    'fixed mt-2 bg-zinc-800 border border-zinc-700 shadow-2xl z-50 w-64 min-w-max',
  optionsList: 'py-2',

  // Options
  optionSelected: 'bg-emerald-500/20 text-emerald-400',
  optionDisabled: 'opacity-50 cursor-not-allowed hover:bg-transparent',
  optionLabel: 'text-sm text-zinc-300',

  // Empty state
  emptyState: 'px-4 py-2 text-sm text-zinc-500 text-center',
  warningMessage: 'mt-1 text-xs text-red-400',
} as const;
