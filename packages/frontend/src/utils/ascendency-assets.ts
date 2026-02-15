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

const ASCENDENCY_IMAGES: Record<Ascendency, string | null> = {
  Titan: titanImage,
  Warbringer: warbringerImage,
  'Smith of Katava': smithOfKatavaImage,
  Stormweaver: stormweaverImage,
  Chronomancer: chronomancerImage,
  Deadeye: deadeyeImage,
  Pathfinder: pathfinderImage,
  Ritualist: ritualistImage,
  Amazon: amazonImage,
  Invoker: invokerImage,
  'Acolyte of Chayula': acolyteImage,
  'Gemling Legionnaire': gemlingImage,
  Tactitian: tacticianImage,
  Witchhunter: witchHunterImage,
  'Blood Mage': bloodMageImage,
  Infernalist: infernalistImage,
  Lich: lichImage,
  Shaman: shamanImage,
  Oracle: oracleImage,
};

export function getAscendencyImage(ascendency: Ascendency): string | null {
  return ASCENDENCY_IMAGES[ascendency] || null;
}
