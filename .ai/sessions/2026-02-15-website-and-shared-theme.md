# Session: Website and Shared Theme Implementation

**Date:** 2026-02-15
**PRD:** `.ai/archive/completed-prds/2026-02-15-prd-website-and-shared-theme.md`
**Status:** ✅ Complete

## Summary

Successfully implemented shared theme package and Next.js website per PRD specification. All success criteria met.

## Implementation Phases

### Phase 1: Create `packages/theme` ✅
- Created `@poe2-overlord/theme` package with no build step
- Extracted `@theme` block from frontend's `globals.css` to `tokens.css`
- Moved `cn()` and `getThemeHexColor()` utilities to shared package
- Set up TypeScript config with DOM lib for `document` access

### Phase 2: Migrate Frontend ✅
- Updated frontend to depend on `@poe2-overlord/theme`
- Replaced frontend's `@theme` block with `@import` from shared package
- Updated 6 files to import utilities from theme package:
  - 3 components importing `cn`: button.tsx, accordion.tsx, modal.tsx
  - 3 utils importing `getThemeHexColor`: class-colors.ts, league-colors.ts, act-colors.ts
- Deleted old utility files: `tailwind.ts`, `theme-utils.ts`
- Removed `clsx` and `tailwind-merge` from frontend dependencies (now transitive)

### Phase 3: Create Website ✅
- Scaffolded `packages/website` with Next.js 15
- Configured `transpilePackages` to consume theme as TypeScript source
- Used `next/font/google` for optimized font loading
- Created basic pages: home, downloads, docs
- Created placeholder header and footer components
- Added PostCSS config with `@tailwindcss/postcss`

### Phase 4: Root Configuration ✅
- Added website scripts to root `package.json`
- Updated `typecheck` script to cover all packages
- Added `.next/` to `biome.json` ignore list
- Added Next.js entries to `.gitignore`
- Created `vercel.json` for deployment config
- Created `vercel-ignore.sh` to skip builds when only desktop app changes

### Phase 5: Update Documentation ✅
- Added ADR-003 to `.ai/memory/decisions.md` documenting shared theme architecture
- Updated `.ai/memory/patterns.md` with shared theme import patterns
- Updated `CLAUDE.md` with new packages and commands

## Technical Details

**Font Loading Strategy:**
- Desktop (frontend): Google Fonts CSS imports (bundled in Tauri)
- Website: `next/font/google` for automatic optimization and self-hosting

**WebKitGTK Workarounds:**
- Shadow utilities (`.card-shadow`, `.chrome-shadow-*`) remain in frontend
- Website can use standard `box-shadow` without compositor issues

**No Build Step:**
- Theme consumed as TypeScript source
- Vite (frontend) handles TS natively
- Next.js uses `transpilePackages`

## Verification

```bash
pnpm install         # ✅ All dependencies installed
pnpm typecheck       # ✅ All packages type-check
pnpm format          # ✅ Fixed 8 files (4 frontend, 4 website)
pnpm lint            # ✅ All packages pass linting
pnpm test            # ✅ 688 frontend + 512 backend tests pass
pnpm dev:website     # ✅ Website starts on localhost:3000 in 753ms
```

## Files Created (16)

**Theme Package:**
- `packages/theme/package.json`
- `packages/theme/tsconfig.json`
- `packages/theme/src/css/tokens.css`
- `packages/theme/src/cn.ts`
- `packages/theme/src/theme-utils.ts`
- `packages/theme/src/index.ts`

**Website Package:**
- `packages/website/package.json`
- `packages/website/next.config.ts`
- `packages/website/postcss.config.mjs`
- `packages/website/tsconfig.json`
- `packages/website/next-env.d.ts`
- `packages/website/src/global.d.ts`
- `packages/website/src/app/globals.css`
- `packages/website/src/app/layout.tsx`
- `packages/website/src/app/page.tsx`
- `packages/website/src/app/downloads/page.tsx`
- `packages/website/src/app/docs/page.tsx`
- `packages/website/src/components/header.tsx`
- `packages/website/src/components/footer.tsx`

**Deployment:**
- `vercel.json`
- `packages/website/vercel-ignore.sh`

## Files Modified (15)

- `packages/frontend/src/globals.css` - Import tokens from theme
- `packages/frontend/package.json` - Add theme dependency
- `packages/frontend/src/components/ui/button/button.tsx`
- `packages/frontend/src/components/ui/accordion/accordion.tsx`
- `packages/frontend/src/components/ui/modal/modal.tsx`
- `packages/frontend/src/utils/class-colors.ts`
- `packages/frontend/src/utils/league-colors.ts`
- `packages/frontend/src/utils/act-colors.ts`
- `package.json` (root) - Add website scripts
- `biome.json` - Add .next ignore
- `.gitignore` - Add Next.js entries
- `.ai/memory/decisions.md` - Add ADR-003
- `.ai/memory/patterns.md` - Add shared theme patterns
- `CLAUDE.md` - Update architecture docs

## Files Deleted (2)

- `packages/frontend/src/utils/tailwind.ts` - Moved to theme
- `packages/frontend/src/utils/theme-utils.ts` - Moved to theme

## Learnings

1. **Next.js TypeScript**: Needed `next-env.d.ts` and CSS module declarations for imports
2. **Biome Formatting**: Auto-fixed import order in 8 files
3. **pnpm Workspace**: Auto-discovers new packages via `packages/*` glob
4. **Tailwind v4**: `@import "tailwindcss"` must come before theme imports for proper merging

## Next Steps

Website is now ready for:
- Content development (download links, documentation)
- GitHub releases integration
- Analytics setup
- SEO optimization
- Deployment to Vercel
