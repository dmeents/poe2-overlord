// Item Tooltip Styles
// Content-only styles for the item tooltip popup.
// Shell styles (width, background, border, arrow, z-index) are handled by HoverCard.

export const itemTooltipStyles = {
  // Content area
  content: 'p-3 flex flex-col items-center gap-2',

  // Item image
  image: 'w-16 h-16 object-contain',

  // Item name — use a rarity variant alongside this base class
  name: 'font-semibold text-sm text-center',
  nameNormal: 'text-stone-50',
  nameMagic: 'text-arcane-400',
  nameRare: 'text-molten-300',
  nameUnique: 'text-molten-300',

  // Category badge
  categoryBadge: 'text-xs text-stone-400 bg-stone-800 px-2 py-0.5 rounded',

  // Description text (currency use text)
  description: 'text-xs text-stone-300 leading-relaxed text-left w-full',
  descriptionLink: 'text-molten-300 font-medium',

  // Stats grid — shared by weapon, defence, gem, flask, and meta sections
  statsGrid: 'w-full grid grid-cols-2 gap-x-2 gap-y-0.5',
  statsLabel: 'text-xs text-stone-500',
  statsValue: 'text-xs text-stone-300 font-medium text-right',

  // Section — adds a top divider before a group of stats
  section: 'w-full border-t border-stone-700/50 pt-2 flex flex-col gap-0.5',
  sectionHeader: 'text-xs text-stone-500 uppercase tracking-wide mb-0.5',

  // Mod lines
  modLine: 'text-xs text-molten-300 leading-snug',

  // Flavour text
  flavourText: 'text-xs text-stone-500 italic text-center leading-snug',

  // Fallback text when no game data is available
  noData: 'text-xs text-stone-600 italic',
} as const;
