/**
 * Asset paths for images and media files.
 * All assets are copied from packages/frontend/src/assets to packages/website/public.
 */

export const ASSETS = {
  logo: {
    /** Full logo with transparent background (2.9MB PNG) */
    full: '/logo-no-text.png',
    /** Square logo for icons and favicons (1.4MB PNG) */
    square: '/logo-no-text-square.png',
  },
  images: {
    /** Background image for pages (2.2MB PNG) */
    background: '/background.png',
  },
  icons: {
    /** Multi-size favicon (16x16, 32x32, 48x48) */
    favicon: '/favicon.ico',
    /** Apple touch icon (180x180) */
    appleTouchIcon: '/apple-touch-icon.png',
    /** PWA icon (192x192) */
    icon192: '/icon-192.png',
    /** PWA icon (512x512) */
    icon512: '/icon-512.png',
  },
} as const;

export type AssetPath = (typeof ASSETS)[keyof typeof ASSETS][keyof (typeof ASSETS)[keyof typeof ASSETS]];
