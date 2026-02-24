# PRD: Website Package and Shared Theme

## Context

The POE2 Overlord desktop app (Tauri + React) needs a marketing website for downloads, documentation, and interactive tools. Adding a Next.js website to the existing monorepo requires extracting the Tailwind v4 design tokens into a shared theme package so both apps maintain visual consistency without duplicating the color/font/spacing definitions.

**Priority:** Medium
**Estimated Effort:** 1-2 sessions

## Proposed Solution

Add two new workspace packages:
- `packages/theme` (`@poe2-overlord/theme`) - Shared Tailwind v4 tokens + JS utilities
- `packages/website` (`@poe2-overlord/website`) - Next.js 15 app

### Dependency Graph

```
@poe2-overlord/theme        (no workspace deps)
    |
    +--- @poe2-overlord/frontend   (depends on theme)
    |        |
    |        +--- @poe2-overlord/backend  (Tauri builds frontend)
    |
    +--- @poe2-overlord/website    (depends on theme)
```

---

## Phase 1: Create `packages/theme`

### Structure

```
packages/theme/
  package.json
  tsconfig.json
  src/
    css/
      tokens.css      # @theme block extracted from globals.css
    cn.ts             # cn() utility (clsx + tailwind-merge)
    theme-utils.ts    # getThemeHexColor() runtime CSS var reader
    index.ts          # JS barrel export
```

### `package.json`

```json
{
  "name": "@poe2-overlord/theme",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "exports": {
    ".": "./src/index.ts",
    "./tokens.css": "./src/css/tokens.css"
  },
  "main": "./src/index.ts",
  "types": "./src/index.ts",
  "dependencies": {
    "clsx": "^2.1.1",
    "tailwind-merge": "^3.4.0"
  },
  "devDependencies": {
    "typescript": "~5.9.3"
  },
  "scripts": {
    "typecheck": "tsc --noEmit"
  }
}
```

No build step. Vite (frontend) consumes TS source natively. Next.js uses `transpilePackages`. This avoids a build/watch step and keeps dev experience simple.

### `tokens.css`

Extract lines 6-117 from `packages/frontend/src/globals.css` (the entire `@theme { }` block) into this file. This includes:
- Font families (`--font-sans`, `--font-cursive`)
- All color scales (ember, molten, blood, arcane, verdant, spirit, hex, primal, bone, ash, stone)
- Shadow definitions (`--shadow-sm` through `--shadow-left`)
- Layout spacing (`--spacing-sidebar`, `--spacing-titlebar`, `--spacing-statusbar`)

Does NOT include:
- Google Fonts `@import url()` (each app loads fonts independently)
- `@import "tailwindcss"` (each app imports Tailwind themselves)
- WebKitGTK utility classes - these stay in frontend per ADR-002
- App-specific styles (scrollbar, select focus, `.app-background`)

### JS Utilities

**`cn.ts`** - Move from `packages/frontend/src/utils/tailwind.ts`:
```ts
import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
```

**`theme-utils.ts`** - Move from `packages/frontend/src/utils/theme-utils.ts`:
```ts
export function getThemeHexColor(cssVar: string): string {
  return getComputedStyle(document.documentElement).getPropertyValue(`--color-${cssVar}`).trim();
}
```

**`index.ts`** - Barrel export:
```ts
export { cn } from './cn';
export { getThemeHexColor } from './theme-utils';
```

### `tsconfig.json`

Match frontend's compiler options. Include `DOM` in lib since `getThemeHexColor()` uses `document`.

### What stays in frontend

| Utility | Decision | Reason |
|---|---|---|
| `cn()` | MOVE to theme | Generic Tailwind utility |
| `getThemeHexColor()` | MOVE to theme | Generic CSS var reader |
| `class-colors.ts` | STAYS | Game-domain mapping (Warrior -> blood) |
| `league-colors.ts` | STAYS | Game-domain mapping |
| `act-colors.ts` | STAYS | Game-domain mapping |

---

## Phase 2: Migrate `packages/frontend` to Consume Theme

### Add dependency

In `packages/frontend/package.json`:
- Add `"@poe2-overlord/theme": "workspace:*"` to dependencies
- Remove `clsx` and `tailwind-merge` from direct dependencies (now transitive via theme)

### Update `globals.css`

Replace the `@theme` block with an import. The file becomes:

```css
@import url("https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&display=swap");
@import url("https://fonts.googleapis.com/css2?family=Macondo&display=swap");

@import "tailwindcss";
@import "@poe2-overlord/theme/tokens.css";

/* Everything below this line stays unchanged */
html, body { ... }
.app-background { ... }
/* select, input, scrollbar styles */
/* .card-shadow, .chrome-shadow-*, .glow-*, .text-glow-* (ADR-002) */
```

**Critical:** `@import "tailwindcss"` must come before `@import "@poe2-overlord/theme/tokens.css"`. Tailwind v4 merges imported `@theme` blocks with the base theme.

### Update JS imports (6 files)

**3 files importing `cn`:**
- `packages/frontend/src/components/ui/button/button.tsx`
- `packages/frontend/src/components/ui/accordion/accordion.tsx`
- `packages/frontend/src/components/ui/modal/modal.tsx`

Change: `import { cn } from '@/utils/tailwind'` -> `import { cn } from '@poe2-overlord/theme'`

**3 files importing `getThemeHexColor`:**
- `packages/frontend/src/utils/class-colors.ts`
- `packages/frontend/src/utils/league-colors.ts`
- `packages/frontend/src/utils/act-colors.ts`

Change: `import { getThemeHexColor } from './theme-utils'` -> `import { getThemeHexColor } from '@poe2-overlord/theme'`

### Delete migrated files

- `packages/frontend/src/utils/tailwind.ts`
- `packages/frontend/src/utils/theme-utils.ts`

### Verify

```bash
pnpm dev    # Tauri dev mode - verify theme renders correctly
pnpm build  # Production build - verify no regressions
pnpm test   # Run tests
```

---

## Phase 3: Create `packages/website`

### Structure

```
packages/website/
  package.json
  next.config.ts
  postcss.config.mjs
  tsconfig.json
  src/
    app/
      layout.tsx        # Root layout with next/font, theme import
      page.tsx          # Landing page
      globals.css       # Tailwind + shared tokens + website-specific styles
      downloads/
        page.tsx        # Downloads page
      docs/
        page.tsx        # Docs index
    components/
      header.tsx
      footer.tsx
  public/
    favicon.ico
```

### `package.json`

```json
{
  "name": "@poe2-overlord/website",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "lint": "biome check .",
    "format": "biome check --write .",
    "typecheck": "tsc --noEmit"
  },
  "dependencies": {
    "@poe2-overlord/theme": "workspace:*",
    "next": "^15.3.0",
    "react": "^19.2.3",
    "react-dom": "^19.2.3"
  },
  "devDependencies": {
    "@biomejs/biome": "^2.3.14",
    "@tailwindcss/postcss": "^4.1.18",
    "@types/node": "^24.10.7",
    "@types/react": "^19.2.8",
    "@types/react-dom": "^19.2.3",
    "tailwindcss": "^4.1.18",
    "typescript": "~5.9.3"
  }
}
```

Note: Uses `@tailwindcss/postcss` (not `@tailwindcss/vite`) since Next.js uses PostCSS for CSS processing.

### `postcss.config.mjs`

```js
export default {
  plugins: {
    '@tailwindcss/postcss': {},
  },
};
```

### `next.config.ts`

```ts
import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  transpilePackages: ['@poe2-overlord/theme'],
};

export default nextConfig;
```

`transpilePackages` lets Next.js compile the theme package's TypeScript source directly.

### `src/app/globals.css`

```css
@import "tailwindcss";
@import "@poe2-overlord/theme/tokens.css";

/* Website-specific styles */
/* No WebKitGTK constraints - standard box-shadow and backdrop-filter work fine */

html, body {
  background-color: #0c0a09;
  color: #fafaf9;
}
```

No Google Fonts CSS import - fonts loaded via `next/font/google` in the layout for automatic optimization.

### `src/app/layout.tsx`

```tsx
import type { Metadata } from 'next';
import { Roboto, Macondo } from 'next/font/google';
import './globals.css';

const roboto = Roboto({
  subsets: ['latin'],
  variable: '--font-sans',
  display: 'swap',
});

const macondo = Macondo({
  weight: '400',
  subsets: ['latin'],
  variable: '--font-cursive',
  display: 'swap',
});

export const metadata: Metadata = {
  title: 'POE2 Overlord - Companion App for Path of Exile 2',
  description: 'Track characters, zone statistics, campaign progress, and more.',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className={`${roboto.variable} ${macondo.variable}`}>
      <body className="font-sans antialiased">{children}</body>
    </html>
  );
}
```

Font `variable` props create CSS custom properties (`--font-sans`, `--font-cursive`) that align with the theme's font declarations, giving us optimized font loading per platform.

### `src/app/page.tsx`

Minimal landing page to validate theme tokens work:

```tsx
export default function Home() {
  return (
    <main className="min-h-screen">
      <section className="mx-auto max-w-6xl px-6 py-24 text-center">
        <h1 className="font-cursive text-5xl text-ember-400">POE2 Overlord</h1>
        <p className="mt-4 text-lg text-bone-200">
          A powerful companion app for Path of Exile 2
        </p>
      </section>
    </main>
  );
}
```

---

## Phase 4: Root Configuration Updates

### Root `package.json` - Add scripts

```json
"dev:website": "pnpm --filter @poe2-overlord/website dev",
"build:website": "pnpm --filter @poe2-overlord/website build",
"start:website": "pnpm --filter @poe2-overlord/website start",
"lint:website": "pnpm --filter @poe2-overlord/website lint",
"typecheck:website": "pnpm --filter @poe2-overlord/website typecheck"
```

Update `typecheck` script to include all packages:
```json
"typecheck": "pnpm --filter '@poe2-overlord/*' --parallel run typecheck"
```

Existing `format`, `lint`, `test` scripts use `--filter '@poe2-overlord/*'` glob and automatically pick up new packages.

### `biome.json` - Add `.next` ignore

Add `"!**/.next"` to the `files.includes` array.

### `.gitignore` - Add Next.js entries

```
# Next.js
.next/
out/
.vercel
```

### `pnpm-workspace.yaml` - No changes

Already uses `packages/*` glob which auto-discovers new packages.

### Vercel Deployment

**`vercel.json`** at monorepo root:
```json
{
  "framework": "nextjs",
  "buildCommand": "pnpm --filter @poe2-overlord/website build",
  "outputDirectory": "packages/website/.next",
  "installCommand": "pnpm install"
}
```

**`packages/website/vercel-ignore.sh`** - Skip builds when only desktop app changes:
```bash
#!/bin/bash
echo "Checking for changes in packages/website and packages/theme..."
git diff HEAD^ HEAD --quiet packages/website/ packages/theme/
```

Set in Vercel project settings: Ignored Build Step -> `bash packages/website/vercel-ignore.sh`

---

## Phase 5: Update AI Memory and Docs

### `.ai/memory/decisions.md` - Add ADR-003

Document the shared theme architecture decision: no build step, CSS token export via `@import`, font loading per-platform, WebKitGTK workarounds stay in frontend.

### `.ai/memory/patterns.md` - Add shared theme import pattern

Document how new packages should import the theme (CSS `@import` + JS `import from`).

### `CLAUDE.md` - Update architecture section

Add theme and website packages to the project overview, document new scripts.

---

## Files to Create

| File | Purpose |
|---|---|
| `packages/theme/package.json` | Package config with exports |
| `packages/theme/tsconfig.json` | TypeScript config |
| `packages/theme/src/css/tokens.css` | Extracted @theme block |
| `packages/theme/src/cn.ts` | cn() utility |
| `packages/theme/src/theme-utils.ts` | getThemeHexColor() |
| `packages/theme/src/index.ts` | Barrel export |
| `packages/website/package.json` | Next.js package config |
| `packages/website/next.config.ts` | Next.js config with transpilePackages |
| `packages/website/postcss.config.mjs` | PostCSS with @tailwindcss/postcss |
| `packages/website/tsconfig.json` | TypeScript config for Next.js |
| `packages/website/src/app/globals.css` | Tailwind + shared tokens |
| `packages/website/src/app/layout.tsx` | Root layout with next/font |
| `packages/website/src/app/page.tsx` | Landing page stub |
| `packages/website/src/app/downloads/page.tsx` | Downloads page stub |
| `packages/website/src/app/docs/page.tsx` | Docs page stub |
| `packages/website/src/components/header.tsx` | Header component stub |
| `packages/website/src/components/footer.tsx` | Footer component stub |
| `vercel.json` | Vercel deployment config |
| `packages/website/vercel-ignore.sh` | Build skip script |

## Files to Modify

| File | Change |
|---|---|
| `packages/frontend/src/globals.css` | Replace @theme block with `@import` from theme |
| `packages/frontend/package.json` | Add theme dep, remove clsx/tailwind-merge |
| `packages/frontend/src/components/ui/button/button.tsx` | Update cn import |
| `packages/frontend/src/components/ui/accordion/accordion.tsx` | Update cn import |
| `packages/frontend/src/components/ui/modal/modal.tsx` | Update cn import |
| `packages/frontend/src/utils/class-colors.ts` | Update getThemeHexColor import |
| `packages/frontend/src/utils/league-colors.ts` | Update getThemeHexColor import |
| `packages/frontend/src/utils/act-colors.ts` | Update getThemeHexColor import |
| `package.json` (root) | Add website scripts, update typecheck |
| `biome.json` | Add .next ignore |
| `.gitignore` | Add Next.js entries |
| `.ai/memory/decisions.md` | Add ADR-003 |
| `.ai/memory/patterns.md` | Add shared theme pattern |
| `CLAUDE.md` | Update architecture docs |

## Files to Delete

| File | Reason |
|---|---|
| `packages/frontend/src/utils/tailwind.ts` | Moved to @poe2-overlord/theme |
| `packages/frontend/src/utils/theme-utils.ts` | Moved to @poe2-overlord/theme |

## Risk Mitigation

1. **CSS @import resolution** - Both Vite and PostCSS Tailwind plugins resolve bare specifiers through node_modules. pnpm workspace symlinks make `@poe2-overlord/theme/tokens.css` resolvable. Fallback: relative path `../../theme/src/css/tokens.css`.

2. **Tailwind v4 @theme merging** - Tailwind v4 explicitly supports `@theme` in imported files. Import order matters: `@import "tailwindcss"` first, then the token file.

3. **transpilePackages** - Well-tested Next.js monorepo feature. Fallback: add `tsup` build step to theme package.

4. **Vercel pnpm workspace support** - Natively supported. `pnpm install` at root installs all workspace packages.

## Success Criteria

- [ ] `packages/theme` created with tokens, cn(), getThemeHexColor()
- [ ] `packages/frontend` imports theme from shared package
- [ ] `pnpm dev` (desktop) works identically - no visual regressions
- [ ] `pnpm build` (desktop) produces working binary
- [ ] `pnpm test` passes
- [ ] `packages/website` scaffolded with Next.js 15
- [ ] `pnpm dev:website` starts Next.js dev server with shared theme colors
- [ ] `pnpm build:website` produces production build
- [ ] Shared theme tokens (ember, stone, etc.) render correctly in website
- [ ] Linting and formatting pass across all packages
