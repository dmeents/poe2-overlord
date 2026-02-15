import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { CharacterListControlsForm } from './character-list-controls-form';

// Create stable mock data and functions to prevent infinite render loops
const mockLeagues = [
  { value: 'Standard', label: 'Standard' },
  { value: 'Rise of the Abyssal', label: 'Rise of the Abyssal' },
];

const mockCharacterClasses = [
  { value: 'Warrior', label: 'Warrior' },
  { value: 'Sorceress', label: 'Sorceress' },
];

const mockAscendencies = [
  { value: 'Titan', label: 'Titan' },
  { value: 'Warbringer', label: 'Warbringer' },
];

const mockGetAscendenciesForClass = vi.fn(() => mockAscendencies);

// Mock the useCharacterConfig hook with stable references
vi.mock('../../../hooks/useCharacterConfig', () => ({
  useCharacterConfig: () => ({
    leagues: mockLeagues,
    characterClasses: mockCharacterClasses,
    getAscendenciesForClass: mockGetAscendenciesForClass,
  }),
}));

describe('CharacterListControlsForm', () => {
  // Create stable mock functions once to prevent infinite render loops
  const mockFilterChange = vi.fn();
  const mockClearFilters = vi.fn();
  const mockSortChange = vi.fn();
  const mockResetSort = vi.fn();

  const defaultProps = {
    filters: {
      league: 'All' as const,
      hardcore: null,
      soloSelfFound: null,
      classes: [],
      ascendencies: [],
      nameSearch: '',
    },
    onFilterChange: mockFilterChange,
    onClearFilters: mockClearFilters,
    hasActiveFilters: false,
    sort: {
      field: 'last_played' as const,
      direction: 'desc' as const,
    },
    onSortChange: mockSortChange,
    onResetSort: mockResetSort,
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Static Rendering', () => {
    it('renders all core elements correctly', () => {
      render(<CharacterListControlsForm {...defaultProps} />);

      expect(screen.getByPlaceholderText(/Enter character name/i)).toBeInTheDocument();
      expect(screen.getByText(/All Filters/i)).toBeInTheDocument();
      expect(screen.getByText(/Sort/i)).toBeInTheDocument();
      expect(screen.getByRole('button', { name: /Reset All/i })).toBeInTheDocument();
    });

    it('renders Clear All button only when hasActiveFilters is true', async () => {
      const user = userEvent.setup();
      const { rerender } = render(<CharacterListControlsForm {...defaultProps} />);

      // Expand filters - Clear All button should not be present
      await user.click(screen.getByText(/All Filters/i));
      expect(screen.queryByText(/Clear All Filters/i)).not.toBeInTheDocument();

      // Close filters before rerender
      await user.click(screen.getByText(/All Filters/i));

      // Rerender with active filters
      rerender(<CharacterListControlsForm {...defaultProps} hasActiveFilters={true} />);

      // Expand filters - now Clear All button should be visible
      await user.click(screen.getByText(/Filters Active/i));
      expect(screen.getByText(/Clear All Filters/i)).toBeInTheDocument();
    });
  });

  describe('Interactions', () => {
    it('expands and collapses filters when toggle is clicked', async () => {
      const user = userEvent.setup();
      render(<CharacterListControlsForm {...defaultProps} />);

      const toggleButton = screen.getByText(/All Filters/i);
      expect(toggleButton).toBeInTheDocument();

      // Filter controls should not be visible initially
      expect(screen.queryByText('League')).not.toBeInTheDocument();

      await user.click(toggleButton);

      // After clicking, filter labels should be visible
      expect(screen.getByText('League')).toBeInTheDocument();
      expect(screen.getByText('Hardcore')).toBeInTheDocument();
      expect(screen.getByText('Character Class')).toBeInTheDocument();
    });

    it('calls onClearFilters when Clear All button is clicked', async () => {
      const user = userEvent.setup();
      render(<CharacterListControlsForm {...defaultProps} hasActiveFilters={true} />);

      // Need to expand filters first to see the Clear All Filters button
      const toggleButton = screen.getByText(/Filters Active/i);
      await user.click(toggleButton);

      await user.click(screen.getByText(/Clear All Filters/i));

      expect(mockClearFilters).toHaveBeenCalledTimes(1);
    });

    it('calls onResetSort and onClearFilters when Reset All is clicked', async () => {
      const user = userEvent.setup();
      render(<CharacterListControlsForm {...defaultProps} />);

      await user.click(screen.getByRole('button', { name: /Reset All/i }));

      expect(mockResetSort).toHaveBeenCalledTimes(1);
      expect(mockClearFilters).toHaveBeenCalledTimes(1);
    });
  });
});
