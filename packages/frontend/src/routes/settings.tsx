import { PageLayout } from '@/components/layout/page-layout/page-layout';
import { SettingsForm } from '@/components/forms/settings-form/settings-form';
import { Card } from '@/components/ui/card/card';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/settings')({
  component: RouteComponent,
});

function RouteComponent() {
  const leftColumn = (
    <Card
      title="System Settings"
      subtitle="Core application configuration and process monitoring preferences."
    >
      <SettingsForm />
    </Card>
  );

  const rightColumn = (
    <Card title="Configuration Help">
      <div className="space-y-4 p-4 text-sm text-zinc-300">
        <div>
          <h4 className="font-medium text-white mb-2">POE Client Log Path</h4>
          <p>
            This should point to the client.txt file in your Path of Exile
            installation directory. The application monitors this file to track
            your gameplay progress and character activities.
          </p>
        </div>
        <div>
          <h4 className="font-medium text-white mb-2">Log Level</h4>
          <p>
            Controls the verbosity of application logging. Use "Info" for normal
            operation, "Debug" for troubleshooting, or "Error" for minimal
            logging.
          </p>
        </div>
      </div>
    </Card>
  );

  return <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} />;
}
