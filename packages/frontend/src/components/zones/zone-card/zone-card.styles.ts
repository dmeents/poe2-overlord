/**
 * Zone Card Styling Functions
 *
 * This file contains styling-related functions and constants
 * for the ZoneCard component.
 * Uses stone/arcane/spirit/verdant/molten color palette from theme.
 */

/**
 * Get main zone card container classes
 */
export function getZoneCardClasses(): string {
  return 'p-4 bg-stone-900/80 border border-stone-700';
}

/**
 * Get zone card header classes
 */
export function getZoneCardHeaderClasses(): string {
  return 'flex items-center justify-between mb-3';
}

/**
 * Get zone title container classes
 */
export function getZoneTitleContainerClasses(): string {
  return 'flex-1 min-w-0';
}

/**
 * Get zone title classes
 */
export function getZoneTitleClasses(): string {
  return 'text-white font-semibold text-lg truncate';
}

/**
 * Get zone stats container classes
 */
export function getZoneStatsContainerClasses(): string {
  return 'text-right';
}

/**
 * Get zone duration classes
 */
export function getZoneDurationClasses(): string {
  return 'text-stone-400 font-mono text-lg';
}

/**
 * Get zone metadata container classes
 */
export function getZoneMetadataContainerClasses(): string {
  return 'flex items-center justify-between text-sm text-stone-400';
}

/**
 * Get zone metadata items container classes
 */
export function getZoneMetadataItemsClasses(): string {
  return 'flex items-center space-x-2 flex-wrap';
}

/**
 * Get zone pill classes
 */
export function getZonePillClasses(): string {
  return 'text-xs font-medium px-2 py-0.5 rounded';
}

/**
 * Get zone pill base classes
 */
export function getZonePillBaseClasses(): string {
  return 'bg-stone-700/50';
}

/**
 * Get zone pill color classes for different types
 */
export function getZonePillColorClasses(type: string): string {
  switch (type) {
    case 'Zone':
      return 'text-arcane-400';
    case 'Act':
      return 'text-spirit-400';
    case 'Hideout':
      return 'text-verdant-400';
    case 'Town':
      return 'text-molten-400';
    default:
      return 'text-stone-400';
  }
}

/**
 * Get zone wiki button classes
 */
export function getZoneWikiButtonClasses(): string {
  return 'text-xs text-stone-400 font-medium bg-stone-700/50 px-2 py-0.5 rounded hover:bg-stone-600/50 transition-colors duration-200 flex items-center space-x-1 cursor-pointer';
}

/**
 * Get zone status indicator classes
 */
export function getZoneStatusIndicatorClasses(): string {
  return 'flex items-center space-x-1';
}

/**
 * Get zone active indicator classes
 */
export function getZoneActiveIndicatorClasses(): string {
  return 'w-2 h-2 bg-verdant-400 rounded-full animate-pulse';
}

/**
 * Get zone active text classes
 */
export function getZoneActiveTextClasses(): string {
  return 'text-xs text-verdant-400 font-medium';
}
