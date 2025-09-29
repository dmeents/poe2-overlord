import { Button } from '@/components';
import { MinusIcon, WindowIcon, XMarkIcon } from '@heroicons/react/24/outline';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { windowTitleStyles } from './window-title.styles';

const appWindow = getCurrentWindow();

export const WindowTitle = () => {
  return (
    <div data-tauri-drag-region className={windowTitleStyles.container}>
      <div data-tauri-drag-region className={windowTitleStyles.title}>
        POE Overlord
      </div>
      <div className={windowTitleStyles.controls}>
        <Button
          title='minimize'
          variant='ghost'
          size='sm'
          onClick={() => appWindow.minimize()}
          className={windowTitleStyles.controlButton}
        >
          <MinusIcon className='w-4 h-4' />
        </Button>
        <Button
          title='maximize'
          variant='ghost'
          size='sm'
          onClick={() => appWindow.toggleMaximize()}
          className={windowTitleStyles.controlButton}
        >
          <WindowIcon className='w-4 h-4' />
        </Button>
        <Button
          title='close'
          variant='ghost'
          size='sm'
          onClick={() => appWindow.close()}
          className={windowTitleStyles.controlButton}
        >
          <XMarkIcon className='w-4 h-4' />
        </Button>
      </div>
    </div>
  );
};
