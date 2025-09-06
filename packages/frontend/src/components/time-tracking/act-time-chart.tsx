import type { LocationStats } from '@/types';
import { formatDuration } from '@/utils';

interface ActTimeChartProps {
  stats: LocationStats[];
  className?: string;
}

interface ActData {
  name: string;
  totalTimeSeconds: number;
  percentage: number;
  visits: number;
  averageSessionSeconds: number;
}

export function ActTimeChart({ stats, className = '' }: ActTimeChartProps) {
  // Filter and process ACT data
  const actData: ActData[] = stats
    .filter(stat => stat.location_type === 'Act')
    .map(stat => ({
      name: stat.location_name,
      totalTimeSeconds: stat.total_time_seconds,
      percentage: 0, // Will be calculated after we know the total
      visits: stat.total_visits,
      averageSessionSeconds: stat.average_session_seconds,
    }))
    .sort((a, b) => b.totalTimeSeconds - a.totalTimeSeconds);

  // Calculate total time for percentage calculations
  const totalActTime = actData.reduce(
    (sum, act) => sum + act.totalTimeSeconds,
    0
  );

  // Update percentages
  actData.forEach(act => {
    act.percentage =
      totalActTime > 0 ? (act.totalTimeSeconds / totalActTime) * 100 : 0;
  });

  if (actData.length === 0) {
    return (
      <div
        className={`bg-zinc-900/50 p-4 rounded-lg border border-zinc-800 ${className}`}
      >
        <h3 className='text-lg font-semibold text-white mb-2'>
          ACT Time Distribution
        </h3>
        <div className='text-center text-zinc-500 py-8'>
          <div className='mb-2'>
            <svg
              className='mx-auto h-8 w-8 text-zinc-600'
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
          <p className='text-sm'>No Act data available</p>
          <p className='text-xs text-zinc-600 mt-1'>
            Play through different Acts to see time distribution
          </p>
        </div>
      </div>
    );
  }

  // Color palette for different ACTs
  const actColors = [
    'bg-emerald-500',
    'bg-blue-500',
    'bg-purple-500',
    'bg-amber-500',
    'bg-red-500',
    'bg-cyan-500',
    'bg-pink-500',
    'bg-indigo-500',
    'bg-orange-500',
    'bg-teal-500',
  ];

  return (
    <div
      className={`bg-zinc-900/50 p-4 rounded-lg border border-zinc-800 ${className}`}
    >
      <div className='flex items-center justify-between mb-4'>
        <h3 className='text-lg font-semibold text-white'>
          Act Time Distribution
        </h3>
        <div className='text-sm text-zinc-400'>
          Total: {formatDuration(totalActTime)}
        </div>
      </div>

      <div className='space-y-3'>
        {actData.map((act, index) => (
          <div key={act.name} className='group'>
            {/* ACT Header */}
            <div className='flex items-center justify-between mb-2'>
              <div className='flex items-center gap-2'>
                <div
                  className={`w-3 h-3 rounded-sm ${actColors[index % actColors.length]}`}
                />
                <span className='text-white font-medium'>{act.name}</span>
              </div>
              <div className='flex items-center gap-3 text-sm'>
                <span className='text-zinc-300 font-mono'>
                  {formatDuration(act.totalTimeSeconds)}
                </span>
                <span className='text-zinc-500 min-w-[3rem] text-right'>
                  {act.percentage.toFixed(1)}%
                </span>
              </div>
            </div>

            {/* Progress Bar */}
            <div className='relative'>
              <div className='w-full h-2 bg-zinc-800 rounded-sm overflow-hidden'>
                <div
                  className={`h-full ${actColors[index % actColors.length]} transition-all duration-500 ease-out`}
                  style={{ width: `${act.percentage}%` }}
                />
              </div>

              {/* Hover tooltip */}
              <div className='absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-200 pointer-events-none'>
                <div className='absolute top-0 left-0 transform -translate-y-full -translate-x-1/2 bg-zinc-800 border border-zinc-700 rounded px-2 py-1 text-xs text-white whitespace-nowrap z-10'>
                  <div className='font-medium'>{act.name}</div>
                  <div className='text-zinc-300'>
                    {formatDuration(act.totalTimeSeconds)} (
                    {act.percentage.toFixed(1)}%)
                  </div>
                  <div className='text-zinc-400'>
                    {act.visits} visits • Avg:{' '}
                    {formatDuration(Math.round(act.averageSessionSeconds))}
                  </div>
                </div>
              </div>
            </div>

            {/* Additional Stats */}
            <div className='flex items-center justify-between mt-1 text-xs text-zinc-500'>
              <span>{act.visits} visits</span>
              <span>
                Avg: {formatDuration(Math.round(act.averageSessionSeconds))}
              </span>
            </div>
          </div>
        ))}
      </div>

      {/* Summary Stats */}
      {actData.length > 1 && (
        <div className='mt-4 pt-4 border-t border-zinc-800'>
          <div className='grid grid-cols-2 gap-4 text-sm'>
            <div>
              <span className='text-zinc-500 block text-xs uppercase tracking-wide'>
                Most Time
              </span>
              <span className='text-white font-medium'>
                {actData[0]?.name || 'N/A'}
              </span>
            </div>
            <div>
              <span className='text-zinc-500 block text-xs uppercase tracking-wide'>
                Least Time
              </span>
              <span className='text-white font-medium'>
                {actData[actData.length - 1]?.name || 'N/A'}
              </span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
