import { render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { CurrencyExchangeRate } from '@/types/economy';
import { EconomyList } from './economy-list';

// Mock child components
vi.mock('../economy-row/economy-row', () => ({
  EconomyRow: vi.fn(({ currency }) => (
    <div data-testid={`economy-row-${currency.id}`}>{currency.name}</div>
  )),
}));

vi.mock('../currency-list-controls-form/currency-list-controls-form', () => ({
  CurrencyListControlsForm: vi.fn(() => <div data-testid="currency-list-controls">Controls</div>),
}));

vi.mock('@/components/ui/loading-spinner/loading-spinner', () => ({
  LoadingSpinner: vi.fn(() => <div data-testid="loading-spinner">Loading...</div>),
}));

// Mock the useCurrencyList hook
const mockUseCurrencyList = vi.hoisted(() => vi.fn());

vi.mock('@/hooks/useCurrencyList', () => ({
  useCurrencyList: mockUseCurrencyList,
}));

const createMockCurrency = (
  overrides: Partial<CurrencyExchangeRate> = {},
): CurrencyExchangeRate => ({
  id: 'chaos',
  name: 'Chaos Orb',
  image_url: 'https://example.com/chaos.png',
  display_value: {
    tier: 'Primary',
    value: 1,
    inverted: false,
    currency_id: 'divine',
    currency_name: 'Divine Orb',
    currency_image_url: '/divine.png',
  },
  primary_value: 0.01,
  secondary_value: 1,
  tertiary_value: 0.1,
  volume: 10000,
  change_percent: 0,
  price_history: [1, 1, 1],
  ...overrides,
});

describe('EconomyList', () => {
  const defaultProps = {
    currencies: [createMockCurrency()],
    onCurrencyClick: vi.fn(),
    searchQuery: '',
    onSearchChange: vi.fn(),
    isSearching: false,
    searchResultsCount: 1,
  };

  beforeEach(() => {
    vi.clearAllMocks();
    mockUseCurrencyList.mockReturnValue({
      sort: { field: 'chaos_equivalent', direction: 'desc' },
      updateSort: vi.fn(),
      resetSort: vi.fn(),
      sortedCurrencies: [createMockCurrency()],
      currencyCount: 1,
      totalCount: 1,
    });
  });

  describe('Rendering', () => {
    it('renders controls form', () => {
      render(<EconomyList {...defaultProps} />);

      expect(screen.getByTestId('currency-list-controls')).toBeInTheDocument();
    });

    it('renders currency rows for each currency', () => {
      const currencies = [
        createMockCurrency({ id: 'chaos', name: 'Chaos Orb' }),
        createMockCurrency({ id: 'divine', name: 'Divine Orb' }),
      ];

      mockUseCurrencyList.mockReturnValue({
        sort: { field: 'chaos_equivalent', direction: 'desc' },
        updateSort: vi.fn(),
        resetSort: vi.fn(),
        sortedCurrencies: currencies,
        currencyCount: 2,
        totalCount: 2,
      });

      render(<EconomyList {...defaultProps} currencies={currencies} />);

      expect(screen.getByTestId('economy-row-chaos')).toBeInTheDocument();
      expect(screen.getByTestId('economy-row-divine')).toBeInTheDocument();
    });
  });

  describe('Loading State', () => {
    it('shows loading spinner when isSearching is true', () => {
      render(<EconomyList {...defaultProps} isSearching={true} />);

      expect(screen.getByTestId('loading-spinner')).toBeInTheDocument();
    });

    it('does not show currency rows when loading', () => {
      render(<EconomyList {...defaultProps} isSearching={true} />);

      expect(screen.queryByTestId('economy-row-chaos')).not.toBeInTheDocument();
    });
  });

  describe('Empty States', () => {
    it('shows search empty message when no results and searchQuery exists', () => {
      mockUseCurrencyList.mockReturnValue({
        sort: { field: 'chaos_equivalent', direction: 'desc' },
        updateSort: vi.fn(),
        resetSort: vi.fn(),
        sortedCurrencies: [],
        currencyCount: 0,
        totalCount: 10,
      });

      render(<EconomyList {...defaultProps} searchQuery="nonexistent" />);

      expect(screen.getByText(/No currencies found matching "nonexistent"/)).toBeInTheDocument();
    });

    it('shows no data message when no currencies at all', () => {
      mockUseCurrencyList.mockReturnValue({
        sort: { field: 'chaos_equivalent', direction: 'desc' },
        updateSort: vi.fn(),
        resetSort: vi.fn(),
        sortedCurrencies: [],
        currencyCount: 0,
        totalCount: 0,
      });

      render(<EconomyList {...defaultProps} currencies={[]} searchQuery="" />);

      expect(screen.getByText('No currency data available')).toBeInTheDocument();
    });
  });
});
