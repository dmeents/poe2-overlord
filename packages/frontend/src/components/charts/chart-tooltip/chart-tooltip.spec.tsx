import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { ChartTooltip, formatCountTooltip, formatDurationTooltip } from './chart-tooltip';

describe('ChartTooltip', () => {
  const mockFormatContent = vi.fn((data) => `${data.value} (${data.percentage}%)`);

  const mockPayload = [
    {
      payload: {
        name: 'Test Item',
        value: 100,
        percentage: 25.5,
      },
    },
  ];

  it('returns null when not active', () => {
    const { container } = render(
      <ChartTooltip active={false} payload={mockPayload} formatContent={mockFormatContent} />,
    );

    expect(container.firstChild).toBeNull();
  });

  it('returns null when payload is empty', () => {
    const { container } = render(
      <ChartTooltip active={true} payload={[]} formatContent={mockFormatContent} />,
    );

    expect(container.firstChild).toBeNull();
  });

  it('returns null when payload is undefined', () => {
    const { container } = render(
      <ChartTooltip active={true} payload={undefined} formatContent={mockFormatContent} />,
    );

    expect(container.firstChild).toBeNull();
  });

  it('renders tooltip when active with payload', () => {
    render(
      <ChartTooltip active={true} payload={mockPayload} formatContent={mockFormatContent} />,
    );

    expect(screen.getByText('Test Item')).toBeInTheDocument();
    expect(mockFormatContent).toHaveBeenCalledWith({
      name: 'Test Item',
      value: 100,
      percentage: 25.5,
    });
  });

  it('renders formatted content', () => {
    render(
      <ChartTooltip active={true} payload={mockPayload} formatContent={mockFormatContent} />,
    );

    expect(screen.getByText('100 (25.5%)')).toBeInTheDocument();
  });
});

describe('formatCountTooltip', () => {
  it('formats singular count correctly', () => {
    const result = formatCountTooltip({ value: 1, percentage: 10.5 });
    expect(result).toBe('1 character (10.5%)');
  });

  it('formats plural count correctly', () => {
    const result = formatCountTooltip({ value: 5, percentage: 25.3 });
    expect(result).toBe('5 characters (25.3%)');
  });

  it('formats zero count correctly', () => {
    const result = formatCountTooltip({ value: 0, percentage: 0 });
    expect(result).toBe('0 characters (0.0%)');
  });

  it('rounds percentage to 1 decimal place', () => {
    const result = formatCountTooltip({ value: 3, percentage: 33.333333 });
    expect(result).toBe('3 characters (33.3%)');
  });
});

describe('formatDurationTooltip', () => {
  const mockFormatDuration = vi.fn((ms: number) => `${ms}s`);

  it('formats duration with percentage', () => {
    const result = formatDurationTooltip(
      { value: 3600, percentage: 50.5 },
      mockFormatDuration,
    );

    expect(mockFormatDuration).toHaveBeenCalledWith(3600);
    expect(result).toBe('3600s (50.5%)');
  });

  it('rounds percentage to 1 decimal place', () => {
    const result = formatDurationTooltip(
      { value: 1800, percentage: 33.333333 },
      mockFormatDuration,
    );

    expect(result).toBe('1800s (33.3%)');
  });
});
