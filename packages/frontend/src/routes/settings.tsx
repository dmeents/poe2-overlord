import { DangerSection, PageHeader } from '@/components';
import { SettingsForm } from '@/components/settings-form';
import type { AppConfig } from '@/types';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/settings')({
  component: RouteComponent,
});

function RouteComponent() {
  const handleConfigUpdate = (config: AppConfig) => {
    console.log('Configuration updated:', config);
    // You can add additional logic here, such as:
    // - Updating global state
    // - Triggering other components to refresh
    // - Sending notifications
  };

  return (
    <div className='min-h-screen bg-zinc-900 text-white'>
      <PageHeader
        title='Settings'
        subtitle='Configure your POE2 Overlord application preferences and monitoring settings.'
      />
      <div className='max-w-7xl mx-auto px-6'>
        <div className='space-y-6'>
          {/* System Settings */}
          <div className='bg-zinc-900/50 p-6 rounded-lg border border-zinc-800'>
            <div className='mb-6 pb-4 border-b border-zinc-700'>
              <h2 className='text-xl font-semibold text-white mb-2'>
                System Settings
              </h2>
              <p className='text-zinc-300 text-sm'>
                Core application configuration and process monitoring preferences.
              </p>
            </div>
            <SettingsForm onConfigUpdate={handleConfigUpdate} />
          </div>

          {/* Danger Section */}
          <div className='bg-zinc-900/50 p-6 rounded-lg border border-zinc-800'>
            <div className='mb-6 pb-4 border-b border-zinc-700'>
              <h2 className='text-xl font-semibold text-red-400 mb-2'>
                Danger Zone
              </h2>
              <p className='text-zinc-300 text-sm'>
                Irreversible actions that will permanently delete data.
              </p>
            </div>
            <DangerSection />
          </div>
        </div>
      </div>
    </div>
  );
}
