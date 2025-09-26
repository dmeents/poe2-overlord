import type { ProcessInfo } from '@/types';
import { logMonitoringStatusStyles } from './log-monitoring-status.styles';

interface MonitoringStatusProps {
  isMonitoring: boolean;
  poeProcessStatus: ProcessInfo | null;
  logFileSize: number;
}

export function MonitoringStatus({
  isMonitoring,
  poeProcessStatus,
  logFileSize,
}: MonitoringStatusProps) {
  return (
    <div className={logMonitoringStatusStyles.container}>
      <h3 className={logMonitoringStatusStyles.title}>Monitoring Status</h3>
      <div className={logMonitoringStatusStyles.statusList}>
        <div className={logMonitoringStatusStyles.statusItem}>
          <span className={logMonitoringStatusStyles.label}>Status:</span>
          <span
            className={`${logMonitoringStatusStyles.statusBadge} ${
              isMonitoring
                ? logMonitoringStatusStyles.statusActive
                : logMonitoringStatusStyles.statusInactive
            }`}
          >
            {isMonitoring ? 'Active' : 'Inactive'}
          </span>
        </div>
        <div className={logMonitoringStatusStyles.statusItem}>
          <span className={logMonitoringStatusStyles.label}>POE Process:</span>
          <span
            className={`${logMonitoringStatusStyles.statusBadge} ${
              poeProcessStatus?.running
                ? logMonitoringStatusStyles.statusActive
                : logMonitoringStatusStyles.statusInactive
            }`}
          >
            {poeProcessStatus?.running ? 'Running' : 'Not Running'}
          </span>
        </div>
        {poeProcessStatus?.running && (
          <div className={logMonitoringStatusStyles.statusItem}>
            <span className={logMonitoringStatusStyles.label}>Process ID:</span>
            <span className={logMonitoringStatusStyles.value}>
              {poeProcessStatus.pid}
            </span>
          </div>
        )}
        <div className={logMonitoringStatusStyles.statusItem}>
          <span className={logMonitoringStatusStyles.label}>
            Log File Size:
          </span>
          <span className={logMonitoringStatusStyles.value}>
            {(logFileSize / 1024).toFixed(1)} KB
          </span>
        </div>
      </div>
    </div>
  );
}
