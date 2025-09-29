// Unified Form Input Styles
// Shared styling for all input variants (text, number, search)

export const formInputStyles = {
  container: 'space-y-2',
  label: 'block text-sm font-medium text-zinc-300 uppercase tracking-wide mb-2',
  inputContainer: 'relative',
  input:
    'w-full h-10 px-3 py-2 bg-zinc-700/50 border border-zinc-600 text-white placeholder-zinc-400 focus:outline-none focus:ring-1 focus:ring-emerald-500/50 focus:border-emerald-500/50 transition-colors',
  searchInput: 'pl-10', // Extra left padding for search icon
  validInput: 'border-zinc-600',
  invalidInput: 'border-red-500 focus:ring-red-500/50 focus:border-red-500',
  iconContainer: 'absolute left-3 top-1/2 transform -translate-y-1/2',
  searchIcon: 'w-4 h-4 text-zinc-400',
  clearButton:
    'absolute right-3 top-1/2 transform -translate-y-1/2 p-1 text-zinc-400 hover:text-white transition-colors',
  clearIcon: 'w-4 h-4',
  warningMessage: 'text-sm text-red-400',
} as const;
