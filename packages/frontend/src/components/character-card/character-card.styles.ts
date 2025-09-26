/**
 * Character Card Styling Functions
 *
 * This file contains all styling-related functions and color mappings
 * for the CharacterCard component.
 */

// Character class color mappings
const CLASS_COLORS = {
  Warrior: 'text-red-400',
  Sorceress: 'text-blue-400',
  Ranger: 'text-green-400',
  Huntress: 'text-yellow-400',
  Monk: 'text-purple-400',
  Mercenary: 'text-orange-400',
  Witch: 'text-pink-400',
} as const;

const CLASS_BORDER_COLORS = {
  Warrior: 'border-red-500',
  Sorceress: 'border-blue-500',
  Ranger: 'border-green-500',
  Huntress: 'border-yellow-500',
  Monk: 'border-purple-500',
  Mercenary: 'border-orange-500',
  Witch: 'border-pink-500',
} as const;

const CLASS_BG_COLORS = {
  Warrior: 'from-red-500/10 to-red-600/5',
  Sorceress: 'from-blue-500/10 to-blue-600/5',
  Ranger: 'from-green-500/10 to-green-600/5',
  Huntress: 'from-yellow-500/10 to-yellow-600/5',
  Monk: 'from-purple-500/10 to-purple-600/5',
  Mercenary: 'from-orange-500/10 to-orange-600/5',
  Witch: 'from-pink-500/10 to-pink-600/5',
} as const;

const CLASS_LEVEL_COLORS = {
  Warrior: {
    bg: 'from-red-500/20 to-red-600/20',
    border: 'border-red-500/30',
    text: 'text-red-400',
  },
  Sorceress: {
    bg: 'from-blue-500/20 to-blue-600/20',
    border: 'border-blue-500/30',
    text: 'text-blue-400',
  },
  Ranger: {
    bg: 'from-green-500/20 to-green-600/20',
    border: 'border-green-500/30',
    text: 'text-green-400',
  },
  Huntress: {
    bg: 'from-yellow-500/20 to-yellow-600/20',
    border: 'border-yellow-500/30',
    text: 'text-yellow-400',
  },
  Monk: {
    bg: 'from-purple-500/20 to-purple-600/20',
    border: 'border-purple-500/30',
    text: 'text-purple-400',
  },
  Mercenary: {
    bg: 'from-orange-500/20 to-orange-600/20',
    border: 'border-orange-500/30',
    text: 'text-orange-400',
  },
  Witch: {
    bg: 'from-pink-500/20 to-pink-600/20',
    border: 'border-pink-500/30',
    text: 'text-pink-400',
  },
} as const;

const DEFAULT_COLORS = {
  text: 'text-zinc-400',
  border: 'border-zinc-500',
  bg: 'from-zinc-500/10 to-zinc-600/5',
  level: {
    bg: 'from-zinc-500/20 to-zinc-600/20',
    border: 'border-zinc-500/30',
    text: 'text-zinc-400',
  },
} as const;

/**
 * Get text color class for a character class
 */
export function getClassColor(characterClass: string): string {
  return (
    CLASS_COLORS[characterClass as keyof typeof CLASS_COLORS] ||
    DEFAULT_COLORS.text
  );
}

/**
 * Get border color class for a character class
 */
export function getClassBorderColor(characterClass: string): string {
  return (
    CLASS_BORDER_COLORS[characterClass as keyof typeof CLASS_BORDER_COLORS] ||
    DEFAULT_COLORS.border
  );
}

/**
 * Get background gradient classes for a character class
 */
export function getClassBgColor(characterClass: string): string {
  return (
    CLASS_BG_COLORS[characterClass as keyof typeof CLASS_BG_COLORS] ||
    DEFAULT_COLORS.bg
  );
}

/**
 * Get level badge colors for a character class
 */
export function getClassLevelColors(characterClass: string): {
  bg: string;
  border: string;
  text: string;
} {
  return (
    CLASS_LEVEL_COLORS[characterClass as keyof typeof CLASS_LEVEL_COLORS] ||
    DEFAULT_COLORS.level
  );
}

/**
 * Format a date string to a readable format
 */
export function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  });
}
