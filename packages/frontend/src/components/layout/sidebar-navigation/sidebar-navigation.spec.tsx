import { render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { SidebarNavigation } from './sidebar-navigation';

const mockUseLocation = vi.hoisted(() => vi.fn(() => ({ pathname: '/' })));

vi.mock('@tanstack/react-router', () => ({
  Link: ({
    to,
    children,
    className,
    'aria-label': ariaLabel,
    'aria-current': ariaCurrent,
  }: {
    to: string;
    children: React.ReactNode;
    className?: string;
    'aria-label'?: string;
    'aria-current'?: 'page' | undefined;
  }) => (
    <a
      href={to}
      className={className}
      data-testid={`link-${to}`}
      aria-label={ariaLabel}
      aria-current={ariaCurrent}>
      {children}
    </a>
  ),
  useLocation: mockUseLocation,
}));

describe('SidebarNavigation', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockUseLocation.mockReturnValue({ pathname: '/' });
  });

  describe('Static Rendering', () => {
    it('renders navigation information correctly', () => {
      const { container } = render(<SidebarNavigation />);

      // Navigation container
      expect(container.querySelector('div')).toBeInTheDocument();

      // Dashboard link
      const dashboardLinks = screen.getAllByTestId('link-/');
      expect(dashboardLinks.length).toBeGreaterThanOrEqual(2);
      expect(screen.getByTitle('Dashboard')).toBeInTheDocument();

      // Walkthrough link
      expect(screen.getByTestId('link-/walkthrough')).toBeInTheDocument();
      expect(screen.getByTitle('Walkthrough')).toBeInTheDocument();

      // Playtime link
      expect(screen.getByTestId('link-/playtime')).toBeInTheDocument();
      expect(screen.getByTitle('Playtime')).toBeInTheDocument();

      // Economy link
      expect(screen.getByTestId('link-/economy')).toBeInTheDocument();
      expect(screen.getByTitle('Economy')).toBeInTheDocument();

      // Character link
      expect(screen.getByTestId('link-/character')).toBeInTheDocument();
      expect(screen.getByTitle('Character')).toBeInTheDocument();

      // Characters link
      expect(screen.getByTestId('link-/characters')).toBeInTheDocument();
      expect(screen.getByTitle('Characters')).toBeInTheDocument();

      // Notes link
      expect(screen.getByTestId('link-/notes')).toBeInTheDocument();
      expect(screen.getByTitle('Notes')).toBeInTheDocument();

      // Settings link
      expect(screen.getByTestId('link-/settings')).toBeInTheDocument();
      expect(screen.getByTitle('Settings')).toBeInTheDocument();

      // All navigation icons
      const svgs = container.querySelectorAll('svg');
      expect(svgs.length).toBe(8);

      // Active Dashboard link styling
      const dashboardNav = screen.getByTitle('Dashboard');
      expect(dashboardNav.className).toContain('text-ember-400');

      // Inactive state for non-current routes
      const walkthroughNav = screen.getByTitle('Walkthrough');
      expect(walkthroughNav.className).toContain('text-stone-500');
      expect(walkthroughNav.className).not.toContain('shadow-lg');
    });
  });

  it('applies custom className', () => {
    const { container } = render(<SidebarNavigation className="custom-class" />);

    expect(container.firstChild).toHaveClass('custom-class');
  });

  it('highlights active Walkthrough link', () => {
    mockUseLocation.mockReturnValue({ pathname: '/walkthrough' });
    render(<SidebarNavigation />);

    const walkthroughNav = screen.getByTitle('Walkthrough');
    expect(walkthroughNav.className).toContain('text-ember-400');
  });

  it('highlights active Settings link', () => {
    mockUseLocation.mockReturnValue({ pathname: '/settings' });
    render(<SidebarNavigation />);

    const settingsNav = screen.getByTitle('Settings');
    expect(settingsNav.className).toContain('text-ember-400');
  });

  describe('accessibility', () => {
    it('has correct accessibility attributes when on Dashboard', () => {
      const { container } = render(<SidebarNavigation />);

      // No aria-current on inactive links
      const inactiveLink = screen.getByTestId('link-/walkthrough');
      expect(inactiveLink).not.toHaveAttribute('aria-current');

      // aria-label on all navigation links
      const dashboardLinks = screen.getAllByTestId('link-/');
      const dashboardNavLink = dashboardLinks.find(
        link => link.getAttribute('aria-label') === 'Dashboard',
      );
      expect(dashboardNavLink).toHaveAttribute('aria-label', 'Dashboard');
      expect(screen.getByTestId('link-/walkthrough')).toHaveAttribute('aria-label', 'Walkthrough');
      expect(screen.getByTestId('link-/playtime')).toHaveAttribute('aria-label', 'Playtime');
      expect(screen.getByTestId('link-/economy')).toHaveAttribute('aria-label', 'Economy');
      expect(screen.getByTestId('link-/characters')).toHaveAttribute('aria-label', 'Characters');
      expect(screen.getByTestId('link-/notes')).toHaveAttribute('aria-label', 'Notes');
      expect(screen.getByTestId('link-/settings')).toHaveAttribute('aria-label', 'Settings');

      // aria-label on navigation regions
      expect(screen.getByRole('navigation', { name: 'Primary navigation' })).toBeInTheDocument();
      expect(screen.getByRole('navigation', { name: 'Secondary navigation' })).toBeInTheDocument();

      // Hides decorative icons from screen readers
      const icons = container.querySelectorAll('svg');
      icons.forEach(icon => {
        expect(icon).toHaveAttribute('aria-hidden', 'true');
      });
    });

    it('sets aria-current="page" on active link', () => {
      mockUseLocation.mockReturnValue({ pathname: '/walkthrough' });
      render(<SidebarNavigation />);

      const activeLink = screen.getByTestId('link-/walkthrough');
      expect(activeLink).toHaveAttribute('aria-current', 'page');
    });
  });
});
