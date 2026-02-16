# Website Public Assets

This directory contains static assets for the POE2 Overlord marketing website.

## Asset Sources

All assets are copied from `packages/frontend/src/assets/`:

### Logos
- `logo-no-text.png` - Full logo with transparent background (2.9MB)
- `logo-no-text-square.png` - Square logo for icons/favicons (1.4MB)

### Images
- `background.png` - Background image for pages (2.2MB)

### Icons (Generated)
- `favicon.ico` - Multi-size favicon (16x16, 32x32, 48x48)
- `apple-touch-icon.png` - Apple touch icon (180x180)
- `icon-192.png` - PWA icon (192x192)
- `icon-512.png` - PWA icon (512x512)

### PWA Manifest
- `manifest.json` - Progressive Web App manifest

## Usage in Code

Import assets using the `ASSETS` constant from `src/lib/assets.ts`:

```tsx
import { ASSETS } from '@/lib/assets';
import Image from 'next/image';

<Image src={ASSETS.logo.full} alt="POE2 Overlord" width={200} height={200} />
```

## Regenerating Icons

If the logo changes, regenerate icons using ImageMagick:

```bash
cd packages/website/public

# Create multi-size favicon
magick logo-no-text-square.png -resize 16x16 favicon-16.png
magick logo-no-text-square.png -resize 32x32 favicon-32.png
magick logo-no-text-square.png -resize 48x48 favicon-48.png
magick favicon-16.png favicon-32.png favicon-48.png favicon.ico
rm favicon-16.png favicon-32.png favicon-48.png

# Create touch icons
magick logo-no-text-square.png -resize 180x180 apple-touch-icon.png
magick logo-no-text-square.png -resize 192x192 icon-192.png
magick logo-no-text-square.png -resize 512x512 icon-512.png
```
