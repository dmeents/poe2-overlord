/**
 * Character Card Styles
 *
 * Hero-style card with class-specific theming using the centralized design system.
 * All colors use theme tokens (stone, ember, blood, arcane, etc.)
 */

import { getClassTheme } from '@/utils/class-colors';

// Class theme to header gradient
const CLASS_HEADER_GRADIENT: Record<string, string> = {
  blood: 'from-blood-500/15 via-blood-600/5 to-transparent',
  arcane: 'from-arcane-500/15 via-arcane-600/5 to-transparent',
  verdant: 'from-verdant-500/15 via-verdant-600/5 to-transparent',
  molten: 'from-molten-500/15 via-molten-600/5 to-transparent',
  spirit: 'from-spirit-500/15 via-spirit-600/5 to-transparent',
  ember: 'from-ember-500/15 via-ember-600/5 to-transparent',
  hex: 'from-hex-500/15 via-hex-600/5 to-transparent',
  primal: 'from-primal-500/15 via-primal-600/5 to-transparent',
  ash: 'from-ash-500/15 via-ash-600/5 to-transparent',
};

// Class theme to level badge styles
const CLASS_LEVEL_BADGE: Record<string, { bg: string; border: string; text: string }> = {
  blood: {
    bg: 'bg-gradient-to-br from-blood-500/30 to-blood-700/20',
    border: 'border-blood-500/50',
    text: 'text-blood-400',
  },
  arcane: {
    bg: 'bg-gradient-to-br from-arcane-500/30 to-arcane-600/20',
    border: 'border-arcane-500/50',
    text: 'text-arcane-400',
  },
  verdant: {
    bg: 'bg-gradient-to-br from-verdant-500/30 to-verdant-600/20',
    border: 'border-verdant-500/50',
    text: 'text-verdant-400',
  },
  molten: {
    bg: 'bg-gradient-to-br from-molten-500/30 to-molten-600/20',
    border: 'border-molten-500/50',
    text: 'text-molten-400',
  },
  spirit: {
    bg: 'bg-gradient-to-br from-spirit-500/30 to-spirit-600/20',
    border: 'border-spirit-500/50',
    text: 'text-spirit-400',
  },
  ember: {
    bg: 'bg-gradient-to-br from-ember-500/30 to-ember-600/20',
    border: 'border-ember-500/50',
    text: 'text-ember-400',
  },
  hex: {
    bg: 'bg-gradient-to-br from-hex-500/30 to-hex-600/20',
    border: 'border-hex-500/50',
    text: 'text-hex-400',
  },
  primal: {
    bg: 'bg-gradient-to-br from-primal-500/30 to-primal-600/20',
    border: 'border-primal-500/50',
    text: 'text-primal-400',
  },
  ash: {
    bg: 'bg-gradient-to-br from-ash-500/30 to-ash-600/20',
    border: 'border-ash-500/50',
    text: 'text-ash-400',
  },
};

// Class theme to text color
const CLASS_TEXT: Record<string, string> = {
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

// Class theme to border color (active state) - uses ! for specificity over base border
const CLASS_BORDER: Record<string, string> = {
  blood: '!border-blood-500/60',
  arcane: '!border-arcane-500/60',
  verdant: '!border-verdant-500/60',
  molten: '!border-molten-500/60',
  spirit: '!border-spirit-500/60',
  ember: '!border-ember-500/60',
  hex: '!border-hex-500/60',
  primal: '!border-primal-500/60',
  ash: '!border-ash-500/60',
};

// Class theme to hover border color
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

export const characterCardStyles = {
  // Base card container
  // Note: overflow-hidden is required to clip the ascendancy background image
  base: 'group relative bg-stone-900 border border-stone-800 card-shadow overflow-hidden',
  baseInteractive: 'cursor-pointer',
  baseHoverBg: 'hover:bg-gradient-to-r hover:from-stone-800/50 hover:to-transparent',

  // Hover border (class-colored)
  hoverBorder: (characterClass: string) =>
    CLASS_HOVER_BORDER[getClassTheme(characterClass)] ?? CLASS_HOVER_BORDER.ash,

  // Active state border (class-colored, always visible)
  activeBorder: (characterClass: string) =>
    CLASS_BORDER[getClassTheme(characterClass)] ?? CLASS_BORDER.ash,

  // Header section
  header: 'relative px-4 py-3',
  headerGradient: (characterClass: string) =>
    `bg-gradient-to-r ${CLASS_HEADER_GRADIENT[getClassTheme(characterClass)] ?? CLASS_HEADER_GRADIENT.ash}`,
  headerContent: 'flex items-start gap-3',

  // Level badge
  levelBadge: 'w-12 h-12 flex items-center justify-center flex-shrink-0 border',
  levelBadgeStyles: (characterClass: string) => {
    const theme = getClassTheme(characterClass);
    const styles = CLASS_LEVEL_BADGE[theme] ?? CLASS_LEVEL_BADGE.ash;
    return `${styles.bg} ${styles.border}`;
  },
  levelText: (characterClass: string) => {
    const theme = getClassTheme(characterClass);
    return `text-lg font-bold ${CLASS_LEVEL_BADGE[theme]?.text ?? CLASS_LEVEL_BADGE.ash.text}`;
  },

  // Character info
  info: 'flex-1 min-w-0',
  name: 'text-lg font-bold text-stone-50 truncate',
  details: 'flex items-center gap-2 text-sm mt-0.5',
  classText: (characterClass: string) =>
    `font-medium ${CLASS_TEXT[getClassTheme(characterClass)] ?? CLASS_TEXT.ash}`,
  separator: 'text-stone-600',
  ascendency: 'text-stone-400',

  // League info row
  leagueRow: 'flex items-center gap-2 mt-1',
  leagueBadge: 'text-xs text-stone-500',
  hardcoreBadge: 'text-xs font-medium text-blood-400 bg-blood-500/10 px-1.5 py-0.5',
  ssfBadge: 'text-xs font-medium text-molten-400 bg-molten-500/10 px-1.5 py-0.5',

  // Action buttons
  // Note: backdrop-blur removed - buttons are on opaque card background, nothing to blur
  actions:
    'flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200 ml-auto flex-shrink-0',
  actionButton: 'bg-stone-800/80',
  deleteButton: 'text-blood-400 hover:text-blood-300 hover:border-blood-500 bg-stone-800/80',

  // Stats footer - uses filter shadow for WebKitGTK compatibility
  footer: 'relative px-4 py-2.5 bg-stone-950 border-t border-stone-800 chrome-shadow-top',
  statsGrid: 'flex items-center justify-between gap-4',
  stat: 'flex items-center gap-1.5',
  statIcon: 'w-4 h-4 text-stone-400',
  statValue: 'text-sm font-medium text-stone-200',
  statLabel: 'text-xs text-stone-400 hidden sm:inline',

  // Ascendency background
  ascendencyBg: 'absolute inset-0 pointer-events-none',
  ascendencyOverlay: 'absolute inset-0',
} as const;

/**
 * Get background image styles for ascendency artwork
 */
export function getAscendencyBackgroundStyles(ascendencyImage: string | null): React.CSSProperties {
  if (!ascendencyImage) return {};

  return {
    backgroundImage: `url(${ascendencyImage})`,
    backgroundSize: 'cover',
    backgroundPosition: 'calc(50% + 180px) -150px',
    backgroundRepeat: 'no-repeat',
  };
}

/**
 * Get overlay gradient that fades the ascendency image
 * Uses stone-900 base to match theme
 * Opaque on left, fades to transparent on right to reveal character art
 */
export function getAscendencyOverlayStyles(): React.CSSProperties {
  return {
    background: `
      linear-gradient(90deg,
        rgba(28, 25, 23, 1) 30%,
        rgba(28, 25, 23, 0.95) 35%,
        rgba(28, 25, 23, 0.7) 50%,
        rgba(28, 25, 23, 0.3) 70%,
        transparent 100%
      )
    `,
  };
}

/**
 * Get hardcore accent gradient (top and bottom red glow)
 */
export function getHardcoreAccentStyles(): React.CSSProperties {
  return {
    background: `
      linear-gradient(180deg,
        rgba(220, 38, 38, 0.4) 0%,
        rgba(220, 38, 38, 0.1) 8%,
        transparent 15%
      ),
      linear-gradient(0deg,
        rgba(220, 38, 38, 0.3) 0%,
        rgba(220, 38, 38, 0.1) 8%,
        transparent 15%
      )
    `,
  };
}

/**
 * Format a date string to a readable format
 */
export function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
  });
}
