// Card Styles
// Centralized styling utilities for the Card component
// Uses stone/ember/molten color palette from theme

export const cardStyles = {
  // Base card container
  base: 'bg-stone-900 border border-stone-800 overflow-hidden shadow-[0_2px_4px_rgba(0,0,0,0.5)]',

  // Header section with gradient accent
  header: 'px-5 py-3 border-b border-stone-800/50',

  headerContent: 'flex items-center justify-between',
  headerLeft: 'flex items-center gap-2',

  // Title typography
  title: 'text-xs font-medium uppercase tracking-wider',

  // Subtitle
  subtitle: 'text-xs text-stone-400 font-normal',

  // Status indicator dot
  statusDot: 'w-2 h-2 rounded-full animate-pulse',

  // Icon container
  icon: 'w-4 h-4',

  // Body section
  body: '',

  // Accent color variants - gradients for header backgrounds
  accentGradient: {
    ember: 'bg-gradient-to-r from-ember-500/10 to-transparent',
    molten: 'bg-gradient-to-r from-molten-400/10 to-transparent',
    blood: 'bg-gradient-to-r from-blood-500/10 to-transparent',
    ash: 'bg-gradient-to-r from-ash-500/10 to-transparent',
    stone: 'bg-gradient-to-r from-stone-700/20 to-transparent',
  },

  // Accent color variants - text colors
  accentText: {
    ember: 'text-ember-400',
    molten: 'text-molten-400',
    blood: 'text-blood-400',
    ash: 'text-ash-400',
    stone: 'text-stone-50',
  },

  // Accent color variants - status dot colors
  accentDot: {
    ember: 'bg-ember-400',
    molten: 'bg-molten-400',
    blood: 'bg-blood-400',
    ash: 'bg-ash-400',
    stone: 'bg-stone-400',
  },

  // Card variants
  variant: {
    default: '',
    elevated: 'shadow-[0_4px_6px_rgba(0,0,0,0.7)]',
    glow: 'shadow-[0_0_20px_rgba(249,115,22,0.15)]', // Subtle ember glow
  },
} as const;

// Type exports for variant props
export type CardAccentColor = keyof typeof cardStyles.accentGradient;
export type CardVariant = keyof typeof cardStyles.variant;
