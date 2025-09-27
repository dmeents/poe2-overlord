import {
  ActTimeChart,
  LocationStats,
  SessionHistory,
  StatCard,
} from '@/components';
import { LoadingSpinner } from '@/components/loading-spinner';
import { PageHeader } from '@/components/page-header';
import { useCharacterManagement } from '@/hooks';
import { formatDuration } from '@/utils';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/time-tracking')({
  component: TimeTrackingPage,
});

function TimeTrackingPage() {
  const { activeCharacter, isLoading } = useCharacterManagement();
  const summary = activeCharacter?.summary;
  const zones = activeCharacter?.zones || [];

  if (isLoading && !summary) {
    return (
      <div className='min-h-screen bg-zinc-900 text-white'>
        <PageHeader
          title='Time Tracking'
          subtitle='Monitor your time spent in different game locations'
        />
        <div className='container mx-auto px-6 py-8'>
          <div className='flex items-center justify-center py-12'>
            <LoadingSpinner />
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className='min-h-screen bg-zinc-900 text-white'>
      <PageHeader
        title='Time Tracking'
        subtitle='Monitor your time spent in different game locations'
      />
      <div className='container mx-auto px-6 py-8'>
        <div className='space-y-6'>
          {activeCharacter && summary && (
            <div className='grid grid-cols-1 md:grid-cols-4 gap-4'>
              <StatCard
                value={formatDuration(summary.total_play_time)}
                label='Total Play Time'
              />
              <StatCard
                value={formatDuration(summary.total_play_time)}
                label='Play Time This Session'
              />
              <StatCard
                value={formatDuration(summary.total_hideout_time)}
                label='Time in Hideout'
              />
              <StatCard
                value={zones.length > 0 ? zones[0].location_name : 'N/A'}
                label='Most Time Spent'
              />
            </div>
          )}
          <ActTimeChart stats={zones} />
          {activeCharacter && (
            <div className='grid grid-cols-1 lg:grid-cols-2 gap-6'>
              <div className='space-y-6'>
                <LocationStats stats={zones} />
              </div>
              <div className='space-y-6'>
                <SessionHistory sessions={[]} />
              </div>
            </div>
          )}
          {!isLoading && activeCharacter && zones.length === 0 && (
            <div className='text-center py-12'>
              <div className='text-zinc-500 mb-4'>
                <svg
                  className='mx-auto h-12 w-12'
                  fill='none'
                  viewBox='0 0 24 24'
                  stroke='currentColor'
                >
                  <path
                    strokeLinecap='round'
                    strokeLinejoin='round'
                    strokeWidth={2}
                    d='M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z'
                  />
                </svg>
              </div>
              <h3 className='text-lg font-medium text-zinc-300 mb-2'>
                No Time Tracking Data
              </h3>
              <p className='text-zinc-500'>
                Start playing Path of Exile 2 with {activeCharacter?.name} to
                begin tracking your time in different locations.
              </p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
