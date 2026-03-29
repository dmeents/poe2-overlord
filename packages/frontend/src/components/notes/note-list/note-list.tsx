import { PlusIcon } from '@heroicons/react/24/outline';
import { memo } from 'react';
import type { CharacterSummaryData } from '@/types/character';
import type { NoteData } from '@/types/notes';
import { Button } from '../../ui/button/button';
import { NoteListItem } from '../note-list-item/note-list-item';
import { noteListStyles } from './note-list.styles';

interface NoteListProps {
  notes: NoteData[];
  selectedNoteId: string | null;
  characters: CharacterSummaryData[];
  onSelectNote: (noteId: string) => void;
  onCreateNote: () => void;
}

export const NoteList = memo(function NoteList({
  notes,
  selectedNoteId,
  characters,
  onSelectNote,
  onCreateNote,
}: NoteListProps) {
  return (
    <div className={noteListStyles.container}>
      <div className={noteListStyles.header}>
        <span className={noteListStyles.headerTitle}>
          Notes <span className={noteListStyles.headerCount}>({notes.length})</span>
        </span>
        <Button variant="icon" size="xs" onClick={onCreateNote} title="New note">
          <PlusIcon className="w-4 h-4" />
        </Button>
      </div>

      {notes.length === 0 ? (
        <div className={noteListStyles.emptyContainer}>
          <p className={noteListStyles.emptyText}>No notes yet.</p>
          <p className={noteListStyles.emptyText}>Create your first note to get started.</p>
        </div>
      ) : (
        <div className={noteListStyles.scrollArea}>
          {notes.map(note => (
            <NoteListItem
              key={note.id}
              note={note}
              isActive={note.id === selectedNoteId}
              characters={characters}
              onClick={() => onSelectNote(note.id)}
            />
          ))}
        </div>
      )}
    </div>
  );
});
