import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi, beforeEach } from 'vitest';
import { TopItemsCard } from './top-items-card';
import type { TopCurrencyItem } from '@/types/economy';

const mockUseEconomy = vi.hoisted(() => vi.fn());

vi.mock('@/contexts/EconomyContext', () => ({
  useEconomy: mockUseEconomy,
}));

const createMockTopCurrency = (overrides: Partial<TopCurrencyItem> = {}): TopCurrencyItem => ({
  id: 'divine',
  name: 'Divine Orb',
  economy_type: 'currency',
  image_url: 'https://example.com/divine.png',
  primary_value: 100,
  primary_currency_name: 'Chaos',
  primary_currency_image_url: 'https://example.com/chaos.png',
  volume: 5000,
  change_percent: 5.5,
  ...overrides,
});

describe('TopItemsCard', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Rendering', () => {
    it('renders card with title', () => {
      mockUseEconomy.mockReturnValue({
        aggregatedTopCurrencies: [createMockTopCurrency()],
        isLoadingAggregated: false,
      });

      render(<TopItemsCard />);

      expect(screen.getByText('Top Items')).toBeInTheDocument();
    });

    it('renders currency name', () => {
      mockUseEconomy.mockReturnValue({
        aggregatedTopCurrencies: [createMockTopCurrency()],
        isLoadingAggregated: false,
      });

      render(<TopItemsCard />);

      expect(screen.getByText('Divine Orb')).toBeInTheDocument();
    });

    it('renders primary value', () => {
      mockUseEconomy.mockReturnValue({
        aggregatedTopCurrencies: [createMockTopCurrency({ primary_value: 100 })],
        isLoadingAggregated: false,
      });

      render(<TopItemsCard />);

      expect(screen.getByText('100')).toBeInTheDocument();
    });

    it('renders volume per hour when available', () => {
      mockUseEconomy.mockReturnValue({
        aggregatedTopCurrencies: [createMockTopCurrency({ volume: 5000, primary_value: 100 })],
        isLoadingAggregated: false,
      });

      render(<TopItemsCard />);

      // 5000 / 100 = 50.00 / hr
      expect(screen.getByText('50.00 / hr')).toBeInTheDocument();
    });

    it('renders change percent with correct color for positive', () => {
      mockUseEconomy.mockReturnValue({
        aggregatedTopCurrencies: [createMockTopCurrency({ change_percent: 5.5 })],
        isLoadingAggregated: false,
      });

      render(<TopItemsCard />);

      const changeElement = screen.getByText('+5.50%');
      expect(changeElement).toBeInTheDocument();
      expect(changeElement.className).toContain('text-emerald-400');
    });

    it('renders change percent with correct color for negative', () => {
      mockUseEconomy.mockReturnValue({
        aggregatedTopCurrencies: [createMockTopCurrency({ change_percent: -3.25 })],
        isLoadingAggregated: false,
      });

      render(<TopItemsCard />);

      const changeElement = screen.getByText('-3.25%');
      expect(changeElement).toBeInTheDocument();
      expect(changeElement.className).toContain('text-red-400');
    });

    it('renders multiple items', () => {
      mockUseEconomy.mockReturnValue({
        aggregatedTopCurrencies: [
          createMockTopCurrency({ id: 'divine', name: 'Divine Orb' }),
          createMockTopCurrency({ id: 'exalted', name: 'Exalted Orb' }),
        ],
        isLoadingAggregated: false,
      });

      render(<TopItemsCard />);

      expect(screen.getByText('Divine Orb')).toBeInTheDocument();
      expect(screen.getByText('Exalted Orb')).toBeInTheDocument();
    });
  });

  describe('Loading State', () => {
    it('shows loading spinner when loading', () => {
      mockUseEconomy.mockReturnValue({
        aggregatedTopCurrencies: [],
        isLoadingAggregated: true,
      });

      render(<TopItemsCard />);

      expect(screen.queryByText('Divine Orb')).not.toBeInTheDocument();
    });
  });

  describe('Empty State', () => {
    it('shows empty message when no data', () => {
      mockUseEconomy.mockReturnValue({
        aggregatedTopCurrencies: [],
        isLoadingAggregated: false,
      });

      render(<TopItemsCard />);

      expect(screen.getByText(/No aggregated data yet/)).toBeInTheDocument();
    });
  });

  describe('Currency Images', () => {
    it('renders currency images with correct alt text', () => {
      mockUseEconomy.mockReturnValue({
        aggregatedTopCurrencies: [createMockTopCurrency()],
        isLoadingAggregated: false,
      });

      render(<TopItemsCard />);

      expect(screen.getByAltText('Divine Orb')).toBeInTheDocument();
      expect(screen.getByAltText('Chaos')).toBeInTheDocument();
    });
  });

  describe('Optional Fields', () => {
    it('does not render volume when null', () => {
      mockUseEconomy.mockReturnValue({
        aggregatedTopCurrencies: [createMockTopCurrency({ volume: null })],
        isLoadingAggregated: false,
      });

      render(<TopItemsCard />);

      expect(screen.queryByText(/\/ hr/)).not.toBeInTheDocument();
    });

    it('does not render change percent when null', () => {
      mockUseEconomy.mockReturnValue({
        aggregatedTopCurrencies: [createMockTopCurrency({ change_percent: null })],
        isLoadingAggregated: false,
      });

      render(<TopItemsCard />);

      expect(screen.queryByText(/%/)).not.toBeInTheDocument();
    });
  });
});
