import { usePoe2Process } from '@/hooks/usePoe2Process';
import {
  ComputerDesktopIcon,
  ServerIcon,
  UserIcon,
} from '@heroicons/react/16/solid';
import { StatusIndicator } from './status-indicator';

export const StatusBar = () => {
  const { processInfo } = usePoe2Process();
  const isOnline = processInfo?.running || false;

  return (
    <div className='w-full py-1 px-4 border-b bg-zinc-950 border-zinc-950 flex justify-end gap-2'>
      <div
        title={isOnline ? 'POE2 is running' : 'POE2 is stopped'}
        className='cursor-pointer'
      >
        <StatusIndicator
          status={isOnline}
          icon={<ComputerDesktopIcon />}
          size='sm'
        />
      </div>
      <div title='POE2 servers are down' className='cursor-pointer'>
        <StatusIndicator status={false} icon={<ServerIcon />} size='sm' />
      </div>
      <div title='Logged out of POE2' className='cursor-pointer'>
        <StatusIndicator status={false} icon={<UserIcon />} size='sm' />
      </div>
    </div>
  );
};
