/**
 * Class Colors Utility
 *
 * Centralized color definitions for all character classes.
 * Provides both Tailwind CSS classes and hex color values for consistency
 * across the application (character cards, charts, etc.)
 */

import type { CharacterClass } from '../types/character';

export interface ClassColorScheme {
  // Tailwind classes
  text: string;
  border: string;
  bgGradient: string;
  level: {
    bg: string;
    border: string;
    text: string;
  };
  // Hex values for charts
  hex: {
    primary: string;
    secondary: string;
  };
}

/**
 * Complete color scheme for each character class
 */
export const CLASS_COLOR_SCHEMES: Record<CharacterClass, ClassColorScheme> = {
  Warrior: {
    text: 'text-red-400',
    border: 'border-red-500',
    bgGradient: 'from-red-500/10 to-red-600/5',
    level: {
      bg: 'from-red-500/20 to-red-600/20',
      border: 'border-red-500/30',
      text: 'text-red-400',
    },
    hex: {
      primary: '#ef4444', // red-500
      secondary: '#dc2626', // red-600
    },
  },
  Sorceress: {
    text: 'text-blue-400',
    border: 'border-blue-500',
    bgGradient: 'from-blue-500/10 to-blue-600/5',
    level: {
      bg: 'from-blue-500/20 to-blue-600/20',
      border: 'border-blue-500/30',
      text: 'text-blue-400',
    },
    hex: {
      primary: '#3b82f6', // blue-500
      secondary: '#2563eb', // blue-600
    },
  },
  Ranger: {
    text: 'text-green-400',
    border: 'border-green-500',
    bgGradient: 'from-green-500/10 to-green-600/5',
    level: {
      bg: 'from-green-500/20 to-green-600/20',
      border: 'border-green-500/30',
      text: 'text-green-400',
    },
    hex: {
      primary: '#22c55e', // green-500
      secondary: '#16a34a', // green-600
    },
  },
  Huntress: {
    text: 'text-yellow-400',
    border: 'border-yellow-500',
    bgGradient: 'from-yellow-500/10 to-yellow-600/5',
    level: {
      bg: 'from-yellow-500/20 to-yellow-600/20',
      border: 'border-yellow-500/30',
      text: 'text-yellow-400',
    },
    hex: {
      primary: '#eab308', // yellow-500
      secondary: '#ca8a04', // yellow-600
    },
  },
  Monk: {
    text: 'text-purple-400',
    border: 'border-purple-500',
    bgGradient: 'from-purple-500/10 to-purple-600/5',
    level: {
      bg: 'from-purple-500/20 to-purple-600/20',
      border: 'border-purple-500/30',
      text: 'text-purple-400',
    },
    hex: {
      primary: '#a855f7', // purple-500
      secondary: '#9333ea', // purple-600
    },
  },
  Mercenary: {
    text: 'text-orange-400',
    border: 'border-orange-500',
    bgGradient: 'from-orange-500/10 to-orange-600/5',
    level: {
      bg: 'from-orange-500/20 to-orange-600/20',
      border: 'border-orange-500/30',
      text: 'text-orange-400',
    },
    hex: {
      primary: '#f97316', // orange-500
      secondary: '#ea580c', // orange-600
    },
  },
  Witch: {
    text: 'text-pink-400',
    border: 'border-pink-500',
    bgGradient: 'from-pink-500/10 to-pink-600/5',
    level: {
      bg: 'from-pink-500/20 to-pink-600/20',
      border: 'border-pink-500/30',
      text: 'text-pink-400',
    },
    hex: {
      primary: '#ec4899', // pink-500
      secondary: '#db2777', // pink-600
    },
  },
  Druid: {
    text: 'text-teal-400',
    border: 'border-teal-500',
    bgGradient: 'from-teal-500/10 to-teal-600/5',
    level: {
      bg: 'from-teal-500/20 to-teal-600/20',
      border: 'border-teal-500/30',
      text: 'text-teal-400',
    },
    hex: {
      primary: '#14b8a6', // teal-500
      secondary: '#0d9488', // teal-600
    },
  },
};

/**
 * Default color scheme for unknown classes
 */
export const DEFAULT_CLASS_COLORS: ClassColorScheme = {
  text: 'text-zinc-400',
  border: 'border-zinc-500',
  bgGradient: 'from-zinc-500/10 to-zinc-600/5',
  level: {
    bg: 'from-zinc-500/20 to-zinc-600/20',
    border: 'border-zinc-500/30',
    text: 'text-zinc-400',
  },
  hex: {
    primary: '#71717a', // zinc-500
    secondary: '#52525b', // zinc-600
  },
};

/**
 * Get the complete color scheme for a character class
 */
export function getClassColorScheme(characterClass: string): ClassColorScheme {
  return (
    CLASS_COLOR_SCHEMES[characterClass as CharacterClass] ||
    DEFAULT_CLASS_COLORS
  );
}

/**
 * Get text color class for a character class
 */
export function getClassTextColor(characterClass: string): string {
  return getClassColorScheme(characterClass).text;
}

/**
 * Get border color class for a character class
 */
export function getClassBorderColor(characterClass: string): string {
  return getClassColorScheme(characterClass).border;
}

/**
 * Get background gradient classes for a character class
 */
export function getClassBgGradient(characterClass: string): string {
  return getClassColorScheme(characterClass).bgGradient;
}

/**
 * Get level badge colors for a character class
 */
export function getClassLevelColors(characterClass: string): {
  bg: string;
  border: string;
  text: string;
} {
  return getClassColorScheme(characterClass).level;
}

/**
 * Get hex color value for a character class (for charts)
 */
export function getClassHexColor(characterClass: string): string {
  return getClassColorScheme(characterClass).hex.primary;
}

/**
 * Get secondary hex color value for a character class (for gradients in charts)
 */
export function getClassSecondaryHexColor(characterClass: string): string {
  return getClassColorScheme(characterClass).hex.secondary;
}
