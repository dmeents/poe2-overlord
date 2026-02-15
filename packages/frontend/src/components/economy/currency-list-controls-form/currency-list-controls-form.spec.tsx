import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { CurrencyListControlsForm } from './currency-list-controls-form';

// Mock the EconomyContext
vi.mock('@/contexts/EconomyContext', () => ({
  useEconomy: () => ({
    selectedEconomyType: 'Currency',
    setSelectedEconomyType: vi.fn(),
  }),
}));

describe('CurrencyListControlsForm', () => {
  const defaultProps = {
    searchQuery: '',
    onSearchChange: vi.fn(),
    isSearching: false,
    sort: {
      field: 'primary_value' as const,
      direction: 'desc' as const,
    },
    onSortChange: vi.fn(),
    onResetSort: vi.fn(),
    currencyCount: 50,
    totalCount: 100,
  };

  it('renders economy type buttons', () => {
    render(<CurrencyListControlsForm {...defaultProps} />);

    expect(screen.getByText(/Currency/i)).toBeInTheDocument();
  });

  it('renders search input', () => {
    render(<CurrencyListControlsForm {...defaultProps} />);

    expect(screen.getByPlaceholderText(/Search all currencies/i)).toBeInTheDocument();
  });

  it('calls onSearchChange when typing in search', async () => {
    const user = userEvent.setup();
    const handleSearchChange = vi.fn();

    render(<CurrencyListControlsForm {...defaultProps} onSearchChange={handleSearchChange} />);

    const input = screen.getByPlaceholderText(/Search all currencies/i);
    await user.type(input, 't');

    expect(handleSearchChange).toHaveBeenCalledWith('t');
  });

  it('renders clear search button when searchQuery is not empty', () => {
    render(<CurrencyListControlsForm {...defaultProps} searchQuery="test" />);

    const clearButton = screen.getByTitle('Clear search');
    expect(clearButton).toBeInTheDocument();
  });

  it('does not render clear search button when searchQuery is empty', () => {
    render(<CurrencyListControlsForm {...defaultProps} searchQuery="" />);

    expect(screen.queryByTitle('Clear search')).not.toBeInTheDocument();
  });

  it('calls onSearchChange with empty string when clear button is clicked', async () => {
    const user = userEvent.setup();
    const handleSearchChange = vi.fn();

    render(
      <CurrencyListControlsForm
        {...defaultProps}
        searchQuery="test"
        onSearchChange={handleSearchChange}
      />,
    );

    await user.click(screen.getByTitle('Clear search'));

    expect(handleSearchChange).toHaveBeenCalledWith('');
  });

  it('displays "Showing all X items" when counts match', () => {
    render(<CurrencyListControlsForm {...defaultProps} currencyCount={100} totalCount={100} />);

    expect(screen.getByText(/Showing all 100 items/)).toBeInTheDocument();
  });

  it('displays "Showing X of Y items" when counts differ', () => {
    render(<CurrencyListControlsForm {...defaultProps} currencyCount={50} totalCount={100} />);

    expect(screen.getByText(/Showing 50 of 100 items/)).toBeInTheDocument();
  });

  it('displays "Found X results" when searching', () => {
    render(
      <CurrencyListControlsForm
        {...defaultProps}
        searchQuery="test"
        isSearching={false}
        currencyCount={25}
      />,
    );

    expect(screen.getByText(/Found 25 results across all types/)).toBeInTheDocument();
  });

  it('displays "Found X result" (singular) when only one result', () => {
    render(
      <CurrencyListControlsForm
        {...defaultProps}
        searchQuery="test"
        isSearching={false}
        currencyCount={1}
      />,
    );

    expect(screen.getByText(/Found 1 result across all types/)).toBeInTheDocument();
  });

  it('displays "Searching..." when isSearching is true', () => {
    render(<CurrencyListControlsForm {...defaultProps} isSearching={true} />);

    expect(screen.getByText(/Searching\.\.\./)).toBeInTheDocument();
  });
});
