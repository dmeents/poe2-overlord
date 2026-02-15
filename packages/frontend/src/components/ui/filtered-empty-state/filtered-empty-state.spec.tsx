import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { FilteredEmptyState } from './filtered-empty-state';

describe('FilteredEmptyState', () => {
  const defaultProps = {
    itemType: 'characters',
    onClearFilters: vi.fn(),
  };

  describe('Static Rendering', () => {
    it('renders filtered empty state information correctly', () => {
      const { container } = render(<FilteredEmptyState {...defaultProps} />);

      // Message with itemType
      expect(screen.getByText('No characters found')).toBeInTheDocument();

      // Search icon
      const icon = container.querySelector('svg');
      expect(icon).toBeInTheDocument();

      // Clear All Filters button
      expect(screen.getByRole('button', { name: 'Clear All Filters' })).toBeInTheDocument();

      // Helper text
      expect(screen.getByText(/Try adjusting your filters or search terms/)).toBeInTheDocument();
    });
  });

  it('displays the correct message with itemType', () => {
    render(<FilteredEmptyState {...defaultProps} itemType="zones" />);

    expect(screen.getByText('No zones found')).toBeInTheDocument();
    expect(
      screen.getByText(/No zones match your current search and filter criteria/),
    ).toBeInTheDocument();
  });

  it('calls onClearFilters when button is clicked', async () => {
    const user = userEvent.setup();
    const handleClearFilters = vi.fn();

    render(<FilteredEmptyState {...defaultProps} onClearFilters={handleClearFilters} />);

    await user.click(screen.getByRole('button', { name: 'Clear All Filters' }));

    expect(handleClearFilters).toHaveBeenCalledTimes(1);
  });
});
