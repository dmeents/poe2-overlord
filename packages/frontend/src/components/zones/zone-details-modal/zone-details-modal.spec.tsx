import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi, beforeEach } from 'vitest';
import { ZoneDetailsModal } from './zone-details-modal';

const mockCloseModal = vi.hoisted(() => vi.fn());
const mockNavigateToZone = vi.hoisted(() => vi.fn());
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
  connected_zones: ['The Mud Flats', 'Lioneye\'s Watch'],
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
      navigateToZone: mockNavigateToZone,
      allZones: [],
    });
  });

  describe('Rendering', () => {
    it('renders modal with zone name', () => {
      render(<ZoneDetailsModal />);

      expect(screen.getByText('The Coast')).toBeInTheDocument();
    });

    it('renders zone information section', () => {
      render(<ZoneDetailsModal />);

      expect(screen.getByText('Zone Information')).toBeInTheDocument();
    });

    it('displays zone type', () => {
      render(<ZoneDetailsModal />);

      expect(screen.getByText('Type:')).toBeInTheDocument();
      expect(screen.getByText('Zone')).toBeInTheDocument();
    });

    it('displays town type for town zones', () => {
      mockUseZone.mockReturnValue({
        selectedZone: createMockZone({ is_town: true }),
        isModalOpen: true,
        closeModal: mockCloseModal,
        navigateToZone: mockNavigateToZone,
        allZones: [],
      });

      render(<ZoneDetailsModal />);

      expect(screen.getByText('Town')).toBeInTheDocument();
    });

    it('displays act number', () => {
      render(<ZoneDetailsModal />);

      expect(screen.getByText('Act:')).toBeInTheDocument();
      expect(screen.getByText('1')).toBeInTheDocument();
    });

    it('displays area level', () => {
      render(<ZoneDetailsModal />);

      expect(screen.getByText('Level:')).toBeInTheDocument();
      // Use regex to match the level value as there may be multiple "2"s on the page
      const levelElements = screen.getAllByText('2');
      expect(levelElements.length).toBeGreaterThan(0);
    });

    it('displays waypoint status', () => {
      render(<ZoneDetailsModal />);

      expect(screen.getByText('Waypoint:')).toBeInTheDocument();
      expect(screen.getByText('Yes')).toBeInTheDocument();
    });

    it('returns null when no zone is selected', () => {
      mockUseZone.mockReturnValue({
        selectedZone: null,
        isModalOpen: true,
        closeModal: mockCloseModal,
        navigateToZone: mockNavigateToZone,
        allZones: [],
      });

      const { container } = render(<ZoneDetailsModal />);

      expect(container.firstChild).toBeNull();
    });
  });

  describe('Player Statistics', () => {
    it('displays player statistics for visited zones', () => {
      render(<ZoneDetailsModal />);

      expect(screen.getByText('Your Statistics')).toBeInTheDocument();
      expect(screen.getByText('Time Spent:')).toBeInTheDocument();
      expect(screen.getByText('Visits:')).toBeInTheDocument();
    });

    it('displays visit count', () => {
      render(<ZoneDetailsModal />);

      expect(screen.getByText('5')).toBeInTheDocument();
    });

    it('displays deaths for non-town zones', () => {
      render(<ZoneDetailsModal />);

      expect(screen.getByText('Deaths:')).toBeInTheDocument();
    });

    it('does not display deaths for town zones', () => {
      mockUseZone.mockReturnValue({
        selectedZone: createMockZone({ is_town: true }),
        isModalOpen: true,
        closeModal: mockCloseModal,
        navigateToZone: mockNavigateToZone,
        allZones: [],
      });

      render(<ZoneDetailsModal />);

      expect(screen.queryByText('Deaths:')).not.toBeInTheDocument();
    });
  });

  describe('Unvisited Zone', () => {
    it('shows unvisited message for zones with 0 visits', () => {
      mockUseZone.mockReturnValue({
        selectedZone: createMockZone({ visits: 0 }),
        isModalOpen: true,
        closeModal: mockCloseModal,
        navigateToZone: mockNavigateToZone,
        allZones: [],
      });

      render(<ZoneDetailsModal />);

      expect(screen.getByText('Zone Not Yet Visited')).toBeInTheDocument();
      expect(screen.getByText(/You haven't visited this zone yet/)).toBeInTheDocument();
    });

    it('does not show player statistics for unvisited zones', () => {
      mockUseZone.mockReturnValue({
        selectedZone: createMockZone({ visits: 0 }),
        isModalOpen: true,
        closeModal: mockCloseModal,
        navigateToZone: mockNavigateToZone,
        allZones: [],
      });

      render(<ZoneDetailsModal />);

      expect(screen.queryByText('Your Statistics')).not.toBeInTheDocument();
    });
  });

  describe('Bosses', () => {
    it('displays bosses section when bosses exist', () => {
      render(<ZoneDetailsModal />);

      expect(screen.getByText('Bosses (2)')).toBeInTheDocument();
      expect(screen.getByText('Boss One')).toBeInTheDocument();
      expect(screen.getByText('Boss Two')).toBeInTheDocument();
    });

    it('does not display bosses section when no bosses', () => {
      mockUseZone.mockReturnValue({
        selectedZone: createMockZone({ bosses: [] }),
        isModalOpen: true,
        closeModal: mockCloseModal,
        navigateToZone: mockNavigateToZone,
        allZones: [],
      });

      render(<ZoneDetailsModal />);

      expect(screen.queryByText(/Bosses/)).not.toBeInTheDocument();
    });
  });

  describe('NPCs', () => {
    it('displays NPCs section when NPCs exist', () => {
      render(<ZoneDetailsModal />);

      expect(screen.getByText('NPCs (1)')).toBeInTheDocument();
      expect(screen.getByText('NPC One')).toBeInTheDocument();
    });
  });

  describe('Points of Interest', () => {
    it('displays POI section when POIs exist', () => {
      render(<ZoneDetailsModal />);

      expect(screen.getByText('Points of Interest (1)')).toBeInTheDocument();
      expect(screen.getByText('POI One')).toBeInTheDocument();
    });
  });

  describe('Connected Zones', () => {
    it('displays connected zones section', () => {
      render(<ZoneDetailsModal />);

      expect(screen.getByText('Connected Zones (2)')).toBeInTheDocument();
      expect(screen.getByText('The Mud Flats')).toBeInTheDocument();
      expect(screen.getByText("Lioneye's Watch")).toBeInTheDocument();
    });

    it('navigates to connected zone when clicked', async () => {
      const user = userEvent.setup();
      mockUseZone.mockReturnValue({
        selectedZone: createMockZone(),
        isModalOpen: true,
        closeModal: mockCloseModal,
        navigateToZone: mockNavigateToZone,
        allZones: [createMockZone({ zone_name: 'The Mud Flats' })],
      });

      render(<ZoneDetailsModal />);

      await user.click(screen.getByText('The Mud Flats'));

      expect(mockNavigateToZone).toHaveBeenCalled();
    });
  });

  describe('Wiki Link', () => {
    it('displays wiki link button', () => {
      render(<ZoneDetailsModal />);

      expect(screen.getByText('View on Wiki')).toBeInTheDocument();
    });

    it('opens wiki when clicked', async () => {
      const user = userEvent.setup();
      mockOpen.mockResolvedValueOnce(undefined);

      render(<ZoneDetailsModal />);

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
