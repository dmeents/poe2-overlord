import type { CharacterData } from '@/types';
import { formatDuration } from '@/utils';
import { ChartPieIcon } from '@heroicons/react/24/outline';
import { Cell, Pie, PieChart, ResponsiveContainer, Tooltip } from 'recharts';
import { actDistributionChartStyles } from './act-distribution-chart.styles';

interface ActDistributionChartProps {
  character: CharacterData;
  className?: string;
}

interface ActData {
  name: string;
  time: number;
  percentage: number;
  color: string;
  hexColor: string;
}

export function ActDistributionChart({
  character,
  className = '',
}: ActDistributionChartProps) {
  // Extract act data from character summary
  const actData: ActData[] = [
    {
      name: 'Act 1',
      time: character.summary.play_time_act1,
      percentage: 0, // Will be calculated
      color: 'bg-emerald-500',
      hexColor: '#10b981',
    },
    {
      name: 'Act 2',
      time: character.summary.play_time_act2,
      percentage: 0, // Will be calculated
      color: 'bg-blue-500',
      hexColor: '#3b82f6',
    },
    {
      name: 'Act 3',
      time: character.summary.play_time_act3,
      percentage: 0, // Will be calculated
      color: 'bg-purple-500',
      hexColor: '#8b5cf6',
    },
    {
      name: 'Act 4',
      time: character.summary.play_time_act4,
      percentage: 0, // Will be calculated
      color: 'bg-amber-500',
      hexColor: '#f59e0b',
    },
    {
      name: 'Interlude',
      time: character.summary.play_time_interlude,
      percentage: 0, // Will be calculated
      color: 'bg-red-500',
      hexColor: '#ef4444',
    },
  ];

  // Calculate total time for main acts only
  const totalTime = actData.reduce((sum, act) => sum + act.time, 0);

  // Calculate percentages
  actData.forEach(act => {
    act.percentage = totalTime > 0 ? (act.time / totalTime) * 100 : 0;
  });

  // Filter out acts with no time
  const activeActs = actData.filter(act => act.time > 0);

  if (activeActs.length === 0) {
    return (
      <div className={`${actDistributionChartStyles.container} ${className}`}>
        <h3 className={actDistributionChartStyles.title}>
          <ChartPieIcon className='w-5 h-5 mr-2 text-zinc-400' />
          Act Distribution
        </h3>
        <div className={actDistributionChartStyles.emptyState}>
          <div className={actDistributionChartStyles.emptyIcon}>
            <ChartPieIcon className='w-8 h-8' />
          </div>
          <div className={actDistributionChartStyles.emptyTitle}>
            No Act Data
          </div>
          <div className={actDistributionChartStyles.emptySubtitle}>
            Start playing to see act distribution
          </div>
        </div>
      </div>
    );
  }

  // Prepare data for Recharts
  const chartData = activeActs.map(act => ({
    name: act.name,
    value: act.time,
    percentage: act.percentage,
    color: act.hexColor,
  }));

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
            {formatDuration(data.value)} ({data.percentage.toFixed(1)}%)
          </p>
        </div>
      );
    }
    return null;
  };

  return (
    <div className={`${actDistributionChartStyles.container} ${className}`}>
      <h3 className={actDistributionChartStyles.title}>
        <ChartPieIcon className='w-5 h-5 mr-2 text-zinc-400' />
        Act Distribution
      </h3>

      {/* Chart and Legend */}
      <div className={actDistributionChartStyles.chartSection}>
        {/* Donut Chart */}
        <div className={actDistributionChartStyles.donutContainer}>
          <ResponsiveContainer width='100%' height={200}>
            <PieChart>
              <Pie
                data={chartData}
                cx='50%'
                cy='50%'
                innerRadius={60}
                outerRadius={80}
                paddingAngle={2}
                dataKey='value'
                stroke='none'
                className='transition-all duration-300'
              >
                {chartData.map((entry, index) => (
                  <Cell
                    key={`cell-${index}`}
                    fill={entry.color}
                    className='transition-all duration-300 hover:opacity-80 cursor-pointer'
                  />
                ))}
              </Pie>
              <Tooltip content={<CustomTooltip />} />
            </PieChart>
          </ResponsiveContainer>

          {/* Center text */}
          <div className={actDistributionChartStyles.centerText}>
            <div className={actDistributionChartStyles.centerValue}>
              {activeActs.length}
            </div>
            <div className={actDistributionChartStyles.centerLabel}>Acts</div>
          </div>
        </div>

        {/* Total Campaign Time Row */}
        <div className={actDistributionChartStyles.totalTimeItem}>
          <div className={actDistributionChartStyles.totalTimeInfo}>
            <div className={actDistributionChartStyles.totalTimeName}>
              Total Campaign Time
            </div>
          </div>
          <div className={actDistributionChartStyles.totalTimeValue}>
            {formatDuration(totalTime)}
          </div>
        </div>

        {/* Legend */}
        <div className={actDistributionChartStyles.legend}>
          {activeActs.map(act => (
            <div
              key={act.name}
              className={actDistributionChartStyles.legendItem}
            >
              <div
                className={`${actDistributionChartStyles.legendColor} ${act.color}`}
              />
              <div className={actDistributionChartStyles.legendInfo}>
                <div className={actDistributionChartStyles.legendName}>
                  {act.name}
                </div>
                <div className={actDistributionChartStyles.legendTime}>
                  {formatDuration(act.time)}
                </div>
              </div>
              <div className={actDistributionChartStyles.legendPercentage}>
                {act.percentage.toFixed(1)}%
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
