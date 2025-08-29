import { useTimeTracking } from '@/hooks/useTimeTracking';
import { Button } from '../button';
import { LoadingSpinner } from '../loading-spinner';
import { ActiveSessions } from './active-sessions';
import { LocationStats } from './location-stats';
import { SessionHistory } from './session-history';

export function TimeTrackingDashboard() {
  const {
    activeSessions,
    completedSessions,
    allStats,
    summary,
    isLoading,
    error,
    notification,
    refreshData,
    clearAllData,
    endSession,
    endAllActiveSessions,
    clearNotification,
    checkForStaleSessions,
  } = useTimeTracking();

  const handleClearData = async () => {
    if (
      confirm(
        'Are you sure you want to clear all time tracking data? This action cannot be undone.'
      )
    ) {
      await clearAllData();
    }
  };

  const handleEndAllSessions = async () => {
    if (
      confirm(
        'Are you sure you want to end all active time tracking sessions? This will mark all current sessions as completed.'
      )
    ) {
      await endAllActiveSessions();
    }
  };

  if (isLoading && !summary) {
    return (
      <div className='flex items-center justify-center py-12'>
        <LoadingSpinner />
      </div>
    );
  }

  return (
    <div className='space-y-6'>
      {/* Header */}
      <div className='flex items-center justify-between'>
        <div>
          <h2 className='text-2xl font-bold text-white'>
            Time Tracking Dashboard
          </h2>
          <p className='text-zinc-400 mt-1'>
            Monitor your time spent in different game locations
          </p>
        </div>
        <div className='flex items-center gap-3'>
          <Button onClick={refreshData} variant='outline' size='sm'>
            Refresh
          </Button>
          <Button
            onClick={checkForStaleSessions}
            variant='outline'
            size='sm'
            className='text-blue-400 hover:text-blue-300'
          >
            Check Stale Sessions
          </Button>
          {activeSessions.length > 0 && (
            <Button
              onClick={handleEndAllSessions}
              variant='outline'
              size='sm'
              className='text-yellow-400 hover:text-yellow-300'
            >
              End All Sessions
            </Button>
          )}
          <Button
            onClick={handleClearData}
            variant='outline'
            size='sm'
            className='text-red-400 hover:text-red-300'
          >
            Clear All Data
          </Button>
        </div>
      </div>

      {/* Error Display */}
      {error && (
        <div className='bg-red-900/20 border border-red-800 text-red-300 px-4 py-3 rounded-lg'>
          <strong>Error:</strong> {error}
        </div>
      )}

      {/* Notification Display */}
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

      {/* Summary Cards */}
      {summary && (
        <div className='grid grid-cols-1 md:grid-cols-4 gap-4'>
          <div className='bg-zinc-900/50 p-4 rounded-lg border border-zinc-800'>
            <div className='text-2xl font-bold text-white'>
              {summary.total_active_sessions}
            </div>
            <div className='text-sm text-zinc-400'>Active Sessions</div>
          </div>
          <div className='bg-zinc-900/50 p-4 rounded-lg border border-zinc-800'>
            <div className='text-2xl font-bold text-white'>
              {summary.total_locations_tracked}
            </div>
            <div className='text-sm text-zinc-400'>Locations Tracked</div>
          </div>
          <div className='bg-zinc-900/50 p-4 rounded-lg border border-zinc-800'>
            <div className='text-2xl font-bold text-white'>
              {completedSessions.length}
            </div>
            <div className='text-sm text-zinc-400'>Completed Sessions</div>
          </div>
          <div className='bg-zinc-900/50 p-4 rounded-lg border border-zinc-800'>
            <div className='text-2xl font-bold text-white'>
              {summary.top_locations.length > 0
                ? summary.top_locations[0].location_name
                : 'N/A'}
            </div>
            <div className='text-sm text-zinc-400'>Most Visited Location</div>
          </div>
        </div>
      )}

      {/* Main Content Grid */}
      <div className='grid grid-cols-1 lg:grid-cols-2 gap-6'>
        {/* Left Column */}
        <div className='space-y-6'>
          <ActiveSessions sessions={activeSessions} onEndSession={endSession} />
          <LocationStats stats={allStats} />
        </div>

        {/* Right Column */}
        <div className='space-y-6'>
          <SessionHistory sessions={completedSessions} />
        </div>
      </div>

      {/* Empty State */}
      {!isLoading &&
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
              Start playing Path of Exile 2 to begin tracking your time in
              different locations.
            </p>
          </div>
        )}
    </div>
  );
}
