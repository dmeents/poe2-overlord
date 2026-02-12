// Unified Form Input Styles
// Shared styling for all input variants (text, number, search)
// Uses stone/ember color palette from theme

export const formInputStyles = {
  container: 'space-y-2',
  label: 'block text-sm font-medium text-stone-300 uppercase tracking-wide mb-2',
  inputContainer: 'relative',
  input:
    'w-full h-10 px-3 py-2 bg-stone-800/50 border border-stone-600 text-white placeholder-stone-500 focus:outline-none focus:ring-1 focus:ring-ember-500/50 focus:border-ember-500/50 transition-colors rounded',
  searchInput: 'pl-10', // Extra left padding for search icon
  validInput: 'border-stone-600',
  invalidInput: 'border-blood-500 focus:ring-blood-500/50 focus:border-blood-500',
  iconContainer: 'absolute left-3 top-1/2 transform -translate-y-1/2',
  searchIcon: 'w-4 h-4 text-stone-400',
  clearButton:
    'absolute right-3 top-1/2 transform -translate-y-1/2 p-1 text-stone-400 hover:text-white transition-colors',
  clearIcon: 'w-4 h-4',
  warningMessage: 'text-sm text-blood-400',
} as const;
