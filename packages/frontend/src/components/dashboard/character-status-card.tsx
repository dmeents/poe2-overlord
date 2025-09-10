import { useCharacterTimeTracking } from '../../hooks/useCharacterTimeTracking';
import { formatDuration } from '../../utils';

interface CharacterStatusCardProps {
  className?: string;
}

export function CharacterStatusCard({
  className = '',
}: CharacterStatusCardProps) {
  const { activeCharacter, timeTrackingData, isLoading } =
    useCharacterTimeTracking();

  if (isLoading) {
    return (
      <div
        className={`bg-zinc-900/50 p-6 rounded-lg border border-zinc-800 ${className}`}
      >
        <div className='animate-pulse'>
          <div className='h-6 bg-zinc-700 rounded mb-3 w-3/4'></div>
          <div className='h-4 bg-zinc-700 rounded mb-2 w-1/2'></div>
          <div className='h-4 bg-zinc-700 rounded w-2/3'></div>
        </div>
      </div>
    );
  }

  if (!activeCharacter) {
    return (
      <div
        className={`bg-zinc-900/50 p-6 rounded-lg border border-zinc-800 ${className}`}
      >
        <h3 className='text-lg font-semibold text-white mb-3'>
          Active Character
        </h3>
        <div className='text-zinc-400 text-sm'>
          <p>No active character selected</p>
          <p className='mt-2 text-xs'>
            Create or select a character to start tracking
          </p>
        </div>
      </div>
    );
  }

  const currentLocation = activeCharacter.last_known_location;
  const totalPlayTime = timeTrackingData?.summary?.total_play_time_seconds || 0;
  const totalLocations =
    timeTrackingData?.summary?.total_locations_tracked || 0;

  return (
    <div
      className={`bg-zinc-900/50 p-6 rounded-lg border border-zinc-800 ${className}`}
    >
      <h3 className='text-lg font-semibold text-white mb-3'>
        Active Character
      </h3>

      <div className='space-y-3'>
        {/* Character Info */}
        <div>
          <div className='flex items-center gap-2 mb-1'>
            <h4 className='text-white font-medium'>{activeCharacter.name}</h4>
            {activeCharacter.hardcore && (
              <span className='px-2 py-1 text-xs bg-red-600 text-white rounded'>
                HC
              </span>
            )}
            {activeCharacter.solo_self_found && (
              <span className='px-2 py-1 text-xs bg-blue-600 text-white rounded'>
                SSF
              </span>
            )}
          </div>
          <p className='text-zinc-300 text-sm'>
            {activeCharacter.class} • {activeCharacter.ascendency} •{' '}
            {activeCharacter.league}
          </p>
          <div className='flex items-center gap-3 mt-1'>
            <span className='text-lg font-bold text-blue-400'>
              Level {activeCharacter.level}
            </span>
            {activeCharacter.death_count > 0 && (
              <span className='text-sm text-red-400'>
                {activeCharacter.death_count} death{activeCharacter.death_count !== 1 ? 's' : ''}
              </span>
            )}
          </div>
        </div>

        {/* Current Location */}
        {currentLocation && (
          <div>
            <p className='text-zinc-400 text-xs mb-1'>Current Location</p>
            <p className='text-white text-sm'>
              {currentLocation.location_name}
            </p>
            <p className='text-zinc-400 text-xs'>
              {currentLocation.location_type} • Entered{' '}
              {new Date(currentLocation.entry_timestamp).toLocaleTimeString()}
            </p>
          </div>
        )}

        {/* Quick Stats */}
        <div className='grid grid-cols-2 gap-3 pt-2 border-t border-zinc-700'>
          <div>
            <p className='text-zinc-400 text-xs'>Total Play Time</p>
            <p className='text-white text-sm font-medium'>
              {formatDuration(totalPlayTime)}
            </p>
          </div>
          <div>
            <p className='text-zinc-400 text-xs'>Locations Visited</p>
            <p className='text-white text-sm font-medium'>{totalLocations}</p>
          </div>
        </div>
      </div>
    </div>
  );
}
