import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { FilteredEmptyState } from './filtered-empty-state';

describe('FilteredEmptyState', () => {
  const defaultProps = {
    itemType: 'characters',
    onClearFilters: vi.fn(),
  };

  it('renders the component', () => {
    render(<FilteredEmptyState {...defaultProps} />);

    expect(screen.getByText('No characters found')).toBeInTheDocument();
  });

  it('displays the correct message with itemType', () => {
    render(<FilteredEmptyState {...defaultProps} itemType="zones" />);

    expect(screen.getByText('No zones found')).toBeInTheDocument();
    expect(
      screen.getByText(/No zones match your current search and filter criteria/),
    ).toBeInTheDocument();
  });

  it('renders the search icon', () => {
    const { container } = render(<FilteredEmptyState {...defaultProps} />);

    const icon = container.querySelector('svg');
    expect(icon).toBeInTheDocument();
  });

  it('renders the Clear All Filters button', () => {
    render(<FilteredEmptyState {...defaultProps} />);

    expect(screen.getByRole('button', { name: 'Clear All Filters' })).toBeInTheDocument();
  });

  it('calls onClearFilters when button is clicked', async () => {
    const user = userEvent.setup();
    const handleClearFilters = vi.fn();

    render(<FilteredEmptyState {...defaultProps} onClearFilters={handleClearFilters} />);

    await user.click(screen.getByRole('button', { name: 'Clear All Filters' }));

    expect(handleClearFilters).toHaveBeenCalledTimes(1);
  });

  it('displays helper text', () => {
    render(<FilteredEmptyState {...defaultProps} />);

    expect(
      screen.getByText(/Try adjusting your filters or search terms/),
    ).toBeInTheDocument();
  });
});
