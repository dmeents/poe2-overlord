import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { CharacterFilters, SortOption } from '@/hooks/useCharacterList';
import type { CharacterData } from '@/types/character';
import { CharacterList } from './character-list';

// Mock child components to simplify testing
vi.mock('../character-card/character-card', () => ({
  CharacterCard: vi.fn(({ character, isActive, onSelect, onEdit, onDelete }) => (
    <div data-testid={`character-card-${character.id}`}>
      <span>{character.name}</span>
      {isActive && <span data-testid="active-indicator">Active</span>}
      <button onClick={onSelect} data-testid={`select-${character.id}`}>
        Select
      </button>
      <button onClick={onEdit} data-testid={`edit-${character.id}`}>
        Edit
      </button>
      <button onClick={onDelete} data-testid={`delete-${character.id}`}>
        Delete
      </button>
    </div>
  )),
}));

vi.mock('../character-list-controls-form/character-list-controls-form', () => ({
  CharacterListControlsForm: vi.fn(({ onClearFilters }) => (
    <div data-testid="character-list-controls">
      <button onClick={onClearFilters} data-testid="clear-filters-controls">
        Clear Filters
      </button>
    </div>
  )),
}));

const createMockCharacter = (overrides: Partial<CharacterData> = {}): CharacterData => ({
  id: 'char-1',
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
    character_id: 'char-1',
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
  ...overrides,
});

const defaultFilters: CharacterFilters = {
  league: 'All',
  hardcore: null,
  soloSelfFound: null,
  classes: [],
  ascendencies: [],
  nameSearch: '',
};

const defaultSort: SortOption = {
  field: 'last_played',
  direction: 'desc',
};

describe('CharacterList', () => {
  const defaultProps = {
    characters: [createMockCharacter()],
    activeCharacterId: undefined,
    onSelectCharacter: vi.fn(),
    onEditCharacter: vi.fn(),
    onDeleteCharacter: vi.fn(),
    onCreateCharacter: vi.fn(),
    filters: defaultFilters,
    onFilterChange: vi.fn(),
    onClearFilters: vi.fn(),
    hasActiveFilters: false,
    sort: defaultSort,
    onSortChange: vi.fn(),
    onResetSort: vi.fn(),
    totalCount: 1,
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Empty State', () => {
    it('shows empty state when totalCount is 0', () => {
      render(<CharacterList {...defaultProps} characters={[]} totalCount={0} />);

      expect(screen.getByText('No Characters')).toBeInTheDocument();
      expect(screen.getByText(/Create your first character/)).toBeInTheDocument();
    });

    it('shows create character button in empty state', () => {
      render(<CharacterList {...defaultProps} characters={[]} totalCount={0} />);

      expect(screen.getByRole('button', { name: 'Create Character' })).toBeInTheDocument();
    });

    it('calls onCreateCharacter when create button is clicked', async () => {
      const user = userEvent.setup();
      const handleCreate = vi.fn();

      render(
        <CharacterList
          {...defaultProps}
          characters={[]}
          totalCount={0}
          onCreateCharacter={handleCreate}
        />,
      );

      await user.click(screen.getByRole('button', { name: 'Create Character' }));

      expect(handleCreate).toHaveBeenCalledTimes(1);
    });
  });

  describe('Character Rendering', () => {
    it('renders character cards for each character', () => {
      const characters = [
        createMockCharacter({ id: 'char-1', name: 'Character One' }),
        createMockCharacter({ id: 'char-2', name: 'Character Two' }),
        createMockCharacter({ id: 'char-3', name: 'Character Three' }),
      ];

      render(<CharacterList {...defaultProps} characters={characters} totalCount={3} />);

      expect(screen.getByTestId('character-card-char-1')).toBeInTheDocument();
      expect(screen.getByTestId('character-card-char-2')).toBeInTheDocument();
      expect(screen.getByTestId('character-card-char-3')).toBeInTheDocument();
    });

    it('marks the active character correctly', () => {
      const characters = [
        createMockCharacter({ id: 'char-1', name: 'Character One' }),
        createMockCharacter({ id: 'char-2', name: 'Character Two' }),
      ];

      render(
        <CharacterList
          {...defaultProps}
          characters={characters}
          activeCharacterId="char-2"
          totalCount={2}
        />,
      );

      // Only the active character should have the active indicator
      const activeIndicators = screen.getAllByTestId('active-indicator');
      expect(activeIndicators).toHaveLength(1);
    });

    it('renders the controls form', () => {
      render(<CharacterList {...defaultProps} />);

      expect(screen.getByTestId('character-list-controls')).toBeInTheDocument();
    });
  });

  describe('Character Interactions', () => {
    it('calls onSelectCharacter when select button is clicked', async () => {
      const user = userEvent.setup();
      const handleSelect = vi.fn();
      const character = createMockCharacter({ id: 'char-1' });

      render(
        <CharacterList
          {...defaultProps}
          characters={[character]}
          onSelectCharacter={handleSelect}
        />,
      );

      await user.click(screen.getByTestId('select-char-1'));

      expect(handleSelect).toHaveBeenCalledWith('char-1');
    });

    it('calls onEditCharacter when edit button is clicked', async () => {
      const user = userEvent.setup();
      const handleEdit = vi.fn();
      const character = createMockCharacter({ id: 'char-1' });

      render(
        <CharacterList {...defaultProps} characters={[character]} onEditCharacter={handleEdit} />,
      );

      await user.click(screen.getByTestId('edit-char-1'));

      expect(handleEdit).toHaveBeenCalledWith(character);
    });

    it('calls onDeleteCharacter when delete button is clicked', async () => {
      const user = userEvent.setup();
      const handleDelete = vi.fn();
      const character = createMockCharacter({ id: 'char-1' });

      render(
        <CharacterList
          {...defaultProps}
          characters={[character]}
          onDeleteCharacter={handleDelete}
        />,
      );

      await user.click(screen.getByTestId('delete-char-1'));

      expect(handleDelete).toHaveBeenCalledWith('char-1');
    });
  });

  describe('Filtered Empty State', () => {
    it('shows no results message when characters array is empty but totalCount > 0', () => {
      render(
        <CharacterList {...defaultProps} characters={[]} totalCount={5} hasActiveFilters={true} />,
      );

      expect(screen.getByText('No characters found')).toBeInTheDocument();
      expect(screen.getByText(/No characters match your current search/)).toBeInTheDocument();
    });

    it('shows clear filters button in filtered empty state', () => {
      render(
        <CharacterList {...defaultProps} characters={[]} totalCount={5} hasActiveFilters={true} />,
      );

      expect(screen.getByText('Clear All Filters')).toBeInTheDocument();
    });

    it('calls onClearFilters when clear filters button is clicked', async () => {
      const user = userEvent.setup();
      const handleClearFilters = vi.fn();

      render(
        <CharacterList
          {...defaultProps}
          characters={[]}
          totalCount={5}
          hasActiveFilters={true}
          onClearFilters={handleClearFilters}
        />,
      );

      await user.click(screen.getByText('Clear All Filters'));

      expect(handleClearFilters).toHaveBeenCalledTimes(1);
    });
  });

  describe('Filter Controls', () => {
    it('passes filter props to controls form', () => {
      render(<CharacterList {...defaultProps} />);

      // Controls form should be rendered
      expect(screen.getByTestId('character-list-controls')).toBeInTheDocument();
    });

    it('calls onClearFilters from controls form', async () => {
      const user = userEvent.setup();
      const handleClearFilters = vi.fn();

      render(<CharacterList {...defaultProps} onClearFilters={handleClearFilters} />);

      await user.click(screen.getByTestId('clear-filters-controls'));

      expect(handleClearFilters).toHaveBeenCalledTimes(1);
    });
  });
});
