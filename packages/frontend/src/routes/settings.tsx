import { createFileRoute } from '@tanstack/react-router';
import { SettingsForm } from '@/components/forms/settings-form/settings-form';
import { PageLayout } from '@/components/layout/page-layout/page-layout';
import { Card } from '@/components/ui/card/card';
import { WalkthroughSettingsPanel } from '@/components/walkthrough/walkthrough-settings-panel/walkthrough-settings-panel';

export const Route = createFileRoute('/settings')({
  component: RouteComponent,
});

function RouteComponent() {
  const leftColumn = (
    <>
      <Card
        title="System Settings"
        subtitle="Core application configuration and process monitoring preferences.">
        <SettingsForm />
      </Card>
      <WalkthroughSettingsPanel variant="card" />
    </>
  );

  const rightColumn = (
    <Card title="Configuration Help">
      <div className="space-y-4 p-4 text-sm text-stone-300">
        <div>
          <h4 className="font-medium text-stone-50 mb-2">POE Client Log Path</h4>
          <p>
            This should point to the client.txt file in your Path of Exile installation directory.
            The application monitors this file to track your gameplay progress and character
            activities.
          </p>
        </div>
        <div>
          <h4 className="font-medium text-stone-50 mb-2">Log Level</h4>
          <p>
            Controls the verbosity of application logging. Use "Info" for normal operation, "Debug"
            for troubleshooting, or "Error" for minimal logging.
          </p>
        </div>
        <div>
          <h4 className="font-medium text-stone-50 mb-2">Walkthrough Display</h4>
          <p>
            Toggle walkthrough content visibility. Changes apply instantly and persist across
            sessions. Use these to declutter the view by hiding optional or league-start-only
            objectives, flavor text, and objective detail blocks.
          </p>
        </div>
      </div>
    </Card>
  );

  return <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} />;
}
