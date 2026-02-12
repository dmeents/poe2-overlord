// Sidebar Navigation Styles
// Centralized styling utilities for the SidebarNavigation component

export const sidebarNavigationStyles = {
  container: 'fixed left-0 top-[28px] bottom-6 w-12 bg-zinc-950 flex flex-col z-50',
  primaryNav: 'flex-1 flex flex-col',
  secondaryNav: 'flex flex-col',
  navButton:
    'w-full h-16 flex items-center justify-center transition-all duration-200 cursor-pointer',
  navButtonActive: 'bg-gradient-to-r from-zinc-950 to-zinc-900 text-white shadow-lg',
  navButtonInactive:
    'text-zinc-400 hover:bg-gradient-to-r hover:from-zinc-950 hover:to-zinc-900 hover:text-zinc-200',
  icon: 'w-6 h-6',
} as const;
