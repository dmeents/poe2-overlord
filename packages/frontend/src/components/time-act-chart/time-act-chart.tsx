import type { ZoneStats } from '@/types';
import { formatDuration } from '@/utils';
import { timeActChartStyles } from './time-act-chart.styles';

interface ActTimeChartProps {
  stats: ZoneStats[];
  className?: string;
}

interface ActData {
  name: string;
  totalTimeSeconds: number;
  percentage: number;
  visits: number;
  averageVisitSeconds: number;
}

export function ActTimeChart({ stats, className = '' }: ActTimeChartProps) {
  // Filter and process ACT data
  const actData: ActData[] = stats
    .filter(stat => stat.location_type === 'Act')
    .map(stat => ({
      name: stat.location_name,
      totalTimeSeconds: stat.duration,
      percentage: 0, // Will be calculated after we know the total
      visits: stat.visits,
      averageVisitSeconds:
        stat.visits > 0 ? Math.round(stat.duration / stat.visits) : 0,
    }))
    .sort((a, b) => {
      // Extract act numbers for chronological sorting
      const getActNumber = (name: string) => {
        const match = name.match(/Act (\d+)/i);
        return match ? parseInt(match[1], 10) : 0;
      };
      return getActNumber(a.name) - getActNumber(b.name);
    });

  // Calculate total time for display purposes
  const totalActTime = actData.reduce(
    (sum, act) => sum + act.totalTimeSeconds,
    0
  );

  // Find the act with the most playtime to use as the basis
  const maxActTime =
    actData.length > 0
      ? Math.max(...actData.map(act => act.totalTimeSeconds))
      : 0;

  // Update percentages based on the act with the most playtime
  actData.forEach(act => {
    act.percentage =
      maxActTime > 0 ? (act.totalTimeSeconds / maxActTime) * 100 : 0;
  });

  if (actData.length === 0) {
    return (
      <div className={`${timeActChartStyles.container} ${className}`}>
        <h3 className={timeActChartStyles.title}>ACT Time Distribution</h3>
        <div className={timeActChartStyles.emptyState}>
          <div className='mb-2'>
            <svg
              className={timeActChartStyles.emptyIcon}
              fill='none'
              viewBox='0 0 24 24'
              stroke='currentColor'
            >
              <path
                strokeLinecap='round'
                strokeLinejoin='round'
                strokeWidth={2}
                d='M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z'
              />
            </svg>
          </div>
          <p className={timeActChartStyles.emptyTitle}>No Act data available</p>
          <p className={timeActChartStyles.emptySubtitle}>
            Play through different Acts to see time distribution
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className={`${timeActChartStyles.container} ${className}`}>
      <div className={timeActChartStyles.header}>
        <h3 className={timeActChartStyles.title}>Act Time Distribution</h3>
        <div className={timeActChartStyles.totalTime}>
          Total: {formatDuration(totalActTime)}
        </div>
      </div>

      <div className={timeActChartStyles.chartContainer}>
        {actData.map((act, index) => (
          <div key={act.name} className={timeActChartStyles.actItem}>
            {/* ACT Header */}
            <div className={timeActChartStyles.actHeader}>
              <div className={timeActChartStyles.actInfo}>
                <div
                  className={`${timeActChartStyles.actColor} ${timeActChartStyles.actColors[index % timeActChartStyles.actColors.length]}`}
                />
                <span className={timeActChartStyles.actName}>{act.name}</span>
              </div>
              <div className={timeActChartStyles.actTime}>
                <span className={timeActChartStyles.actTimeValue}>
                  {formatDuration(act.totalTimeSeconds)}
                </span>
              </div>
            </div>

            {/* Progress Bar */}
            <div className={timeActChartStyles.progressContainer}>
              <div className={timeActChartStyles.progressBar}>
                <div
                  className={`${timeActChartStyles.progressFill} ${timeActChartStyles.actColors[index % timeActChartStyles.actColors.length]}`}
                  style={{ width: `${act.percentage}%` }}
                />
              </div>

              {/* Hover tooltip */}
              <div className={timeActChartStyles.tooltip}>
                <div className={timeActChartStyles.tooltipContent}>
                  <div className={timeActChartStyles.tooltipTitle}>
                    {act.name}
                  </div>
                  <div className={timeActChartStyles.tooltipTime}>
                    {formatDuration(act.totalTimeSeconds)}
                  </div>
                  <div className={timeActChartStyles.tooltipDetails}>
                    {act.visits} visits • Avg:{' '}
                    {formatDuration(act.averageVisitSeconds)}
                  </div>
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
