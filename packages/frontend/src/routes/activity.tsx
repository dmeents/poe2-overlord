import type { ActChangeEvent, ProcessInfo, ZoneChangeEvent } from '@/types';
import { createFileRoute } from '@tanstack/react-router';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';
import { Button } from '../components/button';
import { ActivityLog } from '../components/log-monitor/activity-log';
import { MonitoringStatus } from '../components/log-monitor/monitoring-status';
import { RecentLogLines } from '../components/log-monitor/recent-log-lines';
import { PageHeader } from '../components/page-header';

// Combined event type for unified display
type SceneEvent = {
  type: 'zone' | 'act';
  data: ZoneChangeEvent | ActChangeEvent;
  timestamp: string;
};

export const Route = createFileRoute('/activity')({
  component: ActivityMonitor,
});

function ActivityMonitor() {
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

    // Set up event listeners
    const unlistenZone = listen('log-zone-change', event => {
      const zoneEvent = event.payload as ZoneChangeEvent;
      setSceneEvents(prev => [
        {
          type: 'zone',
          data: zoneEvent,
          timestamp: zoneEvent.timestamp,
        },
        ...prev,
      ]);
    });

    const unlistenAct = listen('log-act-change', event => {
      const actEvent = event.payload as ActChangeEvent;
      setSceneEvents(prev => [
        {
          type: 'act',
          data: actEvent,
          timestamp: actEvent.timestamp,
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
      unlistenZone.then(f => f());
      unlistenAct.then(f => f());
      unlistenProcess.then(f => f());
    };
  }, []);

  const checkMonitoringStatus = async () => {
    try {
      const status = await invoke<boolean>('is_log_monitoring_active');
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
    <div className='min-h-screen'>
      <div className='w-full py-3 px-4 sm:px-6 lg:px-8'>
        <PageHeader
          title='Activity Monitor'
          subtitle='Monitor your Path of Exile 2 gameplay in real-time, track zone changes, act transitions, and character movements.'
        />

        {/* Content Container */}
        <div className='shadow-lg border border-zinc-700 bg-zinc-900 p-6 w-full'>
          <div className='space-y-6'>
            {/* Header */}
            <div className='flex items-center justify-between'>
              <h2 className='text-xl font-semibold text-zinc-200'>
                Scene Monitor
              </h2>
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
        </div>
      </div>
    </div>
  );
}
