import type { LocationSession } from '@/types';
import { useEffect, useState } from 'react';
import { Button } from '../button';
import { TimeDisplay } from './time-display';

interface ActiveSessionsProps {
  sessions: LocationSession[];
  onEndSession: (locationId: string) => Promise<void>;
  className?: string;
}

export function ActiveSessions({
  sessions,
  onEndSession,
  className = '',
}: ActiveSessionsProps) {
  const [elapsedTimes, setElapsedTimes] = useState<Record<string, number>>({});

  // Update elapsed times every second for active sessions
  useEffect(() => {
    const interval = setInterval(() => {
      const now = Date.now();
      const newElapsedTimes: Record<string, number> = {};

      sessions.forEach(session => {
        const entryTime = new Date(session.entry_timestamp).getTime();
        const elapsed = Math.floor((now - entryTime) / 1000);
        newElapsedTimes[session.location_id] = elapsed;
      });

      setElapsedTimes(newElapsedTimes);
    }, 1000);

    return () => clearInterval(interval);
  }, [sessions]);

  if (sessions.length === 0) {
    return (
      <div
        className={`bg-zinc-900/50 p-4 rounded-lg border border-zinc-800 ${className}`}
      >
        <h3 className='text-lg font-semibold text-white mb-2'>
          Active Sessions
        </h3>
        <div className='text-center text-zinc-500 py-4'>No active sessions</div>
      </div>
    );
  }

  return (
    <div
      className={`bg-zinc-900/50 p-4 rounded-lg border border-zinc-800 ${className}`}
    >
      <h3 className='text-lg font-semibold text-white mb-4'>Active Sessions</h3>
      <div className='space-y-3'>
        {sessions.map(session => (
          <div
            key={session.location_id}
            className='flex items-center justify-between p-3 bg-zinc-800/50 rounded-lg border border-zinc-700'
          >
            <div className='flex-1'>
              <div className='flex items-center gap-2 mb-1'>
                <span className='text-sm font-medium text-zinc-300'>
                  {session.location_type}
                </span>
                <span className='text-white font-semibold'>
                  {session.location_name}
                </span>
              </div>
              <div className='text-sm text-zinc-400'>
                Started:{' '}
                {new Date(session.entry_timestamp).toLocaleTimeString()}
              </div>
              <div className='text-sm text-emerald-400 font-mono'>
                Elapsed:{' '}
                <TimeDisplay seconds={elapsedTimes[session.location_id] || 0} />
              </div>
            </div>
            <Button
              onClick={() => onEndSession(session.location_id)}
              variant='outline'
              size='sm'
              className='ml-4'
            >
              End Session
            </Button>
          </div>
        ))}
      </div>
    </div>
  );
}
