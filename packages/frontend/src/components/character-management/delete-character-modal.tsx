import type { Character } from '../../types';
import { Button } from '../button';
import { Modal } from '../modal';

interface DeleteCharacterModalProps {
  isOpen: boolean;
  character?: Character;
  onConfirm: () => void;
  onCancel: () => void;
  isLoading?: boolean;
}

export function DeleteCharacterModal({
  isOpen,
  character,
  onConfirm,
  onCancel,
  isLoading,
}: DeleteCharacterModalProps) {
  if (!character) return null;

  const warningIcon = (
    <svg
      className='h-6 w-6 text-red-400'
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
  );

  return (
    <Modal
      isOpen={isOpen}
      onClose={onCancel}
      size='md'
      title='Delete Character'
      icon={warningIcon}
      disabled={isLoading}
    >
      <div className='mb-6'>
        <p className='text-zinc-300 mb-4'>
          Are you sure you want to delete{' '}
          <strong className='text-white'>{character.name}</strong>?
        </p>

        <div className='bg-zinc-900/50 p-4 rounded-lg border border-zinc-700'>
          <div className='text-sm text-zinc-400 space-y-1'>
            <div>
              <span className='text-zinc-500'>Class:</span> {character.class}
            </div>
            <div>
              <span className='text-zinc-500'>Ascendency:</span>{' '}
              {character.ascendency}
            </div>
            <div>
              <span className='text-zinc-500'>League:</span> {character.league}
            </div>
            {character.hardcore && (
              <div>
                <span className='text-red-400'>Hardcore</span>
              </div>
            )}
            {character.solo_self_found && (
              <div>
                <span className='text-yellow-400'>Solo Self-Found</span>
              </div>
            )}
          </div>
        </div>

        <div className='mt-4 p-3 bg-red-500/10 border border-red-500/20 rounded-lg'>
          <p className='text-sm text-red-400'>
            <strong>Warning:</strong> This action cannot be undone. All time
            tracking data and statistics for this character will be permanently
            deleted.
          </p>
        </div>
      </div>

      <div className='flex justify-end gap-3'>
        <Button variant='outline' onClick={onCancel} disabled={isLoading}>
          Cancel
        </Button>
        <Button
          variant='outline'
          onClick={onConfirm}
          disabled={isLoading}
          className='text-red-400 hover:text-red-300 hover:border-red-400'
        >
          {isLoading ? 'Deleting...' : 'Delete Character'}
        </Button>
      </div>
    </Modal>
  );
}
