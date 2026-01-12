import { Button } from '@/components/ui/button/button';
import { MinusIcon, WindowIcon, XMarkIcon } from '@heroicons/react/24/outline';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { windowTitleStyles } from './window-title.styles';

const appWindow = getCurrentWindow();

export const WindowTitle = () => {
  // Wrap Tauri API calls with error handling to prevent crashes in browser dev mode
  const handleMinimize = async () => {
    try {
      await appWindow.minimize();
    } catch (error) {
      console.error('Failed to minimize window:', error);
    }
  };

  const handleMaximize = async () => {
    try {
      await appWindow.toggleMaximize();
    } catch (error) {
      console.error('Failed to maximize window:', error);
    }
  };

  const handleClose = async () => {
    try {
      await appWindow.close();
    } catch (error) {
      console.error('Failed to close window:', error);
    }
  };

  return (
    <div data-tauri-drag-region className={windowTitleStyles.container}>
      <div data-tauri-drag-region className={windowTitleStyles.title}>
        POE Overlord
      </div>
      <div className={windowTitleStyles.controls}>
        <Button
          title="Minimize window"
          variant="ghost"
          size="sm"
          onClick={handleMinimize}
          className={windowTitleStyles.controlButton}
        >
          <MinusIcon className="w-4 h-4" aria-hidden="true" />
        </Button>
        <Button
          title="Maximize window"
          variant="ghost"
          size="sm"
          onClick={handleMaximize}
          className={windowTitleStyles.controlButton}
        >
          <WindowIcon className="w-4 h-4" aria-hidden="true" />
        </Button>
        <Button
          title="Close window"
          variant="ghost"
          size="sm"
          onClick={handleClose}
          className={windowTitleStyles.controlButton}
        >
          <XMarkIcon className="w-4 h-4" aria-hidden="true" />
        </Button>
      </div>
    </div>
  );
};
