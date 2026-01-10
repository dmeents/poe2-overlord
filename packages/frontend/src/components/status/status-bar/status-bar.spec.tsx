import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { StatusBar } from './status-bar';

const mockNavigate = vi.hoisted(() => vi.fn());

const mockUseGameProcess = vi.hoisted(() =>
  vi.fn(() => ({
    processInfo: null,
  }))
);

const mockUseServerStatus = vi.hoisted(() =>
  vi.fn(() => ({
    serverStatus: null,
  }))
);

const mockUseCharacter = vi.hoisted(() =>
  vi.fn(() => ({
    activeCharacter: null,
  }))
);

vi.mock('@/contexts/GameProcessContext', () => ({
  useGameProcess: mockUseGameProcess,
}));

vi.mock('@/contexts/ServerStatusContext', () => ({
  useServerStatus: mockUseServerStatus,
}));

vi.mock('@/contexts/CharacterContext', () => ({
  useCharacter: mockUseCharacter,
}));

vi.mock('@tanstack/react-router', () => ({
  useNavigate: () => mockNavigate,
}));

describe('StatusBar', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockUseGameProcess.mockReturnValue({ processInfo: null });
    mockUseServerStatus.mockReturnValue({ serverStatus: null });
    mockUseCharacter.mockReturnValue({ activeCharacter: null });
  });

  it('renders status bar container', () => {
    const { container } = render(<StatusBar />);

    expect(container.firstChild).toBeInTheDocument();
  });

  it('shows POE2 is stopped when not running', () => {
    render(<StatusBar />);

    expect(screen.getByTitle('POE2 is stopped')).toBeInTheDocument();
  });

  it('shows POE2 is running when process is active', () => {
    mockUseGameProcess.mockReturnValue({
      processInfo: { running: true },
    });

    render(<StatusBar />);

    expect(screen.getByTitle('POE2 is running')).toBeInTheDocument();
  });

  it('shows no active character when none selected', () => {
    render(<StatusBar />);

    expect(screen.getByText('No active character')).toBeInTheDocument();
  });

  it('shows character name when active', () => {
    mockUseCharacter.mockReturnValue({
      activeCharacter: {
        name: 'TestCharacter',
        current_location: null,
      },
    });

    render(<StatusBar />);

    expect(screen.getByText('TestCharacter')).toBeInTheDocument();
  });

  it('shows character with zone location', () => {
    mockUseCharacter.mockReturnValue({
      activeCharacter: {
        name: 'TestCharacter',
        current_location: {
          zone_name: 'The Coast',
          act: 1,
          is_town: false,
          has_waypoint: true,
          area_level: 2,
        },
      },
    });

    render(<StatusBar />);

    expect(screen.getByText(/TestCharacter/)).toBeInTheDocument();
    expect(screen.getByText(/The Coast/)).toBeInTheDocument();
  });

  it('shows server attempting to connect when status is null', () => {
    render(<StatusBar />);

    expect(
      screen.getByTitle('Attempting to connect to POE2 server...')
    ).toBeInTheDocument();
  });

  it('shows server online status', () => {
    mockUseServerStatus.mockReturnValue({
      serverStatus: {
        is_online: true,
        ip_address: '192.168.1.1',
        latency_ms: 50,
      },
    });

    render(<StatusBar />);

    const serverStatusIndicator = screen.getByTitle(/POE2 server is online/);
    expect(serverStatusIndicator).toBeInTheDocument();
  });

  it('shows server offline status', () => {
    mockUseServerStatus.mockReturnValue({
      serverStatus: {
        is_online: false,
        ip_address: '192.168.1.1',
      },
    });

    render(<StatusBar />);

    const serverStatusIndicator = screen.getByTitle(/POE2 server is offline/);
    expect(serverStatusIndicator).toBeInTheDocument();
  });

  it('renders settings button', () => {
    render(<StatusBar />);

    expect(screen.getByTitle('Settings')).toBeInTheDocument();
  });

  it('navigates to settings when settings button is clicked', async () => {
    const user = userEvent.setup();

    render(<StatusBar />);

    // The title is on a wrapper div, but the button is inside it
    const settingsWrapper = screen.getByTitle('Settings');
    const button = settingsWrapper.querySelector('button');
    expect(button).not.toBeNull();

    await user.click(button!);

    expect(mockNavigate).toHaveBeenCalledWith({ to: '/settings' });
  });

  it('renders all status indicators', () => {
    const { container } = render(<StatusBar />);

    // Should have 3 status indicators (game, server, user)
    const statusIndicators = container.querySelectorAll('svg');
    expect(statusIndicators.length).toBeGreaterThanOrEqual(3);
  });

  it('shows character with act in location', () => {
    mockUseCharacter.mockReturnValue({
      activeCharacter: {
        name: 'TestCharacter',
        current_location: {
          zone_name: 'The Coast',
          act: 2,
          is_town: false,
          has_waypoint: true,
          area_level: 15,
        },
      },
    });

    render(<StatusBar />);

    // getDisplayAct returns just the act number for regular acts
    expect(screen.getByText(/TestCharacter - 2 - The Coast/)).toBeInTheDocument();
  });
});
