import {
  BanknotesIcon,
  BookOpenIcon,
  ClockIcon,
  Cog6ToothIcon,
  DocumentTextIcon,
  HomeIcon,
  UserGroupIcon,
} from '@heroicons/react/24/outline';
import { Link, useLocation } from '@tanstack/react-router';
import logoNoText from '@/assets/logo-no-text-square.png';
import { sidebarNavigationStyles } from './sidebar-navigation.styles';

interface SidebarNavigationProps {
  className?: string;
}

export function SidebarNavigation({ className = '' }: SidebarNavigationProps) {
  const location = useLocation();

  const isActive = (path: string) => {
    return location.pathname === path;
  };

  // Navigation items for easier mapping
  const primaryNavItems = [
    { path: '/', title: 'Dashboard', icon: HomeIcon },
    { path: '/walkthrough', title: 'Walkthrough', icon: BookOpenIcon },
    { path: '/playtime', title: 'Playtime', icon: ClockIcon },
    { path: '/economy', title: 'Economy', icon: BanknotesIcon },
    { path: '/characters', title: 'Characters', icon: UserGroupIcon },
    { path: '/notes', title: 'Notes', icon: DocumentTextIcon },
  ];

  const secondaryNavItems = [{ path: '/settings', title: 'Settings', icon: Cog6ToothIcon }];

  const renderNavItem = ({
    path,
    title,
    icon: Icon,
  }: {
    path: string;
    title: string;
    icon: React.ElementType;
  }) => {
    const active = isActive(path);
    return (
      <Link
        key={path}
        to={path}
        className="block"
        aria-label={title}
        aria-current={active ? 'page' : undefined}>
        <div
          title={title}
          className={`${sidebarNavigationStyles.navButton} ${
            active
              ? sidebarNavigationStyles.navButtonActive
              : sidebarNavigationStyles.navButtonInactive
          }`}>
          <Icon className={sidebarNavigationStyles.icon} aria-hidden="true" />
        </div>
      </Link>
    );
  };

  return (
    <div className={`${sidebarNavigationStyles.container} ${className}`}>
      <Link to="/" className={sidebarNavigationStyles.logo} aria-label="Home">
        <img src={logoNoText} alt="" className={sidebarNavigationStyles.logoImage} />
      </Link>
      <nav className={sidebarNavigationStyles.primaryNav} aria-label="Primary navigation">
        {primaryNavItems.map(renderNavItem)}
      </nav>
      <nav className={sidebarNavigationStyles.secondaryNav} aria-label="Secondary navigation">
        {secondaryNavItems.map(renderNavItem)}
      </nav>
    </div>
  );
}
