import { Link } from '@tanstack/react-router';
import { Button } from '../../ui/button';

interface EmptyCharacterStateProps {
  className?: string;
}

export function EmptyCharacterState({
  className = '',
}: EmptyCharacterStateProps) {
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
