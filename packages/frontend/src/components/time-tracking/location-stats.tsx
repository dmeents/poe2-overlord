import type { LocationStats } from '@/types';
import { TimeDisplay } from './time-display';

interface LocationStatsProps {
  stats: LocationStats[];
  className?: string;
}

export function LocationStats({ stats, className = '' }: LocationStatsProps) {
  if (stats.length === 0) {
    return (
      <div
        className={`bg-zinc-900/50 p-4 rounded-lg border border-zinc-800 ${className}`}
      >
        <h3 className='text-lg font-semibold text-white mb-2'>
          Location Statistics
        </h3>
        <div className='text-center text-zinc-500 py-4'>
          No location data available
        </div>
      </div>
    );
  }

  // Sort stats by total time (descending)
  const sortedStats = [...stats].sort(
    (a, b) => b.total_time_seconds - a.total_time_seconds
  );

  return (
    <div
      className={`bg-zinc-900/50 p-4 rounded-lg border border-zinc-800 ${className}`}
    >
      <h3 className='text-lg font-semibold text-white mb-4'>
        Location Statistics
      </h3>
      <div className='space-y-3'>
        {sortedStats.map(stat => (
          <div
            key={stat.location_id}
            className='p-3 bg-zinc-800/50 rounded-lg border border-zinc-700'
          >
            <div className='flex items-center justify-between mb-2'>
              <div className='flex items-center gap-2'>
                <span className='text-sm font-medium text-zinc-300'>
                  {stat.location_type}
                </span>
                <span className='text-white font-semibold'>
                  {stat.location_name}
                </span>
              </div>
              <div className='text-sm text-emerald-400 font-mono'>
                <TimeDisplay
                  seconds={stat.total_time_seconds}
                  showSeconds={false}
                />
              </div>
            </div>

            <div className='grid grid-cols-3 gap-4 text-sm text-zinc-400'>
              <div>
                <span className='block text-zinc-500 text-xs uppercase tracking-wide'>
                  Visits
                </span>
                <span className='text-white font-medium'>
                  {stat.total_visits}
                </span>
              </div>
              <div>
                <span className='block text-zinc-500 text-xs uppercase tracking-wide'>
                  Avg Session
                </span>
                <span className='text-white font-medium'>
                  <TimeDisplay
                    seconds={Math.round(stat.average_session_seconds)}
                  />
                </span>
              </div>
              <div>
                <span className='block text-zinc-500 text-xs uppercase tracking-wide'>
                  Last Visit
                </span>
                <span className='text-white font-medium'>
                  {stat.last_visited
                    ? new Date(stat.last_visited).toLocaleDateString()
                    : 'Never'}
                </span>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
