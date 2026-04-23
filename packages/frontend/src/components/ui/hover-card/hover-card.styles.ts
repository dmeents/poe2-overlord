// HoverCard Styles
// Centralized styling for the shared HoverCard component.
// Positioning is handled by @floating-ui/react — these classes cover appearance only.
// Uses stone color palette from theme.

export const hoverCardStyles = {
  container: 'relative inline-block',
  trigger: 'inline-flex items-center gap-1 cursor-pointer',
  icon: 'w-4 h-4 text-stone-400 hover:text-stone-300 transition-colors',
  // z-40: Must exceed z-30 fixed chrome (sidebar, titlebar). Semantically sits between
  // dropdowns (z-20) and notifications (z-40) — value is correct but shares the toast layer.
  // Width is applied dynamically via the `width` prop.
  card: 'z-40 p-3 bg-stone-900 border border-stone-700 text-stone-200 text-sm card-shadow rounded',
  // FloatingArrow SVG — filled to match card background, stroked to match border.
  arrow:
    'fill-stone-900 [&>path:first-of-type]:stroke-stone-700 [&>path:last-of-type]:stroke-stone-900',
} as const;
