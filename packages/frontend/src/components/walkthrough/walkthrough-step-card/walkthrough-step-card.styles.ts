// Walkthrough Step Card Styles
// Uses stone/ember/molten color palette from theme

export const walkthroughStepCardStyles = {
  // No active step state
  noStepCard: 'border-ember-700/50 bg-ember-500/10',
  noStepContent: 'text-center py-8',
  noStepIcon: 'w-16 h-16 text-stone-400 mx-auto mb-4',
  noStepTitle: 'text-lg font-semibold text-white mb-2',
  noStepText: 'text-stone-300 mb-4',

  // Active step card
  activeCard: 'border-ember-700/50 bg-ember-500/10',

  // Zone flow (Current → Completion)
  zoneFlow: 'flex items-center justify-end gap-2 px-4 text-sm',
  zoneFlowCurrent:
    '!text-stone-300 hover:!text-stone-200 underline decoration-stone-500/50 hover:decoration-stone-400 font-medium',
  zoneFlowArrow: 'w-4 h-4 text-ember-400 flex-shrink-0',
  zoneFlowTarget:
    '!text-stone-200 hover:!text-white underline decoration-ember-500/50 hover:decoration-ember-400 font-semibold',

  // Description section
  descriptionText: 'text-sm text-stone-400 leading-relaxed px-4',

  // Footer (extends to card edges)
  footer:
    'flex justify-between items-center py-2 px-4 border-t border-stone-700/30 -mx-4 -mb-4 mt-4',
  footerTimestamp: 'flex items-center gap-2 text-xs text-stone-500',
  footerTimestampIcon: 'w-3 h-3',
  footerActions: 'flex gap-2',
  footerActionsEnd: 'flex justify-end w-full',
} as const;
