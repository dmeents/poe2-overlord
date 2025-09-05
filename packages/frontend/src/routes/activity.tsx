import type { SceneChangeEvent } from '@/types';
import { getSceneEventTimestamp } from '@/types';
import { createFileRoute } from '@tanstack/react-router';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

import { ActivityLog } from '../components/log-monitor/activity-log';
import { MonitoringStatus } from '../components/log-monitor/monitoring-status';
import { RecentLogLines } from '../components/log-monitor/recent-log-lines';
import { PageHeader } from '../components/page-header';
import { useCharacterQuery } from '../hooks/useCharacterQuery';
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
  const { activeCharacter } = useCharacterQuery();
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
    <div className='min-h-screen'>
      <PageHeader
        title='Activity Monitor'
        subtitle='Monitor your Path of Exile 2 gameplay in real-time, track zone changes, act transitions, and character movements.'
      />
      <div className='my-3 mx-4 sm:mx-6 lg:mx-8 '>
        <div className='space-y-6'>
          {/* Character Indicator */}
          {activeCharacter && (
            <div className='bg-blue-500/10 border border-blue-500/20 rounded-lg p-4'>
              <div className='flex items-center gap-3'>
                <div className='w-2 h-2 bg-blue-500 rounded-full'></div>
                <div>
                  <p className='text-sm text-blue-400 font-medium'>
                    Time Tracking Active
                  </p>
                  <p className='text-white font-semibold'>
                    {activeCharacter.name} - {activeCharacter.class} (
                    {activeCharacter.ascendency})
                  </p>
                </div>
              </div>
            </div>
          )}

          {!activeCharacter && (
            <div className='bg-yellow-500/10 border border-yellow-500/20 rounded-lg p-4'>
              <div className='flex items-center gap-3'>
                <div className='flex-shrink-0'>
                  <svg
                    className='h-5 w-5 text-yellow-400'
                    fill='none'
                    viewBox='0 0 24 24'
                    stroke='currentColor'
                  >
                    <path
                      strokeLinecap='round'
                      strokeLinejoin='round'
                      strokeWidth={2}
                      d='M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z'
                    />
                  </svg>
                </div>
                <div>
                  <p className='text-sm text-yellow-400 font-medium'>
                    No Active Character
                  </p>
                  <p className='text-yellow-300 text-sm'>
                    Scene changes are being monitored but time tracking is
                    disabled. Select a character to enable time tracking.
                  </p>
                </div>
              </div>
            </div>
          )}

          <div className='grid grid-cols-1 md:grid-cols-2 gap-4'>
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
