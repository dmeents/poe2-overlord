import { UsersIcon } from '@heroicons/react/24/outline';
import { useMemo } from 'react';
import { Cell, Pie, PieChart, ResponsiveContainer, Tooltip } from 'recharts';
import { DataItem, SectionHeader } from '../../ui';
import { classDistributionChartStyles } from './class-distribution-chart.styles';

interface ClassDistributionChartProps {
  classDistribution: Record<string, number>;
  className?: string;
}


// Color scheme for character classes - matching character card colors
// Only base classes, ascendency classes will use their root class color
const CLASS_COLORS = {
  Warrior: { color: 'bg-red-500', hexColor: '#ef4444' },
  Sorceress: { color: 'bg-blue-500', hexColor: '#3b82f6' },
  Ranger: { color: 'bg-green-500', hexColor: '#22c55e' },
  Huntress: { color: 'bg-yellow-500', hexColor: '#eab308' },
  Monk: { color: 'bg-purple-500', hexColor: '#a855f7' },
  Mercenary: { color: 'bg-orange-500', hexColor: '#f97316' },
  Witch: { color: 'bg-pink-500', hexColor: '#ec4899' },
} as const;

export function ClassDistributionChart({
  classDistribution,
  className = '',
}: ClassDistributionChartProps) {
  // Memoize chart data to prevent infinite re-renders
  const chartData = useMemo(() => {
    // Convert class distribution to chart data
    const totalCharacters = Object.values(classDistribution).reduce(
      (sum, count) => sum + count,
      0
    );

    return Object.entries(classDistribution)
      .map(([className, count]) => {
        const percentage =
          totalCharacters > 0 ? (count / totalCharacters) * 100 : 0;
        const classColor = CLASS_COLORS[
          className as keyof typeof CLASS_COLORS
        ] || {
          color: 'bg-zinc-500',
          hexColor: '#71717a',
        };

        return {
          name: className,
          value: count,
          percentage,
          color: classColor.color,
          hexColor: classColor.hexColor,
        };
      })
      .sort((a, b) => b.value - a.value); // Sort by count descending
  }, [classDistribution]);

  // Custom tooltip component
  const CustomTooltip = ({
    active,
    payload,
  }: {
    active?: boolean;
    payload?: Array<{
      payload: { name: string; value: number; percentage: number };
    }>;
  }) => {
    if (active && payload && payload.length) {
      const data = payload[0].payload;
      return (
        <div className='bg-zinc-800 border border-zinc-700 rounded-lg p-3 shadow-lg relative z-50'>
          <p className='text-white font-medium'>{data.name}</p>
          <p className='text-zinc-300 text-sm'>
            {data.value} character{data.value !== 1 ? 's' : ''} (
            {data.percentage.toFixed(1)}%)
          </p>
        </div>
      );
    }
    return null;
  };

  if (chartData.length === 0) {
    return (
      <div className={`${classDistributionChartStyles.container} ${className}`}>
        <h4 className={classDistributionChartStyles.title}>
          <UsersIcon className='w-4 h-4 mr-2 text-zinc-400' />
          Classes
        </h4>
        <div className={classDistributionChartStyles.emptyState}>
          <div className={classDistributionChartStyles.emptyIcon}>
            <UsersIcon className='w-8 h-8' />
          </div>
          <div className={classDistributionChartStyles.emptyTitle}>
            No Class Data
          </div>
          <div className={classDistributionChartStyles.emptySubtitle}>
            Create characters to see class distribution
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className={`${classDistributionChartStyles.container} ${className}`}>
      <h4 className={classDistributionChartStyles.title}>
        <UsersIcon className='w-4 h-4 mr-2 text-zinc-400' />
        Classes
      </h4>

      {/* Chart and Legend */}
      <div className={classDistributionChartStyles.chartSection}>
        {/* Donut Chart */}
        <div className={classDistributionChartStyles.donutContainer}>
          <ResponsiveContainer width='100%' height={160}>
            <PieChart>
              <Pie
                data={chartData}
                cx='50%'
                cy='50%'
                innerRadius={40}
                outerRadius={60}
                paddingAngle={1}
                dataKey='value'
                stroke='none'
                className='transition-all duration-300'
              >
                {chartData.map((entry, index) => (
                  <Cell
                    key={`cell-${index}`}
                    fill={entry.hexColor}
                    className='transition-all duration-300 hover:opacity-80 cursor-pointer'
                  />
                ))}
              </Pie>
              <Tooltip content={<CustomTooltip />} />
            </PieChart>
          </ResponsiveContainer>

          {/* Center text */}
          <div className={classDistributionChartStyles.centerText}>
            <div className={classDistributionChartStyles.centerValue}>
              {chartData.length}
            </div>
            <div className={classDistributionChartStyles.centerLabel}>
              Classes
            </div>
          </div>
        </div>

        {/* Legend */}
        <SectionHeader
          title='Classes'
          icon={<UsersIcon className='w-4 h-4' />}
        />
        <div className='space-y-2'>
          {chartData.map(classItem => (
            <DataItem
              key={classItem.name}
              label={classItem.name}
              value={classItem.value}
              subValue={`${classItem.percentage.toFixed(1)}%`}
              color={classItem.hexColor}
            />
          ))}
        </div>
      </div>
    </div>
  );
}
