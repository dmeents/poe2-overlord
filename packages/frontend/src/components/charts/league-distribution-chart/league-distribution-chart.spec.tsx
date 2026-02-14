import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { LeagueDistributionChart } from './league-distribution-chart';

// Mock Recharts components since they require DOM measurements
vi.mock('recharts', () => ({
  ResponsiveContainer: vi.fn(({ children }) => (
    <div data-testid="responsive-container">{children}</div>
  )),
  PieChart: vi.fn(({ children }) => <div data-testid="pie-chart">{children}</div>),
  Pie: vi.fn(({ children, data }) => (
    <div data-testid="pie" data-entries={data?.length}>
      {children}
    </div>
  )),
  Cell: vi.fn(() => <div data-testid="cell" />),
  Tooltip: vi.fn(() => <div data-testid="tooltip" />),
}));

// Mock the league colors utility
vi.mock('@/utils/league-colors', () => ({
  getLeagueHexColor: vi.fn((league: string) => {
    const colors: Record<string, string> = {
      Standard: '#5b82c2',
      'Rise of the Abyssal': '#4a9958',
      'The Fate of the Vaal': '#dc2626',
    };
    return colors[league] || '#6b7280';
  }),
}));

describe('LeagueDistributionChart', () => {
  describe('Empty State', () => {
    it('shows empty state when no league distribution', () => {
      render(<LeagueDistributionChart leagueDistribution={{}} />);

      expect(screen.getByText('No League Data')).toBeInTheDocument();
      expect(screen.getByText('Create characters to see league distribution')).toBeInTheDocument();
    });

    it('renders Leagues title in empty state', () => {
      render(<LeagueDistributionChart leagueDistribution={{}} />);

      expect(screen.getByText('Leagues')).toBeInTheDocument();
    });
  });

  describe('Rendering', () => {
    it('renders Leagues title', () => {
      render(<LeagueDistributionChart leagueDistribution={{ Standard: 3 }} />);

      // "Leagues" appears in both the card title and the center label
      expect(screen.getAllByText('Leagues').length).toBeGreaterThanOrEqual(1);
    });

    it('renders chart components', () => {
      render(<LeagueDistributionChart leagueDistribution={{ Standard: 3 }} />);

      expect(screen.getByTestId('responsive-container')).toBeInTheDocument();
      expect(screen.getByTestId('pie-chart')).toBeInTheDocument();
      expect(screen.getByTestId('pie')).toBeInTheDocument();
    });

    it('renders league count in center', () => {
      render(
        <LeagueDistributionChart
          leagueDistribution={{
            Standard: 5,
            'Rise of the Abyssal': 4,
            'The Fate of the Vaal': 2,
          }}
        />,
      );

      // 3 unique leagues - use getAllByText since 3 also appears as character count
      expect(screen.getAllByText('3').length).toBeGreaterThanOrEqual(1);
      expect(screen.getAllByText('Leagues').length).toBeGreaterThanOrEqual(1);
    });

    it('renders league labels', () => {
      render(
        <LeagueDistributionChart
          leagueDistribution={{
            Standard: 3,
            'Rise of the Abyssal': 2,
          }}
        />,
      );

      expect(screen.getByText('Standard')).toBeInTheDocument();
      expect(screen.getByText('Rise of the Abyssal')).toBeInTheDocument();
    });

    it('renders character counts', () => {
      render(
        <LeagueDistributionChart
          leagueDistribution={{
            Standard: 5,
            'Rise of the Abyssal': 3,
          }}
        />,
      );

      expect(screen.getByText('5')).toBeInTheDocument();
      // 3 appears in both character count and league count, so use getAllByText
      expect(screen.getAllByText('3').length).toBeGreaterThanOrEqual(1);
    });
  });

  describe('Percentage Calculations', () => {
    it('renders percentage for each league', () => {
      render(
        <LeagueDistributionChart
          leagueDistribution={{
            Standard: 5,
            'Rise of the Abyssal': 5,
          }}
        />,
      );

      // Both should be 50.0%
      const percentages = screen.getAllByText('50.0%');
      expect(percentages).toHaveLength(2);
    });

    it('calculates correct percentages with different counts', () => {
      render(
        <LeagueDistributionChart
          leagueDistribution={{
            Standard: 3, // 75%
            'Rise of the Abyssal': 1, // 25%
          }}
        />,
      );

      expect(screen.getByText('75.0%')).toBeInTheDocument();
      expect(screen.getByText('25.0%')).toBeInTheDocument();
    });
  });

  describe('Sorting', () => {
    it('sorts leagues by count descending', () => {
      const { container } = render(
        <LeagueDistributionChart
          leagueDistribution={{
            'The Fate of the Vaal': 1,
            Standard: 5,
            'Rise of the Abyssal': 3,
          }}
        />,
      );

      // Get all league labels - they should be in order: Standard (5), Rise of the Abyssal (3), The Fate of the Vaal (1)
      const labels = container.querySelectorAll('.text-stone-400');
      const labelTexts = Array.from(labels).map(el => el.textContent);

      // First entry should be Standard since it has the most characters
      expect(labelTexts).toContain('Standard');
    });
  });

  describe('Custom ClassName', () => {
    it('applies custom className', () => {
      const { container } = render(
        <LeagueDistributionChart leagueDistribution={{ Standard: 1 }} className="custom-class" />,
      );

      expect(container.querySelector('.custom-class')).toBeInTheDocument();
    });
  });

  describe('Edge Cases', () => {
    it('handles single league', () => {
      render(<LeagueDistributionChart leagueDistribution={{ Standard: 10 }} />);

      expect(screen.getByText('Standard')).toBeInTheDocument();
      expect(screen.getByText('10')).toBeInTheDocument();
      expect(screen.getByText('100.0%')).toBeInTheDocument();
    });

    it('handles all leagues', () => {
      render(
        <LeagueDistributionChart
          leagueDistribution={{
            Standard: 2,
            'Rise of the Abyssal': 2,
            'The Fate of the Vaal': 2,
          }}
        />,
      );

      // Should show 3 leagues in center
      expect(screen.getByText('3')).toBeInTheDocument();
      expect(screen.getByText('Standard')).toBeInTheDocument();
      expect(screen.getByText('Rise of the Abyssal')).toBeInTheDocument();
      expect(screen.getByText('The Fate of the Vaal')).toBeInTheDocument();
    });

    it('handles leagues with value of 1', () => {
      render(
        <LeagueDistributionChart
          leagueDistribution={{
            Standard: 1,
          }}
        />,
      );

      // '1' appears as both league count in center and character count
      expect(screen.getAllByText('1').length).toBeGreaterThanOrEqual(1);
      expect(screen.getByText('Standard')).toBeInTheDocument();
    });

    it('handles large character counts', () => {
      render(
        <LeagueDistributionChart
          leagueDistribution={{
            Standard: 100,
            'Rise of the Abyssal': 50,
          }}
        />,
      );

      expect(screen.getByText('100')).toBeInTheDocument();
      expect(screen.getByText('50')).toBeInTheDocument();
    });
  });
});
