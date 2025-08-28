import type { ProcessInfo } from '@/types';

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
    <div className='bg-zinc-900/50 p-4 rounded-lg border border-zinc-800'>
      <h3 className='text-lg font-semibold text-white mb-3'>
        Monitoring Status
      </h3>
      <div className='space-y-2'>
        <div className='flex items-center justify-between'>
          <span className='text-zinc-300'>Status:</span>
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
          <span className='text-zinc-300'>POE Process:</span>
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
            <span className='text-zinc-300'>Process ID:</span>
            <span className='text-white font-mono'>{poeProcessStatus.pid}</span>
          </div>
        )}
        <div className='flex items-center justify-between'>
          <span className='text-zinc-300'>Log File Size:</span>
          <span className='text-white font-mono'>
            {(logFileSize / 1024).toFixed(1)} KB
          </span>
        </div>
      </div>
    </div>
  );
}
