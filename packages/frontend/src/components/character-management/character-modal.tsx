import type { Character } from '../../types';
import { CharacterForm, type CharacterFormData } from './character-form';

interface CharacterModalProps {
  isOpen: boolean;
  character?: Character;
  onSubmit: (data: CharacterFormData) => void;
  onClose: () => void;
  isLoading?: boolean;
}

export function CharacterModal({
  isOpen,
  character,
  onSubmit,
  onClose,
  isLoading,
}: CharacterModalProps) {
  if (!isOpen) return null;

  return (
    <div className='fixed inset-0 z-50 overflow-y-auto'>
      <div className='flex min-h-full items-center justify-center p-4'>
        {/* Backdrop */}
        <div
          className='fixed inset-0 bg-black/50 transition-opacity'
          onClick={onClose}
        />

        {/* Modal */}
        <div className='relative w-full max-w-2xl bg-zinc-800 rounded-lg shadow-xl border border-zinc-700'>
          <div className='p-6'>
            <div className='flex items-center justify-between mb-6'>
              <h2 className='text-2xl font-bold text-white'>
                {character ? 'Edit Character' : 'Create Character'}
              </h2>
              <button
                onClick={onClose}
                className='text-zinc-400 hover:text-white transition-colors'
                disabled={isLoading}
              >
                <svg
                  className='h-6 w-6'
                  fill='none'
                  viewBox='0 0 24 24'
                  stroke='currentColor'
                >
                  <path
                    strokeLinecap='round'
                    strokeLinejoin='round'
                    strokeWidth={2}
                    d='M6 18L18 6M6 6l12 12'
                  />
                </svg>
              </button>
            </div>

            <CharacterForm
              character={character}
              onSubmit={onSubmit}
              onCancel={onClose}
              isLoading={isLoading}
            />
          </div>
        </div>
      </div>
    </div>
  );
}
