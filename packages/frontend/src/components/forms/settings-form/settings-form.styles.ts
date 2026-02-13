// Settings Form Styles
// Centralized styling utilities for the SettingsForm component
// Uses stone color palette from theme

export const settingsFormStyles = {
  container: 'space-y-0 p-4',
  messagesContainer: 'mb-6',
  actionButtons: 'flex gap-3 pt-6',
  codeBlock: 'bg-stone-700 px-1 text-stone-200',
  tooltipContent: 'mb-2',
  tooltipDescription: 'mt-2 text-stone-300',
  tooltipList: 'list-disc list-inside text-stone-300 mt-1 space-y-1',
  logLevelOptions: 'space-y-1 text-stone-300',
} as const;
