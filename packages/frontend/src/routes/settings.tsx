import { Button } from '@/components';
import { SettingsForm } from '@/components/settings-form';
import type { AppConfig } from '@/types';
import { ArrowLeftIcon } from '@heroicons/react/24/outline';
import { createFileRoute, useNavigate } from '@tanstack/react-router';

export const Route = createFileRoute('/settings')({
  component: RouteComponent,
});

function RouteComponent() {
  const navigate = useNavigate();

  const handleConfigUpdate = (config: AppConfig) => {
    console.log('Configuration updated:', config);
    // You can add additional logic here, such as:
    // - Updating global state
    // - Triggering other components to refresh
    // - Sending notifications
  };

  const handleBackClick = () => {
    navigate({ to: '/' });
  };

  return (
    <div className='min-h-screen'>
      <div className='w-full py-3 px-4 sm:px-6 lg:px-8'>
        {/* Header */}
        <div className='mb-8'>
          <div className='flex items-center justify-between mb-4'>
            <h1 className='text-3xl font-bold text-zinc-100 font-cusrive'>
              Settings
            </h1>
            <Button
              variant='outline'
              size='sm'
              onClick={handleBackClick}
              className='flex items-center gap-2'
            >
              <ArrowLeftIcon className='w-4 h-4' />
              Back
            </Button>
          </div>
          <p className='mt-2 text-zinc-400'>
            Configure your POE2 Overlord application preferences and monitoring
            settings.
          </p>
        </div>

        {/* Settings Form Container */}
        <div className='shadow-lg border border-zinc-700 bg-zinc-900 p-6 w-full'>
          {/* System Settings Section */}
          <div className='mb-6 pb-4 border-b border-zinc-700'>
            <h2 className='text-xl font-semibold text-zinc-200 mb-2'>
              System Settings
            </h2>
            <p className='text-zinc-400 text-sm'>
              Core application configuration and process monitoring preferences.
            </p>
          </div>
          <SettingsForm onConfigUpdate={handleConfigUpdate} />
        </div>
      </div>
    </div>
  );
}
