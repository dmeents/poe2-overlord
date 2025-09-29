// Dashboard Insights Styles
// Centralized styling utilities for the DashboardInsights component

export const dashboardInsightsStyles = {
  container: 'bg-zinc-800/50 border border-zinc-700/50 p-6 shadow-lg',
  title: 'text-lg font-semibold text-white mb-6 flex items-center',
  featuredSection: 'mt-6 mb-4 p-4 bg-zinc-900/80 border border-zinc-700/50',
  featuredTitle: 'text-sm font-medium text-zinc-300 mb-2 flex items-center',
  featuredValue: 'text-white font-medium',
  featuredSubtext: 'text-zinc-400 text-sm',
  grid: 'grid grid-cols-2 gap-4',
  loadingContainer: 'animate-pulse',
  loadingItem: 'h-4 bg-zinc-700 rounded mb-2',
  loadingValue: 'h-6 bg-zinc-700 rounded',
  statItem: 'text-center p-4 bg-zinc-900/80 border border-zinc-700/50',
  statLabel: 'text-sm text-zinc-400 uppercase tracking-wide',
  statValue: 'text-2xl font-bold text-white mb-1',
  statValueSmall: 'text-2xl font-bold text-white mb-1',
  statValueZone: 'text-2xl font-bold text-white mb-1 break-words leading-tight',
  statSubtext: 'text-zinc-400 text-xs',
  distributionSection: 'mt-6 space-y-4',
  distributionTitle: 'text-sm font-medium text-zinc-300 mb-3 flex items-center',
  distributionItem:
    'flex items-center justify-between p-3 bg-zinc-900/80 border border-zinc-700/50',
  distributionLabel: 'text-zinc-300 font-medium',
  distributionValue: 'text-zinc-400 text-sm',
} as const;
