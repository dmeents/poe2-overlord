import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { CharacterFormModal } from './character-form-modal';
import type { CharacterData } from '@/types/character';

vi.mock('@/hooks/useCharacterConfig', () => ({
  useCharacterConfig: () => ({
    characterClasses: [
      { value: 'Warrior', label: 'Warrior' },
      { value: 'Sorceress', label: 'Sorceress' },
      { value: 'Monk', label: 'Monk' },
    ],
    leagues: [
      { value: 'Standard', label: 'Standard' },
      { value: 'Rise of the Abyssal', label: 'Rise of the Abyssal' },
    ],
    getAscendenciesForClass: (characterClass: string) => {
      if (characterClass === 'Warrior') {
        return [
          { value: 'Titan', label: 'Titan' },
          { value: 'Warbringer', label: 'Warbringer' },
        ];
      }
      if (characterClass === 'Sorceress') {
        return [
          { value: 'Stormweaver', label: 'Stormweaver' },
          { value: 'Chronomancer', label: 'Chronomancer' },
        ];
      }
      return [{ value: 'Unknown', label: 'Unknown' }];
    },
    getDefaultFormData: () => ({
      name: '',
      class: 'Warrior',
      ascendency: 'Titan',
      league: 'Standard',
      hardcore: false,
      solo_self_found: false,
    }),
  }),
}));

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

describe('CharacterFormModal', () => {
  const defaultProps = {
    isOpen: true,
    onSubmit: vi.fn(),
    onClose: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Create Mode', () => {
    it('renders create modal title', () => {
      render(<CharacterFormModal {...defaultProps} />);

      expect(
        screen.getByRole('heading', { name: 'Create Character' })
      ).toBeInTheDocument();
    });

    it('renders character name field', () => {
      render(<CharacterFormModal {...defaultProps} />);

      expect(screen.getByText('Character Name')).toBeInTheDocument();
      expect(
        screen.getByPlaceholderText('Enter character name')
      ).toBeInTheDocument();
    });

    it('renders class field', () => {
      render(<CharacterFormModal {...defaultProps} />);

      expect(screen.getByText('Class')).toBeInTheDocument();
    });

    it('renders ascendency field', () => {
      render(<CharacterFormModal {...defaultProps} />);

      expect(screen.getByText('Ascendency')).toBeInTheDocument();
    });

    it('renders league field', () => {
      render(<CharacterFormModal {...defaultProps} />);

      expect(screen.getByText('League')).toBeInTheDocument();
    });

    it('renders hardcore checkbox', () => {
      render(<CharacterFormModal {...defaultProps} />);

      expect(screen.getByLabelText('Hardcore')).toBeInTheDocument();
    });

    it('renders SSF checkbox', () => {
      render(<CharacterFormModal {...defaultProps} />);

      expect(
        screen.getByLabelText('Solo Self-Found (SSF)')
      ).toBeInTheDocument();
    });

    it('renders create button', () => {
      render(<CharacterFormModal {...defaultProps} />);

      expect(
        screen.getByRole('button', { name: 'Create Character' })
      ).toBeInTheDocument();
    });

    it('renders cancel button', () => {
      render(<CharacterFormModal {...defaultProps} />);

      expect(screen.getByRole('button', { name: 'Cancel' })).toBeInTheDocument();
    });

    it('renders name generator buttons', () => {
      render(<CharacterFormModal {...defaultProps} />);

      expect(screen.getByTitle('Generate masculine name')).toBeInTheDocument();
      expect(screen.getByTitle('Generate feminine name')).toBeInTheDocument();
    });
  });

  describe('Edit Mode', () => {
    it('renders edit modal title', () => {
      render(<CharacterFormModal {...defaultProps} character={mockCharacter} />);

      expect(
        screen.getByRole('heading', { name: 'Edit Character' })
      ).toBeInTheDocument();
    });

    it('pre-fills character name', () => {
      render(<CharacterFormModal {...defaultProps} character={mockCharacter} />);

      expect(screen.getByDisplayValue('TestCharacter')).toBeInTheDocument();
    });

    it('renders update button', () => {
      render(<CharacterFormModal {...defaultProps} character={mockCharacter} />);

      expect(
        screen.getByRole('button', { name: 'Update Character' })
      ).toBeInTheDocument();
    });
  });

  describe('Form Interactions', () => {
    it('calls onClose when cancel button is clicked', async () => {
      const user = userEvent.setup();
      const handleClose = vi.fn();

      render(<CharacterFormModal {...defaultProps} onClose={handleClose} />);

      await user.click(screen.getByRole('button', { name: 'Cancel' }));

      expect(handleClose).toHaveBeenCalledTimes(1);
    });

    it('calls onSubmit with form data when submitted', async () => {
      const user = userEvent.setup();
      const handleSubmit = vi.fn();

      render(<CharacterFormModal {...defaultProps} onSubmit={handleSubmit} />);

      await user.type(
        screen.getByPlaceholderText('Enter character name'),
        'NewCharacter'
      );
      await user.click(
        screen.getByRole('button', { name: 'Create Character' })
      );

      expect(handleSubmit).toHaveBeenCalledWith(
        expect.objectContaining({
          name: 'NewCharacter',
          class: 'Warrior',
          ascendency: 'Titan',
          league: 'Standard',
          hardcore: false,
          solo_self_found: false,
        })
      );
    });

    it('validates required name field', async () => {
      const user = userEvent.setup();
      const handleSubmit = vi.fn();

      render(<CharacterFormModal {...defaultProps} onSubmit={handleSubmit} />);

      await user.click(
        screen.getByRole('button', { name: 'Create Character' })
      );

      expect(handleSubmit).not.toHaveBeenCalled();
      expect(screen.getByText(/is required/i)).toBeInTheDocument();
    });

    it('toggles hardcore checkbox', async () => {
      const user = userEvent.setup();

      render(<CharacterFormModal {...defaultProps} />);

      const hardcoreCheckbox = screen.getByLabelText('Hardcore');
      expect(hardcoreCheckbox).not.toBeChecked();

      await user.click(hardcoreCheckbox);

      expect(hardcoreCheckbox).toBeChecked();
    });

    it('toggles SSF checkbox', async () => {
      const user = userEvent.setup();

      render(<CharacterFormModal {...defaultProps} />);

      const ssfCheckbox = screen.getByLabelText('Solo Self-Found (SSF)');
      expect(ssfCheckbox).not.toBeChecked();

      await user.click(ssfCheckbox);

      expect(ssfCheckbox).toBeChecked();
    });
  });

  describe('Loading State', () => {
    it('shows saving text when loading', () => {
      render(<CharacterFormModal {...defaultProps} isLoading={true} />);

      expect(
        screen.getByRole('button', { name: 'Saving...' })
      ).toBeInTheDocument();
    });

    it('disables buttons when loading', () => {
      render(<CharacterFormModal {...defaultProps} isLoading={true} />);

      expect(screen.getByRole('button', { name: 'Cancel' })).toBeDisabled();
      expect(screen.getByRole('button', { name: 'Saving...' })).toBeDisabled();
    });
  });

  describe('Modal States', () => {
    it('does not render when isOpen is false', () => {
      render(<CharacterFormModal {...defaultProps} isOpen={false} />);

      expect(
        screen.queryByRole('heading', { name: 'Create Character' })
      ).not.toBeInTheDocument();
    });
  });
});
