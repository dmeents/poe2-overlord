import type { LocationSession } from '@/types';
import { TimeDisplay } from '../time-display';
import { timeSessionHistoryStyles } from './time-session-history.styles';

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
      <div className={`${timeSessionHistoryStyles.container} ${className}`}>
        <h3 className={timeSessionHistoryStyles.title}>Session History</h3>
        <div className={timeSessionHistoryStyles.emptyState}>
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
    <div className={`${timeSessionHistoryStyles.container} ${className}`}>
      <h3 className={timeSessionHistoryStyles.titleWithMargin}>
        Session History
      </h3>
      <div className={timeSessionHistoryStyles.sessionsContainer}>
        {sortedSessions.map((session, index) => (
          <div
            key={`${session.location_id}-${index}`}
            className={timeSessionHistoryStyles.sessionItem}
          >
            <div className={timeSessionHistoryStyles.sessionHeader}>
              <div className={timeSessionHistoryStyles.sessionInfo}>
                <span className={timeSessionHistoryStyles.sessionType}>
                  {session.location_type}
                </span>
                <span className={timeSessionHistoryStyles.sessionName}>
                  {session.location_name}
                </span>
              </div>
              {session.duration_seconds && (
                <div className={timeSessionHistoryStyles.sessionDuration}>
                  <TimeDisplay seconds={session.duration_seconds} />
                </div>
              )}
            </div>

            <div className={timeSessionHistoryStyles.sessionGrid}>
              <div>
                <span className={timeSessionHistoryStyles.sessionLabel}>
                  Started
                </span>
                <span className={timeSessionHistoryStyles.sessionValue}>
                  {new Date(session.entry_timestamp).toLocaleString()}
                </span>
              </div>
              <div>
                <span className={timeSessionHistoryStyles.sessionLabel}>
                  Ended
                </span>
                <span className={timeSessionHistoryStyles.sessionValue}>
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
