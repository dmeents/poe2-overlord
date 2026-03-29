import { TrashIcon } from '@heroicons/react/24/outline';
import type { NoteData } from '@/types/notes';
import { Button } from '../../ui/button/button';
import { Modal } from '../../ui/modal/modal';
import { deleteNoteModalStyles } from './delete-note-modal.styles';

interface DeleteNoteModalProps {
  isOpen: boolean;
  note?: NoteData | null;
  onConfirm: () => void;
  onCancel: () => void;
  isLoading?: boolean;
}

export function DeleteNoteModal({
  isOpen,
  note,
  onConfirm,
  onCancel,
  isLoading,
}: DeleteNoteModalProps) {
  if (!note) return null;

  return (
    <Modal
      isOpen={isOpen}
      onClose={onCancel}
      size="sm"
      title="Delete Note"
      icon={<TrashIcon className="w-4 h-4" />}
      disabled={isLoading}>
      <div className={deleteNoteModalStyles.content}>
        <p className="text-stone-300 text-sm">
          Are you sure you want to delete <strong className="text-white">{note.title}</strong>?
        </p>
        <p className={deleteNoteModalStyles.warning}>This action cannot be undone.</p>
      </div>

      <div className={deleteNoteModalStyles.actions}>
        <Button variant="outline" size="sm" onClick={onCancel} disabled={isLoading}>
          Cancel
        </Button>
        <Button
          variant="danger"
          size="sm"
          onClick={onConfirm}
          disabled={isLoading}
          loading={isLoading}>
          Delete Note
        </Button>
      </div>
    </Modal>
  );
}
