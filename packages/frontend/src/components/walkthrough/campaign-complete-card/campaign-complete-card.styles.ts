// Campaign Complete Card Styles
// Dark fantasy aesthetic with ember/molten victory theme

export const campaignCompleteCardStyles = {
  container: 'relative overflow-hidden',

  // Gradient background with ember/molten accent
  background: 'bg-gradient-to-br from-stone-900 via-stone-900 to-ember-950/30',

  // Content layout
  content: 'flex flex-col items-center gap-3 p-6 text-center',

  // Title - weathered parchment aesthetic
  title: 'text-xl font-semibold text-bone-100 tracking-wide',
  titleGlow: 'text-glow-molten',

  // Timestamp
  timestamp: 'flex items-center gap-2 text-xs text-stone-400',
  timestampIcon: 'w-3 h-3',

  // Message
  message: 'text-sm text-stone-300 max-w-md',

  // Decorative border accent
  borderAccent:
    'absolute inset-x-0 top-0 h-1 bg-gradient-to-r from-transparent via-ember-600 to-transparent',

  // Footer
  footer: 'flex justify-center items-center py-2 px-4 border-t border-stone-700/30',
} as const;
