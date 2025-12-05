/**
 * Character Card Styling Functions
 *
 * This file contains all styling-related functions and color mappings
 * for the CharacterCard component.
 */

import {
  getClassTextColor,
  getClassBorderColor as getClassBorderColorUtil,
  getClassBgGradient,
  getClassLevelColors as getClassLevelColorsUtil,
} from '../../../utils/class-colors';

/**
 * Get text color class for a character class
 */
export function getClassColor(characterClass: string): string {
  return getClassTextColor(characterClass);
}

/**
 * Get border color class for a character class
 */
export function getClassBorderColor(characterClass: string): string {
  return getClassBorderColorUtil(characterClass);
}

/**
 * Get background gradient classes for a character class
 */
export function getClassBgColor(characterClass: string): string {
  return getClassBgGradient(characterClass);
}

/**
 * Get level badge colors for a character class
 */
export function getClassLevelColors(characterClass: string): {
  bg: string;
  border: string;
  text: string;
} {
  return getClassLevelColorsUtil(characterClass);
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

/**
 * Get background image styles for a character card with ascendency image
 */
export function getAscendencyBackgroundStyles(ascendencyImage: string | null): {
  backgroundImage?: string;
  backgroundSize: string;
  backgroundPosition: string;
  backgroundRepeat: string;
  backgroundAttachment?: string;
} {
  if (!ascendencyImage) {
    return {
      backgroundSize: 'cover',
      backgroundPosition: 'center center',
      backgroundRepeat: 'no-repeat',
    };
  }

  return {
    backgroundImage: `url(${ascendencyImage})`,
    backgroundSize: 'cover',
    backgroundPosition: 'calc(50% + 200px) -150px',
    backgroundRepeat: 'no-repeat',
    backgroundAttachment: 'local',
  };
}

/**
 * Get overlay styles for ascendency background images
 * Provides a smooth gradient overlay that transitions from solid to transparent
 * Adds a narrow red accent gradient for hardcore characters
 */
export function getAscendencyOverlayStyles(): {
  background: string;
  boxShadow: string;
  border: string;
} {
  return {
    background: `
      linear-gradient(90deg,
        rgba(0, 0, 0, 0.95) 0%,
        rgba(0, 0, 0, 0.85) 15%,
        rgba(0, 0, 0, 0.7) 30%,
        rgba(0, 0, 0, 0.5) 45%,
        rgba(0, 0, 0, 0.3) 60%,
        rgba(0, 0, 0, 0.15) 75%,
        rgba(0, 0, 0, 0.05) 90%,
        transparent 100%
      ),
      linear-gradient(135deg,
        rgba(255, 255, 255, 0.1) 0%,
        rgba(255, 255, 255, 0.05) 25%,
        transparent 50%,
        rgba(255, 255, 255, 0.02) 75%,
        transparent 100%
      )
    `,
    boxShadow:
      'inset 0 1px 0 rgba(255, 255, 255, 0.1), inset 0 -1px 0 rgba(0, 0, 0, 0.1)',
    border: '1px solid rgba(255, 255, 255, 0.1)',
  };
}

/**
 * Get header section background styles for better transition with ascendency images
 * Creates a solid background that gradually fades to reveal the background image
 * Adds a narrow red accent gradient for hardcore characters
 */
export function getHeaderSectionBackgroundStyles(isHardcore: boolean = false): {
  background: string;
} {
  const hardcoreGradient = isHardcore
    ? `linear-gradient(0deg,
        rgba(139, 0, 0, 0.5) 0%,
        rgba(139, 0, 0, 0.2) 5%,
        rgba(139, 0, 0, 0.1) 10%,
        transparent 15%
      ),
      linear-gradient(180deg,
          rgba(139, 0, 0, 0.5) 0%,
          rgba(139, 0, 0, 0.2) 5%,
          rgba(139, 0, 0, 0.1) 10%,
          transparent 15%
        ),`
    : '';

  return {
    background: `
      ${hardcoreGradient}
      linear-gradient(90deg,
        rgba(0, 0, 0, 0.95) 0%,
        rgba(0, 0, 0, 0.9) 20%,
        rgba(0, 0, 0, 0.8) 40%,
        rgba(0, 0, 0, 0.6) 60%,
        rgba(0, 0, 0, 0.3) 80%,
        transparent 100%
      )
    `,
  };
}

/**
 * Get overlay styles for the details section to obscure the underlying image
 */
export function getDetailsSectionOverlayStyles(): {
  background: string;
} {
  return {
    background:
      'linear-gradient(to-r, rgba(0, 0, 0, 0.9) 0%, rgba(0, 0, 0, 0.8) 100%)',
  };
}
