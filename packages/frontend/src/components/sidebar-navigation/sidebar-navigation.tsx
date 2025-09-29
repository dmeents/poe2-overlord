import {
  ClockIcon,
  Cog6ToothIcon,
  HomeIcon,
  UserGroupIcon,
} from '@heroicons/react/24/outline';
import { Link, useLocation } from '@tanstack/react-router';
import { sidebarNavigationStyles } from './sidebar-navigation.styles';

interface SidebarNavigationProps {
  className?: string;
}

export function SidebarNavigation({ className = '' }: SidebarNavigationProps) {
  const location = useLocation();

  const isActive = (path: string) => {
    return location.pathname === path;
  };

  return (
    <div className={`${sidebarNavigationStyles.container} ${className}`}>
      {/* Primary Navigation */}
      <nav className={sidebarNavigationStyles.primaryNav}>
        <Link to='/' className='block'>
          <div
            title='Dashboard'
            className={`${sidebarNavigationStyles.navButton} ${
              isActive('/')
                ? sidebarNavigationStyles.navButtonActive
                : sidebarNavigationStyles.navButtonInactive
            }`}
          >
            <HomeIcon className={sidebarNavigationStyles.icon} />
          </div>
        </Link>
        <Link to='/characters' className='block'>
          <div
            title='Characters'
            className={`${sidebarNavigationStyles.navButton} ${
              isActive('/characters')
                ? sidebarNavigationStyles.navButtonActive
                : sidebarNavigationStyles.navButtonInactive
            }`}
          >
            <UserGroupIcon className={sidebarNavigationStyles.icon} />
          </div>
        </Link>
        <Link to='/playtime' className='block'>
          <div
            title='Playtime'
            className={`${sidebarNavigationStyles.navButton} ${
              isActive('/playtime')
                ? sidebarNavigationStyles.navButtonActive
                : sidebarNavigationStyles.navButtonInactive
            }`}
          >
            <ClockIcon className={sidebarNavigationStyles.icon} />
          </div>
        </Link>
      </nav>

      {/* Secondary Navigation (Settings) */}
      <nav className={sidebarNavigationStyles.secondaryNav}>
        <Link to='/settings' className='block'>
          <div
            title='Settings'
            className={`${sidebarNavigationStyles.navButton} ${
              isActive('/settings')
                ? sidebarNavigationStyles.navButtonActive
                : sidebarNavigationStyles.navButtonInactive
            }`}
          >
            <Cog6ToothIcon className={sidebarNavigationStyles.icon} />
          </div>
        </Link>
      </nav>
    </div>
  );
}
