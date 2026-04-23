import { DocumentTextIcon } from '@heroicons/react/24/outline';
import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';
import { PageLayout } from '../components/layout/page-layout/page-layout';
import { DeleteNoteModal } from '../components/notes/delete-note-modal/delete-note-modal';
import { NoteEditor } from '../components/notes/note-editor/note-editor';
import { NoteList } from '../components/notes/note-list/note-list';
import { Card } from '../components/ui/card/card';
import { EmptyState } from '../components/ui/empty-state/empty-state';
import { LoadingSpinner } from '../components/ui/loading-spinner/loading-spinner';
import { useCharacter } from '../contexts/CharacterContext';
import { noteListConfig } from '../hooks/configs/note-list.config';
import { useListControls } from '../hooks/useListControls';
import {
  useCreateNote,
  useDeleteNote,
  useNotes,
  useToggleNotePin,
  useUpdateNote,
} from '../queries/notes';
import type { NoteData } from '../types/notes';

export const Route = createFileRoute('/notes')({
  component: NotesPage,
});

function NotesPage() {
  const { characters } = useCharacter();
  const { data: notes = [], isLoading } = useNotes();
  const createNote = useCreateNote();
  const updateNote = useUpdateNote();
  const deleteNote = useDeleteNote();
  const togglePin = useToggleNotePin();

  const [selectedNoteId, setSelectedNoteId] = useState<string | null>(null);
  const [isCreating, setIsCreating] = useState(false);
  const [deletingNote, setDeletingNote] = useState<NoteData | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const { result: filteredNotes } = useListControls(notes, noteListConfig);

  const selectedNote = notes.find(n => n.id === selectedNoteId) ?? null;

  const handleSelectNote = (noteId: string) => {
    setSelectedNoteId(noteId);
    setIsCreating(false);
  };

  const handleCreateNew = () => {
    setSelectedNoteId(null);
    setIsCreating(true);
  };

  const handleSave = async (data: {
    title: string;
    content: string;
    character_id: string | null;
  }) => {
    try {
      setIsSubmitting(true);
      if (isCreating) {
        const created = await createNote.mutateAsync({
          title: data.title,
          content: data.content,
          character_id: data.character_id,
        });
        setSelectedNoteId(created.id);
        setIsCreating(false);
      } else if (selectedNoteId) {
        await updateNote.mutateAsync({
          noteId: selectedNoteId,
          params: {
            title: data.title,
            content: data.content,
            character_id: data.character_id,
          },
        });
      }
    } catch {
      // handled by mutation
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleCancel = () => {
    setIsCreating(false);
    setSelectedNoteId(null);
  };

  const handleTogglePin = async () => {
    if (!selectedNoteId) return;
    try {
      await togglePin.mutateAsync(selectedNoteId);
    } catch {
      // handled by mutation
    }
  };

  const handleDeleteRequest = () => {
    if (selectedNote) setDeletingNote(selectedNote);
  };

  const handleDeleteConfirm = async () => {
    if (!deletingNote) return;
    try {
      setIsSubmitting(true);
      await deleteNote.mutateAsync(deletingNote.id);
      setDeletingNote(null);
      setSelectedNoteId(null);
    } catch {
      // handled by mutation
    } finally {
      setIsSubmitting(false);
    }
  };

  const showEditor = isCreating || selectedNoteId !== null;

  const leftColumn = (
    <Card
      title={isCreating ? 'New Note' : selectedNote ? selectedNote.title : 'Notes'}
      icon={<DocumentTextIcon className="w-5 h-5" />}
      accentColor="ember">
      {isLoading ? (
        <LoadingSpinner message="Loading notes..." className="py-8" />
      ) : showEditor ? (
        <div className="p-5">
          <NoteEditor
            key={isCreating ? 'new' : selectedNoteId}
            note={isCreating ? null : selectedNote}
            characters={characters ?? []}
            onSave={handleSave}
            onDelete={isCreating ? undefined : handleDeleteRequest}
            onTogglePin={isCreating ? undefined : handleTogglePin}
            onCancel={handleCancel}
            isLoading={isSubmitting}
            isCreating={isCreating}
          />
        </div>
      ) : (
        <EmptyState
          icon={<DocumentTextIcon className="w-12 h-12" />}
          title="No note selected"
          description="Select a note from the list or create a new one"
          className="py-12"
        />
      )}
    </Card>
  );

  const rightColumn = (
    <Card title="All Notes" icon={<DocumentTextIcon className="w-5 h-5" />}>
      {isLoading ? (
        <LoadingSpinner message="Loading..." className="py-8" />
      ) : (
        <NoteList
          notes={filteredNotes}
          selectedNoteId={selectedNoteId}
          characters={characters ?? []}
          onSelectNote={handleSelectNote}
          onCreateNote={handleCreateNew}
        />
      )}
    </Card>
  );

  return (
    <>
      <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} showCharacterCard />
      <DeleteNoteModal
        isOpen={deletingNote !== null}
        note={deletingNote}
        onConfirm={handleDeleteConfirm}
        onCancel={() => setDeletingNote(null)}
        isLoading={isSubmitting}
      />
    </>
  );
}
