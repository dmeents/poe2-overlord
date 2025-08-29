import { PageHeader } from '@/components/page-header';
import { TimeTrackingDashboard } from '@/components/time-tracking';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/time-tracking')({
  component: TimeTrackingPage,
});

function TimeTrackingPage() {
  return (
    <div className='min-h-screen bg-zinc-900 text-white'>
      <PageHeader
        title='Time Tracking'
        subtitle='Monitor your time spent in different game locations'
      />
      <div className='container mx-auto px-6 py-8'>
        <TimeTrackingDashboard />
      </div>
    </div>
  );
}
