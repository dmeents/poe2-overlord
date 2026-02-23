import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import type { CharacterSummaryData } from '@/types/character';
import { CharacterFormModal } from './character-form-modal';

// Define stable mock data and functions outside the mock to prevent infinite re-renders
const mockCharacterClasses = [
  { value: 'Warrior', label: 'Warrior' },
  { value: 'Sorceress', label: 'Sorceress' },
  { value: 'Monk', label: 'Monk' },
];

const mockLeagues = [
  { value: 'Standard', label: 'Standard' },
  { value: 'Rise of the Abyssal', label: 'Rise of the Abyssal' },
];

const mockGetAscendenciesForClass = (characterClass: string) => {
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
};

const mockGetDefaultFormData = () => ({
  name: '',
  class: 'Warrior',
  ascendency: 'Titan',
  league: 'Standard',
  hardcore: false,
  solo_self_found: false,
});

vi.mock('@/hooks/useCharacterConfig', () => ({
  useCharacterConfig: () => ({
    characterClasses: mockCharacterClasses,
    leagues: mockLeagues,
    getAscendenciesForClass: mockGetAscendenciesForClass,
    getDefaultFormData: mockGetDefaultFormData,
  }),
}));

const mockCharacter: CharacterSummaryData = {
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
  is_active: false,
  walkthrough_progress: {
    current_step_id: null,
    is_completed: false,
    last_updated: '2024-01-10T00:00:00Z',
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

  describe('Create Mode - Static Rendering', () => {
    it('renders all form fields and buttons correctly', () => {
      render(<CharacterFormModal {...defaultProps} />);

      // Modal title
      expect(screen.getByRole('heading', { name: 'Create Character' })).toBeInTheDocument();

      // Form fields
      expect(screen.getByText('Character Name')).toBeInTheDocument();
      expect(screen.getByPlaceholderText('Enter character name')).toBeInTheDocument();
      expect(screen.getByText('Class')).toBeInTheDocument();
      expect(screen.getByText('Ascendency')).toBeInTheDocument();
      expect(screen.getByText('League')).toBeInTheDocument();

      // Checkboxes
      expect(screen.getByLabelText('Hardcore')).toBeInTheDocument();
      expect(screen.getByLabelText('Solo Self-Found (SSF)')).toBeInTheDocument();

      // Buttons
      expect(screen.getByRole('button', { name: 'Create Character' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Cancel' })).toBeInTheDocument();

      // Name generator buttons
      expect(screen.getByTitle('Generate masculine name')).toBeInTheDocument();
      expect(screen.getByTitle('Generate feminine name')).toBeInTheDocument();
    });
  });

  describe('Edit Mode - Static Rendering', () => {
    it('renders edit modal with pre-filled data and update button', () => {
      render(<CharacterFormModal {...defaultProps} character={mockCharacter} />);

      // Modal title
      expect(screen.getByRole('heading', { name: 'Edit Character' })).toBeInTheDocument();

      // Pre-filled character name
      expect(screen.getByDisplayValue('TestCharacter')).toBeInTheDocument();

      // Update button
      expect(screen.getByRole('button', { name: 'Update Character' })).toBeInTheDocument();
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

      await user.type(screen.getByPlaceholderText('Enter character name'), 'NewCharacter');
      await user.click(screen.getByRole('button', { name: 'Create Character' }));

      expect(handleSubmit).toHaveBeenCalledWith(
        expect.objectContaining({
          name: 'NewCharacter',
          class: 'Warrior',
          ascendency: 'Titan',
          league: 'Standard',
          hardcore: false,
          solo_self_found: false,
        }),
      );
    });

    it('validates required name field', async () => {
      const user = userEvent.setup();
      const handleSubmit = vi.fn();

      render(<CharacterFormModal {...defaultProps} onSubmit={handleSubmit} />);

      await user.click(screen.getByRole('button', { name: 'Create Character' }));

      // Form submission should be prevented when name is empty
      expect(handleSubmit).not.toHaveBeenCalled();
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
    it('shows saving text and disables buttons when loading', () => {
      render(<CharacterFormModal {...defaultProps} isLoading={true} />);

      expect(screen.getByRole('button', { name: 'Saving...' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Cancel' })).toBeDisabled();
      expect(screen.getByRole('button', { name: 'Saving...' })).toBeDisabled();
    });
  });

  describe('Modal States', () => {
    it('does not render when isOpen is false', () => {
      render(<CharacterFormModal {...defaultProps} isOpen={false} />);

      expect(screen.queryByRole('heading', { name: 'Create Character' })).not.toBeInTheDocument();
    });
  });
});
