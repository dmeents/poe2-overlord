import type { ProcessInfo, ZoneChangeEvent } from '@/types';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';
import { Button } from './button';

export function LogMonitor() {
  const [isMonitoring, setIsMonitoring] = useState(false);
  const [zoneEvents, setZoneEvents] = useState<ZoneChangeEvent[]>([]);
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
      setZoneEvents(prev => [...prev, event.payload as ZoneChangeEvent]);
    });

    const unlistenProcess = listen('poe2-process-status', event => {
      const processInfo = event.payload as ProcessInfo;
      setPoeProcessStatus(processInfo);
      setIsMonitoring(processInfo.running);
    });

    // Cleanup listeners
    return () => {
      unlistenZone.then(f => f());
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
    setZoneEvents([]);
  };

  const formatTimestamp = (timestamp: string) => {
    try {
      const date = new Date(timestamp);
      return date.toLocaleTimeString();
    } catch {
      return timestamp;
    }
  };

  return (
    <div className='space-y-6'>
      {/* Header */}
      <div className='flex items-center justify-between'>
        <h2 className='text-2xl font-bold text-white'>Zone Monitor</h2>
        <div className='flex items-center space-x-3'>
          <Button onClick={refreshLogInfo} variant='secondary' size='sm'>
            Refresh
          </Button>
        </div>
      </div>

      {/* Status and Info */}
      <div className='grid grid-cols-1 md:grid-cols-2 gap-4'>
        <div className='bg-gray-800/50 p-4 rounded-lg border border-gray-700'>
          <h3 className='text-lg font-semibold text-white mb-3'>
            Monitoring Status
          </h3>
          <div className='space-y-2'>
            <div className='flex items-center justify-between'>
              <span className='text-gray-300'>Status:</span>
              <span
                className={`px-2 py-1 rounded text-sm font-medium ${
                  isMonitoring
                    ? 'bg-green-900/50 text-green-300'
                    : 'bg-red-900/50 text-red-300'
                }`}
              >
                {isMonitoring ? 'Active' : 'Inactive'}
              </span>
            </div>
            <div className='flex items-center justify-between'>
              <span className='text-gray-300'>POE Process:</span>
              <span
                className={`px-2 py-1 rounded text-sm font-medium ${
                  poeProcessStatus?.running
                    ? 'bg-green-900/50 text-green-300'
                    : 'bg-red-900/50 text-red-300'
                }`}
              >
                {poeProcessStatus?.running ? 'Running' : 'Not Running'}
              </span>
            </div>
            {poeProcessStatus?.running && (
              <div className='flex items-center justify-between'>
                <span className='text-gray-300'>Process ID:</span>
                <span className='text-white font-mono'>
                  {poeProcessStatus.pid}
                </span>
              </div>
            )}
            <div className='flex items-center justify-between'>
              <span className='text-gray-300'>Log File Size:</span>
              <span className='text-white font-mono'>
                {(logFileSize / 1024).toFixed(1)} KB
              </span>
            </div>
          </div>
        </div>

        <div className='bg-gray-800/50 p-4 rounded-lg border border-gray-700'>
          <h3 className='text-lg font-semibold text-white mb-3'>
            Recent Log Lines
          </h3>
          <div className='space-y-1 max-h-32 overflow-y-auto'>
            {lastLines.length > 0 ? (
              lastLines.map((line, index) => (
                <div key={index} className='text-sm text-gray-300 font-mono'>
                  {line.length > 60 ? `${line.substring(0, 60)}...` : line}
                </div>
              ))
            ) : (
              <div className='text-gray-500 text-sm'>No recent lines</div>
            )}
          </div>
        </div>
      </div>

      {/* Zone Change Events */}
      <div className='bg-gray-800/50 p-4 rounded-lg border border-gray-700'>
        <div className='flex items-center justify-between mb-4'>
          <h3 className='text-lg font-semibold text-white'>Zone Changes</h3>
          <Button
            onClick={clearEvents}
            variant='outline'
            size='sm'
            disabled={zoneEvents.length === 0}
          >
            Clear Events
          </Button>
        </div>

        <div className='space-y-3 max-h-96 overflow-y-auto'>
          {zoneEvents.length > 0 ? (
            zoneEvents.map((event, index) => (
              <div
                key={index}
                className='bg-blue-900/20 border-l-4 border-blue-500 p-3 rounded-r'
              >
                <div className='flex items-center justify-between'>
                  <span className='text-blue-300 font-medium'>Zone Change</span>
                  <span className='text-gray-400 text-sm'>
                    {formatTimestamp(event.timestamp)}
                  </span>
                </div>
                <div className='text-white mt-1'>{event.zone_name}</div>
              </div>
            ))
          ) : (
            <div className='text-center text-gray-500 py-8'>
              {isMonitoring
                ? 'No zone changes detected yet. Changes will appear here as they occur.'
                : 'Zone monitoring is inactive. Start Path of Exile 2 to begin monitoring.'}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
