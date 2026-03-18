import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { ZoneDetailsModal } from './zone-details-modal';

const mockCloseModal = vi.hoisted(() => vi.fn());
const mockOpenZone = vi.hoisted(() => vi.fn());
const mockOpen = vi.hoisted(() => vi.fn());

const createMockZone = (overrides = {}) => ({
  zone_name: 'The Coast',
  act: 1,
  area_level: 2,
  is_town: false,
  has_waypoint: true,
  visits: 5,
  duration: 3600,
  deaths: 2,
  first_visited: '2024-01-01T00:00:00Z',
  last_visited: new Date().toISOString(),
  wiki_url: 'https://wiki.example.com/the-coast',
  bosses: ['Boss One', 'Boss Two'],
  npcs: ['NPC One'],
  points_of_interest: ['POI One'],
  connected_zones: ['The Mud Flats', "Lioneye's Watch"],
  ...overrides,
});

const mockUseZone = vi.hoisted(() => vi.fn());

vi.mock('@/contexts/ZoneContext', () => ({
  useZone: mockUseZone,
}));

vi.mock('@tauri-apps/plugin-shell', () => ({
  open: mockOpen,
}));

describe('ZoneDetailsModal', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockUseZone.mockReturnValue({
      selectedZone: createMockZone(),
      isModalOpen: true,
      closeModal: mockCloseModal,
      openZone: mockOpenZone,
    });
  });

  describe('Zone Information Display', () => {
    it('renders all zone information for a visited zone', () => {
      render(<ZoneDetailsModal />);

      // Zone name
      expect(screen.getByText('The Coast')).toBeInTheDocument();

      // Zone information section
      expect(screen.getByText('Zone Information')).toBeInTheDocument();
      expect(screen.getByText('Type:')).toBeInTheDocument();
      expect(screen.getByText('Zone')).toBeInTheDocument();
      expect(screen.getByText('Act:')).toBeInTheDocument();
      expect(screen.getByText('Act 1')).toBeInTheDocument();
      expect(screen.getByText('Level:')).toBeInTheDocument();
      expect(screen.getByText('Waypoint:')).toBeInTheDocument();
      expect(screen.getByText('Yes')).toBeInTheDocument();
    });

    it('displays town type for town zones', () => {
      mockUseZone.mockReturnValue({
        selectedZone: createMockZone({ is_town: true }),
        isModalOpen: true,
        closeModal: mockCloseModal,
        openZone: mockOpenZone,
      });

      render(<ZoneDetailsModal />);

      expect(screen.getByText('Town')).toBeInTheDocument();
    });

    it('returns null when no zone is selected', () => {
      mockUseZone.mockReturnValue({
        selectedZone: null,
        isModalOpen: true,
        closeModal: mockCloseModal,
        openZone: mockOpenZone,
      });

      const { container } = render(<ZoneDetailsModal />);

      expect(container.firstChild).toBeNull();
    });
  });

  describe('Player Statistics', () => {
    it('displays player statistics for visited zones with deaths for non-town zones', () => {
      render(<ZoneDetailsModal />);

      expect(screen.getByText('Your Statistics')).toBeInTheDocument();
      expect(screen.getByText('Time Spent:')).toBeInTheDocument();
      expect(screen.getByText('Visits:')).toBeInTheDocument();
      expect(screen.getByText('5')).toBeInTheDocument();
      expect(screen.getByText('Deaths:')).toBeInTheDocument();
    });

    it('does not display deaths for town zones', () => {
      mockUseZone.mockReturnValue({
        selectedZone: createMockZone({ is_town: true }),
        isModalOpen: true,
        closeModal: mockCloseModal,
        openZone: mockOpenZone,
      });

      render(<ZoneDetailsModal />);

      expect(screen.queryByText('Deaths:')).not.toBeInTheDocument();
    });
  });

  describe('Unvisited Zone', () => {
    it('shows unvisited message and hides statistics for zones with 0 visits', () => {
      mockUseZone.mockReturnValue({
        selectedZone: createMockZone({ visits: 0 }),
        isModalOpen: true,
        closeModal: mockCloseModal,
        openZone: mockOpenZone,
      });

      render(<ZoneDetailsModal />);

      expect(screen.getByText('Zone Not Yet Visited')).toBeInTheDocument();
      expect(screen.getByText(/You haven't visited this zone yet/)).toBeInTheDocument();
      expect(screen.queryByText('Your Statistics')).not.toBeInTheDocument();
    });
  });

  describe('Zone Features', () => {
    it('displays all zone features when present', () => {
      render(<ZoneDetailsModal />);

      // Bosses
      expect(screen.getByText('Bosses (2)')).toBeInTheDocument();
      expect(screen.getByText('Boss One')).toBeInTheDocument();
      expect(screen.getByText('Boss Two')).toBeInTheDocument();

      // NPCs
      expect(screen.getByText('NPCs (1)')).toBeInTheDocument();
      expect(screen.getByText('NPC One')).toBeInTheDocument();

      // Points of Interest
      expect(screen.getByText('Points of Interest (1)')).toBeInTheDocument();
      expect(screen.getByText('POI One')).toBeInTheDocument();

      // Connected Zones
      expect(screen.getByText('Connected Zones (2)')).toBeInTheDocument();
      expect(screen.getByText('The Mud Flats')).toBeInTheDocument();
      expect(screen.getByText("Lioneye's Watch")).toBeInTheDocument();
    });

    it('hides bosses section when no bosses present', () => {
      mockUseZone.mockReturnValue({
        selectedZone: createMockZone({ bosses: [] }),
        isModalOpen: true,
        closeModal: mockCloseModal,
        openZone: mockOpenZone,
      });

      render(<ZoneDetailsModal />);

      expect(screen.queryByText(/Bosses/)).not.toBeInTheDocument();
    });
  });

  describe('Connected Zones Navigation', () => {
    it('calls openZone when a connected zone is clicked', async () => {
      const user = userEvent.setup();

      render(<ZoneDetailsModal />);

      await user.click(screen.getByText('The Mud Flats'));

      expect(mockOpenZone).toHaveBeenCalledWith('The Mud Flats');
    });
  });

  describe('Wiki Link', () => {
    it('displays wiki link button and opens wiki when clicked', async () => {
      const user = userEvent.setup();
      mockOpen.mockResolvedValueOnce(undefined);

      render(<ZoneDetailsModal />);

      expect(screen.getByText('View on Wiki')).toBeInTheDocument();

      await user.click(screen.getByText('View on Wiki'));

      expect(mockOpen).toHaveBeenCalledWith('https://wiki.example.com/the-coast');
    });

    it('handles wiki open error gracefully', async () => {
      const user = userEvent.setup();
      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
      mockOpen.mockRejectedValueOnce(new Error('Failed to open'));

      render(<ZoneDetailsModal />);

      await user.click(screen.getByText('View on Wiki'));

      expect(consoleSpy).toHaveBeenCalledWith('Failed to open wiki link:', expect.any(Error));
      consoleSpy.mockRestore();
    });
  });
});
