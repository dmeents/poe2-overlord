import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import type { CharacterData } from '@/types/character';
import type { GameProcessStatus } from '@/types/process';
import type { ServerStatus } from '@/types/server';
import { StatusBar } from './status-bar';

const mockNavigate = vi.hoisted(() => vi.fn());

const mockUseGameProcess = vi.hoisted(() =>
  vi.fn(() => ({
    processInfo: null as GameProcessStatus | null,
  }))
);

const mockUseServerStatus = vi.hoisted(() =>
  vi.fn(() => ({
    serverStatus: null as ServerStatus | null,
  }))
);

const mockUseCharacter = vi.hoisted(() =>
  vi.fn(() => ({
    activeCharacter: null as CharacterData | null,
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

const createMockCharacter = (
  name: string,
  currentLocation?: {
    zone_name: string;
    act: number;
    is_town: boolean;
    has_waypoint: boolean;
    area_level: number;
  } | null
): CharacterData => ({
  id: 'test-id',
  name,
  class: 'Warrior',
  ascendency: 'Titan',
  level: 50,
  league: 'Standard',
  hardcore: false,
  solo_self_found: false,
  created_at: '2024-01-01T00:00:00Z',
  last_updated: '2024-01-10T00:00:00Z',
  current_location: currentLocation
    ? {
        ...currentLocation,
        location_type: 'Zone',
        last_updated: '2024-01-10T00:00:00Z',
      }
    : undefined,
  summary: {
    character_id: 'test-id',
    total_play_time: 3600,
    total_hideout_time: 600,
    total_zones_visited: 20,
    total_deaths: 5,
    play_time_act1: 900,
    play_time_act2: 900,
    play_time_act3: 900,
    play_time_act4: 300,
    play_time_interlude: 0,
    play_time_endgame: 0,
  },
  zones: [],
  walkthrough_progress: {
    current_step_id: null,
    is_completed: false,
    last_updated: '2024-01-10T00:00:00Z',
  },
});

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
      processInfo: {
        name: 'PathOfExileSteam.exe',
        pid: 12345,
        running: true,
        detected_at: '2024-01-10T00:00:00Z',
      },
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
      activeCharacter: createMockCharacter('TestCharacter'),
    });

    render(<StatusBar />);

    expect(screen.getByText('TestCharacter')).toBeInTheDocument();
  });

  it('shows character with zone location', () => {
    mockUseCharacter.mockReturnValue({
      activeCharacter: createMockCharacter('TestCharacter', {
        zone_name: 'The Coast',
        act: 1,
        is_town: false,
        has_waypoint: true,
        area_level: 2,
      }),
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
        port: 6112,
        latency_ms: 50,
        timestamp: '2024-01-10T00:00:00Z',
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
        port: 6112,
        latency_ms: null,
        timestamp: '2024-01-10T00:00:00Z',
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
      activeCharacter: createMockCharacter('TestCharacter', {
        zone_name: 'The Coast',
        act: 2,
        is_town: false,
        has_waypoint: true,
        area_level: 15,
      }),
    });

    render(<StatusBar />);

    // getDisplayAct returns consistent format "Act N" for regular acts
    expect(
      screen.getByText(/TestCharacter - Act 2 - The Coast/)
    ).toBeInTheDocument();
  });
});
