import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import type { CharacterSummaryData } from '@/types/character';
import type { NoteData } from '@/types/notes';
import { NoteListItem } from './note-list-item';

const mockNote: NoteData = {
  id: 'note-1',
  title: 'Trade Log',
  content: '- Chaos orb x10\n- Divine orb x1',
  is_pinned: false,
  character_id: null,
  created_at: '2024-01-01T00:00:00Z',
  updated_at: '2024-01-10T12:00:00Z',
};

const mockCharacter: CharacterSummaryData = {
  id: 'char-1',
  name: 'Blademaster',
  class: 'Warrior',
  ascendency: 'Titan',
  level: 80,
  league: 'Standard',
  hardcore: false,
  solo_self_found: false,
  created_at: '2024-01-01T00:00:00Z',
  last_updated: '2024-01-10T00:00:00Z',
  last_played: '2024-01-10T00:00:00Z',
  current_location: undefined,
  summary: {
    character_id: 'char-1',
    total_play_time: 3600,
    total_hideout_time: 0,
    total_town_time: 0,
    total_zones_visited: 10,
    total_deaths: 0,
    play_time_act1: 900,
    play_time_act2: 900,
    play_time_act3: 900,
    play_time_act4: 900,
    play_time_act5: 0,
    play_time_interlude: 0,
    play_time_endgame: 0,
  },
  is_active: false,
  walkthrough_progress: {
    current_step_id: null,
    is_completed: false,
    last_updated: '2024-01-10T00:00:00Z',
  },
};

describe('NoteListItem', () => {
  const defaultProps = {
    note: mockNote,
    isActive: false,
    characters: [],
    onClick: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders the note title', () => {
    render(<NoteListItem {...defaultProps} />);

    expect(screen.getByText('Trade Log')).toBeInTheDocument();
  });

  it('renders a content preview showing note content', () => {
    render(<NoteListItem {...defaultProps} />);

    // Content is visible (markdown syntax chars like # * ` are stripped)
    expect(screen.getByText(/Chaos orb/)).toBeInTheDocument();
  });

  it('does not show the pin icon when note is not pinned', () => {
    render(<NoteListItem {...defaultProps} />);

    expect(screen.queryByText('📌')).not.toBeInTheDocument();
  });

  it('shows the pin icon when note is pinned', () => {
    const pinnedNote = { ...mockNote, is_pinned: true };
    render(<NoteListItem {...defaultProps} note={pinnedNote} />);

    expect(screen.getByText('📌')).toBeInTheDocument();
  });

  it('does not show character badge when no character is associated', () => {
    render(<NoteListItem {...defaultProps} />);

    expect(screen.queryByText('Blademaster')).not.toBeInTheDocument();
  });

  it('shows the character badge when a character is associated', () => {
    const noteWithChar = { ...mockNote, character_id: 'char-1' };
    render(<NoteListItem {...defaultProps} note={noteWithChar} characters={[mockCharacter]} />);

    expect(screen.getByText('Blademaster')).toBeInTheDocument();
  });

  it('calls onClick when clicked', async () => {
    const user = userEvent.setup();
    const handleClick = vi.fn();

    render(<NoteListItem {...defaultProps} onClick={handleClick} />);

    await user.click(screen.getByRole('button'));

    expect(handleClick).toHaveBeenCalledTimes(1);
  });

  it('does not show a preview when content is empty', () => {
    const emptyNote = { ...mockNote, content: '' };
    render(<NoteListItem {...defaultProps} note={emptyNote} />);

    // Only title and meta should be present, no preview paragraph
    expect(screen.getByText('Trade Log')).toBeInTheDocument();
    expect(screen.queryByText(/Chaos orb/)).not.toBeInTheDocument();
  });
});
