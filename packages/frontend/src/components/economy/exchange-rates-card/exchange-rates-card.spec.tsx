import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi, beforeEach } from 'vitest';
import { ExchangeRatesCard } from './exchange-rates-card';

const mockUseEconomy = vi.hoisted(() => vi.fn());

vi.mock('@/contexts/EconomyContext', () => ({
  useEconomy: mockUseEconomy,
}));

const createMockCurrencyData = () => ({
  primary_currency: {
    name: 'Divine Orb',
    image_url: 'https://example.com/divine.png',
  },
  secondary_currency: {
    name: 'Chaos Orb',
    image_url: 'https://example.com/chaos.png',
  },
  secondary_rate: 100.5,
  tertiary_currency: {
    name: 'Exalted Orb',
    image_url: 'https://example.com/exalted.png',
  },
  tertiary_rate: 50,
});

describe('ExchangeRatesCard', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Rendering', () => {
    it('renders card with title', () => {
      mockUseEconomy.mockReturnValue({
        currencyData: createMockCurrencyData(),
        isLoading: false,
      });

      render(<ExchangeRatesCard />);

      expect(screen.getByText('Exchange Rates')).toBeInTheDocument();
    });

    it('renders primary currency', () => {
      mockUseEconomy.mockReturnValue({
        currencyData: createMockCurrencyData(),
        isLoading: false,
      });

      render(<ExchangeRatesCard />);

      expect(screen.getByText('Divine Orb')).toBeInTheDocument();
      expect(screen.getByText('1')).toBeInTheDocument();
    });

    it('renders secondary currency with rate', () => {
      mockUseEconomy.mockReturnValue({
        currencyData: createMockCurrencyData(),
        isLoading: false,
      });

      render(<ExchangeRatesCard />);

      expect(screen.getByText('Chaos Orb')).toBeInTheDocument();
      expect(screen.getByText('100.50')).toBeInTheDocument();
    });

    it('renders tertiary currency when available', () => {
      mockUseEconomy.mockReturnValue({
        currencyData: createMockCurrencyData(),
        isLoading: false,
      });

      render(<ExchangeRatesCard />);

      expect(screen.getByText('Exalted Orb')).toBeInTheDocument();
      expect(screen.getByText('50')).toBeInTheDocument();
    });

    it('does not render tertiary currency when not available', () => {
      const dataWithoutTertiary = createMockCurrencyData();
      dataWithoutTertiary.tertiary_currency = undefined;
      dataWithoutTertiary.tertiary_rate = undefined;

      mockUseEconomy.mockReturnValue({
        currencyData: dataWithoutTertiary,
        isLoading: false,
      });

      render(<ExchangeRatesCard />);

      expect(screen.queryByText('Exalted Orb')).not.toBeInTheDocument();
    });

    it('renders exchange arrows', () => {
      mockUseEconomy.mockReturnValue({
        currencyData: createMockCurrencyData(),
        isLoading: false,
      });

      render(<ExchangeRatesCard />);

      const arrows = screen.getAllByText('↔');
      expect(arrows.length).toBeGreaterThan(0);
    });
  });

  describe('Loading State', () => {
    it('shows loading spinner when loading', () => {
      mockUseEconomy.mockReturnValue({
        currencyData: null,
        isLoading: true,
      });

      render(<ExchangeRatesCard />);

      // LoadingSpinner component should render
      expect(screen.queryByText('Divine Orb')).not.toBeInTheDocument();
    });
  });

  describe('Empty State', () => {
    it('shows no data message when currencyData is null', () => {
      mockUseEconomy.mockReturnValue({
        currencyData: null,
        isLoading: false,
      });

      render(<ExchangeRatesCard />);

      expect(screen.getByText('No exchange rate data available')).toBeInTheDocument();
    });
  });

  describe('Currency Images', () => {
    it('renders currency images with correct alt text', () => {
      mockUseEconomy.mockReturnValue({
        currencyData: createMockCurrencyData(),
        isLoading: false,
      });

      render(<ExchangeRatesCard />);

      expect(screen.getByAltText('Divine Orb')).toBeInTheDocument();
      expect(screen.getByAltText('Chaos Orb')).toBeInTheDocument();
      expect(screen.getByAltText('Exalted Orb')).toBeInTheDocument();
    });
  });
});
