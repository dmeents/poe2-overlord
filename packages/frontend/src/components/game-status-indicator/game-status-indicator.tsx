import { useGameProcess } from '../../hooks/useGameProcess';
import type { ServerStatus } from '../../hooks/useServerStatus';
import { useServerStatus } from '../../hooks/useServerStatus';
import { gameStatusIndicatorStyles } from './game-status-indicator.styles';

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
    <div className={`${gameStatusIndicatorStyles.container} ${className}`}>
      <h3 className={gameStatusIndicatorStyles.title}>System Status</h3>

      <div className={gameStatusIndicatorStyles.statusContainer}>
        {/* Game Process Status */}
        <div>
          <div className={gameStatusIndicatorStyles.statusItem}>
            <div
              className={`${gameStatusIndicatorStyles.statusDot} ${
                gameRunning
                  ? gameStatusIndicatorStyles.statusDotOnline
                  : gameStatusIndicatorStyles.statusDotOffline
              }`}
            ></div>
            <span className={gameStatusIndicatorStyles.statusText}>
              {gameRunning ? 'POE2 Running' : 'POE2 Not Running'}
            </span>
          </div>

          {processInfo && (
            <div className={gameStatusIndicatorStyles.statusDetails}>
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
            <p className={gameStatusIndicatorStyles.loadingText}>
              Checking process...
            </p>
          )}
        </div>

        {/* Server Status */}
        <div className={gameStatusIndicatorStyles.divider}>
          <div className={gameStatusIndicatorStyles.statusItem}>
            <div
              className={`${gameStatusIndicatorStyles.statusDot} ${
                typedServerStatus?.is_online
                  ? gameStatusIndicatorStyles.statusDotOnline
                  : gameStatusIndicatorStyles.statusDotOffline
              }`}
            ></div>
            <span className={gameStatusIndicatorStyles.statusText}>
              {typedServerStatus?.is_online
                ? 'Server Online'
                : 'Server Offline'}
            </span>
          </div>

          {typedServerStatus && (
            <div className={gameStatusIndicatorStyles.statusDetails}>
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
            <p className={gameStatusIndicatorStyles.loadingText}>
              Checking server...
            </p>
          )}
        </div>
      </div>
    </div>
  );
}
