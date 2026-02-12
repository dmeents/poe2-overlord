# Code Patterns

## Theming & Styling

### Theme File (`packages/frontend/src/theme.ts`)

All reusable colors and design tokens **must** be defined in the centralized theme file. This ensures consistency across the application and makes it easy to update the design system.

**What belongs in `theme.ts`:**
- Color palettes (ember, molten, blood, bone, stone)
- Semantic color mappings (background, text, interactive, state, border)
- Typography tokens (font families, sizes)
- Spacing constants
- Effect definitions (glows, shadows, overlays)
- Pre-composed Tailwind class combinations (`tw` helpers)

**Example usage:**
```tsx
import { colors, semanticColors, tw } from '@/theme';

// Use semantic colors for consistency
<div style={{ background: semanticColors.background.surface }}>

// Use tw helpers for common patterns
<button className={tw.button.primary}>
```

### Tailwind Custom Colors (`globals.css`)

Custom colors are registered in `globals.css` under the `@theme` block so they can be used as Tailwind classes:

```css
@theme {
  --color-ember-500: #f97316;
  --color-stone-900: #1c1917;
  /* etc. */
}
```

This enables usage like `bg-ember-500`, `text-stone-400`, `border-blood-600`.

### Color Palette

| Token | Purpose |
|-------|---------|
| `ember` | Primary accent (volcanic orange) |
| `molten` | Secondary accent (gold/amber) |
| `blood` | Danger states, hardcore mode |
| `bone` | Muted text, subtle highlights |
| `stone` | Neutral backgrounds (warm gray) |

### Component Styles

Each component has a co-located `.styles.ts` file containing Tailwind class compositions:

```
components/
  ui/
    button/
      button.tsx
      button.styles.ts  # Contains buttonStyles object
```

**Guidelines:**
- Use theme colors (`stone-*`, `ember-*`) instead of default Tailwind colors (`zinc-*`, `emerald-*`)
- Keep styles in the `.styles.ts` file, not inline in components
- Reference `theme.ts` for color values when needed in JS

### Background Pattern

The app uses a volcanic background image with a gradient overlay, applied via CSS multiple backgrounds in `globals.css`:

```css
.app-background {
  background:
    linear-gradient(...overlay...),
    url("/background.png") center / cover no-repeat fixed;
}
```

This approach avoids z-index complexity by compositing the overlay directly onto the image.
