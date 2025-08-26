import { Button } from '@/components';
import { MinusIcon, WindowIcon, XMarkIcon } from '@heroicons/react/24/outline';
import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();

export const WindowTitle = () => {
  return (
    <div data-tauri-drag-region className='titlebar'>
      <div data-tauri-drag-region></div>
      <div className='controls'>
        <Button
          title='minimize'
          variant='ghost'
          size='sm'
          onClick={() => appWindow.minimize()}
          className='w-8 h-8 p-0'
        >
          <MinusIcon className='w-4 h-4' />
        </Button>
        <Button
          title='maximize'
          variant='ghost'
          size='sm'
          onClick={() => appWindow.toggleMaximize()}
          className='w-8 h-8 p-0'
        >
          <WindowIcon className='w-4 h-4' />
        </Button>
        <Button
          title='close'
          variant='ghost'
          size='sm'
          onClick={() => appWindow.close()}
          className='w-8 h-8 p-0'
        >
          <XMarkIcon className='w-4 h-4' />
        </Button>
      </div>
    </div>
  );
};
