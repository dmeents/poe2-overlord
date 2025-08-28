import { PageHeader } from '@/components';
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
    <div className='min-h-screen'>
      <div className='w-full py-3 px-4 sm:px-6 lg:px-8'>
        <PageHeader
          title='Settings'
          subtitle='Configure your POE2 Overlord application preferences and monitoring settings.'
        />

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
