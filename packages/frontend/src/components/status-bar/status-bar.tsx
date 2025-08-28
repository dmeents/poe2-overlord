import { usePoe2Process, useZoneMonitoring } from '@/hooks';
import {
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
  const { currentZone, isMonitoring } = useZoneMonitoring();
  const navigate = useNavigate();
  const isOnline = processInfo?.running || false;

  const handleSettingsClick = () => {
    navigate({ to: '/settings' });
  };

  const getZoneDisplayText = () => {
    if (!isMonitoring || !isOnline) {
      return 'No active character';
    }

    if (currentZone) {
      return `Current Zone: ${currentZone}`;
    }

    return 'Character active - monitoring zones...';
  };

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
        <div title='POE2 servers are down'>
          <StatusIndicator status={false} icon={<ServerIcon />} size='sm' />
        </div>
        <div title='Logged out of POE2'>
          <StatusIndicator status={false} icon={<UserIcon />} size='sm' />
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
