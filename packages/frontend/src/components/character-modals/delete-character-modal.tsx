import type { Character } from '../../types';
import { Button } from '../button';
import { Modal } from '../modal';
import {
  getCharacterInfoClasses,
  getCharacterInfoLabelClasses,
  getCharacterInfoTextClasses,
  getDeleteButtonClasses,
  getHardcoreClasses,
  getModalActionsClasses,
  getModalContentClasses,
  getSSFClasses,
  getWarningContainerClasses,
  getWarningIconClasses,
  getWarningStrongClasses,
  getWarningTextClasses,
} from './delete-character-modal.styles';

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
      className={getWarningIconClasses()}
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
      <div className={getModalContentClasses()}>
        <p className='text-zinc-300 mb-4'>
          Are you sure you want to delete{' '}
          <strong className='text-white'>{character.name}</strong>?
        </p>

        <div className={getCharacterInfoClasses()}>
          <div className={getCharacterInfoTextClasses()}>
            <div>
              <span className={getCharacterInfoLabelClasses()}>Class:</span>{' '}
              {character.class}
            </div>
            <div>
              <span className={getCharacterInfoLabelClasses()}>
                Ascendency:
              </span>{' '}
              {character.ascendency}
            </div>
            <div>
              <span className={getCharacterInfoLabelClasses()}>League:</span>{' '}
              {character.league}
            </div>
            {character.hardcore && (
              <div>
                <span className={getHardcoreClasses()}>Hardcore</span>
              </div>
            )}
            {character.solo_self_found && (
              <div>
                <span className={getSSFClasses()}>Solo Self-Found</span>
              </div>
            )}
          </div>
        </div>

        <div className={getWarningContainerClasses()}>
          <p className={getWarningTextClasses()}>
            <strong className={getWarningStrongClasses()}>Warning:</strong> This
            action cannot be undone. All time tracking data and statistics for
            this character will be permanently deleted.
          </p>
        </div>
      </div>

      <div className={getModalActionsClasses()}>
        <Button variant='outline' onClick={onCancel} disabled={isLoading}>
          Cancel
        </Button>
        <Button
          variant='outline'
          onClick={onConfirm}
          disabled={isLoading}
          className={getDeleteButtonClasses()}
        >
          {isLoading ? 'Deleting...' : 'Delete Character'}
        </Button>
      </div>
    </Modal>
  );
}
