import { render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { SidebarNavigation } from './sidebar-navigation';

const mockUseLocation = vi.hoisted(() => vi.fn(() => ({ pathname: '/' })));

vi.mock('@tanstack/react-router', () => ({
  Link: ({
    to,
    children,
    className,
  }: {
    to: string;
    children: React.ReactNode;
    className?: string;
  }) => (
    <a href={to} className={className} data-testid={`link-${to}`}>
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

  it('renders navigation container', () => {
    const { container } = render(<SidebarNavigation />);

    expect(container.querySelector('div')).toBeInTheDocument();
  });

  it('renders Dashboard link', () => {
    render(<SidebarNavigation />);

    expect(screen.getByTestId('link-/')).toBeInTheDocument();
    expect(screen.getByTitle('Dashboard')).toBeInTheDocument();
  });

  it('renders Walkthrough link', () => {
    render(<SidebarNavigation />);

    expect(screen.getByTestId('link-/walkthrough')).toBeInTheDocument();
    expect(screen.getByTitle('Walkthrough')).toBeInTheDocument();
  });

  it('renders Playtime link', () => {
    render(<SidebarNavigation />);

    expect(screen.getByTestId('link-/playtime')).toBeInTheDocument();
    expect(screen.getByTitle('Playtime')).toBeInTheDocument();
  });

  it('renders Economy link', () => {
    render(<SidebarNavigation />);

    expect(screen.getByTestId('link-/economy')).toBeInTheDocument();
    expect(screen.getByTitle('Economy')).toBeInTheDocument();
  });

  it('renders Characters link', () => {
    render(<SidebarNavigation />);

    expect(screen.getByTestId('link-/characters')).toBeInTheDocument();
    expect(screen.getByTitle('Characters')).toBeInTheDocument();
  });

  it('renders Settings link', () => {
    render(<SidebarNavigation />);

    expect(screen.getByTestId('link-/settings')).toBeInTheDocument();
    expect(screen.getByTitle('Settings')).toBeInTheDocument();
  });

  it('applies custom className', () => {
    const { container } = render(
      <SidebarNavigation className='custom-class' />
    );

    expect(container.firstChild).toHaveClass('custom-class');
  });

  it('highlights active Dashboard link', () => {
    mockUseLocation.mockReturnValue({ pathname: '/' });
    render(<SidebarNavigation />);

    const dashboardNav = screen.getByTitle('Dashboard');
    expect(dashboardNav.className).toContain('shadow-lg');
    expect(dashboardNav.className).toContain('text-white');
  });

  it('highlights active Walkthrough link', () => {
    mockUseLocation.mockReturnValue({ pathname: '/walkthrough' });
    render(<SidebarNavigation />);

    const walkthroughNav = screen.getByTitle('Walkthrough');
    expect(walkthroughNav.className).toContain('shadow-lg');
    expect(walkthroughNav.className).toContain('text-white');
  });

  it('highlights active Settings link', () => {
    mockUseLocation.mockReturnValue({ pathname: '/settings' });
    render(<SidebarNavigation />);

    const settingsNav = screen.getByTitle('Settings');
    expect(settingsNav.className).toContain('shadow-lg');
    expect(settingsNav.className).toContain('text-white');
  });

  it('shows inactive state for non-current routes', () => {
    mockUseLocation.mockReturnValue({ pathname: '/' });
    render(<SidebarNavigation />);

    const walkthroughNav = screen.getByTitle('Walkthrough');
    expect(walkthroughNav.className).toContain('text-zinc-400');
    expect(walkthroughNav.className).not.toContain('shadow-lg');
  });

  it('renders all navigation icons', () => {
    const { container } = render(<SidebarNavigation />);

    const svgs = container.querySelectorAll('svg');
    expect(svgs.length).toBe(6);
  });
});
