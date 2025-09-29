import { PageLayout, SettingsForm } from '@/components';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/settings')({
  component: RouteComponent,
});

function RouteComponent() {
  const leftColumn = (
    <div className='bg-zinc-900/50 p-6 rounded-lg border border-zinc-800'>
      <div className='mb-6 pb-4 border-b border-zinc-700'>
        <h2 className='text-xl font-semibold text-white mb-2'>
          System Settings
        </h2>
        <p className='text-zinc-300 text-sm'>
          Core application configuration and process monitoring preferences.
        </p>
      </div>
      <SettingsForm />
    </div>
  );

  const rightColumn = (
    <div className='space-y-6'>
      <div className='bg-zinc-900/50 p-6 rounded-lg border border-zinc-800'>
        <h3 className='text-lg font-semibold text-white mb-4'>
          Configuration Help
        </h3>
        <div className='space-y-4 text-sm text-zinc-300'>
          <div>
            <h4 className='font-medium text-white mb-2'>POE Client Log Path</h4>
            <p>
              This should point to the client.txt file in your Path of Exile
              installation directory. The application monitors this file to
              track your gameplay progress and character activities.
            </p>
          </div>
          <div>
            <h4 className='font-medium text-white mb-2'>Log Level</h4>
            <p>
              Controls the verbosity of application logging. Use "Info" for
              normal operation, "Debug" for troubleshooting, or "Error" for
              minimal logging.
            </p>
          </div>
        </div>
      </div>
    </div>
  );

  return <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} />;
}
