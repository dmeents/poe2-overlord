import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { CurrencyExchangeRate } from '@/types/economy';
import { EconomyRow } from './economy-row';

// ItemTooltip uses useItemByName which requires a QueryClient.
// Mock it here since EconomyRow tests aren't testing the item data query.
vi.mock('@/queries/item-data', () => ({
  useItemByName: vi.fn(() => ({ data: null, isLoading: false })),
}));

const mockCurrencyData = {
  primary_currency: {
    id: 'divine',
    name: 'Divine Orb',
    image_url: '/divine.png',
  },
  secondary_currency: {
    id: 'chaos',
    name: 'Chaos Orb',
    image_url: '/chaos.png',
  },
  tertiary_currency: {
    id: 'exalted',
    name: 'Exalted Orb',
    image_url: '/exalted.png',
  },
  secondary_rate: 150,
  tertiary_rate: 10,
  currencies: [],
  fetched_at: '2024-01-01T00:00:00Z',
};

const mockUseEconomy = vi.hoisted(() =>
  vi.fn(() => ({
    currencyData: mockCurrencyData,
  })),
);

vi.mock('@/contexts/EconomyContext', () => ({
  useEconomy: mockUseEconomy,
}));

const mockCurrency: CurrencyExchangeRate = {
  id: 'test-currency',
  name: 'Test Currency',
  image_url: '/test-currency.png',
  display_value: {
    tier: 'Primary',
    value: 10.5,
    inverted: false,
    currency_id: 'divine',
    currency_name: 'Divine Orb',
    currency_image_url: '/divine.png',
  },
  primary_value: 10.5,
  secondary_value: 1575,
  tertiary_value: 105,
  volume: 1000,
  change_percent: 5.25,
  price_history: [10, 10.2, 10.5],
};

describe('EconomyRow', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockUseEconomy.mockReturnValue({
      currencyData: mockCurrencyData,
    });
  });

  describe('Static Rendering', () => {
    it('renders economy row information correctly', () => {
      const { container } = render(<EconomyRow currency={mockCurrency} />);

      // Currency name
      expect(screen.getByText('Test Currency')).toBeInTheDocument();

      // Currency image
      const images = screen.getAllByAltText('Test Currency');
      expect(images.length).toBeGreaterThan(0);

      // Formatted display value
      expect(screen.getByText('10.5')).toBeInTheDocument();

      // Change percent with positive indicator
      expect(screen.getByText('+5.25%')).toBeInTheDocument();

      // Items sold per hour (volume / primary_value = 1000 / 10.5 = 95.24)
      expect(screen.getByText('95.24 / hr')).toBeInTheDocument();

      // No cursor-pointer when onClick not provided
      const row = container.firstChild as HTMLElement;
      expect(row.className).not.toContain('cursor-pointer');
    });
  });

  it('renders change percent with negative indicator', () => {
    const negativeCurrency = {
      ...mockCurrency,
      change_percent: -3.5,
    };

    render(<EconomyRow currency={negativeCurrency} />);

    expect(screen.getByText('-3.50%')).toBeInTheDocument();
  });

  it('calls onClick when row is clicked', async () => {
    const user = userEvent.setup();
    const handleClick = vi.fn();

    render(<EconomyRow currency={mockCurrency} onClick={handleClick} />);

    await user.click(screen.getByText('Test Currency'));

    expect(handleClick).toHaveBeenCalledWith(mockCurrency);
  });

  it('renders cursor-pointer when onClick is provided', () => {
    const { container } = render(<EconomyRow currency={mockCurrency} onClick={vi.fn()} />);

    const row = container.firstChild as HTMLElement;
    expect(row.className).toContain('cursor-pointer');
  });

  it('does not render items sold per hour when volume is null', () => {
    const noCurrencyData = {
      ...mockCurrency,
      volume: null,
    };

    render(<EconomyRow currency={noCurrencyData} />);

    expect(screen.queryByText(/\/ hr/)).not.toBeInTheDocument();
  });

  it('renders inverted display value correctly', () => {
    const invertedCurrency: CurrencyExchangeRate = {
      ...mockCurrency,
      display_value: {
        ...mockCurrency.display_value,
        inverted: true,
        value: 2.5,
      },
    };

    render(<EconomyRow currency={invertedCurrency} />);

    expect(screen.getByText('2.50')).toBeInTheDocument();
    expect(screen.getByText('1')).toBeInTheDocument();
  });

  it('does not render change percent when null', () => {
    const noChangeCurrency = {
      ...mockCurrency,
      change_percent: null,
    };

    render(<EconomyRow currency={noChangeCurrency} />);

    expect(screen.queryByText(/%/)).not.toBeInTheDocument();
  });

  it('formats large values with one decimal place', () => {
    const largeCurrency: CurrencyExchangeRate = {
      ...mockCurrency,
      display_value: {
        ...mockCurrency.display_value,
        value: 150.75,
      },
    };

    render(<EconomyRow currency={largeCurrency} />);

    expect(screen.getByText('150.8')).toBeInTheDocument();
  });

  it('formats small values with two decimal places', () => {
    const smallCurrency: CurrencyExchangeRate = {
      ...mockCurrency,
      display_value: {
        ...mockCurrency.display_value,
        value: 5.123,
      },
    };

    render(<EconomyRow currency={smallCurrency} />);

    expect(screen.getByText('5.12')).toBeInTheDocument();
  });
});
