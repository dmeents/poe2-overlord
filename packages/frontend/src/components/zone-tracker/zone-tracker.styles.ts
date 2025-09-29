// Zone Tracker Styles
// Centralized styling utilities for the ZoneTracker component

export const zoneTrackerStyles = {
  container: 'bg-zinc-800/50 border border-zinc-700/50 p-6 shadow-lg',
  header: 'flex items-center justify-between mb-6',
  title: 'flex items-center text-xl font-bold text-white',
  controlsToggle:
    'flex items-center justify-between px-4 py-2 bg-zinc-700/50 hover:bg-zinc-700 border border-zinc-600 text-zinc-300 hover:text-white transition-colors h-10 w-full',
  controls: 'space-y-3 mb-6',
  searchContainer: 'w-full',
  searchInput:
    'w-full h-10 px-4 py-2 bg-zinc-700/50 border border-zinc-600 text-white placeholder-zinc-400 focus:outline-none focus:ring-2 focus:ring-emerald-500/50 focus:border-emerald-500/50 transition-colors',
  filterSortContainer: 'space-y-3',
  filterButton:
    'flex items-center justify-between px-4 py-2 bg-zinc-700/50 hover:bg-zinc-700 border border-zinc-600 text-zinc-300 hover:text-white transition-colors h-10 w-full',
  sortButton:
    'flex items-center justify-between px-4 py-2 bg-zinc-700/50 hover:bg-zinc-700 border border-zinc-600 text-zinc-300 hover:text-white transition-colors h-10 w-full',
  resetButton:
    'px-4 py-2 h-10 bg-zinc-600/50 hover:bg-zinc-600 border border-zinc-500 text-zinc-300 hover:text-white transition-colors text-sm font-medium w-full',

  // Stats summary
  statsSummary:
    'grid grid-cols-2 sm:grid-cols-4 gap-4 mb-6 p-4 bg-zinc-900/80 border border-zinc-700/50',
  statItem: 'text-center',
  statValue: 'text-lg font-bold text-white',
  statLabel: 'text-sm text-zinc-400 uppercase tracking-wide',

  // Zone list
  zonesContainer: 'grid grid-cols-1 gap-4',
  emptyState: 'text-center py-12',
  emptyIcon: 'mx-auto h-16 w-16 text-zinc-500 mb-4',
  emptyTitle: 'text-lg font-medium text-zinc-300 mb-2',
  emptyDescription: 'text-zinc-500',

  // Filter dropdown
  filterDropdown:
    'absolute top-full left-0 mt-2 bg-zinc-800 border border-zinc-700 shadow-2xl z-10 p-6 w-96 min-w-max',
  filterGrid: 'grid grid-cols-1 sm:grid-cols-2 gap-6',
  filterGroup: 'space-y-2',
  filterLabel: 'text-sm font-medium text-zinc-300 uppercase tracking-wide',
  filterSelect:
    'w-full h-8 px-3 py-1 bg-zinc-700/50 border border-zinc-600 text-white text-sm focus:outline-none focus:ring-1 focus:ring-emerald-500/50 focus:border-emerald-500/50 transition-colors',
  filterCheckbox: 'flex items-center space-x-2',
  filterCheckboxInput:
    'w-4 h-4 text-emerald-500 bg-zinc-700 border-zinc-600 focus:ring-emerald-500/50 focus:ring-2',
  filterCheckboxLabel: 'text-sm text-zinc-300',

  // Sort dropdown
  sortDropdown:
    'absolute top-full left-0 mt-2 bg-zinc-800 border border-zinc-700 shadow-2xl z-10 p-4 w-64 min-w-max',
  sortOptions: 'space-y-2',
  sortOption:
    'flex items-center justify-between px-3 py-2 hover:bg-zinc-700/50 cursor-pointer transition-colors',
  sortOptionActive: 'bg-emerald-500/20 text-emerald-400',
  sortOptionLabel: 'text-sm text-zinc-300',
  sortOptionIcon: 'w-4 h-4 text-emerald-400',
} as const;
