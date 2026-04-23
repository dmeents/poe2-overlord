import { memo, useCallback, useState } from 'react';
import ReactMarkdown from 'react-markdown';
import type { CharacterSummaryData } from '@/types/character';
import type { NoteData } from '@/types/notes';
import { Button } from '../../ui/button/button';
import { noteEditorStyles } from './note-editor.styles';

interface NoteEditorProps {
  note?: NoteData | null;
  characters: CharacterSummaryData[];
  onSave: (data: { title: string; content: string; character_id: string | null }) => void;
  onDelete?: () => void;
  onTogglePin?: () => void;
  onCancel?: () => void;
  isLoading?: boolean;
  isCreating?: boolean;
}

export const NoteEditor = memo(function NoteEditor({
  note,
  characters,
  onSave,
  onDelete,
  onTogglePin,
  onCancel,
  isLoading = false,
  isCreating = false,
}: NoteEditorProps) {
  const [title, setTitle] = useState(note?.title ?? '');
  const [content, setContent] = useState(note?.content ?? '');
  const [characterId, setCharacterId] = useState<string | null>(note?.character_id ?? null);
  const [mode, setMode] = useState<'write' | 'preview'>(isCreating ? 'write' : 'preview');

  // Reset form when note changes
  const noteId = note?.id;
  const [prevNoteId, setPrevNoteId] = useState(noteId);
  if (noteId !== prevNoteId) {
    setPrevNoteId(noteId);
    setTitle(note?.title ?? '');
    setContent(note?.content ?? '');
    setCharacterId(note?.character_id ?? null);
    setMode(noteId ? 'preview' : 'write');
  }

  const handleSave = useCallback(() => {
    if (title.trim()) {
      onSave({ title: title.trim(), content, character_id: characterId });
    }
  }, [title, content, characterId, onSave]);

  const isDirty =
    title !== (note?.title ?? '') ||
    content !== (note?.content ?? '') ||
    characterId !== (note?.character_id ?? null);

  return (
    <div className={noteEditorStyles.container}>
      <input
        className={noteEditorStyles.titleInput}
        placeholder="Note title..."
        value={title}
        onChange={e => setTitle(e.target.value)}
        disabled={isLoading}
        maxLength={200}
      />

      <div className={noteEditorStyles.tabBar}>
        <button
          type="button"
          className={`${noteEditorStyles.tab} ${mode === 'write' ? noteEditorStyles.tabActive : noteEditorStyles.tabInactive}`}
          onClick={() => setMode('write')}>
          Write
        </button>
        <button
          type="button"
          className={`${noteEditorStyles.tab} ${mode === 'preview' ? noteEditorStyles.tabActive : noteEditorStyles.tabInactive}`}
          onClick={() => setMode('preview')}>
          Preview
        </button>
      </div>

      {mode === 'write' ? (
        <textarea
          className={noteEditorStyles.textarea}
          placeholder="Write your note in markdown..."
          value={content}
          onChange={e => setContent(e.target.value)}
          disabled={isLoading}
        />
      ) : (
        <div className={noteEditorStyles.preview}>
          {content.trim() ? (
            <ReactMarkdown>{content}</ReactMarkdown>
          ) : (
            <p className={noteEditorStyles.emptyPreview}>Nothing to preview yet.</p>
          )}
        </div>
      )}

      <div className={noteEditorStyles.footer}>
        {characters.length > 0 && (
          <div className={noteEditorStyles.characterSelect}>
            <select
              className="bg-stone-800 border border-stone-700 rounded text-xs text-stone-300 px-2 py-1 focus:outline-none focus:border-ember-500/60"
              value={characterId ?? ''}
              onChange={e => setCharacterId(e.target.value || null)}
              disabled={isLoading}>
              <option value="">No character</option>
              {characters.map(c => (
                <option key={c.id} value={c.id}>
                  {c.name}
                </option>
              ))}
            </select>
          </div>
        )}

        <div className={noteEditorStyles.actions}>
          {!isCreating && onTogglePin && (
            <Button
              variant="ghost"
              size="sm"
              onClick={onTogglePin}
              disabled={isLoading}
              title={note?.is_pinned ? 'Unpin note' : 'Pin to dashboard'}>
              {note?.is_pinned ? '📌 Pinned' : '📍 Pin'}
            </Button>
          )}
          {!isCreating && onDelete && (
            <Button variant="ghost" size="sm" onClick={onDelete} disabled={isLoading}>
              Delete
            </Button>
          )}
          {onCancel && (
            <Button variant="outline" size="sm" onClick={onCancel} disabled={isLoading}>
              Cancel
            </Button>
          )}
          <Button
            variant="primary"
            size="sm"
            onClick={handleSave}
            disabled={isLoading || !title.trim() || (!isCreating && !isDirty)}
            loading={isLoading}>
            {isCreating ? 'Create Note' : 'Save'}
          </Button>
        </div>
      </div>
    </div>
  );
});
