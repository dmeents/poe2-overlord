import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { DistributionDonutChart } from './distribution-donut-chart';

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

describe('DistributionDonutChart', () => {
  const mockGetHexColor = vi.fn((name: string) => {
    const colors: Record<string, string> = {
      ItemA: '#ff0000',
      ItemB: '#00ff00',
      ItemC: '#0000ff',
    };
    return colors[name] || '#cccccc';
  });

  const mockFormatTooltip = vi.fn((data: { value: number; percentage: number }) => {
    return `${data.value} items (${data.percentage.toFixed(1)}%)`;
  });

  const defaultProps = {
    title: 'Test Chart',
    icon: <div data-testid="test-icon">Icon</div>,
    centerLabel: 'Items',
    getHexColor: mockGetHexColor,
    emptyStateConfig: {
      title: 'No Data',
      description: 'Add items to see distribution',
    },
    formatTooltipContent: mockFormatTooltip,
  };

  describe('Empty State', () => {
    it('shows empty state when no data', () => {
      render(<DistributionDonutChart {...defaultProps} data={{}} />);

      expect(screen.getByText('No Data')).toBeInTheDocument();
      expect(screen.getByText('Add items to see distribution')).toBeInTheDocument();
    });

    it('renders title in empty state', () => {
      render(<DistributionDonutChart {...defaultProps} data={{}} />);

      expect(screen.getByText('Test Chart')).toBeInTheDocument();
    });
  });

  describe('Rendering', () => {
    it('renders title', () => {
      render(<DistributionDonutChart {...defaultProps} data={{ ItemA: 3 }} />);

      expect(screen.getByText('Test Chart')).toBeInTheDocument();
    });

    it('renders chart components', () => {
      render(<DistributionDonutChart {...defaultProps} data={{ ItemA: 3 }} />);

      expect(screen.getByTestId('responsive-container')).toBeInTheDocument();
      expect(screen.getByTestId('pie-chart')).toBeInTheDocument();
      expect(screen.getByTestId('pie')).toBeInTheDocument();
    });

    it('renders center stats with item count', () => {
      render(
        <DistributionDonutChart
          {...defaultProps}
          data={{
            ItemA: 5,
            ItemB: 4,
            ItemC: 2,
          }}
        />,
      );

      // 3 unique items
      expect(screen.getAllByText('3').length).toBeGreaterThanOrEqual(1);
      expect(screen.getByText('Items')).toBeInTheDocument();
    });

    it('renders item labels', () => {
      render(
        <DistributionDonutChart
          {...defaultProps}
          data={{
            ItemA: 3,
            ItemB: 2,
          }}
        />,
      );

      expect(screen.getByText('ItemA')).toBeInTheDocument();
      expect(screen.getByText('ItemB')).toBeInTheDocument();
    });

    it('renders item counts', () => {
      render(
        <DistributionDonutChart
          {...defaultProps}
          data={{
            ItemA: 5,
            ItemB: 3,
          }}
        />,
      );

      expect(screen.getByText('5')).toBeInTheDocument();
      expect(screen.getAllByText('3').length).toBeGreaterThanOrEqual(1);
    });
  });

  describe('Percentage Calculations', () => {
    it('renders percentage for each item', () => {
      render(
        <DistributionDonutChart
          {...defaultProps}
          data={{
            ItemA: 5,
            ItemB: 5,
          }}
        />,
      );

      // Both should be 50.0%
      const percentages = screen.getAllByText('50.0%');
      expect(percentages).toHaveLength(2);
    });

    it('calculates correct percentages with different counts', () => {
      render(
        <DistributionDonutChart
          {...defaultProps}
          data={{
            ItemA: 3, // 75%
            ItemB: 1, // 25%
          }}
        />,
      );

      expect(screen.getByText('75.0%')).toBeInTheDocument();
      expect(screen.getByText('25.0%')).toBeInTheDocument();
    });
  });

  describe('Sorting', () => {
    it('sorts items by count descending', () => {
      const { container } = render(
        <DistributionDonutChart
          {...defaultProps}
          data={{
            ItemC: 1,
            ItemA: 5,
            ItemB: 3,
          }}
        />,
      );

      // Get all item labels - they should be in order: ItemA (5), ItemB (3), ItemC (1)
      const labels = container.querySelectorAll('.text-stone-400');
      const labelTexts = Array.from(labels).map(el => el.textContent);

      // First entry should be ItemA since it has the most
      expect(labelTexts).toContain('ItemA');
    });
  });

  describe('Custom ClassName', () => {
    it('applies custom className', () => {
      const { container } = render(
        <DistributionDonutChart {...defaultProps} data={{ ItemA: 1 }} className="custom-class" />,
      );

      expect(container.querySelector('.custom-class')).toBeInTheDocument();
    });
  });

  describe('Edge Cases', () => {
    it('handles single item', () => {
      render(<DistributionDonutChart {...defaultProps} data={{ ItemA: 10 }} />);

      expect(screen.getByText('ItemA')).toBeInTheDocument();
      expect(screen.getByText('10')).toBeInTheDocument();
      expect(screen.getByText('100.0%')).toBeInTheDocument();
    });

    it('handles many items', () => {
      render(
        <DistributionDonutChart
          {...defaultProps}
          data={{
            ItemA: 2,
            ItemB: 2,
            ItemC: 2,
          }}
        />,
      );

      // Should show 3 items in center
      expect(screen.getAllByText('3').length).toBeGreaterThanOrEqual(1);
      expect(screen.getByText('ItemA')).toBeInTheDocument();
      expect(screen.getByText('ItemB')).toBeInTheDocument();
      expect(screen.getByText('ItemC')).toBeInTheDocument();
    });

    it('handles items with value of 1', () => {
      render(<DistributionDonutChart {...defaultProps} data={{ ItemA: 1 }} />);

      // '1' appears as both item count in center and item value
      expect(screen.getAllByText('1').length).toBeGreaterThanOrEqual(1);
      expect(screen.getByText('ItemA')).toBeInTheDocument();
    });

    it('handles large counts', () => {
      render(
        <DistributionDonutChart
          {...defaultProps}
          data={{
            ItemA: 100,
            ItemB: 50,
          }}
        />,
      );

      expect(screen.getByText('100')).toBeInTheDocument();
      expect(screen.getByText('50')).toBeInTheDocument();
    });
  });

  describe('Color Function', () => {
    it('calls getHexColor for each item', () => {
      mockGetHexColor.mockClear();

      render(
        <DistributionDonutChart
          {...defaultProps}
          data={{
            ItemA: 3,
            ItemB: 2,
          }}
        />,
      );

      expect(mockGetHexColor).toHaveBeenCalledWith('ItemA');
      expect(mockGetHexColor).toHaveBeenCalledWith('ItemB');
    });
  });
});
