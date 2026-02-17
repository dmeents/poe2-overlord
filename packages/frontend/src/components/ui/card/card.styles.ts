export const cardStyles = {
  base: 'bg-gradient-to-br from-stone-900 via-stone-900 to-stone-950 border border-stone-800 card-shadow',
  header: 'px-5 py-3 border-b border-stone-800/50',
  headerContent: 'flex items-center justify-between',
  headerLeft: 'flex items-center gap-2',
  title: 'text-xs font-medium uppercase tracking-wider',
  subtitle: 'text-xs text-stone-400 font-normal',
  statusDot: 'w-2 h-2 rounded-full animate-pulse',
  icon: 'w-4 h-4',
  body: '',
  accentGradient: {
    ember: 'bg-gradient-to-r from-ember-500/10 to-transparent',
    molten: 'bg-gradient-to-r from-molten-400/10 to-transparent',
    blood: 'bg-gradient-to-r from-blood-500/10 to-transparent',
    ash: 'bg-gradient-to-r from-ash-500/10 to-transparent',
    stone: 'bg-gradient-to-r from-stone-700/20 to-transparent',
  },
  accentText: {
    ember: 'text-ember-400',
    molten: 'text-molten-400',
    blood: 'text-blood-400',
    ash: 'text-ash-400',
    stone: 'text-stone-50',
  },
  accentDot: {
    ember: 'bg-ember-400',
    molten: 'bg-molten-400',
    blood: 'bg-blood-400',
    ash: 'bg-ash-400',
    stone: 'bg-stone-400',
  },
} as const;

export type CardAccentColor = keyof typeof cardStyles.accentGradient;
