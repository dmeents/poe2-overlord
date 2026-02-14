// Accordion Styles
// Collapsible section divider pattern with clean ruled header
// Uses bone/stone tones when collapsed, ember accent when expanded

export const accordionStyles = {
  // No container background - cards breathe against page background
  container: '',

  // Full-width button with group class for coordinated hover
  button: 'group flex items-center gap-3 w-full text-left transition-colors cursor-pointer py-3',

  // Chevron icon - rotates 180deg when expanded
  icon: 'w-4 h-4 transition-transform duration-200',
  iconCollapsed: 'text-stone-400 group-hover:text-stone-300',
  iconExpanded: 'text-ember-400 rotate-180',

  // Title - matches SectionHeader typography
  title: 'text-xs font-medium uppercase tracking-wider transition-colors',
  titleCollapsed: 'text-bone-200 group-hover:text-bone-100',
  titleExpanded: 'text-ember-400',

  // Subtitle - separated by dot
  subtitle: 'text-xs transition-colors',
  subtitleCollapsed: 'text-stone-400 group-hover:text-stone-300',
  subtitleExpanded: 'text-stone-300',

  // Divider line - stretches to fill remaining width
  divider: 'flex-1 h-px transition-colors',
  dividerCollapsed: 'bg-stone-600/50 group-hover:bg-stone-500/50',
  dividerExpanded: 'bg-ember-500/20',

  // Content animation wrapper - CSS Grid trick for smooth height transitions
  contentWrapper: 'grid transition-[grid-template-rows] duration-200 ease-out',
  contentWrapperCollapsed: 'grid-rows-[0fr]',
  contentWrapperExpanded: 'grid-rows-[1fr]',

  // Content section - needs overflow-hidden for grid animation
  content: 'overflow-hidden',
} as const;
