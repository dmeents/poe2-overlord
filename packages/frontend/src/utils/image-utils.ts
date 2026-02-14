import type { SyntheticEvent } from 'react';

/**
 * Handler to hide an image element when it fails to load.
 * Useful for gracefully handling missing or broken image assets.
 *
 * @param e - The error event from the image element
 *
 * @example
 * <img src={icon} alt="Icon" onError={hideOnError} />
 */
export function hideOnError(e: SyntheticEvent<HTMLImageElement>): void {
  e.currentTarget.style.display = 'none';
}
