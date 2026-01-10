import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { DeleteCharacterModal } from './delete-character-modal';
import type { CharacterData } from '@/types/character';

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
  last_played: '2024-01-10T00:00:00Z',
  current_location: {
    zone_name: 'The Coast',
    act: 1,
    is_town: false,
    has_waypoint: true,
    area_level: 2,
  },
  summary: {
    total_play_time: 3600,
    total_deaths: 5,
    total_zones_visited: 20,
    zones_with_waypoints: 10,
    unique_npcs_encountered: 15,
    timestamp: '2024-01-10T00:00:00Z',
  },
};

describe('DeleteCharacterModal', () => {
  const defaultProps = {
    isOpen: true,
    character: mockCharacter,
    onConfirm: vi.fn(),
    onCancel: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders modal title', () => {
    render(<DeleteCharacterModal {...defaultProps} />);

    // Title is in a heading element
    expect(screen.getByRole('heading', { name: 'Delete Character' })).toBeInTheDocument();
  });

  it('renders character name in confirmation message', () => {
    render(<DeleteCharacterModal {...defaultProps} />);

    expect(screen.getByText('TestCharacter')).toBeInTheDocument();
    expect(
      screen.getByText(/Are you sure you want to delete/)
    ).toBeInTheDocument();
  });

  it('renders character class', () => {
    render(<DeleteCharacterModal {...defaultProps} />);

    expect(screen.getByText('Warrior')).toBeInTheDocument();
    expect(screen.getByText('Class:')).toBeInTheDocument();
  });

  it('renders character ascendency', () => {
    render(<DeleteCharacterModal {...defaultProps} />);

    expect(screen.getByText('Titan')).toBeInTheDocument();
    expect(screen.getByText('Ascendency:')).toBeInTheDocument();
  });

  it('renders character league', () => {
    render(<DeleteCharacterModal {...defaultProps} />);

    expect(screen.getByText('Standard')).toBeInTheDocument();
    expect(screen.getByText('League:')).toBeInTheDocument();
  });

  it('renders SSF prefix when solo_self_found is true', () => {
    const ssfCharacter = {
      ...mockCharacter,
      solo_self_found: true,
    };

    render(<DeleteCharacterModal {...defaultProps} character={ssfCharacter} />);

    expect(screen.getByText(/SSF/)).toBeInTheDocument();
  });

  it('renders HC prefix when hardcore is true', () => {
    const hardcoreCharacter = {
      ...mockCharacter,
      hardcore: true,
    };

    render(
      <DeleteCharacterModal {...defaultProps} character={hardcoreCharacter} />
    );

    expect(screen.getByText(/HC/)).toBeInTheDocument();
  });

  it('renders warning message', () => {
    render(<DeleteCharacterModal {...defaultProps} />);

    expect(screen.getByText('Warning:')).toBeInTheDocument();
    expect(
      screen.getByText(/This action cannot be undone/)
    ).toBeInTheDocument();
  });

  it('renders cancel button', () => {
    render(<DeleteCharacterModal {...defaultProps} />);

    expect(screen.getByRole('button', { name: 'Cancel' })).toBeInTheDocument();
  });

  it('renders delete button', () => {
    render(<DeleteCharacterModal {...defaultProps} />);

    expect(
      screen.getByRole('button', { name: 'Delete Character' })
    ).toBeInTheDocument();
  });

  it('calls onConfirm when delete button is clicked', async () => {
    const user = userEvent.setup();
    const handleConfirm = vi.fn();

    render(<DeleteCharacterModal {...defaultProps} onConfirm={handleConfirm} />);

    await user.click(screen.getByRole('button', { name: 'Delete Character' }));

    expect(handleConfirm).toHaveBeenCalledTimes(1);
  });

  it('calls onCancel when cancel button is clicked', async () => {
    const user = userEvent.setup();
    const handleCancel = vi.fn();

    render(<DeleteCharacterModal {...defaultProps} onCancel={handleCancel} />);

    await user.click(screen.getByRole('button', { name: 'Cancel' }));

    expect(handleCancel).toHaveBeenCalledTimes(1);
  });

  it('shows loading state when isLoading is true', () => {
    render(<DeleteCharacterModal {...defaultProps} isLoading={true} />);

    expect(screen.getByRole('button', { name: 'Deleting...' })).toBeInTheDocument();
  });

  it('disables buttons when isLoading is true', () => {
    render(<DeleteCharacterModal {...defaultProps} isLoading={true} />);

    expect(screen.getByRole('button', { name: 'Cancel' })).toBeDisabled();
    expect(screen.getByRole('button', { name: 'Deleting...' })).toBeDisabled();
  });

  it('returns null when no character is provided', () => {
    const { container } = render(
      <DeleteCharacterModal
        {...defaultProps}
        character={undefined}
      />
    );

    expect(container.firstChild).toBeNull();
  });

  it('does not render when isOpen is false', () => {
    render(<DeleteCharacterModal {...defaultProps} isOpen={false} />);

    expect(screen.queryByText('Delete Character')).not.toBeInTheDocument();
  });
});
