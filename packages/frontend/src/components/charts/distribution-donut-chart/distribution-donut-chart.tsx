import type { ReactNode } from 'react';
import { Cell, Pie, PieChart, ResponsiveContainer, Tooltip } from 'recharts';
import { Card } from '@/components/ui/card/card';
import { DataItem } from '@/components/ui/data-item/data-item';
import { EmptyState } from '@/components/ui/empty-state/empty-state';
import { ChartTooltip } from '../chart-tooltip/chart-tooltip';
import { chartStyles } from './distribution-donut-chart.styles';

interface DistributionDonutChartProps {
  data: Record<string, number>;
  title: string;
  icon: ReactNode;
  centerLabel: string;
  getHexColor: (name: string) => string;
  emptyStateConfig: {
    title: string;
    description: string;
  };
  formatTooltipContent: (data: { name: string; value: number; percentage: number }) => string;
  className?: string;
}

interface ChartDataItem {
  name: string;
  value: number;
  percentage: number;
  hexColor: string;
  [key: string]: string | number;
}

export function DistributionDonutChart({
  data,
  title,
  icon,
  centerLabel,
  getHexColor,
  emptyStateConfig,
  formatTooltipContent,
  className = '',
}: DistributionDonutChartProps) {
  const total = Object.values(data).reduce((sum, count) => sum + count, 0);

  const chartData: ChartDataItem[] = Object.entries(data)
    .map(([name, count]) => {
      const percentage = total > 0 ? (count / total) * 100 : 0;
      const hexColor = getHexColor(name);

      return {
        name,
        value: count,
        percentage,
        hexColor,
      };
    })
    .sort((a, b) => b.value - a.value); // Sort by count descending

  if (chartData.length === 0) {
    return (
      <Card title={title} icon={icon} className={className}>
        <EmptyState
          icon={<div className="w-8 h-8">{icon}</div>}
          title={emptyStateConfig.title}
          description={emptyStateConfig.description}
        />
      </Card>
    );
  }

  return (
    <Card title={title} icon={icon} className={className}>
      <div className={chartStyles.container}>
        <div className={chartStyles.chartWrapper}>
          <div className={chartStyles.chartBackdrop} />
          <ResponsiveContainer width="100%" height={160}>
            <PieChart>
              <Pie
                data={chartData}
                cx="50%"
                cy="50%"
                innerRadius={40}
                outerRadius={60}
                paddingAngle={1}
                dataKey="value"
                stroke="var(--color-stone-900)"
                strokeWidth={2}
                className={chartStyles.pie}>
                {chartData.map(entry => (
                  <Cell
                    key={`cell-${entry.name}`}
                    fill={entry.hexColor}
                    className={chartStyles.cell}
                  />
                ))}
              </Pie>
              <Tooltip content={<ChartTooltip formatContent={formatTooltipContent} />} />
            </PieChart>
          </ResponsiveContainer>
          <div className={chartStyles.centerStats}>
            <div className={chartStyles.centerValue}>{chartData.length}</div>
            <div className={chartStyles.centerLabel}>{centerLabel}</div>
          </div>
        </div>
        <div>
          {chartData.map(item => (
            <DataItem
              key={item.name}
              label={item.name}
              value={item.value}
              subValue={`${item.percentage.toFixed(1)}%`}
              color={item.hexColor}
            />
          ))}
        </div>
      </div>
    </Card>
  );
}
