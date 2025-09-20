import { useGameProcess } from '../../hooks/useGameProcess';
import { useServerStatus } from '../../hooks/useServerStatus';
import type { ServerStatus } from '../../hooks/useServerStatus';

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
  const typedServerStatus = serverStatus as ServerStatus | null;

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
              {processInfo.startTime && (
                <p>
                  Started:{' '}
                  {new Date(processInfo.startTime).toLocaleTimeString()}
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
                typedServerStatus?.is_online ? 'bg-green-500' : 'bg-red-500'
              }`}
            ></div>
            <span className='text-white text-sm font-medium'>
              {typedServerStatus?.is_online ? 'Server Online' : 'Server Offline'}
            </span>
          </div>

          {typedServerStatus && (
            <div className='text-zinc-400 text-xs space-y-1'>
              <p>IP: {typedServerStatus.ip_address}</p>
              {typedServerStatus.latency_ms !== null && (
                <p>Latency: {typedServerStatus.latency_ms}ms</p>
              )}
              <p>
                Last update:{' '}
                {new Date(typedServerStatus.timestamp).toLocaleTimeString()}
              </p>
            </div>
          )}

          {serverLoading && (
            <p className='text-zinc-400 text-xs'>Checking server...</p>
          )}
        </div>
      </div>
    </div>
  );
}
