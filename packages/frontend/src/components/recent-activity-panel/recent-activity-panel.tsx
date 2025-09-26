import { useCharacterTimeTracking } from '../../hooks/useCharacterTimeTracking';
import { useZoneMonitoring } from '../../hooks/useZoneMonitoring';
import { recentActivityPanelStyles } from './recent-activity-panel.styles';

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
      <div className={`${recentActivityPanelStyles.container} ${className}`}>
        <h3 className={recentActivityPanelStyles.title}>Recent Activity</h3>
        <div className={recentActivityPanelStyles.loadingContainer}>
          <div className={recentActivityPanelStyles.loadingItemWide}></div>
          <div className={recentActivityPanelStyles.loadingItemMedium}></div>
          <div className={recentActivityPanelStyles.loadingItemNarrow}></div>
        </div>
      </div>
    );
  }

  const recentSessions =
    timeTrackingData?.completed_sessions
      ?.sort(
        (a, b) =>
          new Date(b.entry_timestamp).getTime() -
          new Date(a.entry_timestamp).getTime()
      )
      ?.slice(0, 5) || [];

  return (
    <div className={`${recentActivityPanelStyles.container} ${className}`}>
      <h3 className={recentActivityPanelStyles.title}>Recent Activity</h3>

      <div className={recentActivityPanelStyles.contentContainer}>
        {/* Current Zone/Act */}
        {(currentZone || currentAct) && (
          <div>
            <p className={recentActivityPanelStyles.currentStatus}>
              Currently In
            </p>
            <div className={recentActivityPanelStyles.currentStatusItem}>
              {currentZone && (
                <p className={recentActivityPanelStyles.currentStatusText}>
                  Zone:{' '}
                  <span
                    className={recentActivityPanelStyles.currentStatusValue}
                  >
                    {currentZone}
                  </span>
                  {lastZoneChange && (
                    <span
                      className={recentActivityPanelStyles.currentStatusTime}
                    >
                      ({new Date(lastZoneChange).toLocaleTimeString()})
                    </span>
                  )}
                </p>
              )}
              {currentAct && (
                <p className={recentActivityPanelStyles.currentStatusText}>
                  Act:{' '}
                  <span
                    className={recentActivityPanelStyles.currentStatusValue}
                  >
                    {currentAct}
                  </span>
                  {lastActChange && (
                    <span
                      className={recentActivityPanelStyles.currentStatusTime}
                    >
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
            <p className={recentActivityPanelStyles.recentSessions}>
              Recent Sessions
            </p>
            <div className={recentActivityPanelStyles.sessionsList}>
              {recentSessions.map((session, index) => (
                <div
                  key={index}
                  className={recentActivityPanelStyles.sessionItem}
                >
                  <p className={recentActivityPanelStyles.sessionLocation}>
                    {session.location_name}
                    <span className={recentActivityPanelStyles.sessionDuration}>
                      (
                      {session.duration_seconds
                        ? Math.round(session.duration_seconds / 60)
                        : 0}
                      m)
                    </span>
                  </p>
                  <p className={recentActivityPanelStyles.sessionTime}>
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
          <div className={recentActivityPanelStyles.emptyState}>
            <p>No recent activity</p>
            <p className={recentActivityPanelStyles.emptyStateSubtext}>
              Start playing to see activity here
            </p>
          </div>
        )}
      </div>
    </div>
  );
}
