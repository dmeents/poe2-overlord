// Walkthrough Step Card Styles
// Uses stone/arcane/ember/spirit color palette from theme

export const walkthroughStepCardStyles = {
  // No active step state
  noStepCard: 'border-arcane-500 bg-arcane-500/10',
  noStepContent: 'text-center py-8',
  noStepIcon: 'w-16 h-16 text-stone-400 mx-auto mb-4',
  noStepTitle: 'text-lg font-semibold text-white mb-2',
  noStepText: 'text-stone-300 mb-4',
  // Active step card
  activeCard: 'border-arcane-500 bg-arcane-500/10',
  // Completion zone section
  completionZoneContainer: 'bg-arcane-500/5 border border-arcane-500/20 p-3',
  completionZoneContent: 'flex items-center gap-2',
  completionZoneIcon: 'w-4 h-4 text-arcane-400 flex-shrink-0',
  completionZoneLabel: 'text-stone-300 font-medium text-sm',
  completionZoneLink:
    'text-stone-300 hover:text-stone-200 underline decoration-arcane-400 hover:decoration-arcane-300 cursor-pointer font-medium',
  // Description section
  descriptionContainer: 'bg-stone-800/30 border border-stone-700/20 p-3',
  descriptionText: 'text-sm text-stone-300',
  // Objectives section
  objectivesContainer: 'bg-stone-800/40 p-4 border border-stone-700/30',
  objectivesTitle: 'text-sm font-medium text-stone-200 mb-3',
  objectivesList: 'space-y-4',
  objectiveItem: 'text-xs',
  objectiveContent: 'flex items-start gap-2',
  objectiveBullet: 'w-1.5 h-1.5 rounded-full bg-stone-400 mt-1.5 flex-shrink-0',
  objectiveInner: 'flex-1 space-y-1',
  objectiveText: 'font-medium text-stone-200 flex items-center gap-2',
  objectiveRequired: 'w-3 h-3 text-ember-400',
  objectiveOptional: 'w-3 h-3 text-stone-400',
  objectiveDetails: 'border-l-2 border-stone-500 pl-2 ml-1.5 space-y-1',
  objectiveDetailsText: 'text-xs text-stone-400',
  objectiveNotesText: 'text-xs text-arcane-400 italic',
  rewardIcon: 'w-3 h-3 text-spirit-400',
  // Footer
  footer: 'flex justify-between items-center py-2 px-4 border-t border-stone-700/30',
  footerTimestamp: 'flex items-center gap-2 text-xs text-stone-500',
  footerTimestampIcon: 'w-3 h-3',
  footerActions: 'flex gap-2',
  footerActionsEnd: 'flex justify-end w-full',
} as const;
