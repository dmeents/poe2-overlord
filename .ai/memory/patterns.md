# Code Patterns

## Theming & Styling

### Design Tokens (Shared Theme Package)

All design tokens are defined in `@poe2-overlord/theme` (`packages/theme/src/css/tokens.css`). This is the **single source of truth** for colors, shadows, and spacing shared between the desktop app and website.

```css
@theme {
  /* Colors */
  --color-ember-500: #f97316;
  --color-stone-900: #1c1917;

  /* Shadows - high opacity for dark backgrounds */
  --shadow-md: 0 4px 6px rgba(0, 0, 0, 0.7);
  --shadow-right: 4px 0 6px rgba(0, 0, 0, 0.7);

  /* Spacing */
  --spacing-titlebar: 28px;
}
```

**Never hardcode values in components.** Use Tailwind classes that reference these tokens:

```tsx
// Good - uses Tailwind classes
<div className="bg-stone-900 shadow-md">

// Bad - hardcoded values
<div className="bg-[#1c1917] shadow-[0_4px_6px_rgba(0,0,0,0.7)]">

// Good - CSS variable for non-standard values
<div className="top-[--spacing-titlebar]">
```

### Color Palette

**UI Colors:**

| Token   | Purpose                              |
|---------|--------------------------------------|
| `ember` | Primary accent (volcanic orange)     |
| `molten`| Secondary accent (gold/amber)        |
| `blood` | Danger states, hardcore mode         |
| `bone`  | Muted text, subtle highlights        |
| `stone` | Neutral backgrounds (warm gray)      |
| `ash`   | Disabled/muted states (cool gray)    |

**Class Colors** (character identity):

| Token     | Class     | Concept                    |
|-----------|-----------|----------------------------|
| `blood`   | Warrior   | Martial aggression         |
| `arcane`  | Sorceress | Mystical deep blue         |
| `verdant` | Ranger    | Forest moss green          |
| `molten`  | Huntress  | Golden predator            |
| `spirit`  | Monk      | Contemplative violet       |
| `ember`   | Mercenary | Volcanic fire              |
| `hex`     | Witch     | Dark magic magenta         |
| `primal`  | Druid     | Ancient nature teal        |

### Class Colors Utility

Use `@/utils/class-colors.ts` for character-specific styling:

```tsx
import { getClassTextColor, getClassBorderColor, getClassTheme } from '@/utils/class-colors';

// Returns Tailwind classes
getClassTextColor('Warrior')   // 'text-blood-400'
getClassBorderColor('Warrior') // 'border-blood-500'
getClassTheme('Warrior')       // 'blood'

// For charts (returns hex values from CSS variables)
getClassHexColor('Warrior')    // '#dc2626'
```

### Shadow Implementation

**IMPORTANT:** Due to a WebKitGTK compositor bug in Tauri on Linux, all shadows MUST use `filter: drop-shadow()` instead of `box-shadow`. Never mix shadow types. See ADR-002 in `decisions.md`.

**Use these filter-based shadow classes (defined in `globals.css`):**

| Class                 | Use case                              |
|-----------------------|---------------------------------------|
| `.card-shadow`        | Cards, elevated content               |
| `.chrome-shadow-top`  | Bottom-docked panels (statusbar)      |
| `.chrome-shadow-right`| Left-docked panels (sidebar)          |
| `.chrome-shadow-bottom`| Top-docked panels (titlebar)         |

**DO NOT use Tailwind's `shadow-*` utilities** (e.g., `shadow-md`, `shadow-lg`) - these use `box-shadow` and will cause rendering issues when mixed with `filter: drop-shadow()`.

The theme variables (`--shadow-top`, `--shadow-right`, etc.) are defined for reference but should only be used via the filter-based utility classes above.

### Z-Index Scale

Consistent layering prevents z-index conflicts. Use these values:

| Class   | Value | Use case                                      |
|---------|-------|-----------------------------------------------|
| `z-0`   | 0     | Base content (default)                        |
| `z-10`  | 10    | Elevated cards, hover states (if needed)      |
| `z-20`  | 20    | Dropdowns, popovers, tooltips                 |
| `z-30`  | 30    | Fixed UI chrome (titlebar, sidebar, statusbar)|
| `z-40`  | 40    | Notifications, toasts                         |
| `z-50`  | 50    | Modals, dialogs (blocking UI)                 |

**Guidelines:**
- Fixed chrome (titlebar, sidebar, statusbar) uses `z-30` to stay above content but below modals
- Dropdowns/tooltips use `z-20` - they appear above content but below fixed chrome
- Modals use `z-50` - they block everything else
- Avoid arbitrary z-index values; stick to the scale

**Note on shadows:** All shadows use `filter: drop-shadow()` with GPU hints due to a WebKitGTK compositor bug. See ADR-002 in `decisions.md` for the full investigation.

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
- Use theme colors (`stone-*`, `ember-*`) instead of default Tailwind colors
- Use theme shadows (`shadow-md`, `shadow-right`) instead of arbitrary values
- Keep styles in the `.styles.ts` file, not inline in components
- Reference CSS variables for spacing: `top-[--spacing-titlebar]`

### Background Pattern

The app uses a volcanic background image with a gradient overlay, applied via CSS multiple backgrounds in `globals.css`:

```css
.app-background {
  background:
    linear-gradient(...overlay...),
    url("/background.png") center / cover no-repeat fixed;
}
```

### Design System Reference

All design tokens are defined in `globals.css`. Refer to the `@theme` block for:
- Color palettes and their intended usage
- Shadow scale with high opacity for dark backgrounds
- Layout spacing constants

There is no separate theme.ts file - everything is CSS-native for proper Tailwind integration.

### Importing Shared Theme

**In CSS files:**
```css
@import "tailwindcss";
@import "@poe2-overlord/theme/tokens.css";
```

**CRITICAL:** `@import "tailwindcss"` must come BEFORE the theme import. Tailwind v4 merges imported `@theme` blocks with the base theme.

**In TypeScript/React files:**
```tsx
import { cn } from '@poe2-overlord/theme';
import { getThemeHexColor } from '@poe2-overlord/theme';

// cn() - merge and deduplicate Tailwind classes
const className = cn('bg-stone-900', props.className);

// getThemeHexColor() - read CSS variable value
const hexColor = getThemeHexColor('ember-500'); // Returns '#f97316'
```

**Game-domain utilities** (character/league/act colors) remain in frontend:
```tsx
import { getClassTextColor } from '@/utils/class-colors';
import { getLeagueHexColor } from '@/utils/league-colors';
import { getActHexColor } from '@/utils/act-colors';
```
