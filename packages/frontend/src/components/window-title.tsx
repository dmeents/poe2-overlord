import { Button } from '@/components';
import { usePoe2Process } from '@/hooks/usePoe2Process';
import { MinusIcon, WindowIcon, XMarkIcon } from '@heroicons/react/24/outline';
import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();

export const WindowTitle = () => {
  const { processInfo } = usePoe2Process();
  const isOnline = processInfo?.running || false;

  const statusClasses = isOnline
    ? 'bg-green-500 shadow-green-500/50'
    : 'bg-red-800 shadow-red-500/50';

  return (
    <div
      data-tauri-drag-region
      className='px-[16px] h-[30px] bg-zinc-950 select-none grid grid-cols-[auto_max-content] fixed top-0 left-0 right-0'
    >
      <div
        title={isOnline ? 'POE2 is running' : 'POE2 is not running'}
        data-tauri-drag-region
        className='text-sm text-zinc-400 flex items-center gap-2'
      >
        <div className='flex items-center justify-center'>
          <div
            className={`
          w-2 h-2 rounded-full shadow-lg
          transition-all duration-200 ease-in-out
          ${statusClasses}
          ${isOnline ? 'animate-pulse' : ''}
        `}
          />
        </div>
        <span>POE Overlord</span>
      </div>
      <div className='flex items-center gap-2'>
        <Button
          title='minimize'
          variant='ghost'
          size='sm'
          onClick={() => appWindow.minimize()}
          className='w-5 h-5 p-0 text-zinc-500'
        >
          <MinusIcon className='w-4 h-4' />
        </Button>
        <Button
          title='maximize'
          variant='ghost'
          size='sm'
          onClick={() => appWindow.toggleMaximize()}
          className='w-5 h-5 p-0 text-zinc-500'
        >
          <WindowIcon className='w-4 h-4' />
        </Button>
        <Button
          title='close'
          variant='ghost'
          size='sm'
          onClick={() => appWindow.close()}
          className='w-5 h-5 p-0 text-zinc-500'
        >
          <XMarkIcon className='w-4 h-4' />
        </Button>
      </div>
    </div>
  );
};
