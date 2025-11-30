import { useCharacterManagement } from '@/hooks/useCharacterManagement';
import { useGameProcessEvents } from '@/hooks/useGameProcessEvents';
import { useServerStatusEvents } from '@/hooks/useServerStatusEvents';
import type { ServerStatus } from '@/types/server';
import {
  ChartBarIcon,
  CogIcon,
  ComputerDesktopIcon,
  UserIcon,
} from '@heroicons/react/16/solid';
import { useNavigate } from '@tanstack/react-router';
import { Button } from '../../ui/button/button';
import { StatusIndicator } from '../status-indicator/status-indicator';
import { statusBarStyles } from './status-bar.styles';

export const StatusBar = () => {
  const { processInfo } = useGameProcessEvents();
  const { serverStatus } = useServerStatusEvents();
  const { activeCharacter } = useCharacterManagement();
  const navigate = useNavigate();
  const isOnline = processInfo?.running || false;

  const currentLocation = activeCharacter?.current_location;

  const handleSettingsClick = () => {
    navigate({ to: '/settings' });
  };

  const getZoneDisplayText = () => {
    const characterName = activeCharacter?.name || 'No active character';

    if (currentLocation) {
      const parts = [characterName];

      if (currentLocation.act) {
        parts.push(currentLocation.act);
      }

      if (currentLocation.scene) {
        parts.push(currentLocation.scene);
      }

      return parts.join(' - ');
    }

    return characterName;
  };

  const getServerStatusTooltip = () => {
    if (!serverStatus) {
      return 'Attempting to connect to POE2 server...';
    }

    const status = serverStatus as ServerStatus;
    if (status.is_online) {
      const pingText = status.latency_ms ? ` (${status.latency_ms}ms)` : '';

      return `POE2 server is online${pingText}\nServer: ${status.ip_address}`;
    } else {
      return `POE2 server is offline\nLast known server: ${status.ip_address}`;
    }
  };

  return (
    <div className={statusBarStyles.container}>
      <div className={statusBarStyles.leftSection}>
        <div title={isOnline ? 'POE2 is running' : 'POE2 is stopped'}>
          <StatusIndicator
            status={isOnline ? 'success' : 'error'}
            icon={<ComputerDesktopIcon />}
            size='sm'
          />
        </div>
        <div title={getServerStatusTooltip()}>
          <StatusIndicator
            status={
              !serverStatus
                ? 'info'
                : (serverStatus as ServerStatus).is_online
                  ? 'success'
                  : 'error'
            }
            icon={<ChartBarIcon />}
            size='sm'
          />
        </div>
        <div title='Logged out of POE2'>
          <StatusIndicator status='error' icon={<UserIcon />} size='sm' />
        </div>
        {getZoneDisplayText()}
      </div>
      <div className={statusBarStyles.rightSection}>
        <div title='Settings'>
          <Button variant='icon' size='xs' onClick={handleSettingsClick}>
            <CogIcon />
          </Button>
        </div>
      </div>
    </div>
  );
};
