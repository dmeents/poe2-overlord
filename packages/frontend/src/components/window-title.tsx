import { Button } from '@/components';
import { MinusIcon, WindowIcon, XMarkIcon } from '@heroicons/react/24/outline';
import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();

export const WindowTitle = () => {
  return (
    <div
      data-tauri-drag-region
      className='px-[16px] h-[30px] bg-zinc-950 select-none grid grid-cols-[auto_max-content] fixed top-0 left-0 right-0'
    >
      <div data-tauri-drag-region></div>
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
