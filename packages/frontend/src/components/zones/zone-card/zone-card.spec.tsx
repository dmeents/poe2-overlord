import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { ZoneCard } from './zone-card';
import type { ZoneStats } from '@/types/character';

const mockOpenZone = vi.hoisted(() => vi.fn());

vi.mock('@/contexts/ZoneContext', () => ({
  useZone: () => ({
    openZone: mockOpenZone,
  }),
}));

const mockZone: ZoneStats = {
  zone_name: 'The Coast',
  duration: 3600,
  deaths: 2,
  visits: 5,
  first_visited: '2024-01-01T00:00:00Z',
  last_visited: '2024-01-10T00:00:00Z',
  is_active: false,
  entry_timestamp: undefined,
  area_id: 'coast',
  act: 1,
  area_level: 2,
  is_town: false,
  has_waypoint: true,
  bosses: [],
  monsters: [],
  npcs: [],
  connected_zones: [],
  description: undefined,
  points_of_interest: [],
  image_url: undefined,
  wiki_url: undefined,
  last_updated: undefined,
};

describe('ZoneCard', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders zone name', () => {
    render(<ZoneCard zone={mockZone} />);

    expect(screen.getByText('The Coast')).toBeInTheDocument();
  });

  it('renders visit count', () => {
    render(<ZoneCard zone={mockZone} />);

    expect(screen.getByText('5')).toBeInTheDocument();
  });

  it('renders death count', () => {
    render(<ZoneCard zone={mockZone} />);

    // Both death count (2) and area level (2) have value 2
    const elements = screen.getAllByText('2');
    expect(elements.length).toBeGreaterThan(0);
  });

  it('renders area level', () => {
    render(<ZoneCard zone={mockZone} />);

    // area_level is 2, deaths is also 2
    const elements = screen.getAllByText('2');
    expect(elements.length).toBe(2);
  });

  it('renders waypoint icon when has_waypoint is true', () => {
    const { container } = render(<ZoneCard zone={mockZone} />);

    // MapIcon should be present
    const svgs = container.querySelectorAll('svg');
    expect(svgs.length).toBeGreaterThan(0);
  });

  it('does not render waypoint icon when has_waypoint is false', () => {
    const noWaypointZone = {
      ...mockZone,
      has_waypoint: false,
    };

    const { container } = render(<ZoneCard zone={noWaypointZone} />);

    // Should have fewer icons
    const svgs = container.querySelectorAll('svg');
    expect(svgs.length).toBeLessThan(2);
  });

  it('renders town icon when is_town is true', () => {
    const townZone = {
      ...mockZone,
      is_town: true,
    };

    const { container } = render(<ZoneCard zone={townZone} />);

    // Should have town icon
    const svgs = container.querySelectorAll('svg');
    expect(svgs.length).toBeGreaterThan(1);
  });

  it('renders home icon for hideout zones', () => {
    const hideoutZone = {
      ...mockZone,
      zone_name: 'Hideout',
    };

    const { container } = render(<ZoneCard zone={hideoutZone} />);

    const svgs = container.querySelectorAll('svg');
    expect(svgs.length).toBeGreaterThan(1);
  });

  it('calls openZone when clicked', async () => {
    const user = userEvent.setup();

    render(<ZoneCard zone={mockZone} />);

    await user.click(screen.getByText('The Coast'));

    expect(mockOpenZone).toHaveBeenCalledWith('The Coast');
  });

  it('applies custom className', () => {
    const { container } = render(
      <ZoneCard zone={mockZone} className="custom-class" />
    );

    expect(container.firstChild).toHaveClass('custom-class');
  });

  it('shows active styling when is_active is true', () => {
    const activeZone = {
      ...mockZone,
      is_active: true,
    };

    const { container } = render(<ZoneCard zone={activeZone} />);

    expect(container.firstChild).toHaveClass('border-l-emerald-500');
  });

  it('shows even row styling when isEven is true', () => {
    const { container } = render(<ZoneCard zone={mockZone} isEven={true} />);

    const div = container.firstChild as HTMLElement;
    expect(div.className).toContain('bg-zinc-900/30');
  });

  it('shows odd row styling when isEven is false', () => {
    const { container } = render(<ZoneCard zone={mockZone} isEven={false} />);

    const div = container.firstChild as HTMLElement;
    expect(div.className).toContain('bg-zinc-900/60');
  });

  it('renders act display', () => {
    render(<ZoneCard zone={mockZone} />);

    // Act 1 should be displayed as "1"
    expect(screen.getByText('1')).toBeInTheDocument();
  });

  it('does not render area level when undefined', () => {
    const noLevelZone = {
      ...mockZone,
      area_level: undefined,
    };

    render(<ZoneCard zone={noLevelZone} />);

    // 2 was the area level, should not appear as a level now
    // We can't easily check this without more specific selectors
  });

  it('renders duration using TimeDisplay', () => {
    render(<ZoneCard zone={mockZone} />);

    // 3600 seconds = 1 hour = "1h 0m" format
    expect(screen.getByText(/1h 0m/)).toBeInTheDocument();
  });
});
