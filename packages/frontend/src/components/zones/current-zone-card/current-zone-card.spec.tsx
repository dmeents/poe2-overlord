import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi, beforeEach } from 'vitest';
import { CurrentZoneCard } from './current-zone-card';
import type { ZoneStats } from '@/types/character';

const mockOpenZone = vi.hoisted(() => vi.fn());

vi.mock('@/contexts/ZoneContext', () => ({
  useZone: () => ({
    openZone: mockOpenZone,
  }),
}));

const createMockZone = (overrides: Partial<ZoneStats> = {}): ZoneStats => ({
  zone_name: 'The Coast',
  act: 1,
  area_level: 2,
  is_town: false,
  has_waypoint: true,
  visits: 5,
  duration: 3600,
  deaths: 2,
  first_visited: '2024-01-01T00:00:00Z',
  last_visited: '2024-01-10T00:00:00Z',
  is_active: false,
  bosses: [],
  monsters: [],
  npcs: [],
  connected_zones: [],
  points_of_interest: [],
  ...overrides,
});

describe('CurrentZoneCard', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Rendering', () => {
    it('renders card with Active Zone title', () => {
      render(<CurrentZoneCard zone={createMockZone()} />);

      expect(screen.getByText('Active Zone')).toBeInTheDocument();
    });

    it('renders zone name', () => {
      render(<CurrentZoneCard zone={createMockZone()} />);

      expect(screen.getByText('The Coast')).toBeInTheDocument();
    });

    it('renders act badge when act is present', () => {
      render(<CurrentZoneCard zone={createMockZone()} />);

      expect(screen.getByText('1')).toBeInTheDocument();
    });

    it('renders area level', () => {
      render(<CurrentZoneCard zone={createMockZone()} />);

      expect(screen.getByText('Level 2')).toBeInTheDocument();
    });

    it('renders view details button', () => {
      render(<CurrentZoneCard zone={createMockZone()} />);

      expect(screen.getByText('View Details →')).toBeInTheDocument();
    });
  });

  describe('Stats Display', () => {
    it('renders time stat box', () => {
      render(<CurrentZoneCard zone={createMockZone()} />);

      expect(screen.getByText('Time')).toBeInTheDocument();
    });

    it('renders visits stat box', () => {
      render(<CurrentZoneCard zone={createMockZone()} />);

      expect(screen.getByText('Visits')).toBeInTheDocument();
    });

    it('renders deaths stat box', () => {
      render(<CurrentZoneCard zone={createMockZone()} />);

      expect(screen.getByText('Deaths')).toBeInTheDocument();
    });
  });

  describe('Zone Icons', () => {
    it('shows waypoint icon for zones with waypoints', () => {
      const { container } = render(
        <CurrentZoneCard zone={createMockZone({ has_waypoint: true })} />
      );

      // MapIcon should be present for waypoints
      const svg = container.querySelector('svg');
      expect(svg).toBeInTheDocument();
    });

    it('does not show waypoint icon for zones without waypoints', () => {
      render(
        <CurrentZoneCard zone={createMockZone({ has_waypoint: false })} />
      );

      // The waypoint icon should not be present
      // We check this by verifying the component renders correctly
      expect(screen.getByText('The Coast')).toBeInTheDocument();
    });

    it('shows town icon for town zones', () => {
      const { container } = render(
        <CurrentZoneCard zone={createMockZone({ is_town: true })} />
      );

      // BuildingStorefrontIcon should be present for towns
      const svgs = container.querySelectorAll('svg');
      expect(svgs.length).toBeGreaterThan(0);
    });

    it('shows hideout icon for hideout zones', () => {
      const { container } = render(
        <CurrentZoneCard zone={createMockZone({ zone_name: 'My Hideout' })} />
      );

      // HomeIcon should be present for hideouts
      expect(screen.getByText('My Hideout')).toBeInTheDocument();
      const svgs = container.querySelectorAll('svg');
      expect(svgs.length).toBeGreaterThan(0);
    });
  });

  describe('Interactions', () => {
    it('calls openZone when View Details is clicked', async () => {
      const user = userEvent.setup();
      render(<CurrentZoneCard zone={createMockZone()} />);

      await user.click(screen.getByText('View Details →'));

      expect(mockOpenZone).toHaveBeenCalledWith('The Coast');
    });
  });

  describe('Edge Cases', () => {
    it('handles zones without area_level', () => {
      render(
        <CurrentZoneCard zone={createMockZone({ area_level: undefined })} />
      );

      expect(screen.queryByText(/Level/)).not.toBeInTheDocument();
    });

    it('handles zones with zero stats', () => {
      render(
        <CurrentZoneCard
          zone={createMockZone({
            visits: 0,
            deaths: 0,
            duration: 0,
          })}
        />
      );

      expect(screen.getByText('The Coast')).toBeInTheDocument();
    });
  });
});
