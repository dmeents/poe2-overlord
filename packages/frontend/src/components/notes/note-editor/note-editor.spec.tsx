import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import type { NoteData } from '@/types/notes';
import { NoteEditor } from './note-editor';

vi.mock('react-markdown', () => ({
  default: vi.fn(({ children }: { children: string }) => (
    <div data-testid="markdown-preview">{children}</div>
  )),
}));

const mockNote: NoteData = {
  id: 'note-1',
  title: 'Build Guide',
  content: '## Skills\n\nUse slam.',
  is_pinned: false,
  character_id: null,
  created_at: '2024-01-01T00:00:00Z',
  updated_at: '2024-01-10T00:00:00Z',
};

describe('NoteEditor', () => {
  const defaultProps = {
    note: null,
    characters: [],
    onSave: vi.fn(),
    isCreating: true,
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('create mode', () => {
    it('renders empty title and content inputs', () => {
      render(<NoteEditor {...defaultProps} />);

      expect(screen.getByPlaceholderText('Note title...')).toHaveValue('');
      expect(screen.getByPlaceholderText('Write your note in markdown...')).toHaveValue('');
    });

    it('shows Write and Preview tabs', () => {
      render(<NoteEditor {...defaultProps} />);

      expect(screen.getByRole('button', { name: 'Write' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Preview' })).toBeInTheDocument();
    });

    it('shows Create Note button label', () => {
      render(<NoteEditor {...defaultProps} />);

      expect(screen.getByRole('button', { name: 'Create Note' })).toBeInTheDocument();
    });

    it('disables the save button when title is empty', () => {
      render(<NoteEditor {...defaultProps} />);

      expect(screen.getByRole('button', { name: 'Create Note' })).toBeDisabled();
    });

    it('enables the save button once a title is entered', async () => {
      const user = userEvent.setup();
      render(<NoteEditor {...defaultProps} />);

      await user.type(screen.getByPlaceholderText('Note title...'), 'My Note');

      expect(screen.getByRole('button', { name: 'Create Note' })).not.toBeDisabled();
    });

    it('calls onSave with correct data when submitted', async () => {
      const user = userEvent.setup();
      const handleSave = vi.fn();
      render(<NoteEditor {...defaultProps} onSave={handleSave} />);

      await user.type(screen.getByPlaceholderText('Note title...'), 'My Note');
      await user.type(
        screen.getByPlaceholderText('Write your note in markdown...'),
        'Some content',
      );
      await user.click(screen.getByRole('button', { name: 'Create Note' }));

      expect(handleSave).toHaveBeenCalledWith({
        title: 'My Note',
        content: 'Some content',
        character_id: null,
      });
    });

    it('does not show Delete or Pin buttons in create mode', () => {
      render(<NoteEditor {...defaultProps} onDelete={vi.fn()} onTogglePin={vi.fn()} />);

      expect(screen.queryByText(/Delete/)).not.toBeInTheDocument();
      expect(screen.queryByText(/Pin/)).not.toBeInTheDocument();
    });
  });

  describe('edit mode', () => {
    const editProps = {
      note: mockNote,
      characters: [],
      onSave: vi.fn(),
      isCreating: false,
    };

    it('pre-fills title and content from existing note', async () => {
      const user = userEvent.setup();
      render(<NoteEditor {...editProps} />);

      await user.click(screen.getByRole('button', { name: 'Write' }));

      expect(screen.getByPlaceholderText('Note title...')).toHaveValue('Build Guide');
      expect(screen.getByPlaceholderText('Write your note in markdown...')).toHaveValue(
        '## Skills\n\nUse slam.',
      );
    });

    it('shows Save button label', () => {
      render(<NoteEditor {...editProps} />);

      expect(screen.getByRole('button', { name: 'Save' })).toBeInTheDocument();
    });

    it('shows the Delete button when onDelete is provided', () => {
      render(<NoteEditor {...editProps} onDelete={vi.fn()} />);

      expect(screen.getByRole('button', { name: /Delete/ })).toBeInTheDocument();
    });

    it('calls onDelete when delete button is clicked', async () => {
      const user = userEvent.setup();
      const handleDelete = vi.fn();
      render(<NoteEditor {...editProps} onDelete={handleDelete} />);

      await user.click(screen.getByRole('button', { name: /Delete/ }));

      expect(handleDelete).toHaveBeenCalledTimes(1);
    });

    it('shows the Pin button when onTogglePin is provided', () => {
      render(<NoteEditor {...editProps} onTogglePin={vi.fn()} />);

      expect(screen.getByTitle('Pin to dashboard')).toBeInTheDocument();
    });

    it('shows Pinned label when note is already pinned', () => {
      const pinnedNote = { ...mockNote, is_pinned: true };
      render(<NoteEditor {...editProps} note={pinnedNote} onTogglePin={vi.fn()} />);

      expect(screen.getByTitle('Unpin note')).toBeInTheDocument();
    });

    it('calls onCancel when Cancel button is clicked', async () => {
      const user = userEvent.setup();
      const handleCancel = vi.fn();
      render(<NoteEditor {...editProps} onCancel={handleCancel} />);

      await user.click(screen.getByRole('button', { name: 'Cancel' }));

      expect(handleCancel).toHaveBeenCalledTimes(1);
    });
  });

  describe('preview mode', () => {
    it('switches to preview when Preview tab is clicked', async () => {
      const user = userEvent.setup();
      render(<NoteEditor {...defaultProps} note={mockNote} isCreating={false} />);

      await user.click(screen.getByRole('button', { name: 'Preview' }));

      expect(screen.getByTestId('markdown-preview')).toBeInTheDocument();
      expect(
        screen.queryByPlaceholderText('Write your note in markdown...'),
      ).not.toBeInTheDocument();
    });

    it('shows empty preview message when content is blank', async () => {
      const user = userEvent.setup();
      const emptyNote = { ...mockNote, content: '' };
      render(<NoteEditor {...defaultProps} note={emptyNote} isCreating={false} />);

      await user.click(screen.getByRole('button', { name: 'Preview' }));

      expect(screen.getByText('Nothing to preview yet.')).toBeInTheDocument();
    });

    it('switches back to write mode when Write tab is clicked', async () => {
      const user = userEvent.setup();
      render(<NoteEditor {...defaultProps} note={mockNote} isCreating={false} />);

      await user.click(screen.getByRole('button', { name: 'Preview' }));
      await user.click(screen.getByRole('button', { name: 'Write' }));

      expect(screen.getByPlaceholderText('Write your note in markdown...')).toBeInTheDocument();
    });
  });

  describe('loading state', () => {
    it('disables all inputs when isLoading is true', async () => {
      const user = userEvent.setup();
      render(<NoteEditor {...defaultProps} note={mockNote} isCreating={false} isLoading={true} />);

      await user.click(screen.getByRole('button', { name: 'Write' }));

      expect(screen.getByPlaceholderText('Note title...')).toBeDisabled();
      expect(screen.getByPlaceholderText('Write your note in markdown...')).toBeDisabled();
    });
  });
});
