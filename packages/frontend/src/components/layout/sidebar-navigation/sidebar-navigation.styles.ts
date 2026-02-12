// Sidebar Navigation Styles
// Centralized styling utilities for the SidebarNavigation component
// Uses stone/ember color palette from theme

export const sidebarNavigationStyles = {
  // shadow: effects.shadow.right
  container:
    'fixed left-0 top-[28px] bottom-6 w-12 bg-stone-950/95 backdrop-blur-sm flex flex-col z-50 border-r border-stone-800/50 shadow-[4px_0_6px_rgba(0,0,0,0.7)]',
  logo: 'w-full h-12 flex items-center justify-center border-b border-stone-800/50',
  logoImage: 'w-8 h-8',
  primaryNav: 'flex-1 flex flex-col pt-2',
  secondaryNav: 'flex flex-col pb-2',
  navButton:
    'w-full h-14 flex items-center justify-center transition-all duration-200 cursor-pointer relative',
  navButtonActive:
    'bg-gradient-to-r from-stone-900 to-stone-900/50 text-ember-400 shadow-[inset_2px_0_0_0_#ea580c]',
  navButtonInactive:
    'text-stone-500 hover:bg-gradient-to-r hover:from-stone-900 hover:to-transparent hover:text-stone-300',
  icon: 'w-5 h-5',
} as const;
