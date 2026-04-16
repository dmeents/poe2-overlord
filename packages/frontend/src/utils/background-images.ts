import type { BackgroundImage } from '@/types/app-config';

/** Maps BackgroundImage config values to their CSS background-image expressions. */
export const BACKGROUND_IMAGE_CSS: Record<BackgroundImage, string> = {
  None: 'none',
  VolcanicRuins: 'url("/bg-volcanic-ruins.png")',
};
