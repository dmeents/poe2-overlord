// HoverCard Styles
// Centralized styling for the shared HoverCard component
// Uses stone color palette from theme

export const hoverCardStyles = {
  container: 'relative inline-block',
  trigger: 'inline-flex items-center gap-1 cursor-pointer',
  icon: 'w-4 h-4 text-stone-400 hover:text-stone-300 transition-colors',
  // z-20: Tooltips/popovers (see CLAUDE.md z-index scale)
  // Width is applied dynamically via the `width` prop
  card: 'z-20 p-3 bg-stone-900 border border-stone-700 text-stone-200 text-sm card-shadow pointer-events-none rounded',
  content: 'relative',
  arrow:
    'absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-stone-900',
} as const;
