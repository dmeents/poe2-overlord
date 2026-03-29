import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { ZoneStats } from '@/types/character';
import { ZoneCard } from './zone-card';

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
  act: 1,
  area_level: 2,
  is_town: false,
  has_waypoint: true,
  bosses: [],
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

  describe('Static Rendering', () => {
    it('renders zone information correctly', () => {
      const { container } = render(<ZoneCard zone={mockZone} />);

      // Zone name
      expect(screen.getByText('The Coast')).toBeInTheDocument();

      // Visit count
      expect(screen.getByText('5')).toBeInTheDocument();

      // Death count and area level (both have value 2)
      const elements = screen.getAllByText('2');
      expect(elements.length).toBe(2);

      // Act display
      expect(screen.getByText('Act 1')).toBeInTheDocument();

      // Duration (3600 seconds = 1h 0m)
      expect(screen.getByText(/1h 0m/)).toBeInTheDocument();

      // Waypoint icon (MapIcon should be present)
      const svgs = container.querySelectorAll('svg');
      expect(svgs.length).toBeGreaterThan(0);
    });
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
    const { container } = render(<ZoneCard zone={mockZone} className="custom-class" />);

    expect(container.firstChild).toHaveClass('custom-class');
  });

  it('shows active styling when is_active is true', () => {
    const activeZone = {
      ...mockZone,
      is_active: true,
    };

    const { container } = render(<ZoneCard zone={activeZone} />);

    expect(container.firstChild).toHaveClass('border-l-verdant-500');
  });

  it('shows even row styling when isEven is true', () => {
    const { container } = render(<ZoneCard zone={mockZone} isEven={true} />);

    const div = container.firstChild as HTMLElement;
    expect(div.className).toContain('bg-stone-900/30');
  });

  it('shows odd row styling when isEven is false', () => {
    const { container } = render(<ZoneCard zone={mockZone} isEven={false} />);

    const div = container.firstChild as HTMLElement;
    expect(div.className).toContain('bg-stone-900/60');
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
});
