import {
  Button,
  Footer,
  InfoPanel,
  ProcessStatus,
  QuickActions,
} from '@/components';
import { usePoe2Process } from '@/hooks';
import { APP_CONFIG } from '@/utils';
import { createFileRoute } from '@tanstack/react-router';
import { AlertCircle, Settings } from 'lucide-react';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  const { processInfo, poe2Running, checkPoe2Process } = usePoe2Process();

  return (
    <div className='w-full h-full flex flex-col font-mono bg-900'>
      <div className='bg-[var(--color-bg-900)] flex-1 flex flex-col text-[var(--color-text-100)]'>
        <div className='flex items-center justify-between p-4 border-b border-[var(--color-border-600)] bg-[var(--color-bg-800)]'>
          <div className='flex items-center gap-2'>
            <h1 className='text-[var(--color-text-100)] text-lg font-bold m-0'>
              {APP_CONFIG.TITLE}
            </h1>
            {poe2Running && processInfo && (
              <span className='text-[var(--color-text-400)] text-sm'>
                PID: {processInfo.pid}
              </span>
            )}
          </div>
        </div>
        <div className='flex-1 p-6 flex flex-col gap-6'>
          <ProcessStatus
            poe2Running={poe2Running}
            processInfo={processInfo}
            onRefresh={checkPoe2Process}
          />
          <QuickActions />
          <InfoPanel
            title='Application Ready'
            description='POE2 Overlord is now running as a normal desktop application. Use the controls above to interact with game data and settings.'
            icon={<AlertCircle size={16} />}
          />
          <Button
            variant='primary'
            size='md'
            className='w-full flex items-center justify-center gap-2'
          >
            <Settings size={16} />
            <span>Settings</span>
          </Button>
        </div>
        <Footer
          version={APP_CONFIG.VERSION}
          technology={APP_CONFIG.TECHNOLOGY}
        />
      </div>
    </div>
  );
}
