// Danger Section Styles
// Centralized styling utilities for the DangerSection component

export const dangerSectionStyles = {
  container: 'space-y-4',
  errorMessage:
    'bg-red-900/20 border border-red-800 text-red-300 px-4 py-3 rounded-lg',
  successMessage:
    'bg-green-900/20 border border-green-800 text-green-300 px-4 py-3 rounded-lg',
  clearDataSection: 'border border-red-800 rounded-lg p-4 bg-red-950/10',
  sectionHeader: 'flex items-start justify-between',
  sectionContent: 'flex-1',
  title: 'text-lg font-medium text-red-400 mb-2',
  description: 'text-zinc-400 text-sm mb-3',
  list: 'text-zinc-400 text-sm list-disc list-inside space-y-1 mb-4',
  warning: 'text-red-300 text-sm font-medium',
  buttonContainer: 'ml-6',
  clearButton:
    'text-red-400 hover:text-red-300 border-red-600 hover:border-red-500 hover:bg-red-950/20',
} as const;
