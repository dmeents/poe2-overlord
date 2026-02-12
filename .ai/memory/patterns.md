# Code Patterns

## Theming & Styling

### Design Tokens (`globals.css`)

All design tokens are defined in `globals.css` under the `@theme` block. This is the **single source of truth** for colors, shadows, and spacing.

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

| Token   | Purpose                              |
|---------|--------------------------------------|
| `ember` | Primary accent (volcanic orange)     |
| `molten`| Secondary accent (gold/amber)        |
| `blood` | Danger states, hardcore mode         |
| `bone`  | Muted text, subtle highlights        |
| `stone` | Neutral backgrounds (warm gray)      |
| `ash`   | Disabled/muted states (cool gray)    |

### Shadow Scale

| Class          | Use case                    |
|----------------|-----------------------------|
| `shadow-sm`    | Subtle depth                |
| `shadow-md`    | Cards, dropdowns            |
| `shadow-lg`    | Modals, popovers            |
| `shadow-xl`    | Floating elements           |
| `shadow-top`   | Bottom-docked panels        |
| `shadow-right` | Left-docked panels (sidebar)|
| `shadow-bottom`| Top-docked panels (titlebar)|
| `shadow-left`  | Right-docked panels         |

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
