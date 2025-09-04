import { useGameProcess } from '../../hooks/useGameProcess';
import { useServerStatus } from '../../hooks/useServerStatus';
import { useZoneMonitoring } from '../../hooks/useZoneMonitoring';

interface GameStatusIndicatorProps {
  className?: string;
}

export function GameStatusIndicator({
  className = '',
}: GameStatusIndicatorProps) {
  const {
    processInfo,
    gameRunning,
    isLoading: processLoading,
  } = useGameProcess();
  const { serverStatus, isLoading: serverLoading } = useServerStatus();
  const { isMonitoring } = useZoneMonitoring();

  return (
    <div
      className={`bg-zinc-900/50 p-6 rounded-lg border border-zinc-800 ${className}`}
    >
      <h3 className='text-lg font-semibold text-white mb-4'>System Status</h3>

      <div className='space-y-4'>
        {/* Game Process Status */}
        <div>
          <div className='flex items-center gap-2 mb-2'>
            <div
              className={`w-2 h-2 rounded-full ${gameRunning ? 'bg-green-500' : 'bg-red-500'}`}
            ></div>
            <span className='text-white text-sm font-medium'>
              {gameRunning ? 'POE2 Running' : 'POE2 Not Running'}
            </span>
          </div>

          {processInfo && (
            <div className='text-zinc-400 text-xs space-y-1'>
              <p>PID: {processInfo.pid}</p>
              <p>Window: {processInfo.window_title || 'No window detected'}</p>
              {processInfo.last_seen && (
                <p>
                  Last seen:{' '}
                  {new Date(processInfo.last_seen).toLocaleTimeString()}
                </p>
              )}
            </div>
          )}

          {processLoading && (
            <p className='text-zinc-400 text-xs'>Checking process...</p>
          )}
        </div>

        {/* Server Status */}
        <div className='pt-2 border-t border-zinc-700'>
          <div className='flex items-center gap-2 mb-2'>
            <div
              className={`w-2 h-2 rounded-full ${
                serverStatus?.is_online ? 'bg-green-500' : 'bg-red-500'
              }`}
            ></div>
            <span className='text-white text-sm font-medium'>
              {serverStatus?.is_online ? 'Server Online' : 'Server Offline'}
            </span>
          </div>

          {serverStatus && (
            <div className='text-zinc-400 text-xs space-y-1'>
              <p>IP: {serverStatus.ip_address}</p>
              {serverStatus.latency_ms !== null && (
                <p>Latency: {serverStatus.latency_ms}ms</p>
              )}
              <p>
                Last update:{' '}
                {new Date(serverStatus.timestamp).toLocaleTimeString()}
              </p>
            </div>
          )}

          {serverLoading && (
            <p className='text-zinc-400 text-xs'>Checking server...</p>
          )}
        </div>

        {/* Zone Monitoring Status */}
        <div className='pt-2 border-t border-zinc-700'>
          <div className='flex items-center gap-2 mb-2'>
            <div
              className={`w-2 h-2 rounded-full ${isMonitoring ? 'bg-green-500' : 'bg-gray-500'}`}
            ></div>
            <span className='text-white text-sm font-medium'>
              {isMonitoring
                ? 'Zone Monitoring Active'
                : 'Zone Monitoring Inactive'}
            </span>
          </div>
          <p className='text-zinc-400 text-xs'>
            {isMonitoring
              ? 'Tracking zone and act changes in real-time'
              : 'Zone monitoring is not active'}
          </p>
        </div>
      </div>
    </div>
  );
}
