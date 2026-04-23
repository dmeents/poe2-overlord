import type { BackgroundImage } from '@/types/app-config';

/** Maps BackgroundImage config values to their CSS background-image expressions. */
export const BACKGROUND_IMAGE_CSS: Record<BackgroundImage, string> = {
  None: 'none',
  VolcanicRuins: 'url("/bg-volcanic-ruins.png")',
  ManaStormRuins: 'url("/bg-mana-storm-ruins.png")',
  NecroRuins: 'url("/bg-necro-ruins.png")',
  OvergrownForest: 'url("/bg-overgrown-forest.png")',
  SpaceTime: 'url("/bg-space-time.png")',
};
