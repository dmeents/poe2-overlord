/**
 * Character Card Styles
 *
 * Compact horizontal roster card with a portrait region, identity block,
 * and stats column. Class identity via a left accent bar + colored level text.
 */

import { getClassTheme } from '@/utils/class-colors';

const CLASS_ACCENT_BG: Record<string, string> = {
  blood: 'bg-blood-500',
  arcane: 'bg-arcane-500',
  verdant: 'bg-verdant-500',
  molten: 'bg-molten-500',
  spirit: 'bg-spirit-500',
  ember: 'bg-ember-500',
  hex: 'bg-hex-500',
  primal: 'bg-primal-500',
  ash: 'bg-ash-500',
};

const CLASS_ACTIVE_BORDER: Record<string, string> = {
  blood: 'border-blood-500/60',
  arcane: 'border-arcane-500/60',
  verdant: 'border-verdant-500/60',
  molten: 'border-molten-500/60',
  spirit: 'border-spirit-500/60',
  ember: 'border-ember-500/60',
  hex: 'border-hex-500/60',
  primal: 'border-primal-500/60',
  ash: 'border-ash-500/60',
};

const CLASS_HOVER_BORDER: Record<string, string> = {
  blood: 'hover:border-blood-500/40',
  arcane: 'hover:border-arcane-500/40',
  verdant: 'hover:border-verdant-500/40',
  molten: 'hover:border-molten-500/40',
  spirit: 'hover:border-spirit-500/40',
  ember: 'hover:border-ember-500/40',
  hex: 'hover:border-hex-500/40',
  primal: 'hover:border-primal-500/40',
  ash: 'hover:border-ash-500/40',
};

const CLASS_LEVEL_TEXT: Record<string, string> = {
  blood: 'text-blood-400',
  arcane: 'text-arcane-400',
  verdant: 'text-verdant-400',
  molten: 'text-molten-400',
  spirit: 'text-spirit-400',
  ember: 'text-ember-400',
  hex: 'text-hex-400',
  primal: 'text-primal-400',
  ash: 'text-ash-400',
};

const CLASS_CLASS_TEXT: Record<string, string> = {
  blood: 'text-blood-400',
  arcane: 'text-arcane-400',
  verdant: 'text-verdant-400',
  molten: 'text-molten-400',
  spirit: 'text-spirit-400',
  ember: 'text-ember-400',
  hex: 'text-hex-400',
  primal: 'text-primal-400',
  ash: 'text-ash-400',
};

export const characterCardStyles = {
  // overflow-hidden clips the portrait image; border width set per state below
  base: 'group relative bg-stone-900 card-shadow overflow-hidden border',
  borderDefault: 'border-stone-800',
  baseInteractive: 'cursor-pointer',
  baseHoverBg: 'hover:bg-stone-800/20',

  // Left accent bar — absolute positioned so it never fights border utilities
  accentBar: (characterClass: string) =>
    CLASS_ACCENT_BG[getClassTheme(characterClass)] ?? CLASS_ACCENT_BG.ash,

  hoverBorder: (characterClass: string) =>
    CLASS_HOVER_BORDER[getClassTheme(characterClass)] ?? CLASS_HOVER_BORDER.ash,

  activeBorder: (characterClass: string) =>
    CLASS_ACTIVE_BORDER[getClassTheme(characterClass)] ?? CLASS_ACTIVE_BORDER.ash,

  // Three-zone horizontal layout with a minimum height so the portrait has room
  layout: 'flex items-stretch min-h-[130px]',

  // Portrait region — contained image with right-edge fade
  portrait: 'relative w-44 flex-shrink-0 overflow-hidden',
  portraitImg: 'absolute inset-0 w-full h-full object-cover object-center',
  // Fade starts at 55% so most of the image shows before blending into the card
  portraitFade: 'absolute inset-0 bg-gradient-to-r from-transparent from-55% to-stone-900',
  portraitFallback: 'w-full h-full bg-stone-800',
  portraitHardcoreOverlay:
    'absolute inset-0 bg-gradient-to-b from-blood-900/30 via-transparent to-blood-900/20',

  // Identity block — grows to fill remaining space
  identity: 'flex-1 min-w-0 px-4 py-3 flex flex-col justify-between',
  nameRow: 'flex items-center gap-2 min-w-0',
  name: 'text-base font-bold text-stone-50 truncate',
  badges: 'flex items-center gap-1.5 flex-shrink-0',
  hardcoreBadge: 'text-xs font-medium text-blood-400 bg-blood-500/10 px-1.5 py-0.5',
  ssfBadge: 'text-xs font-medium text-molten-400 bg-molten-500/10 px-1.5 py-0.5',

  classRow: 'flex items-center gap-1.5 mt-1.5',
  levelText: (characterClass: string) =>
    `text-sm font-bold ${CLASS_LEVEL_TEXT[getClassTheme(characterClass)] ?? CLASS_LEVEL_TEXT.ash}`,
  classDot: 'text-stone-600 text-xs select-none',
  classText: (characterClass: string) =>
    `text-sm font-medium ${CLASS_CLASS_TEXT[getClassTheme(characterClass)] ?? CLASS_CLASS_TEXT.ash}`,
  separator: 'text-stone-600 text-xs',
  ascendencyText: 'text-sm text-stone-400',

  bottomRow: 'flex items-center justify-between',
  leagueBadge: 'text-xs text-stone-500',
  // Action buttons revealed on group-hover
  actions: 'flex gap-1.5 opacity-0 group-hover:opacity-100 transition-opacity duration-200',
  actionButton: 'bg-stone-800/80',
  deleteButton: 'text-blood-400 hover:text-blood-300 hover:border-blood-500 bg-stone-800/80',

  // Stats column — 2×2 grid of value + label cells
  statsColumn: 'flex-none basis-[40%] px-4 py-3 bg-stone-950/50 border-l border-stone-800',
  statsGrid: 'grid grid-cols-2 gap-x-4 gap-y-3',
  statCell: 'flex flex-col gap-0.5',
  statRow: 'flex items-center gap-1.5',
  statIcon: 'w-3.5 h-3.5 text-stone-500 flex-shrink-0',
  statValue: 'text-sm font-semibold text-stone-100 tabular-nums',
  statLabel: 'text-xs text-stone-500 uppercase tracking-wide',
} as const;

export function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
  });
}
