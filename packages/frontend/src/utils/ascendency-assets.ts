/**
 * Ascendency Asset Management
 *
 * This file contains utilities for managing ascendency images and assets.
 * It provides type-safe access to ascendency background images and fallback handling.
 */

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
import oracleImage from '../assets/ascendencies/oracle.png';
import pathfinderImage from '../assets/ascendencies/pathfinder.jpeg';
import ritualistImage from '../assets/ascendencies/ritualist.webp';
import shamanImage from '../assets/ascendencies/shaman.png';
import smithOfKatavaImage from '../assets/ascendencies/smith-of-kitava.webp';
import stormweaverImage from '../assets/ascendencies/stormweaver.jpeg';
import tacticianImage from '../assets/ascendencies/tactician.webp';
import titanImage from '../assets/ascendencies/titan.jpeg';
import warbringerImage from '../assets/ascendencies/warbringer.jpeg';
import witchHunterImage from '../assets/ascendencies/witch_hunter.jpeg';
import type { Ascendency } from '../types/character';

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

  // Druid ascendencies
  Shaman: shamanImage,
  Oracle: oracleImage,
};

/**
 * Get the background image URL for a specific ascendency
 * Returns null if no image is available for the ascendency
 */
export function getAscendencyImage(ascendency: Ascendency): string | null {
  return ASCENDENCY_IMAGES[ascendency] || null;
}
