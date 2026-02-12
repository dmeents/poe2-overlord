/**
 * POE2 Overlord Design System
 *
 * Centralized theme configuration with colors extracted from
 * the volcanic/infernal aesthetic of the logo and background assets.
 *
 * Usage:
 * - Import tokens directly: `import { colors } from '@/theme'`
 * - Use Tailwind classes with custom colors: `bg-ember-500`, `text-molten-400`
 */

// =============================================================================
// COLOR PALETTE
// =============================================================================

/**
 * Core color palette derived from logo and background assets.
 * Volcanic oranges, molten golds, blood reds, and stone grays.
 */
export const colors = {
  // Ember - Primary accent (volcanic orange-red glow)
  ember: {
    50: '#fff7ed',
    100: '#ffedd5',
    200: '#fed7aa',
    300: '#fdba74',
    400: '#fb923c',
    500: '#f97316', // Primary
    600: '#ea580c',
    700: '#c2410c',
    800: '#9a3412',
    900: '#7c2d12',
    950: '#431407',
  },

  // Molten - Secondary accent (gold/amber highlights)
  molten: {
    50: '#fffbeb',
    100: '#fef3c7',
    200: '#fde68a',
    300: '#fcd34d',
    400: '#fbbf24', // Gold accent
    500: '#d4a437', // Logo text gold
    600: '#b8942a',
    700: '#92400e',
    800: '#78350f',
    900: '#451a03',
    950: '#271004',
  },

  // Blood - Danger/hardcore states
  blood: {
    50: '#fef2f2',
    100: '#fee2e2',
    200: '#fecaca',
    300: '#fca5a5',
    400: '#f87171',
    500: '#dc2626', // Primary danger
    600: '#b91c1c',
    700: '#8b0000', // Deep blood
    800: '#6b0000',
    900: '#450a0a',
    950: '#2a0505',
  },

  // Stone - Neutral backgrounds (warm gray)
  stone: {
    50: '#fafaf9',
    100: '#f5f5f4',
    200: '#e7e5e4',
    300: '#d6d3d1',
    400: '#a8a29e',
    500: '#78716c',
    600: '#57534e',
    700: '#44403c',
    800: '#292524',
    900: '#1c1917', // Card backgrounds
    950: '#0c0a09', // App background
  },

  // Bone - Muted text and subtle highlights
  bone: {
    50: '#fdfcfb',
    100: '#e8dcc8', // Primary bone
    200: '#d4c5b5',
    300: '#c0aea2',
    400: '#a89888',
    500: '#8a7a6a',
  },

  // Ash - Cool neutral for disabled/muted states
  ash: {
    400: '#9ca3af',
    500: '#6b7280',
    600: '#4b5563',
    700: '#374151',
    800: '#1f2937',
    900: '#111827',
  },
} as const;

// =============================================================================
// SEMANTIC TOKENS
// =============================================================================

/**
 * Semantic color mappings for consistent usage across components.
 * Use these instead of raw color values when possible.
 */
export const semanticColors = {
  // Backgrounds
  background: {
    app: colors.stone[950],
    surface: colors.stone[900],
    surfaceHover: colors.stone[800],
    elevated: colors.stone[800],
    overlay: 'rgba(12, 10, 9, 0.85)',
  },

  // Text
  text: {
    primary: colors.stone[50],
    secondary: colors.stone[400],
    muted: colors.bone[100],
    accent: colors.ember[400],
    gold: colors.molten[500],
  },

  // Interactive
  interactive: {
    primary: colors.ember[500],
    primaryHover: colors.ember[600],
    secondary: colors.stone[700],
    secondaryHover: colors.stone[600],
  },

  // States
  state: {
    success: '#22c55e',
    warning: colors.molten[400],
    danger: colors.blood[500],
    dangerHover: colors.blood[600],
    info: '#3b82f6',
  },

  // Borders
  border: {
    default: colors.stone[700],
    subtle: colors.stone[800],
    accent: colors.ember[500],
    glow: `0 0 10px ${colors.ember[500]}40`,
  },
} as const;

// =============================================================================
// TYPOGRAPHY
// =============================================================================

export const typography = {
  fontFamily: {
    sans: '"Roboto", system-ui, sans-serif',
    display: '"Macondo", cursive',
  },
  fontSize: {
    xs: '0.75rem',
    sm: '0.875rem',
    base: '1rem',
    lg: '1.125rem',
    xl: '1.25rem',
    '2xl': '1.5rem',
    '3xl': '1.875rem',
  },
} as const;

// =============================================================================
// SPACING & LAYOUT
// =============================================================================

export const spacing = {
  sidebar: '48px', // 12 * 4px = w-12
  titlebar: '28px',
  statusbar: '24px', // h-6
} as const;

// =============================================================================
// EFFECTS
// =============================================================================

export const effects = {
  // Glow effects for interactive elements
  glow: {
    ember: `0 0 20px ${colors.ember[500]}30`,
    emberStrong: `0 0 30px ${colors.ember[500]}50`,
    molten: `0 0 15px ${colors.molten[400]}40`,
  },

  // Gradient overlays
  overlay: {
    dark: 'linear-gradient(to bottom, rgba(12, 10, 9, 0.9), rgba(12, 10, 9, 0.95))',
    vignette: `
      radial-gradient(
        ellipse at center,
        transparent 0%,
        rgba(12, 10, 9, 0.3) 50%,
        rgba(12, 10, 9, 0.8) 100%
      )
    `,
  },

  // Box shadows - consistent depth system
  shadow: {
    sm: '0 2px 4px rgba(0, 0, 0, 0.5)',
    md: '0 4px 6px rgba(0, 0, 0, 0.7)',
    lg: '0 8px 12px rgba(0, 0, 0, 0.7)',
    xl: '0 12px 24px rgba(0, 0, 0, 0.8)',
    // Directional variants for docked panels
    top: '0 -4px 6px rgba(0, 0, 0, 0.7)',
    right: '4px 0 6px rgba(0, 0, 0, 0.7)',
    bottom: '0 4px 6px rgba(0, 0, 0, 0.7)',
    left: '-4px 0 6px rgba(0, 0, 0, 0.7)',
  },
} as const;

// =============================================================================
// TAILWIND CLASS HELPERS
// =============================================================================

/**
 * Pre-composed Tailwind class combinations for common patterns.
 * These help maintain consistency and reduce repetition.
 */
export const tw = {
  // Button variants
  button: {
    primary:
      'bg-ember-600 text-white hover:bg-ember-700 border border-ember-700 shadow-[0_0_10px_rgba(249,115,22,0.2)] hover:shadow-[0_0_15px_rgba(249,115,22,0.3)]',
    secondary: 'bg-stone-800 text-stone-200 hover:bg-stone-700 border border-stone-700',
    outline: 'border border-stone-700 bg-stone-900 text-stone-200 hover:bg-stone-800',
    ghost: 'hover:bg-stone-800 hover:text-stone-200',
    danger:
      'bg-blood-600 text-white hover:bg-blood-700 border border-blood-700 shadow-[0_0_10px_rgba(220,38,38,0.2)]',
  },

  // Surface styles
  surface: {
    card: 'bg-stone-900 border border-stone-800 rounded-lg',
    elevated: 'bg-stone-800 border border-stone-700 rounded-lg shadow-lg',
  },

  // Text styles
  text: {
    heading: 'font-cursive text-molten-500',
    body: 'text-stone-200',
    muted: 'text-stone-400',
    accent: 'text-ember-400',
  },
} as const;

// =============================================================================
// CSS VARIABLE EXPORT
// =============================================================================

/**
 * Generate CSS custom properties for use in globals.css
 * This allows the theme to be used in both JS and CSS contexts.
 */
export function generateCSSVariables(): string {
  const lines: string[] = [];

  // Ember colors
  Object.entries(colors.ember).forEach(([shade, value]) => {
    lines.push(`--color-ember-${shade}: ${value};`);
  });

  // Molten colors
  Object.entries(colors.molten).forEach(([shade, value]) => {
    lines.push(`--color-molten-${shade}: ${value};`);
  });

  // Blood colors
  Object.entries(colors.blood).forEach(([shade, value]) => {
    lines.push(`--color-blood-${shade}: ${value};`);
  });

  // Stone colors
  Object.entries(colors.stone).forEach(([shade, value]) => {
    lines.push(`--color-stone-${shade}: ${value};`);
  });

  // Bone colors
  Object.entries(colors.bone).forEach(([shade, value]) => {
    lines.push(`--color-bone-${shade}: ${value};`);
  });

  return lines.join('\n  ');
}
