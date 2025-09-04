import { useCharacterTimeTracking } from '../../hooks/useCharacterTimeTracking';
import { formatDuration } from '../../utils';

interface QuickStatsGridProps {
  className?: string;
}

export function QuickStatsGrid({ className = '' }: QuickStatsGridProps) {
  const { timeTrackingData, activeSessions, isLoading } =
    useCharacterTimeTracking();

  if (isLoading) {
    return (
      <div
        className={`bg-zinc-900/50 p-6 rounded-lg border border-zinc-800 ${className}`}
      >
        <h3 className='text-lg font-semibold text-white mb-4'>Quick Stats</h3>
        <div className='grid grid-cols-2 md:grid-cols-4 gap-4'>
          {[...Array(4)].map((_, i) => (
            <div key={i} className='animate-pulse'>
              <div className='h-4 bg-zinc-700 rounded mb-2'></div>
              <div className='h-6 bg-zinc-700 rounded'></div>
            </div>
          ))}
        </div>
      </div>
    );
  }

  const activeSessionCount = activeSessions.length;
  const todayPlayTime = timeTrackingData?.summary?.total_play_time_seconds || 0;
  const totalLocations =
    timeTrackingData?.summary?.total_locations_tracked || 0;
  const topLocation = timeTrackingData?.summary?.top_locations?.[0];

  return (
    <div
      className={`bg-zinc-900/50 p-6 rounded-lg border border-zinc-800 ${className}`}
    >
      <h3 className='text-lg font-semibold text-white mb-4'>Quick Stats</h3>

      <div className='grid grid-cols-2 md:grid-cols-4 gap-4'>
        {/* Active Sessions */}
        <div className='text-center'>
          <p className='text-zinc-400 text-xs mb-1'>Active Sessions</p>
          <p className='text-white text-lg font-semibold'>
            {activeSessionCount}
          </p>
        </div>

        {/* Today's Play Time */}
        <div className='text-center'>
          <p className='text-zinc-400 text-xs mb-1'>Total Play Time</p>
          <p className='text-white text-lg font-semibold'>
            {formatDuration(todayPlayTime)}
          </p>
        </div>

        {/* Locations Visited */}
        <div className='text-center'>
          <p className='text-zinc-400 text-xs mb-1'>Locations Visited</p>
          <p className='text-white text-lg font-semibold'>{totalLocations}</p>
        </div>

        {/* Most Visited Location */}
        <div className='text-center'>
          <p className='text-zinc-400 text-xs mb-1'>Most Visited</p>
          <p
            className='text-white text-sm font-medium truncate'
            title={topLocation?.location_name}
          >
            {topLocation?.location_name || 'None'}
          </p>
          {topLocation && (
            <p className='text-zinc-400 text-xs'>
              {topLocation.total_visits} visits
            </p>
          )}
        </div>
      </div>
    </div>
  );
}
