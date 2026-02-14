export const chartStyles = {
  container: 'space-y-6',
  chartWrapper: 'relative flex items-center justify-center h-40',
  // Dark backdrop creates a subtle "well" effect behind the donut
  chartBackdrop: 'absolute inset-0 bg-stone-950/20',
  pie: 'transition-all duration-300',
  // Add subtle glow and stronger hover effect to slices
  cell: 'transition-all duration-300 hover:brightness-110 hover:saturate-150 cursor-pointer drop-shadow-[0_0_8px_currentColor]',
  // z-10: Below tooltips (z-20) to prevent overlap
  centerStats:
    'absolute inset-0 flex flex-col items-center justify-center pointer-events-none z-10',
  centerValue: 'text-2xl font-bold text-stone-50',
  centerLabel: 'text-xs text-stone-400 uppercase tracking-wide',
};
