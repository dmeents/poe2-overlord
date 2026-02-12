import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { ZoneFilters, ZoneSortOption } from '@/hooks/useZoneList';
import type { ZoneStats } from '@/types/character';
import { ZoneList } from './zone-list';

// Mock child components
vi.mock('../zone-card/zone-card', () => ({
  ZoneCard: vi.fn(({ zone, isEven }) => (
    <div data-testid={`zone-card-${zone.zone_name}`} data-even={isEven}>
      {zone.zone_name}
    </div>
  )),
}));

vi.mock('../zone-list-controls-form/zone-list-controls-form', () => ({
  ZoneListControlsForm: vi.fn(({ onClearFilters }) => (
    <div data-testid="zone-list-controls">
      <button type="button" onClick={onClearFilters} data-testid="clear-filters-controls">
        Clear Filters
      </button>
    </div>
  )),
}));

vi.mock('@/contexts/ZoneContext', () => ({
  useZone: () => ({
    openZone: vi.fn(),
  }),
}));

const createMockZone = (overrides: Partial<ZoneStats> = {}): ZoneStats => ({
  zone_name: 'The Coast',
  act: 1,
  area_level: 2,
  is_town: false,
  has_waypoint: true,
  visits: 5,
  duration: 3600,
  deaths: 2,
  first_visited: '2024-01-01T00:00:00Z',
  last_visited: '2024-01-10T00:00:00Z',
  is_active: false,
  bosses: [],
  monsters: [],
  npcs: [],
  connected_zones: [],
  points_of_interest: [],
  ...overrides,
});

const defaultFilters: ZoneFilters = {
  search: '',
  act: 'All',
  isTown: null,
  isActive: null,
  minVisits: null,
  maxVisits: null,
  minDeaths: null,
  maxDeaths: null,
  hasBosses: null,
  hasWaypoint: null,
  hasNpcs: null,
};

const defaultSort: ZoneSortOption = {
  field: 'last_visited',
  direction: 'desc',
};

describe('ZoneList', () => {
  const defaultProps = {
    zones: [createMockZone()],
    filters: defaultFilters,
    onFilterChange: vi.fn(),
    onClearFilters: vi.fn(),
    hasActiveFilters: false,
    sort: defaultSort,
    onSortChange: vi.fn(),
    onResetSort: vi.fn(),
    totalCount: 1,
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Empty State', () => {
    it('shows empty state when totalCount is 0', () => {
      render(<ZoneList {...defaultProps} zones={[]} totalCount={0} />);

      expect(screen.getByText('No Zone Data Available')).toBeInTheDocument();
      expect(screen.getByText(/Start playing Path of Exile 2/)).toBeInTheDocument();
    });
  });

  describe('Zone Rendering', () => {
    it('renders zone cards for each zone', () => {
      const zones = [
        createMockZone({ zone_name: 'The Coast' }),
        createMockZone({ zone_name: 'The Mud Flats' }),
        createMockZone({ zone_name: 'The Submerged Passage' }),
      ];

      render(<ZoneList {...defaultProps} zones={zones} totalCount={3} />);

      expect(screen.getByTestId('zone-card-The Coast')).toBeInTheDocument();
      expect(screen.getByTestId('zone-card-The Mud Flats')).toBeInTheDocument();
      expect(screen.getByTestId('zone-card-The Submerged Passage')).toBeInTheDocument();
    });

    it('passes correct isEven prop to zone cards', () => {
      const zones = [
        createMockZone({ zone_name: 'Zone1' }),
        createMockZone({ zone_name: 'Zone2' }),
        createMockZone({ zone_name: 'Zone3' }),
      ];

      render(<ZoneList {...defaultProps} zones={zones} totalCount={3} />);

      expect(screen.getByTestId('zone-card-Zone1')).toHaveAttribute('data-even', 'true');
      expect(screen.getByTestId('zone-card-Zone2')).toHaveAttribute('data-even', 'false');
      expect(screen.getByTestId('zone-card-Zone3')).toHaveAttribute('data-even', 'true');
    });

    it('renders the controls form', () => {
      render(<ZoneList {...defaultProps} />);

      expect(screen.getByTestId('zone-list-controls')).toBeInTheDocument();
    });

    it('renders column headers', () => {
      render(<ZoneList {...defaultProps} />);

      expect(screen.getByText('Zone')).toBeInTheDocument();
      expect(screen.getByText('Act')).toBeInTheDocument();
      expect(screen.getByText('Level')).toBeInTheDocument();
      expect(screen.getByText('Visits')).toBeInTheDocument();
      expect(screen.getByText('Deaths')).toBeInTheDocument();
      expect(screen.getByText('Duration')).toBeInTheDocument();
    });
  });

  describe('Filtered Empty State', () => {
    it('shows no results message when zones array is empty but totalCount > 0', () => {
      render(<ZoneList {...defaultProps} zones={[]} totalCount={5} hasActiveFilters={true} />);

      expect(screen.getByText('No zones found')).toBeInTheDocument();
      expect(screen.getByText(/No zones match your current search/)).toBeInTheDocument();
    });

    it('shows clear filters button in filtered empty state', () => {
      render(<ZoneList {...defaultProps} zones={[]} totalCount={5} hasActiveFilters={true} />);

      expect(screen.getByText('Clear All Filters')).toBeInTheDocument();
    });

    it('calls onClearFilters when clear filters button is clicked', async () => {
      const user = userEvent.setup();
      const handleClearFilters = vi.fn();

      render(
        <ZoneList
          {...defaultProps}
          zones={[]}
          totalCount={5}
          hasActiveFilters={true}
          onClearFilters={handleClearFilters}
        />,
      );

      await user.click(screen.getByText('Clear All Filters'));

      expect(handleClearFilters).toHaveBeenCalledTimes(1);
    });
  });

  describe('Controls Integration', () => {
    it('calls onClearFilters from controls form', async () => {
      const user = userEvent.setup();
      const handleClearFilters = vi.fn();

      render(<ZoneList {...defaultProps} onClearFilters={handleClearFilters} />);

      await user.click(screen.getByTestId('clear-filters-controls'));

      expect(handleClearFilters).toHaveBeenCalledTimes(1);
    });
  });
});
