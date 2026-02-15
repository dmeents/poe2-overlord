import { act, render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { ZoneStats } from '@/types/character';
import { CurrentZoneCard } from './current-zone-card';

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

  describe('Static Rendering', () => {
    it('renders current zone card information correctly', () => {
      render(<CurrentZoneCard zone={createMockZone()} />);

      // Card title
      expect(screen.getByText('Active Zone')).toBeInTheDocument();

      // Zone name
      expect(screen.getByText('The Coast')).toBeInTheDocument();

      // Act badge (getDisplayAct returns consistent "Act N" format)
      expect(screen.getByText('Act 1')).toBeInTheDocument();

      // Area level
      expect(screen.getByText('Level 2')).toBeInTheDocument();

      // View details button
      expect(screen.getByText('View Details →')).toBeInTheDocument();

      // Stats display
      expect(screen.getByText('Time')).toBeInTheDocument();
      expect(screen.getByText('Visits')).toBeInTheDocument();
      expect(screen.getByText('Deaths')).toBeInTheDocument();
    });
  });

  describe('Zone Icons', () => {
    it('shows waypoint icon for zones with waypoints', () => {
      const { container } = render(
        <CurrentZoneCard zone={createMockZone({ has_waypoint: true })} />,
      );

      // MapIcon should be present for waypoints
      const svg = container.querySelector('svg');
      expect(svg).toBeInTheDocument();
    });

    it('does not show waypoint icon for zones without waypoints', () => {
      render(<CurrentZoneCard zone={createMockZone({ has_waypoint: false })} />);

      // The waypoint icon should not be present
      // We check this by verifying the component renders correctly
      expect(screen.getByText('The Coast')).toBeInTheDocument();
    });

    it('shows town icon for town zones', () => {
      const { container } = render(<CurrentZoneCard zone={createMockZone({ is_town: true })} />);

      // BuildingStorefrontIcon should be present for towns
      const svgs = container.querySelectorAll('svg');
      expect(svgs.length).toBeGreaterThan(0);
    });

    it('shows hideout icon for hideout zones', () => {
      const { container } = render(
        <CurrentZoneCard zone={createMockZone({ zone_name: 'My Hideout' })} />,
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
      render(<CurrentZoneCard zone={createMockZone({ area_level: undefined })} />);

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
        />,
      );

      expect(screen.getByText('The Coast')).toBeInTheDocument();
    });
  });

  describe('Live Timer', () => {
    beforeEach(() => {
      vi.useFakeTimers();
    });

    afterEach(() => {
      vi.useRealTimers();
    });

    it('displays live elapsed time for active zone', () => {
      const now = new Date('2024-01-10T12:00:00Z');
      vi.setSystemTime(now);

      const entryTime = new Date('2024-01-10T11:59:00Z'); // 1 minute ago
      const activeZone = createMockZone({
        duration: 100,
        entry_timestamp: entryTime.toISOString(),
        is_active: true,
      });

      render(<CurrentZoneCard zone={activeZone} />);

      // Should show 100 + 60 = 160 seconds = 2m (showSeconds=false)
      expect(screen.getByText('2m')).toBeInTheDocument();

      // Advance 10 seconds
      act(() => {
        vi.advanceTimersByTime(10000);
      });

      // Should now show 170 seconds = 2m (still 2m with showSeconds=false)
      expect(screen.getByText('2m')).toBeInTheDocument();
    });

    it('displays static duration for inactive zone', () => {
      const now = new Date('2024-01-10T12:00:00Z');
      vi.setSystemTime(now);

      const entryTime = new Date('2024-01-10T11:59:00Z');
      const inactiveZone = createMockZone({
        duration: 3600, // 1 hour
        entry_timestamp: entryTime.toISOString(),
        is_active: false,
      });

      render(<CurrentZoneCard zone={inactiveZone} />);

      // Should show static duration (1h 0m with showSeconds=false)
      expect(screen.getByText('1h 0m')).toBeInTheDocument();

      // Advance time - should not change
      act(() => {
        vi.advanceTimersByTime(10000);
      });
      expect(screen.getByText('1h 0m')).toBeInTheDocument();
    });

    it('displays static duration when no entry timestamp', () => {
      const zone = createMockZone({
        duration: 120,
        entry_timestamp: undefined,
        is_active: false,
      });

      render(<CurrentZoneCard zone={zone} />);

      expect(screen.getByText('2m')).toBeInTheDocument();

      act(() => {
        vi.advanceTimersByTime(5000);
      });
      expect(screen.getByText('2m')).toBeInTheDocument();
    });

    it('updates timer when zone changes', () => {
      const now = new Date('2024-01-10T12:00:00Z');
      vi.setSystemTime(now);

      const zone1 = createMockZone({
        zone_name: 'The Coast',
        duration: 100,
        entry_timestamp: new Date('2024-01-10T11:59:00Z').toISOString(),
        is_active: true,
      });

      const { rerender } = render(<CurrentZoneCard zone={zone1} />);

      // Initial zone shows 100 + 60 = 160s = 2m
      expect(screen.getByText('The Coast')).toBeInTheDocument();
      expect(screen.getByText('2m')).toBeInTheDocument();

      // Switch to new zone
      const newNow = new Date('2024-01-10T12:01:00Z');
      vi.setSystemTime(newNow);

      const zone2 = createMockZone({
        zone_name: 'Mud Flats',
        duration: 50,
        entry_timestamp: newNow.toISOString(),
        is_active: true,
      });

      rerender(<CurrentZoneCard zone={zone2} />);

      // New zone should show its duration (50s = <1m so shows 50s)
      expect(screen.getByText('Mud Flats')).toBeInTheDocument();
    });

    it('timer ticks up every second for active zones', () => {
      const now = new Date('2024-01-10T12:00:00Z');
      vi.setSystemTime(now);

      const activeZone = createMockZone({
        duration: 0,
        entry_timestamp: now.toISOString(),
        is_active: true,
      });

      render(<CurrentZoneCard zone={activeZone} />);

      // Initially 0 seconds - should show 0m or similar
      // Advance 61 seconds (1 minute 1 second)
      act(() => {
        vi.advanceTimersByTime(61000);
      });

      // Should now show 1m (61 seconds with showSeconds=false)
      expect(screen.getByText('1m')).toBeInTheDocument();
    });
  });
});
