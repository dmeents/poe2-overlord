import type { SceneChangeEvent } from '@/types';
import { getSceneEventTimestamp } from '@/types';
import { createFileRoute } from '@tanstack/react-router';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

import { ActivityLog, MonitoringStatus, RecentLogLines } from '../components';
import { PageHeader } from '../components/page-header';
import { useGameProcessQuery } from '../hooks/useGameProcessQuery';
import { useMonitoringQuery } from '../hooks/useMonitoringQuery';

// Combined event type for unified display
type SceneEvent = {
  type: 'zone' | 'act' | 'hideout';
  data: SceneChangeEvent;
  timestamp: string;
};

export const Route = createFileRoute('/activity')({
  component: ActivityMonitor,
});

function ActivityMonitor() {
  // Use persistent hooks for state that should survive route changes
  const { processInfo: poeProcessStatus } = useGameProcessQuery();
  const { isMonitoring } = useMonitoringQuery();

  // Local state for UI-specific data that doesn't need persistence
  const [sceneEvents, setSceneEvents] = useState<SceneEvent[]>([]);
  const [logFileSize, setLogFileSize] = useState<number>(0);
  const [lastLines, setLastLines] = useState<string[]>([]);

  useEffect(() => {
    // Set up event listeners for scene changes (UI-specific data)
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

    // Cleanup listeners
    return () => {
      unlistenScene.then(f => f());
    };
  }, []);

  // These functions are no longer needed since we're using persistent hooks

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
    <div className='min-h-screen bg-zinc-900 text-white'>
      <PageHeader
        title='Activity Monitor'
        subtitle='Monitor your Path of Exile 2 gameplay in real-time, track zone changes, act transitions, and character movements.'
      />
      <div className='max-w-7xl mx-auto px-6'>
        <div className='space-y-6'>
          <div className='grid grid-cols-1 md:grid-cols-2 gap-6'>
            <MonitoringStatus
              isMonitoring={isMonitoring}
              poeProcessStatus={poeProcessStatus}
              logFileSize={logFileSize}
            />
            <RecentLogLines lastLines={lastLines} onRefresh={refreshLogInfo} />
          </div>
          <ActivityLog
            sceneEvents={sceneEvents}
            isMonitoring={isMonitoring}
            onClearEvents={clearEvents}
          />
        </div>
      </div>
    </div>
  );
}
