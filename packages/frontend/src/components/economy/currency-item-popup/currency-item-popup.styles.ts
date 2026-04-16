// Currency Item Popup Styles
// Hover popup for economy currency icons showing enriched game data

export const currencyItemPopupStyles = {
  // Wrapper around the trigger element — must not break parent flex layout
  trigger: 'flex-shrink-0',

  // Popup card rendered via portal
  // z-20: tooltips/popovers layer (see CLAUDE.md z-index scale)
  popup: 'z-20 w-56 bg-stone-900 border border-stone-700 rounded card-shadow pointer-events-none',

  // Content area
  content: 'p-3 flex flex-col items-center gap-2',

  // Large item image in the popup
  image: 'w-16 h-16 object-contain',

  // Item name
  name: 'text-white font-semibold text-sm text-center',

  // Category badge
  categoryBadge: 'text-xs text-stone-400 bg-stone-800 px-2 py-0.5 rounded',

  // Currency description text
  description: 'text-xs text-stone-300 leading-relaxed text-left w-full',

  // Metadata row (stack size, drop level, etc.)
  metaGrid: 'w-full grid grid-cols-2 gap-x-2 gap-y-0.5',
  metaLabel: 'text-xs text-stone-500',
  metaValue: 'text-xs text-stone-300 font-medium text-right',

  // Mod lines section (divider + mod text)
  modSection: 'w-full border-t border-stone-700/50 pt-2 flex flex-col gap-0.5',
  modLine: 'text-xs text-molten-300 leading-snug',

  // Flavour text
  flavourText: 'text-xs text-stone-500 italic text-center leading-snug',

  // Fallback text when no game data
  noData: 'text-xs text-stone-600 italic',

  // Arrow pointing down (matches existing Tooltip arrow pattern)
  arrow:
    'absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-stone-900',
} as const;
