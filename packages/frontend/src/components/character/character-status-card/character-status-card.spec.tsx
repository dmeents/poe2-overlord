import { render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import type { CharacterData } from '@/types/character';
import { CharacterStatusCard } from './character-status-card';

const mockActiveCharacter: CharacterData = {
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
    total_town_time: 0,
    total_zones_visited: 20,
    total_deaths: 5,
    play_time_act1: 900,
    play_time_act2: 900,
    play_time_act3: 900,
    play_time_act4: 300,
    play_time_act5: 0,
    play_time_interlude: 0,
    play_time_endgame: 0,
  },
  walkthrough_progress: {
    current_step_id: null,
    is_completed: false,
    last_updated: '2024-01-10T00:00:00Z',
  },
};

const mockUseCharacter = vi.hoisted(() =>
  vi.fn(() => ({
    activeCharacter: null as CharacterData | null,
    isLoading: false,
  })),
);

vi.mock('../../../contexts/CharacterContext', () => ({
  useCharacter: mockUseCharacter,
}));

vi.mock('@tanstack/react-router', () => ({
  Link: ({ to, children }: { to: string; children: React.ReactNode }) => (
    <a href={to} data-testid={`link-${to}`}>
      {children}
    </a>
  ),
}));

describe('CharacterStatusCard', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockUseCharacter.mockReturnValue({
      activeCharacter: null,
      isLoading: false,
    });
  });

  it('renders loading state', () => {
    mockUseCharacter.mockReturnValue({
      activeCharacter: null,
      isLoading: true,
    });

    render(<CharacterStatusCard />);

    expect(screen.getByText('Active Character')).toBeInTheDocument();
    expect(screen.getByText('Loading character data...')).toBeInTheDocument();
  });

  it('renders empty state when no active character', () => {
    mockUseCharacter.mockReturnValue({
      activeCharacter: null,
      isLoading: false,
    });

    render(<CharacterStatusCard />);

    expect(screen.getByText('Active Character')).toBeInTheDocument();
    expect(screen.getByText('No Active Character')).toBeInTheDocument();
    expect(screen.getByText('Create or select a character to start tracking')).toBeInTheDocument();
  });

  it('renders manage characters button in empty state', () => {
    mockUseCharacter.mockReturnValue({
      activeCharacter: null,
      isLoading: false,
    });

    render(<CharacterStatusCard />);

    expect(screen.getByRole('button', { name: 'Manage Characters' })).toBeInTheDocument();
    expect(screen.getByTestId('link-/characters')).toBeInTheDocument();
  });

  it('renders character card when active character exists', () => {
    mockUseCharacter.mockReturnValue({
      activeCharacter: mockActiveCharacter,
      isLoading: false,
    });

    render(<CharacterStatusCard />);

    expect(screen.getByText('TestCharacter')).toBeInTheDocument();
    expect(screen.getByText('50')).toBeInTheDocument();
    expect(screen.getByText('Warrior')).toBeInTheDocument();
  });

  it('applies custom className', () => {
    mockUseCharacter.mockReturnValue({
      activeCharacter: null,
      isLoading: false,
    });

    const { container } = render(<CharacterStatusCard className="custom-class" />);

    expect(container.firstChild).toHaveClass('custom-class');
  });

  it('applies custom className when character is active', () => {
    mockUseCharacter.mockReturnValue({
      activeCharacter: mockActiveCharacter,
      isLoading: false,
    });

    const { container } = render(<CharacterStatusCard className="custom-class" />);

    expect(container.firstChild).toHaveClass('custom-class');
  });

  it('renders character card as non-interactive', () => {
    mockUseCharacter.mockReturnValue({
      activeCharacter: mockActiveCharacter,
      isLoading: false,
    });

    render(<CharacterStatusCard />);

    // Edit and Delete buttons should not be present
    expect(screen.queryByRole('button', { name: 'Edit' })).not.toBeInTheDocument();
    expect(screen.queryByRole('button', { name: 'Delete' })).not.toBeInTheDocument();
  });
});
