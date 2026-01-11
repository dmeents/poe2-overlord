import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi, beforeEach } from 'vitest';
import { PlaytimeInsights } from './playtime-insights';
import type {
  CharacterData,
  ZoneStats,
  CharacterSummary,
} from '@/types/character';

const mockUseCharacter = vi.hoisted(() => vi.fn());

vi.mock('@/contexts/CharacterContext', () => ({
  useCharacter: mockUseCharacter,
}));

const createMockSummary = (
  overrides: Partial<CharacterSummary> = {}
): CharacterSummary => ({
  character_id: 'char-1',
  total_play_time: 7200, // 2 hours
  total_hideout_time: 1800, // 30 minutes
  total_zones_visited: 10,
  total_deaths: 5,
  play_time_act1: 3600,
  play_time_act2: 1800,
  play_time_act3: 1000,
  play_time_act4: 500,
  play_time_interlude: 200,
  play_time_endgame: 100,
  ...overrides,
});

const createMockZone = (overrides: Partial<ZoneStats> = {}): ZoneStats => ({
  zone_name: 'The Coast',
  duration: 600,
  deaths: 1,
  visits: 2,
  first_visited: '2024-01-01T00:00:00Z',
  last_visited: '2024-01-10T00:00:00Z',
  is_active: false,
  is_town: false,
  has_waypoint: true,
  bosses: [],
  monsters: [],
  npcs: [],
  connected_zones: [],
  points_of_interest: [],
  ...overrides,
});

const createMockCharacter = (
  summaryOverrides: Partial<CharacterSummary> = {},
  zones: ZoneStats[] = []
): CharacterData =>
  ({
    id: 'char-1',
    name: 'TestChar',
    class: 'Witch',
    summary: createMockSummary(summaryOverrides),
    zones,
  }) as CharacterData;

describe('PlaytimeInsights', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Loading State', () => {
    it('shows loading spinner when loading', () => {
      mockUseCharacter.mockReturnValue({
        activeCharacter: null,
        isLoading: true,
      });

      render(<PlaytimeInsights />);

      expect(
        screen.getByText('Loading playtime insights...')
      ).toBeInTheDocument();
    });
  });

  describe('Empty State', () => {
    it('shows empty state when no active character', () => {
      mockUseCharacter.mockReturnValue({
        activeCharacter: null,
        isLoading: false,
      });

      render(<PlaytimeInsights />);

      expect(
        screen.getByText('No Character Data Available')
      ).toBeInTheDocument();
      expect(
        screen.getByText('Select a character to view playtime insights')
      ).toBeInTheDocument();
    });

    it('shows empty state when character has no summary', () => {
      mockUseCharacter.mockReturnValue({
        activeCharacter: { id: 'char-1', name: 'Test', summary: null },
        isLoading: false,
      });

      render(<PlaytimeInsights />);

      expect(
        screen.getByText('No Character Data Available')
      ).toBeInTheDocument();
    });
  });

  describe('Rendering', () => {
    it('renders card with Insights title', () => {
      mockUseCharacter.mockReturnValue({
        activeCharacter: createMockCharacter(),
        isLoading: false,
      });

      render(<PlaytimeInsights />);

      expect(screen.getByText('Insights')).toBeInTheDocument();
    });

    it('renders total play time', () => {
      mockUseCharacter.mockReturnValue({
        activeCharacter: createMockCharacter({ total_play_time: 7200 }), // 2 hours
        isLoading: false,
      });

      render(<PlaytimeInsights />);

      expect(screen.getByText('Total Play Time')).toBeInTheDocument();
      expect(screen.getByText('2h')).toBeInTheDocument();
    });

    it('renders active play time with percentage', () => {
      mockUseCharacter.mockReturnValue({
        activeCharacter: createMockCharacter({
          total_play_time: 7200,
          total_hideout_time: 1800,
        }),
        isLoading: false,
      });

      render(<PlaytimeInsights />);

      expect(screen.getByText('Active Play')).toBeInTheDocument();
      expect(screen.getByText('1h 30m')).toBeInTheDocument(); // 7200 - 1800 = 5400 seconds
      expect(screen.getByText('75.0%')).toBeInTheDocument();
    });

    it('renders hideout time with percentage', () => {
      mockUseCharacter.mockReturnValue({
        activeCharacter: createMockCharacter({
          total_play_time: 7200,
          total_hideout_time: 1800,
        }),
        isLoading: false,
      });

      render(<PlaytimeInsights />);

      expect(screen.getByText('Hideout Time')).toBeInTheDocument();
      expect(screen.getByText('30m')).toBeInTheDocument();
      expect(screen.getByText('25.0%')).toBeInTheDocument();
    });

    it('renders deaths with death rate per hour', () => {
      mockUseCharacter.mockReturnValue({
        activeCharacter: createMockCharacter({
          total_play_time: 7200, // 2 hours
          total_deaths: 4,
        }),
        isLoading: false,
      });

      render(<PlaytimeInsights />);

      expect(screen.getByText('Deaths')).toBeInTheDocument();
      expect(screen.getByText('4')).toBeInTheDocument();
      expect(screen.getByText('2.00/hr')).toBeInTheDocument();
    });
  });

  describe('Zone Calculations', () => {
    it('renders average time per zone', () => {
      const zones = [
        createMockZone({ zone_name: 'Zone A', duration: 600, is_town: false }),
        createMockZone({ zone_name: 'Zone B', duration: 900, is_town: false }),
      ];

      mockUseCharacter.mockReturnValue({
        activeCharacter: createMockCharacter({}, zones),
        isLoading: false,
      });

      render(<PlaytimeInsights />);

      expect(screen.getByText('Avg Time per Zone')).toBeInTheDocument();
      expect(screen.getByText('2 zones')).toBeInTheDocument();
    });

    it('excludes towns from average time calculation', () => {
      const zones = [
        createMockZone({ zone_name: 'Town', duration: 3600, is_town: true }),
        createMockZone({ zone_name: 'Zone A', duration: 600, is_town: false }),
      ];

      mockUseCharacter.mockReturnValue({
        activeCharacter: createMockCharacter({}, zones),
        isLoading: false,
      });

      render(<PlaytimeInsights />);

      expect(screen.getByText('1 zones')).toBeInTheDocument();
    });

    it('excludes hideouts from average time calculation', () => {
      const zones = [
        createMockZone({
          zone_name: 'My Hideout',
          duration: 3600,
          is_town: false,
        }),
        createMockZone({ zone_name: 'Zone A', duration: 600, is_town: false }),
      ];

      mockUseCharacter.mockReturnValue({
        activeCharacter: createMockCharacter({}, zones),
        isLoading: false,
      });

      render(<PlaytimeInsights />);

      expect(screen.getByText('1 zones')).toBeInTheDocument();
    });

    it('renders most time spent zone', () => {
      // Using zone with no deaths so it doesn't appear in "Most Deaths"
      const zones = [
        createMockZone({
          zone_name: 'Longest Zone',
          duration: 1200,
          deaths: 0,
        }),
        createMockZone({ zone_name: 'Short Zone', duration: 300, deaths: 0 }),
      ];

      mockUseCharacter.mockReturnValue({
        activeCharacter: createMockCharacter({ total_deaths: 0 }, zones),
        isLoading: false,
      });

      render(<PlaytimeInsights />);

      expect(screen.getByText('Most Time Spent')).toBeInTheDocument();
      expect(screen.getByText('Longest Zone')).toBeInTheDocument();
      expect(screen.getByText('20m')).toBeInTheDocument();
    });

    it('renders zone with most deaths', () => {
      const zones = [
        createMockZone({ zone_name: 'Easy Zone', deaths: 0, duration: 1200 }),
        createMockZone({ zone_name: 'Hard Zone', deaths: 5, duration: 600 }),
      ];

      mockUseCharacter.mockReturnValue({
        activeCharacter: createMockCharacter({}, zones),
        isLoading: false,
      });

      render(<PlaytimeInsights />);

      expect(screen.getByText('Most Deaths')).toBeInTheDocument();
      expect(screen.getByText('Hard Zone')).toBeInTheDocument();
      expect(screen.getByText('5 deaths')).toBeInTheDocument();
    });

    it('does not render most deaths when all zones have 0 deaths', () => {
      const zones = [
        createMockZone({ zone_name: 'Zone A', deaths: 0 }),
        createMockZone({ zone_name: 'Zone B', deaths: 0 }),
      ];

      mockUseCharacter.mockReturnValue({
        activeCharacter: createMockCharacter({ total_deaths: 0 }, zones),
        isLoading: false,
      });

      render(<PlaytimeInsights />);

      expect(screen.queryByText('Most Deaths')).not.toBeInTheDocument();
    });
  });

  describe('Props Override', () => {
    it('uses zones prop when provided instead of context zones', () => {
      const contextZones = [
        createMockZone({ zone_name: 'Context Zone', duration: 100, deaths: 0 }),
      ];
      const propZones = [
        createMockZone({ zone_name: 'Prop Zone', duration: 500, deaths: 0 }),
      ];

      mockUseCharacter.mockReturnValue({
        activeCharacter: {
          ...createMockCharacter({ total_deaths: 0 }, contextZones),
        },
        isLoading: false,
      });

      render(<PlaytimeInsights zones={propZones} />);

      // Zone name appears in "Most Time Spent" row
      expect(screen.getByText('Prop Zone')).toBeInTheDocument();
      expect(screen.queryByText('Context Zone')).not.toBeInTheDocument();
    });
  });

  describe('Edge Cases', () => {
    it('handles zero play time', () => {
      mockUseCharacter.mockReturnValue({
        activeCharacter: createMockCharacter({
          total_play_time: 0,
          total_hideout_time: 0,
          total_deaths: 0,
        }),
        isLoading: false,
      });

      render(<PlaytimeInsights />);

      expect(screen.getByText('Total Play Time')).toBeInTheDocument();
      // Multiple 0m values for zero play time, hideout time, avg time
      expect(screen.getAllByText('0m').length).toBeGreaterThanOrEqual(1);
    });

    it('handles empty zones array', () => {
      mockUseCharacter.mockReturnValue({
        activeCharacter: createMockCharacter({}, []),
        isLoading: false,
      });

      render(<PlaytimeInsights />);

      expect(screen.getByText('Avg Time per Zone')).toBeInTheDocument();
      expect(screen.getByText('0 zones')).toBeInTheDocument();
      expect(screen.queryByText('Most Time Spent')).not.toBeInTheDocument();
    });

    it('formats hours and minutes correctly', () => {
      mockUseCharacter.mockReturnValue({
        activeCharacter: createMockCharacter({ total_play_time: 5430 }), // 1h 30m 30s -> rounds to 1h 31m
        isLoading: false,
      });

      render(<PlaytimeInsights />);

      // 5430 seconds = 90.5 minutes, rounds to 91 minutes = 1h 31m
      expect(screen.getByText('1h 31m')).toBeInTheDocument();
    });
  });
});
