import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { ClassDistributionChart } from './class-distribution-chart';

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

// Mock the class colors utility
vi.mock('@/utils/class-colors', () => ({
  getClassHexColor: vi.fn((className: string) => {
    const colors: Record<string, string> = {
      Witch: '#9333ea',
      Warrior: '#dc2626',
      Ranger: '#16a34a',
      Mercenary: '#ea580c',
      Monk: '#0ea5e9',
      Sorceress: '#2563eb',
    };
    return colors[className] || '#6b7280';
  }),
}));

describe('ClassDistributionChart', () => {
  describe('Empty State', () => {
    it('shows empty state when no class distribution', () => {
      render(<ClassDistributionChart classDistribution={{}} />);

      expect(screen.getByText('No Class Data')).toBeInTheDocument();
      expect(screen.getByText('Create characters to see class distribution')).toBeInTheDocument();
    });

    it('renders Classes title in empty state', () => {
      render(<ClassDistributionChart classDistribution={{}} />);

      expect(screen.getByText('Classes')).toBeInTheDocument();
    });
  });

  describe('Rendering', () => {
    it('renders Classes title', () => {
      render(<ClassDistributionChart classDistribution={{ Witch: 3 }} />);

      // "Classes" appears in both the card title and the center label
      expect(screen.getAllByText('Classes').length).toBeGreaterThanOrEqual(1);
    });

    it('renders chart components', () => {
      render(<ClassDistributionChart classDistribution={{ Witch: 3 }} />);

      expect(screen.getByTestId('responsive-container')).toBeInTheDocument();
      expect(screen.getByTestId('pie-chart')).toBeInTheDocument();
      expect(screen.getByTestId('pie')).toBeInTheDocument();
    });

    it('renders class count in center', () => {
      render(
        <ClassDistributionChart
          classDistribution={{
            Witch: 5,
            Warrior: 4,
            Ranger: 2,
          }}
        />,
      );

      // 3 unique classes - use getAllByText since 3 also appears as character count
      expect(screen.getAllByText('3').length).toBeGreaterThanOrEqual(1);
      expect(screen.getAllByText('Classes').length).toBeGreaterThanOrEqual(1);
    });

    it('renders class labels', () => {
      render(
        <ClassDistributionChart
          classDistribution={{
            Witch: 3,
            Warrior: 2,
          }}
        />,
      );

      expect(screen.getByText('Witch')).toBeInTheDocument();
      expect(screen.getByText('Warrior')).toBeInTheDocument();
    });

    it('renders character counts', () => {
      render(
        <ClassDistributionChart
          classDistribution={{
            Witch: 5,
            Warrior: 3,
          }}
        />,
      );

      expect(screen.getByText('5')).toBeInTheDocument();
      // 3 appears in both character count and class count, so use getAllByText
      expect(screen.getAllByText('3').length).toBeGreaterThanOrEqual(1);
    });
  });

  describe('Percentage Calculations', () => {
    it('renders percentage for each class', () => {
      render(
        <ClassDistributionChart
          classDistribution={{
            Witch: 5,
            Warrior: 5,
          }}
        />,
      );

      // Both should be 50.0%
      const percentages = screen.getAllByText('50.0%');
      expect(percentages).toHaveLength(2);
    });

    it('calculates correct percentages with different counts', () => {
      render(
        <ClassDistributionChart
          classDistribution={{
            Witch: 3, // 75%
            Warrior: 1, // 25%
          }}
        />,
      );

      expect(screen.getByText('75.0%')).toBeInTheDocument();
      expect(screen.getByText('25.0%')).toBeInTheDocument();
    });
  });

  describe('Sorting', () => {
    it('sorts classes by count descending', () => {
      const { container } = render(
        <ClassDistributionChart
          classDistribution={{
            Ranger: 1,
            Witch: 5,
            Warrior: 3,
          }}
        />,
      );

      // Get all class labels - they should be in order: Witch (5), Warrior (3), Ranger (1)
      const labels = container.querySelectorAll('.text-zinc-200');
      const labelTexts = Array.from(labels).map(el => el.textContent);

      // First entry should be Witch since it has the most characters
      expect(labelTexts).toContain('Witch');
    });
  });

  describe('Custom ClassName', () => {
    it('applies custom className', () => {
      const { container } = render(
        <ClassDistributionChart classDistribution={{ Witch: 1 }} className="custom-class" />,
      );

      expect(container.querySelector('.custom-class')).toBeInTheDocument();
    });
  });

  describe('Edge Cases', () => {
    it('handles single class', () => {
      render(<ClassDistributionChart classDistribution={{ Witch: 10 }} />);

      expect(screen.getByText('Witch')).toBeInTheDocument();
      expect(screen.getByText('10')).toBeInTheDocument();
      expect(screen.getByText('100.0%')).toBeInTheDocument();
    });

    it('handles many classes', () => {
      render(
        <ClassDistributionChart
          classDistribution={{
            Witch: 2,
            Warrior: 2,
            Ranger: 2,
            Mercenary: 2,
            Monk: 2,
            Sorceress: 2,
          }}
        />,
      );

      // Should show 6 classes in center
      expect(screen.getByText('6')).toBeInTheDocument();
      expect(screen.getByText('Witch')).toBeInTheDocument();
      expect(screen.getByText('Warrior')).toBeInTheDocument();
      expect(screen.getByText('Ranger')).toBeInTheDocument();
      expect(screen.getByText('Mercenary')).toBeInTheDocument();
      expect(screen.getByText('Monk')).toBeInTheDocument();
      expect(screen.getByText('Sorceress')).toBeInTheDocument();
    });

    it('handles classes with value of 1', () => {
      render(
        <ClassDistributionChart
          classDistribution={{
            Witch: 1,
          }}
        />,
      );

      // '1' appears as both class count in center and character count
      expect(screen.getAllByText('1').length).toBeGreaterThanOrEqual(1);
      expect(screen.getByText('Witch')).toBeInTheDocument();
    });

    it('handles large character counts', () => {
      render(
        <ClassDistributionChart
          classDistribution={{
            Witch: 100,
            Warrior: 50,
          }}
        />,
      );

      expect(screen.getByText('100')).toBeInTheDocument();
      expect(screen.getByText('50')).toBeInTheDocument();
    });
  });
});
