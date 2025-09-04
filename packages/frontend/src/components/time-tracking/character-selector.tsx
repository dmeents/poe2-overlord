import { Link } from '@tanstack/react-router';
import type { Character } from '../../types';
import { Button } from '../button';

interface CharacterSelectorProps {
  activeCharacter: Character | null;
  hasActiveCharacter: boolean;
}

export function CharacterSelector({
  activeCharacter,
  hasActiveCharacter,
}: CharacterSelectorProps) {
  if (!hasActiveCharacter) {
    return (
      <div className='bg-yellow-500/10 border border-yellow-500/20 rounded-lg p-4'>
        <div className='flex items-center gap-3'>
          <div className='flex-shrink-0'>
            <svg
              className='h-5 w-5 text-yellow-400'
              fill='none'
              viewBox='0 0 24 24'
              stroke='currentColor'
            >
              <path
                strokeLinecap='round'
                strokeLinejoin='round'
                strokeWidth={2}
                d='M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z'
              />
            </svg>
          </div>
          <div className='flex-1'>
            <h3 className='text-sm font-medium text-yellow-400'>
              No Active Character
            </h3>
            <p className='text-sm text-yellow-300 mt-1'>
              Time tracking requires an active character. Create or select a
              character to start tracking your play time.
            </p>
          </div>
          <div className='flex-shrink-0'>
            <Link to='/characters'>
              <Button variant='primary' size='sm'>
                Manage Characters
              </Button>
            </Link>
          </div>
        </div>
      </div>
    );
  }

  const getClassColor = (characterClass: string) => {
    const colors: Record<string, string> = {
      Warrior: 'text-red-400',
      Sorceress: 'text-blue-400',
      Ranger: 'text-green-400',
      Huntress: 'text-yellow-400',
      Monk: 'text-purple-400',
      Mercenary: 'text-orange-400',
      Witch: 'text-pink-400',
    };
    return colors[characterClass] || 'text-zinc-400';
  };

  return (
    <div className='bg-blue-500/10 border border-blue-500/20 rounded-lg p-4'>
      <div className='flex items-center justify-between'>
        <div className='flex items-center gap-3'>
          <div className='w-2 h-2 bg-blue-500 rounded-full'></div>
          <div>
            <p className='text-sm text-blue-400 font-medium'>
              Active Character
            </p>
            <div className='flex items-center gap-2 mt-1'>
              <span className='text-white font-semibold'>
                {activeCharacter?.name}
              </span>
              <span className='text-zinc-400'>•</span>
              <span className={getClassColor(activeCharacter?.class || '')}>
                {activeCharacter?.class}
              </span>
              <span className='text-zinc-400'>•</span>
              <span className='text-zinc-400'>
                {activeCharacter?.ascendency}
              </span>
              <span className='text-zinc-400'>•</span>
              <span className='text-zinc-400'>{activeCharacter?.league}</span>
            </div>
          </div>
        </div>
        <div className='flex gap-2'>
          <Link to='/characters'>
            <Button variant='outline' size='sm'>
              Change Character
            </Button>
          </Link>
        </div>
      </div>
    </div>
  );
}
