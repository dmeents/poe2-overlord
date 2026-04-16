// Currency Item Popup Styles
// Content-only styles for the currency item popup.
// Shell styles (width, background, border, arrow, z-index) are handled by HoverCard.

export const currencyItemPopupStyles = {
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
} as const;
