import { Button } from '@/components/button';
import { LoadingSpinner } from '@/components/loading-spinner';
import { PageHeader } from '@/components/page-header';
import { ActTimeChart } from '@/components/time-tracking/act-time-chart';
import { ActiveSessions } from '@/components/time-tracking/active-sessions';
import { CharacterSelector } from '@/components/time-tracking/character-selector';
import { LocationStats } from '@/components/time-tracking/location-stats';
import { SessionHistory } from '@/components/time-tracking/session-history';
import { StatCard } from '@/components/time-tracking/stat-card';
import { useCharacterTimeTracking } from '@/hooks/useCharacterTimeTracking';
import { formatDuration } from '@/utils';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/time-tracking')({
  component: TimeTrackingPage,
});

function TimeTrackingPage() {
  const {
    activeCharacter,
    hasActiveCharacter,
    activeSessions,
    completedSessions,
    allStats,
    summary,
    isLoading,
    error,
    notification,
    refreshData,
    endSession,
    clearNotification,
  } = useCharacterTimeTracking();

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
          <CharacterSelector
            activeCharacter={activeCharacter}
            hasActiveCharacter={hasActiveCharacter}
          />

          <div className='flex items-center justify-between'>
            <div className='flex items-center gap-3'>
              <Button onClick={refreshData} variant='outline' size='sm'>
                Refresh
              </Button>
            </div>
          </div>
          {error && (
            <div className='bg-red-900/20 border border-red-800 text-red-300 px-4 py-3 rounded-lg'>
              <strong>Error:</strong> {error}
            </div>
          )}
          {notification && (
            <div
              className={`px-4 py-3 rounded-lg border ${
                notification.type === 'info'
                  ? 'bg-blue-950/20 border-blue-800 text-blue-300'
                  : notification.type === 'success'
                    ? 'bg-green-950/20 border-green-800 text-green-300'
                    : 'bg-yellow-950/20 border-yellow-800 text-yellow-300'
              }`}
            >
              <div className='flex items-center justify-between'>
                <span>{notification.message}</span>
                <button
                  onClick={clearNotification}
                  className='ml-3 text-current hover:opacity-70 transition-opacity'
                >
                  ×
                </button>
              </div>
            </div>
          )}
          {hasActiveCharacter && summary && (
            <div className='grid grid-cols-1 md:grid-cols-4 gap-4'>
              <StatCard
                value={formatDuration(summary.total_play_time_seconds)}
                label='Total Play Time'
              />
              <StatCard
                value={formatDuration(
                  summary.total_play_time_since_process_start_seconds
                )}
                label='Play Time This Session'
              />
              <StatCard
                value={formatDuration(summary.total_hideout_time_seconds)}
                label='Time in Hideout'
              />
              <StatCard
                value={
                  summary.top_locations.length > 0
                    ? summary.top_locations[0].location_name
                    : 'N/A'
                }
                label='Most Time Spent'
              />
            </div>
          )}
          <ActTimeChart stats={allStats} />
          {hasActiveCharacter && (
            <div className='grid grid-cols-1 lg:grid-cols-2 gap-6'>
              <div className='space-y-6'>
                <LocationStats stats={allStats} />
              </div>
              <div className='space-y-6'>
                <ActiveSessions
                  sessions={activeSessions}
                  onEndSession={endSession}
                />
                <SessionHistory sessions={completedSessions} />
              </div>
            </div>
          )}
          {!isLoading &&
            hasActiveCharacter &&
            activeSessions.length === 0 &&
            completedSessions.length === 0 && (
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
