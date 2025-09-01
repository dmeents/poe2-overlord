import { usePoe2Process, useServerStatus, useZoneMonitoring } from '@/hooks';
import {
  ChartBarIcon,
  CogIcon,
  ComputerDesktopIcon,
  ServerIcon,
  UserIcon,
} from '@heroicons/react/16/solid';
import { useNavigate } from '@tanstack/react-router';
import { Button } from '../button';
import { StatusIndicator } from './status-indicator';

export const StatusBar = () => {
  const { processInfo } = usePoe2Process();
  const { currentZone, currentAct, isMonitoring } = useZoneMonitoring();
  const { serverStatus } = useServerStatus();
  const navigate = useNavigate();
  const isOnline = processInfo?.running || false;

  const handleSettingsClick = () => {
    navigate({ to: '/settings' });
  };

  const handleActivityClick = () => {
    navigate({ to: '/activity' });
  };

  const getZoneDisplayText = () => {
    if (!isMonitoring || !isOnline) {
      return 'No active character';
    }

    if (currentZone) {
      if (currentAct) {
        return `${currentAct} - ${currentZone}`;
      }
      return `Current Zone: ${currentZone}`;
    }

    if (currentAct) {
      return `Current Act: ${currentAct}`;
    }

    return 'Character active - monitoring zones...';
  };

  const getServerStatusTooltip = () => {
    if (!serverStatus) {
      return 'No server information available';
    }

    if (serverStatus.is_online) {
      const pingText = serverStatus.last_ping_ms
        ? ` (${serverStatus.last_ping_ms}ms)`
        : '';
      return `POE2 server is online${pingText}`;
    } else {
      return 'POE2 server is offline';
    }
  };

  console.log('serverStatus', serverStatus);

  return (
    <div className='fixed bottom-0 w-full py-1 px-4 border-b bg-zinc-950 border-zinc-950 flex justify-between gap-2'>
      <div className='text-xs text-zinc-400'>{getZoneDisplayText()}</div>
      <div className='flex items-center gap-2'>
        <div title={isOnline ? 'POE2 is running' : 'POE2 is stopped'}>
          <StatusIndicator
            status={isOnline}
            icon={<ComputerDesktopIcon />}
            size='sm'
          />
        </div>
        <div title={getServerStatusTooltip()}>
          <StatusIndicator
            status={serverStatus?.is_online || false}
            icon={<ServerIcon />}
            size='sm'
          />
        </div>
        <div title='Logged out of POE2'>
          <StatusIndicator status={false} icon={<UserIcon />} size='sm' />
        </div>
        <div title='Activity Monitor'>
          <Button variant='icon' size='xs' onClick={handleActivityClick}>
            <ChartBarIcon />
          </Button>
        </div>
        <div title='Settings'>
          <Button variant='icon' size='xs' onClick={handleSettingsClick}>
            <CogIcon />
          </Button>
        </div>
      </div>
    </div>
  );
};
