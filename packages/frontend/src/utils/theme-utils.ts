/**
 * Gets the hex color value for a CSS custom property from the theme.
 *
 * @param cssVar - The CSS variable name (e.g., 'ember-500', 'blood-400')
 * @returns The hex color value as a string
 *
 * @example
 * getThemeHexColor('ember-500') // Returns '#f97316'
 * getThemeHexColor('blood-400') // Returns '#dc2626'
 */
export function getThemeHexColor(cssVar: string): string {
  return getComputedStyle(document.documentElement).getPropertyValue(`--color-${cssVar}`).trim();
}
