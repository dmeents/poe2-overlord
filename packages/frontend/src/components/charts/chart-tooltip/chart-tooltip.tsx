import { tooltipStyles } from './chart-tooltip.styles';

interface ChartTooltipProps {
  active?: boolean;
  payload?: Array<{
    payload: { name: string; value: number; percentage: number };
  }>;
  formatContent: (data: { name: string; value: number; percentage: number }) => string;
}

export function ChartTooltip({ active, payload, formatContent }: ChartTooltipProps) {
  if (active && payload && payload.length) {
    const data = payload[0].payload;
    return (
      <div className={tooltipStyles.container}>
        <p className={tooltipStyles.name}>{data.name}</p>
        <p className={tooltipStyles.content}>{formatContent(data)}</p>
      </div>
    );
  }
  return null;
}

// Pre-built formatters for common use cases
export function formatCountTooltip(data: { value: number; percentage: number }): string {
  return `${data.value} character${data.value !== 1 ? 's' : ''} (${data.percentage.toFixed(1)}%)`;
}

export function formatDurationTooltip(
  data: { value: number; percentage: number },
  formatDuration: (ms: number) => string,
): string {
  return `${formatDuration(data.value)} (${data.percentage.toFixed(1)}%)`;
}
