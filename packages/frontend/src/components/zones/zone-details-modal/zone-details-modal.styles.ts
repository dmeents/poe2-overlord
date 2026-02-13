// Zone Details Modal Styles
// Uses stone/arcane/blood/spirit/verdant color palette from theme

export const zoneDetailsModalStyles = {
  container: 'space-y-6',
  // Unvisited zone
  unvisitedContainer: 'bg-stone-800/50 border border-stone-700/50 p-6 text-center',
  unvisitedContent: 'flex flex-col items-center gap-3',
  unvisitedIconContainer: 'w-12 h-12 rounded-full bg-stone-800 flex items-center justify-center',
  unvisitedIcon: 'w-6 h-6 text-stone-500',
  unvisitedTitle: 'text-lg font-medium text-stone-300',
  unvisitedText: 'text-sm text-stone-400 max-w-md',
  // Zone image
  imageContainer: 'relative w-full h-64 overflow-hidden bg-stone-800',
  image: 'w-full h-full object-cover',
  // Section containers
  section: 'bg-stone-800/50 p-4 border border-stone-700/50',
  sectionAlt: 'bg-stone-900/80 p-4 border border-stone-700/50',
  sectionTitle: 'text-sm font-medium text-stone-300 mb-2 flex items-center gap-2',
  sectionTitleWithMargin: 'text-sm font-medium text-stone-300 mb-3',
  sectionTitleWithIcon: 'text-sm font-medium text-stone-300 mb-3 flex items-center gap-2',
  sectionText: 'text-sm text-stone-400 leading-relaxed',
  // Grid
  grid: 'grid grid-cols-2 gap-3 text-sm',
  // Labels and values
  label: 'text-stone-500',
  value: 'ml-2 text-stone-300',
  valueMono: 'ml-2 text-stone-300 font-mono',
  valueMonoSmall: 'ml-2 text-stone-300 font-mono text-xs',
  valueDeaths: 'ml-2 text-blood-400',
  // Wiki link
  wikiButton:
    'flex items-center gap-2 text-arcane-400 hover:text-arcane-300 transition-colors cursor-pointer',
  // Tags/pills
  tagContainer: 'flex flex-wrap gap-2',
  tagBoss:
    'px-3 py-1.5 text-xs font-medium bg-blood-500/10 text-blood-400 border border-blood-500/30 rounded',
  tagNpc:
    'px-3 py-1.5 text-xs font-medium bg-arcane-500/10 text-arcane-400 border border-arcane-500/30 rounded',
  tagPoi:
    'px-3 py-1.5 text-xs font-medium bg-spirit-500/10 text-spirit-400 border border-spirit-500/30 rounded',
  tagConnected:
    'px-3 py-1.5 text-xs font-medium bg-verdant-500/10 text-verdant-400 border border-verdant-500/30 rounded hover:bg-verdant-500/20 hover:border-verdant-500/50 transition-colors cursor-pointer',
} as const;
