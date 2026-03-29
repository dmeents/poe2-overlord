import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import type { NoteData } from '@/types/notes';
import { DeleteNoteModal } from './delete-note-modal';

const mockNote: NoteData = {
  id: 'note-1',
  title: 'Boss Strategy Guide',
  content: '## Phase 1\n\nDodge the slam.',
  is_pinned: false,
  character_id: null,
  created_at: '2024-01-01T00:00:00Z',
  updated_at: '2024-01-10T00:00:00Z',
};

describe('DeleteNoteModal', () => {
  const defaultProps = {
    isOpen: true,
    note: mockNote,
    onConfirm: vi.fn(),
    onCancel: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders note title in confirmation message', () => {
    render(<DeleteNoteModal {...defaultProps} />);

    expect(screen.getByRole('heading', { name: 'Delete Note' })).toBeInTheDocument();
    expect(screen.getByText('Boss Strategy Guide')).toBeInTheDocument();
    expect(screen.getByText(/Are you sure you want to delete/)).toBeInTheDocument();
  });

  it('renders warning and action buttons', () => {
    render(<DeleteNoteModal {...defaultProps} />);

    expect(screen.getByText(/This action cannot be undone/)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Cancel' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Delete Note' })).toBeInTheDocument();
  });

  it('calls onConfirm when delete button is clicked', async () => {
    const user = userEvent.setup();
    const handleConfirm = vi.fn();

    render(<DeleteNoteModal {...defaultProps} onConfirm={handleConfirm} />);

    await user.click(screen.getByRole('button', { name: 'Delete Note' }));

    expect(handleConfirm).toHaveBeenCalledTimes(1);
  });

  it('calls onCancel when cancel button is clicked', async () => {
    const user = userEvent.setup();
    const handleCancel = vi.fn();

    render(<DeleteNoteModal {...defaultProps} onCancel={handleCancel} />);

    await user.click(screen.getByRole('button', { name: 'Cancel' }));

    expect(handleCancel).toHaveBeenCalledTimes(1);
  });

  it('disables buttons when isLoading is true', () => {
    render(<DeleteNoteModal {...defaultProps} isLoading={true} />);

    expect(screen.getByRole('button', { name: 'Cancel' })).toBeDisabled();
    expect(screen.getByRole('button', { name: 'Delete Note' })).toBeDisabled();
  });

  it('returns null when no note is provided', () => {
    const { container } = render(<DeleteNoteModal {...defaultProps} note={null} />);

    expect(container.firstChild).toBeNull();
  });

  it('does not render when isOpen is false', () => {
    render(<DeleteNoteModal {...defaultProps} isOpen={false} />);

    expect(screen.queryByText('Delete Note')).not.toBeInTheDocument();
  });
});
