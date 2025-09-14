import { Link } from '@tanstack/react-router';
import { useCharacterManagement } from '../../hooks/useCharacterManagement';
import { useCharacterTimeTracking } from '../../hooks/useCharacterTimeTracking';
import { Button } from '../button';

interface QuickActionsPanelProps {
  className?: string;
}

export function QuickActionsPanel({ className = '' }: QuickActionsPanelProps) {
  const {
    activeCharacter,
    activeSessions,
    isLoading,
  } = useCharacterTimeTracking();

  const { characters } = useCharacterManagement();

  const hasActiveSessions = activeSessions.length > 0;

  if (isLoading) {
    return (
      <div
        className={`bg-zinc-900/50 p-6 rounded-lg border border-zinc-800 ${className}`}
      >
        <h3 className='text-lg font-semibold text-white mb-4'>Quick Actions</h3>
        <div className='animate-pulse space-y-3'>
          <div className='h-10 bg-zinc-700 rounded'></div>
          <div className='h-10 bg-zinc-700 rounded'></div>
        </div>
      </div>
    );
  }

  return (
    <div
      className={`bg-zinc-900/50 p-6 rounded-lg border border-zinc-800 ${className}`}
    >
      <h3 className='text-lg font-semibold text-white mb-4'>Quick Actions</h3>

      <div className='space-y-3'>
        {/* Character Management */}
        <div className='pt-2 border-t border-zinc-700 space-y-2'>
          <Link to='/characters' className='block'>
            <Button variant='outline' size='sm' className='w-full'>
              {activeCharacter ? 'Manage Characters' : 'Create Character'}
            </Button>
          </Link>

          {characters.length > 1 && (
            <Link to='/characters' className='block'>
              <Button variant='outline' size='sm' className='w-full'>
                Switch Character
              </Button>
            </Link>
          )}
        </div>

        {/* Navigation Links */}
        <div className='pt-2 border-t border-zinc-700 space-y-2'>
          <Link to='/time-tracking' className='block'>
            <Button variant='outline' size='sm' className='w-full'>
              Time Tracking
            </Button>
          </Link>

          <Link to='/activity' className='block'>
            <Button variant='outline' size='sm' className='w-full'>
              Activity Monitor
            </Button>
          </Link>
        </div>

        {/* Status Info */}
        {activeCharacter && (
          <div className='pt-2 border-t border-zinc-700'>
            <p className='text-zinc-400 text-xs'>
              Active: {activeCharacter.name}
            </p>
            {hasActiveSessions && (
              <p className='text-green-400 text-xs'>
                {activeSessions.length} active session
                {activeSessions.length !== 1 ? 's' : ''}
              </p>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
