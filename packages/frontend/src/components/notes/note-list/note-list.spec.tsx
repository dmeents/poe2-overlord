import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import type { NoteData } from '@/types/notes';
import { NoteList } from './note-list';

vi.mock('../note-list-item/note-list-item', () => ({
  NoteListItem: vi.fn(({ note, isActive, onClick }) => (
    <div data-testid={`note-item-${note.id}`}>
      <span>{note.title}</span>
      {isActive && <span data-testid="active-indicator">Active</span>}
      <button type="button" onClick={onClick} data-testid={`select-${note.id}`}>
        Select
      </button>
    </div>
  )),
}));

const makeNote = (overrides: Partial<NoteData> = {}): NoteData => ({
  id: 'note-1',
  title: 'Test Note',
  content: 'Some content',
  is_pinned: false,
  character_id: null,
  created_at: '2024-01-01T00:00:00Z',
  updated_at: '2024-01-10T00:00:00Z',
  ...overrides,
});

describe('NoteList', () => {
  const defaultProps = {
    notes: [],
    selectedNoteId: null,
    characters: [],
    onSelectNote: vi.fn(),
    onCreateNote: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('empty state', () => {
    it('shows empty state message when there are no notes', () => {
      render(<NoteList {...defaultProps} />);

      expect(screen.getByText(/No notes yet/)).toBeInTheDocument();
    });

    it('shows hint to create a note', () => {
      render(<NoteList {...defaultProps} />);

      expect(screen.getByText(/Create your first note/)).toBeInTheDocument();
    });
  });

  describe('with notes', () => {
    const notes = [
      makeNote({ id: 'note-1', title: 'First Note' }),
      makeNote({ id: 'note-2', title: 'Second Note' }),
    ];

    it('renders all notes', () => {
      render(<NoteList {...defaultProps} notes={notes} />);

      expect(screen.getByTestId('note-item-note-1')).toBeInTheDocument();
      expect(screen.getByTestId('note-item-note-2')).toBeInTheDocument();
      expect(screen.getByText('First Note')).toBeInTheDocument();
      expect(screen.getByText('Second Note')).toBeInTheDocument();
    });

    it('shows the correct note count', () => {
      render(<NoteList {...defaultProps} notes={notes} />);

      expect(screen.getByText('(2)')).toBeInTheDocument();
    });

    it('marks the selected note as active', () => {
      render(<NoteList {...defaultProps} notes={notes} selectedNoteId="note-1" />);

      expect(screen.getByTestId('active-indicator')).toBeInTheDocument();
    });

    it('calls onSelectNote when a note is clicked', async () => {
      const user = userEvent.setup();
      const handleSelect = vi.fn();

      render(<NoteList {...defaultProps} notes={notes} onSelectNote={handleSelect} />);

      await user.click(screen.getByTestId('select-note-2'));

      expect(handleSelect).toHaveBeenCalledWith('note-2');
    });
  });

  it('calls onCreateNote when the new note button is clicked', async () => {
    const user = userEvent.setup();
    const handleCreate = vi.fn();

    render(<NoteList {...defaultProps} onCreateNote={handleCreate} />);

    await user.click(screen.getByTitle('New note'));

    expect(handleCreate).toHaveBeenCalledTimes(1);
  });
});
