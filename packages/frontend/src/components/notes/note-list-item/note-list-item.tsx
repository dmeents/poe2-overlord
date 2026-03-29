import { memo } from 'react';
import type { CharacterSummaryData } from '@/types/character';
import type { NoteData } from '@/types/notes';
import { noteListItemStyles } from './note-list-item.styles';

interface NoteListItemProps {
  note: NoteData;
  isActive: boolean;
  characters: CharacterSummaryData[];
  onClick: () => void;
}

function formatRelativeTime(dateStr: string): string {
  const date = new Date(dateStr);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMin = Math.floor(diffMs / 60000);
  const diffHrs = Math.floor(diffMin / 60);
  const diffDays = Math.floor(diffHrs / 24);

  if (diffMin < 1) return 'just now';
  if (diffMin < 60) return `${diffMin}m ago`;
  if (diffHrs < 24) return `${diffHrs}h ago`;
  if (diffDays < 7) return `${diffDays}d ago`;
  return date.toLocaleDateString();
}

export const NoteListItem = memo(function NoteListItem({
  note,
  isActive,
  characters,
  onClick,
}: NoteListItemProps) {
  const character = characters.find(c => c.id === note.character_id);

  return (
    <button
      type="button"
      className={`${noteListItemStyles.container} ${isActive ? noteListItemStyles.containerActive : noteListItemStyles.containerInactive}`}
      onClick={onClick}>
      <div className={noteListItemStyles.header}>
        <span className={noteListItemStyles.title}>{note.title}</span>
        {note.is_pinned && <span className={noteListItemStyles.pinIcon}>📌</span>}
      </div>

      {note.content && (
        <p className={noteListItemStyles.preview}>
          {note.content.replace(/[#*`_~[\]]/g, '').slice(0, 100)}
        </p>
      )}

      <div className={noteListItemStyles.meta}>
        <span className={noteListItemStyles.timestamp}>{formatRelativeTime(note.updated_at)}</span>
        {character && <span className={noteListItemStyles.characterBadge}>{character.name}</span>}
      </div>
    </button>
  );
});
