/**
 * Ascendency Asset Management
 *
 * This file contains utilities for managing ascendency images and assets.
 * It provides type-safe access to ascendency background images and fallback handling.
 */

import type { Ascendency } from '../types/character';

// Import ascendency images
import acolyteImage from '../assets/ascendencies/acolyte.jpeg';
import amazonImage from '../assets/ascendencies/amazon.webp';
import bloodMageImage from '../assets/ascendencies/blood_mage.jpeg';
import chronomancerImage from '../assets/ascendencies/chronomancer.jpeg';
import deadeyeImage from '../assets/ascendencies/deadeye.jpeg';
import gemlingImage from '../assets/ascendencies/gemling.jpeg';
import infernalistImage from '../assets/ascendencies/infernalist.jpeg';
import invokerImage from '../assets/ascendencies/invoker.jpeg';
import lichImage from '../assets/ascendencies/lich.webp';
import pathfinderImage from '../assets/ascendencies/pathfinder.jpeg';
import ritualistImage from '../assets/ascendencies/ritualist.webp';
import smithOfKatavaImage from '../assets/ascendencies/smith-of-kitava.webp';
import stormweaverImage from '../assets/ascendencies/stormweaver.jpeg';
import tacticianImage from '../assets/ascendencies/tactician.webp';
import titanImage from '../assets/ascendencies/titan.jpeg';
import warbringerImage from '../assets/ascendencies/warbringer.jpeg';
import witchHunterImage from '../assets/ascendencies/witch_hunter.jpeg';

/**
 * Mapping of ascendency names to their corresponding image assets
 * Add new ascendency images here as they become available
 */
const ASCENDENCY_IMAGES: Record<Ascendency, string | null> = {
  // Warrior ascendencies
  Titan: titanImage,
  Warbringer: warbringerImage,
  'Smith of Katava': smithOfKatavaImage,

  // Sorceress ascendencies
  Stormweaver: stormweaverImage,
  Chronomancer: chronomancerImage,

  // Ranger ascendencies
  Deadeye: deadeyeImage,
  Pathfinder: pathfinderImage,

  // Huntress ascendencies
  Ritualist: ritualistImage,
  Amazon: amazonImage,

  // Monk ascendencies
  Invoker: invokerImage,
  'Acolyte of Chayula': acolyteImage,

  // Mercenary ascendencies
  'Gemling Legionnaire': gemlingImage,
  Tactitian: tacticianImage,
  Witchhunter: witchHunterImage,

  // Witch ascendencies
  'Blood Mage': bloodMageImage,
  Infernalist: infernalistImage,
  Lich: lichImage,
};

/**
 * Get the background image URL for a specific ascendency
 * Returns null if no image is available for the ascendency
 */
export function getAscendencyImage(ascendency: Ascendency): string | null {
  return ASCENDENCY_IMAGES[ascendency] || null;
}

/**
 * Check if an ascendency has an available background image
 */
export function hasAscendencyImage(ascendency: Ascendency): boolean {
  return ASCENDENCY_IMAGES[ascendency] !== null;
}

/**
 * Get all ascendencies that have available images
 */
export function getAvailableAscendencyImages(): Ascendency[] {
  return Object.entries(ASCENDENCY_IMAGES)
    .filter(([_, image]) => image !== null)
    .map(([ascendency, _]) => ascendency as Ascendency);
}

/**
 * Get the filename for an ascendency image (for future asset management)
 * Maps ascendency names to their actual filenames in the assets folder
 */
export function getAscendencyImageFilename(ascendency: Ascendency): string {
  const filenameMap: Record<Ascendency, string> = {
    // Warrior ascendencies
    Titan: 'titan.jpeg',
    Warbringer: 'warbringer.jpeg',
    'Smith of Katava': 'smith-of-kitava.webp',

    // Sorceress ascendencies
    Stormweaver: 'stormweaver.jpeg',
    Chronomancer: 'chronomancer.jpeg',

    // Ranger ascendencies
    Deadeye: 'deadeye.jpeg',
    Pathfinder: 'pathfinder.jpeg',

    // Huntress ascendencies
    Ritualist: 'ritualist.webp',
    Amazon: 'amazon.webp',

    // Monk ascendencies
    Invoker: 'invoker.jpeg',
    'Acolyte of Chayula': 'acolyte.jpeg',

    // Mercenary ascendencies
    'Gemling Legionnaire': 'gemling.jpeg',
    Tactitian: 'tactician.webp',
    Witchhunter: 'witch_hunter.jpeg',

    // Witch ascendencies
    'Blood Mage': 'blood_mage.jpeg',
    Infernalist: 'infernalist.jpeg',
    Lich: 'lich.webp',
  };

  return (
    filenameMap[ascendency] ||
    ascendency.toLowerCase().replace(/\s+/g, '_') + '.jpeg'
  );
}
