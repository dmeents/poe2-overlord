import { DocumentTextIcon } from '@heroicons/react/24/outline';
import { Link } from '@tanstack/react-router';
import { usePinnedNotes } from '@/queries/notes';
import { Card } from '../../ui/card/card';
import { LoadingSpinner } from '../../ui/loading-spinner/loading-spinner';
import { pinnedNotesCardStyles } from './pinned-notes-card.styles';

export function PinnedNotesCard() {
  const { data: pinnedNotes, isLoading } = usePinnedNotes();

  if (isLoading) {
    return (
      <Card title="Pinned Notes" icon={<DocumentTextIcon className="w-5 h-5" />}>
        <LoadingSpinner message="Loading notes..." className="py-4" />
      </Card>
    );
  }

  const notes = pinnedNotes ?? [];

  return (
    <Card
      title="Pinned Notes"
      icon={<DocumentTextIcon className="w-5 h-5" />}
      rightAction={{ label: 'All Notes', onClick: () => {} }}>
      {notes.length === 0 ? (
        <div className={pinnedNotesCardStyles.empty}>
          <p className={pinnedNotesCardStyles.emptyText}>No pinned notes</p>
          <p className={pinnedNotesCardStyles.emptyHint}>
            Pin notes from the{' '}
            <Link to="/notes" className="text-ember-400 hover:text-ember-300">
              Notes
            </Link>{' '}
            page to see them here
          </p>
        </div>
      ) : (
        <div className={pinnedNotesCardStyles.list}>
          {notes.map(note => (
            <Link key={note.id} to="/notes" className="block">
              <div className={pinnedNotesCardStyles.item}>
                <p className={pinnedNotesCardStyles.itemTitle}>{note.title}</p>
                {note.content && (
                  <p className={pinnedNotesCardStyles.itemPreview}>
                    {note.content.replace(/[#*`_~[\]]/g, '').slice(0, 80)}
                  </p>
                )}
              </div>
            </Link>
          ))}
        </div>
      )}
    </Card>
  );
}
