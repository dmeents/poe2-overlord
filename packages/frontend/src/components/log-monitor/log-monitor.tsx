import type { ProcessInfo, SceneChangeEvent } from '@/types';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';
import { Button } from '../button';
import { ActivityLog } from './activity-log';
import { MonitoringStatus } from './monitoring-status';
import { RecentLogLines } from './recent-log-lines';

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
          timestamp:
            sceneEvent.type === 'Zone'
              ? sceneEvent.Zone.timestamp
              : sceneEvent.type === 'Act'
                ? sceneEvent.Act.timestamp
                : sceneEvent.Hideout.timestamp,
        },
        ...prev,
      ]);
    });

    const unlistenProcess = listen('poe2-process-status', event => {
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
    <div className='space-y-6'>
      {/* Header */}
      <div className='flex items-center justify-between'>
        <h2 className='text-2xl font-bold text-white'>Scene Monitor</h2>
        <div className='flex items-center space-x-3'>
          <Button onClick={refreshLogInfo} variant='secondary' size='sm'>
            Refresh
          </Button>
        </div>
      </div>

      {/* Status and Info */}
      <div className='grid grid-cols-1 md:grid-cols-2 gap-4'>
        <MonitoringStatus
          isMonitoring={isMonitoring}
          poeProcessStatus={poeProcessStatus}
          logFileSize={logFileSize}
        />
        <RecentLogLines lastLines={lastLines} />
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
