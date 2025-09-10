import { useCharacterTimeTracking } from '../../hooks/useCharacterTimeTracking';
import { useZoneMonitoring } from '../../hooks/useZoneMonitoring';

interface RecentActivityPanelProps {
  className?: string;
}

export function RecentActivityPanel({
  className = '',
}: RecentActivityPanelProps) {
  const { timeTrackingData, isLoading } = useCharacterTimeTracking();
  const { currentZone, currentAct, lastZoneChange, lastActChange } =
    useZoneMonitoring();

  if (isLoading) {
    return (
      <div
        className={`bg-zinc-900/50 p-6 rounded-lg border border-zinc-800 ${className}`}
      >
        <h3 className='text-lg font-semibold text-white mb-4'>
          Recent Activity
        </h3>
        <div className='animate-pulse space-y-3'>
          <div className='h-4 bg-zinc-700 rounded w-3/4'></div>
          <div className='h-4 bg-zinc-700 rounded w-1/2'></div>
          <div className='h-4 bg-zinc-700 rounded w-2/3'></div>
        </div>
      </div>
    );
  }

  const recentSessions =
    timeTrackingData?.completed_sessions
      ?.sort((a, b) => new Date(b.entry_timestamp).getTime() - new Date(a.entry_timestamp).getTime())
      ?.slice(0, 5) || [];

  return (
    <div
      className={`bg-zinc-900/50 p-6 rounded-lg border border-zinc-800 ${className}`}
    >
      <h3 className='text-lg font-semibold text-white mb-4'>Recent Activity</h3>

      <div className='space-y-4'>
        {/* Current Zone/Act */}
        {(currentZone || currentAct) && (
          <div>
            <p className='text-zinc-400 text-xs mb-2'>Currently In</p>
            <div className='space-y-1'>
              {currentZone && (
                <p className='text-white text-sm'>
                  Zone: <span className='font-medium'>{currentZone}</span>
                  {lastZoneChange && (
                    <span className='text-zinc-400 text-xs ml-2'>
                      ({new Date(lastZoneChange).toLocaleTimeString()})
                    </span>
                  )}
                </p>
              )}
              {currentAct && (
                <p className='text-white text-sm'>
                  Act: <span className='font-medium'>{currentAct}</span>
                  {lastActChange && (
                    <span className='text-zinc-400 text-xs ml-2'>
                      ({new Date(lastActChange).toLocaleTimeString()})
                    </span>
                  )}
                </p>
              )}
            </div>
          </div>
        )}

        {/* Recent Sessions */}
        {recentSessions.length > 0 && (
          <div>
            <p className='text-zinc-400 text-xs mb-2'>Recent Sessions</p>
            <div className='space-y-2'>
              {recentSessions.map((session, index) => (
                <div key={index} className='text-xs'>
                  <p className='text-white'>
                    {session.location_name}
                    <span className='text-zinc-400 ml-2'>
                      (
                      {session.duration_seconds
                        ? Math.round(session.duration_seconds / 60)
                        : 0}
                      m)
                    </span>
                  </p>
                  <p className='text-zinc-500'>
                    {new Date(session.entry_timestamp).toLocaleTimeString()} -
                    {session.exit_timestamp
                      ? new Date(session.exit_timestamp).toLocaleTimeString()
                      : 'Active'}
                  </p>
                </div>
              ))}
            </div>
          </div>
        )}

        {recentSessions.length === 0 && !currentZone && !currentAct && (
          <div className='text-zinc-400 text-sm text-center py-4'>
            <p>No recent activity</p>
            <p className='text-xs mt-1'>Start playing to see activity here</p>
          </div>
        )}
      </div>
    </div>
  );
}
