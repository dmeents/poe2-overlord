import type { CharacterData } from '@/types';
import { formatDuration } from '@/utils';
import { ChartPieIcon } from '@heroicons/react/24/outline';
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

  // Calculate donut chart segments
  let cumulativePercentage = 0;
  const segments = activeActs.map(act => {
    const startAngle = (cumulativePercentage / 100) * 360;
    const endAngle = ((cumulativePercentage + act.percentage) / 100) * 360;
    cumulativePercentage += act.percentage;

    return {
      ...act,
      startAngle,
      endAngle,
    };
  });

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
          <svg
            className={actDistributionChartStyles.donutSvg}
            viewBox='0 0 100 100'
            xmlns='http://www.w3.org/2000/svg'
          >
            {/* Background circle */}
            <circle
              cx='50'
              cy='50'
              r='40'
              fill='none'
              stroke='rgb(39 39 42)'
              strokeWidth='8'
            />

            {/* Act segments */}
            {segments.map(segment => {
              const radius = 40;
              const centerX = 50;
              const centerY = 50;

              const startAngleRad = (segment.startAngle - 90) * (Math.PI / 180);
              const endAngleRad = (segment.endAngle - 90) * (Math.PI / 180);

              const largeArcFlag =
                segment.endAngle - segment.startAngle > 180 ? 1 : 0;

              const x1 = centerX + radius * Math.cos(startAngleRad);
              const y1 = centerY + radius * Math.sin(startAngleRad);
              const x2 = centerX + radius * Math.cos(endAngleRad);
              const y2 = centerY + radius * Math.sin(endAngleRad);

              const pathData = [
                `M ${x1} ${y1}`,
                `A ${radius} ${radius} 0 ${largeArcFlag} 1 ${x2} ${y2}`,
              ].join(' ');

              return (
                <path
                  key={segment.name}
                  d={pathData}
                  fill='none'
                  stroke={segment.hexColor}
                  strokeWidth='8'
                  strokeLinecap='round'
                  className={actDistributionChartStyles.segment}
                />
              );
            })}
          </svg>

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
