# Architecture Decisions

## ADR-001: Centralized Theme System

**Date:** 2025-02-12

**Status:** Accepted

**Context:**
The application needed a consistent design language with colors extracted from the logo and background assets (volcanic/infernal aesthetic).

**Decision:**
- Create a centralized `theme.ts` file containing all design tokens
- Register custom colors in `globals.css` `@theme` block for Tailwind usage
- Use semantic color names (ember, molten, blood, bone, stone) instead of generic Tailwind defaults
- Apply background image with gradient overlay using CSS multiple backgrounds (no z-index needed)

**Consequences:**
- All color changes can be made in one place
- Consistent visual language across components
- Component `.styles.ts` files should use theme colors, not hardcoded values
- Migration needed for existing components still using `zinc-*` or `emerald-*` colors
