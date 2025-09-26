import type { ProcessInfo, SceneChangeEvent } from '@/types';
import { getSceneEventTimestamp } from '@/types';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';
import { Button } from '../button';
import { ActivityLog } from '../log-activity-log';
import { MonitoringStatus } from '../log-monitoring-status';
import { RecentLogLines } from '../log-recent-log-lines';
import { logMonitorStyles } from './log-monitor.styles';

// Combined event type for unified display
export type SceneEvent = {
  type: 'zone' | 'act' | 'hideout';
  data: SceneChangeEvent;
  timestamp: string;
};

export function LogMonitor() {
  const [isMonitoring, setIsMonitoring] = useState(false);
  const [sceneEvents, setSceneEvents] = useState<SceneEvent[]>([]);
  const [logFileSize, setLogFileSize] = useState<number>(0);
  const [lastLines, setLastLines] = useState<string[]>([]);
  const [poeProcessStatus, setPoeProcessStatus] = useState<ProcessInfo | null>(
    null
  );

  useEffect(() => {
    // Check initial monitoring status
    checkMonitoringStatus();

    // Set up event listeners - now using unified scene change events
    const unlistenScene = listen('log-scene-change', event => {
      const sceneEvent = event.payload as SceneChangeEvent;
      setSceneEvents(prev => [
        {
          type: sceneEvent.type.toLowerCase() as 'zone' | 'act' | 'hideout',
          data: sceneEvent,
          timestamp: getSceneEventTimestamp(sceneEvent),
        },
        ...prev,
      ]);
    });

    const unlistenProcess = listen('game-process-status', event => {
      const processInfo = event.payload as ProcessInfo;
      setPoeProcessStatus(processInfo);
      setIsMonitoring(processInfo.running);
    });

    // Cleanup listeners
    return () => {
      unlistenScene.then(f => f());
      unlistenProcess.then(f => f());
    };
  }, []);

  const checkMonitoringStatus = async () => {
    try {
      console.log('Checking monitoring status...');
      const status = await invoke<boolean>('is_log_monitoring_active');
      console.log('Monitoring status:', status);
      setIsMonitoring(status);
    } catch (error) {
      console.error('Failed to check monitoring status:', error);
    }
  };

  const refreshLogInfo = async () => {
    try {
      const [size, lines] = await Promise.all([
        invoke<number>('get_log_file_size'),
        invoke<string[]>('read_last_log_lines', { count: 10 }),
      ]);
      setLogFileSize(size);
      setLastLines(lines);
    } catch (error) {
      console.error('Failed to refresh log info:', error);
    }
  };

  const clearEvents = () => {
    setSceneEvents([]);
  };

  return (
    <div className={logMonitorStyles.container}>
      {/* Header */}
      <div className={logMonitorStyles.header}>
        <h2 className={logMonitorStyles.title}>Scene Monitor</h2>
        <div className={logMonitorStyles.headerActions}>
          <Button onClick={refreshLogInfo} variant='secondary' size='sm'>
            Refresh
          </Button>
        </div>
      </div>

      {/* Status and Info */}
      <div className={logMonitorStyles.statusGrid}>
        <MonitoringStatus
          isMonitoring={isMonitoring}
          poeProcessStatus={poeProcessStatus}
          logFileSize={logFileSize}
        />
        <RecentLogLines lastLines={lastLines} onRefresh={refreshLogInfo} />
      </div>

      {/* Activity Log */}
      <ActivityLog
        sceneEvents={sceneEvents}
        isMonitoring={isMonitoring}
        onClearEvents={clearEvents}
      />
    </div>
  );
}
