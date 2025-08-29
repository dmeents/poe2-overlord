import type { LocationSession } from '@/types';
import { TimeDisplay } from './time-display';

interface SessionHistoryProps {
  sessions: LocationSession[];
  className?: string;
}

export function SessionHistory({
  sessions,
  className = '',
}: SessionHistoryProps) {
  if (sessions.length === 0) {
    return (
      <div
        className={`bg-zinc-900/50 p-4 rounded-lg border border-zinc-800 ${className}`}
      >
        <h3 className='text-lg font-semibold text-white mb-2'>
          Session History
        </h3>
        <div className='text-center text-zinc-500 py-4'>
          No completed sessions
        </div>
      </div>
    );
  }

  // Sort sessions by exit timestamp (most recent first)
  const sortedSessions = [...sessions].sort((a, b) => {
    if (!a.exit_timestamp || !b.exit_timestamp) return 0;
    return (
      new Date(b.exit_timestamp).getTime() -
      new Date(a.exit_timestamp).getTime()
    );
  });

  return (
    <div
      className={`bg-zinc-900/50 p-4 rounded-lg border border-zinc-800 ${className}`}
    >
      <h3 className='text-lg font-semibold text-white mb-4'>Session History</h3>
      <div className='space-y-3 max-h-96 overflow-y-auto'>
        {sortedSessions.map((session, index) => (
          <div
            key={`${session.location_id}-${index}`}
            className='p-3 bg-zinc-800/50 rounded-lg border border-zinc-700'
          >
            <div className='flex items-center justify-between mb-2'>
              <div className='flex items-center gap-2'>
                <span className='text-sm font-medium text-zinc-300'>
                  {session.location_type}
                </span>
                <span className='text-white font-semibold'>
                  {session.location_name}
                </span>
              </div>
              {session.duration_seconds && (
                <div className='text-sm text-emerald-400 font-mono'>
                  <TimeDisplay seconds={session.duration_seconds} />
                </div>
              )}
            </div>

            <div className='grid grid-cols-2 gap-4 text-sm text-zinc-400'>
              <div>
                <span className='block text-zinc-500 text-xs uppercase tracking-wide'>
                  Started
                </span>
                <span className='text-white font-medium'>
                  {new Date(session.entry_timestamp).toLocaleString()}
                </span>
              </div>
              <div>
                <span className='block text-zinc-500 text-xs uppercase tracking-wide'>
                  Ended
                </span>
                <span className='text-white font-medium'>
                  {session.exit_timestamp
                    ? new Date(session.exit_timestamp).toLocaleString()
                    : 'Unknown'}
                </span>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
