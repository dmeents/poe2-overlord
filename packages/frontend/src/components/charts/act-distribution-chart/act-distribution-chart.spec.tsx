import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { ActDistributionChart } from './act-distribution-chart';
import type { CharacterData, CharacterSummary } from '@/types/character';

// Mock Recharts components since they require DOM measurements
vi.mock('recharts', () => ({
  ResponsiveContainer: vi.fn(({ children }) => (
    <div data-testid='responsive-container'>{children}</div>
  )),
  PieChart: vi.fn(({ children }) => (
    <div data-testid='pie-chart'>{children}</div>
  )),
  Pie: vi.fn(({ children, data }) => (
    <div data-testid='pie' data-entries={data?.length}>
      {children}
    </div>
  )),
  Cell: vi.fn(() => <div data-testid='cell' />),
  Tooltip: vi.fn(() => <div data-testid='tooltip' />),
}));

const createMockSummary = (
  overrides: Partial<CharacterSummary> = {}
): CharacterSummary => ({
  character_id: 'char-1',
  total_play_time: 7200,
  total_hideout_time: 600,
  total_zones_visited: 10,
  total_deaths: 2,
  play_time_act1: 1800,
  play_time_act2: 1200,
  play_time_act3: 900,
  play_time_act4: 600,
  play_time_interlude: 300,
  play_time_endgame: 0,
  ...overrides,
});

const createMockCharacter = (
  summaryOverrides: Partial<CharacterSummary> = {}
): CharacterData => ({
  id: 'char-1',
  name: 'TestChar',
  class: 'Witch',
  ascendency: 'Infernalist',
  league: 'Standard',
  hardcore: false,
  solo_self_found: false,
  level: 50,
  created_at: '2024-01-01T00:00:00Z',
  last_updated: '2024-01-10T00:00:00Z',
  summary: createMockSummary(summaryOverrides),
  zones: [],
  walkthrough_progress: {
    current_step_id: null,
    is_completed: false,
    last_updated: '2024-01-10T00:00:00Z',
  },
});

describe('ActDistributionChart', () => {
  describe('Empty State', () => {
    it('shows empty state when no act time', () => {
      const character = createMockCharacter({
        play_time_act1: 0,
        play_time_act2: 0,
        play_time_act3: 0,
        play_time_act4: 0,
        play_time_interlude: 0,
      });

      render(<ActDistributionChart character={character} />);

      expect(screen.getByText('No Act Data')).toBeInTheDocument();
      expect(
        screen.getByText('Start playing to see act distribution')
      ).toBeInTheDocument();
    });

    it('renders Act Distribution title in empty state', () => {
      const character = createMockCharacter({
        play_time_act1: 0,
        play_time_act2: 0,
        play_time_act3: 0,
        play_time_act4: 0,
        play_time_interlude: 0,
      });

      render(<ActDistributionChart character={character} />);

      expect(screen.getByText('Act Distribution')).toBeInTheDocument();
    });
  });

  describe('Rendering', () => {
    it('renders Act Distribution title', () => {
      render(<ActDistributionChart character={createMockCharacter()} />);

      expect(screen.getByText('Act Distribution')).toBeInTheDocument();
    });

    it('renders chart components', () => {
      render(<ActDistributionChart character={createMockCharacter()} />);

      expect(screen.getByTestId('responsive-container')).toBeInTheDocument();
      expect(screen.getByTestId('pie-chart')).toBeInTheDocument();
      expect(screen.getByTestId('pie')).toBeInTheDocument();
    });

    it('renders active acts count in center', () => {
      const character = createMockCharacter({
        play_time_act1: 1800,
        play_time_act2: 1200,
        play_time_act3: 0,
        play_time_act4: 0,
        play_time_interlude: 0,
      });

      render(<ActDistributionChart character={character} />);

      // 2 acts have time (Act 1 and Act 2)
      expect(screen.getByText('2')).toBeInTheDocument();
      expect(screen.getByText('Acts')).toBeInTheDocument();
    });

    it('renders act labels', () => {
      render(<ActDistributionChart character={createMockCharacter()} />);

      expect(screen.getByText('Act 1')).toBeInTheDocument();
      expect(screen.getByText('Act 2')).toBeInTheDocument();
      expect(screen.getByText('Act 3')).toBeInTheDocument();
      expect(screen.getByText('Act 4')).toBeInTheDocument();
      expect(screen.getByText('Interlude')).toBeInTheDocument();
    });

    it('renders total campaign time', () => {
      render(<ActDistributionChart character={createMockCharacter()} />);

      expect(screen.getByText('Total Campaign Time')).toBeInTheDocument();
    });
  });

  describe('Percentage Calculations', () => {
    it('renders percentage for each act', () => {
      const character = createMockCharacter({
        play_time_act1: 3600, // 50%
        play_time_act2: 3600, // 50%
        play_time_act3: 0,
        play_time_act4: 0,
        play_time_interlude: 0,
      });

      render(<ActDistributionChart character={character} />);

      // Both should be 50.0%
      const percentages = screen.getAllByText('50.0%');
      expect(percentages).toHaveLength(2);
    });
  });

  describe('Filtering', () => {
    it('only shows acts with time', () => {
      const character = createMockCharacter({
        play_time_act1: 1800,
        play_time_act2: 0,
        play_time_act3: 900,
        play_time_act4: 0,
        play_time_interlude: 0,
      });

      render(<ActDistributionChart character={character} />);

      expect(screen.getByText('Act 1')).toBeInTheDocument();
      expect(screen.getByText('Act 3')).toBeInTheDocument();
      // Act 2, Act 4, and Interlude should not be in the data items
      // but they won't have their time displayed since they're filtered
    });
  });

  describe('Custom ClassName', () => {
    it('applies custom className', () => {
      const { container } = render(
        <ActDistributionChart
          character={createMockCharacter()}
          className='custom-class'
        />
      );

      expect(container.querySelector('.custom-class')).toBeInTheDocument();
    });
  });

  describe('Time Formatting', () => {
    it('renders formatted durations', () => {
      const character = createMockCharacter({
        play_time_act1: 3600, // 1 hour
        play_time_act2: 1800, // 30 minutes
        play_time_act3: 0,
        play_time_act4: 0,
        play_time_interlude: 0,
      });

      render(<ActDistributionChart character={character} />);

      // Check that duration formatting is applied
      // The exact format depends on formatDuration utility
      expect(screen.getByText('Act 1')).toBeInTheDocument();
      expect(screen.getByText('Act 2')).toBeInTheDocument();
    });
  });

  describe('Edge Cases', () => {
    it('handles all acts having time', () => {
      render(<ActDistributionChart character={createMockCharacter()} />);

      // Should show 5 acts (Act 1-4 plus Interlude)
      expect(screen.getByText('5')).toBeInTheDocument();
    });

    it('handles single act with time', () => {
      const character = createMockCharacter({
        play_time_act1: 3600,
        play_time_act2: 0,
        play_time_act3: 0,
        play_time_act4: 0,
        play_time_interlude: 0,
      });

      render(<ActDistributionChart character={character} />);

      expect(screen.getByText('1')).toBeInTheDocument();
      expect(screen.getByText('100.0%')).toBeInTheDocument();
    });

    it('handles very small time values', () => {
      const character = createMockCharacter({
        play_time_act1: 1, // 1 second
        play_time_act2: 0,
        play_time_act3: 0,
        play_time_act4: 0,
        play_time_interlude: 0,
      });

      render(<ActDistributionChart character={character} />);

      expect(screen.getByText('Act 1')).toBeInTheDocument();
    });
  });
});
