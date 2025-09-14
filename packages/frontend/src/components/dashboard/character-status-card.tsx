import { Link } from '@tanstack/react-router';
import { CharacterCard } from '../character-management/character-card';
import { Button } from '../button';
import { useCharacterTimeTracking } from '../../hooks/useCharacterTimeTracking';

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
          <p className='mt-2 text-xs mb-4'>
            Create or select a character to start tracking
          </p>
          <Link to='/characters'>
            <Button variant='primary' size='sm'>
              Manage Characters
            </Button>
          </Link>
        </div>
      </div>
    );
  }

  const totalPlayTime = timeTrackingData?.summary?.total_play_time_seconds || 0;

  return (
    <div className={className}>
      <CharacterCard
        character={activeCharacter}
        isActive={true}
        onSelect={() => {}} // No-op since it's already the active character
        onEdit={() => {}} // No-op since interactive is false
        onDelete={() => {}} // No-op since interactive is false
        totalPlayTime={totalPlayTime}
        interactive={false}
      />
    </div>
  );
}
