import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import type { CharacterData } from '../../../types/character';
import { CharacterCard } from './character-card';

const mockCharacter: CharacterData = {
  id: 'test-id',
  name: 'TestCharacter',
  class: 'Warrior',
  ascendency: 'Titan',
  level: 50,
  league: 'Standard',
  hardcore: false,
  solo_self_found: false,
  created_at: '2024-01-01T00:00:00Z',
  last_updated: '2024-01-10T00:00:00Z',
  last_played: '2024-01-10T00:00:00Z',
  current_location: {
    zone_name: 'The Coast',
    act: 1,
    is_town: false,
    location_type: 'Zone',
    has_waypoint: true,
    area_level: 2,
    last_updated: '2024-01-10T00:00:00Z',
  },
  summary: {
    character_id: 'test-id',
    total_play_time: 3600,
    total_hideout_time: 600,
    total_zones_visited: 20,
    total_deaths: 5,
    play_time_act1: 900,
    play_time_act2: 900,
    play_time_act3: 900,
    play_time_act4: 300,
    play_time_interlude: 0,
    play_time_endgame: 0,
  },
  zones: [],
  walkthrough_progress: {
    current_step_id: null,
    is_completed: false,
    last_updated: '2024-01-10T00:00:00Z',
  },
};

describe('CharacterCard', () => {
  const defaultProps = {
    character: mockCharacter,
    isActive: false,
    onSelect: vi.fn(),
    onEdit: vi.fn(),
    onDelete: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders character name', () => {
    render(<CharacterCard {...defaultProps} />);

    expect(screen.getByText('TestCharacter')).toBeInTheDocument();
  });

  it('renders character level', () => {
    render(<CharacterCard {...defaultProps} />);

    expect(screen.getByText('50')).toBeInTheDocument();
  });

  it('renders character class', () => {
    render(<CharacterCard {...defaultProps} />);

    expect(screen.getByText('Warrior')).toBeInTheDocument();
  });

  it('renders character ascendency', () => {
    render(<CharacterCard {...defaultProps} />);

    expect(screen.getByText('Titan')).toBeInTheDocument();
  });

  it('renders character league', () => {
    render(<CharacterCard {...defaultProps} />);

    expect(screen.getByText('Standard')).toBeInTheDocument();
  });

  it('calls onSelect when card is clicked', async () => {
    const user = userEvent.setup();
    const handleSelect = vi.fn();

    render(<CharacterCard {...defaultProps} onSelect={handleSelect} />);

    await user.click(screen.getByText('TestCharacter'));

    expect(handleSelect).toHaveBeenCalledTimes(1);
  });

  it('does not call onSelect when interactive is false', async () => {
    const user = userEvent.setup();
    const handleSelect = vi.fn();

    render(
      <CharacterCard
        {...defaultProps}
        onSelect={handleSelect}
        interactive={false}
      />
    );

    await user.click(screen.getByText('TestCharacter'));

    expect(handleSelect).not.toHaveBeenCalled();
  });

  it('renders edit button when interactive', () => {
    render(<CharacterCard {...defaultProps} interactive={true} />);

    expect(screen.getByRole('button', { name: 'Edit' })).toBeInTheDocument();
  });

  it('renders delete button when interactive', () => {
    render(<CharacterCard {...defaultProps} interactive={true} />);

    expect(screen.getByRole('button', { name: 'Delete' })).toBeInTheDocument();
  });

  it('does not render action buttons when interactive is false', () => {
    render(<CharacterCard {...defaultProps} interactive={false} />);

    expect(
      screen.queryByRole('button', { name: 'Edit' })
    ).not.toBeInTheDocument();
    expect(
      screen.queryByRole('button', { name: 'Delete' })
    ).not.toBeInTheDocument();
  });

  it('renders HC prefix when hardcore is true', () => {
    const hardcoreCharacter = {
      ...mockCharacter,
      hardcore: true,
    };

    render(<CharacterCard {...defaultProps} character={hardcoreCharacter} />);

    expect(screen.getByText(/HC/)).toBeInTheDocument();
  });

  it('renders SSF prefix when solo_self_found is true', () => {
    const ssfCharacter = {
      ...mockCharacter,
      solo_self_found: true,
    };

    render(<CharacterCard {...defaultProps} character={ssfCharacter} />);

    expect(screen.getByText(/SSF/)).toBeInTheDocument();
  });
});
