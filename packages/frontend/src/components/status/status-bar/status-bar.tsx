import { ChartBarIcon, CogIcon, ComputerDesktopIcon, UserIcon } from '@heroicons/react/16/solid';
import { useNavigate } from '@tanstack/react-router';
import { useCharacter } from '@/contexts/CharacterContext';
import { useGameProcess } from '@/contexts/GameProcessContext';
import { useServerStatus } from '@/contexts/ServerStatusContext';
import { getDisplayAct } from '@/utils/zone-utils';
import { Button } from '../../ui/button/button';
import { StatusIndicator } from '../status-indicator/status-indicator';
import { statusBarStyles } from './status-bar.styles';

export const StatusBar = () => {
  const { processInfo } = useGameProcess();
  const { serverStatus } = useServerStatus();
  const { activeCharacter } = useCharacter();
  const navigate = useNavigate();
  const isOnline = processInfo?.running || false;
  const currentLocation = activeCharacter?.current_location;

  const getZoneDisplayText = () => {
    const characterName = activeCharacter?.name || 'No active character';

    if (currentLocation) {
      const parts = [characterName];
      const displayAct = getDisplayAct(currentLocation);
      if (displayAct) parts.push(displayAct);
      if (currentLocation.zone_name) parts.push(currentLocation.zone_name);
      return parts.join(' - ');
    }

    return characterName;
  };

  const getServerStatusTooltip = () => {
    if (!serverStatus) {
      return 'Attempting to connect to POE2 server...';
    }

    if (serverStatus.is_online) {
      const pingText = serverStatus.latency_ms ? ` (${serverStatus.latency_ms}ms)` : '';

      return `POE2 server is online${pingText}\nServer: ${serverStatus.ip_address}`;
    } else {
      return `POE2 server is offline\nLast known server: ${serverStatus.ip_address}`;
    }
  };

  return (
    <div className={statusBarStyles.container}>
      <div className={statusBarStyles.leftSection}>
        <div title={isOnline ? 'POE2 is running' : 'POE2 is stopped'}>
          <StatusIndicator
            status={isOnline ? 'success' : 'error'}
            icon={<ComputerDesktopIcon />}
            size="sm"
          />
        </div>
        <div title={getServerStatusTooltip()}>
          <StatusIndicator
            status={!serverStatus ? 'info' : serverStatus.is_online ? 'success' : 'error'}
            icon={<ChartBarIcon />}
            size="sm"
          />
        </div>
        <div title="Logged out of POE2">
          <StatusIndicator status="error" icon={<UserIcon />} size="sm" />
        </div>
        {getZoneDisplayText()}
      </div>
      <div className={statusBarStyles.rightSection}>
        <div title="Settings">
          <Button variant="icon" size="xs" onClick={() => navigate({ to: '/settings' })}>
            <CogIcon />
          </Button>
        </div>
      </div>
    </div>
  );
};
