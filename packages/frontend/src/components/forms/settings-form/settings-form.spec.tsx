import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { SettingsForm } from './settings-form';

const mockGetConfig = vi.hoisted(() =>
  vi.fn(() =>
    Promise.resolve({
      poe_client_log_path: 'C:\\Games\\Path of Exile\\logs\\client.txt',
      log_level: 'info',
      zone_refresh_interval: 'SevenDays',
    }),
  ),
);

const mockUpdateConfig = vi.hoisted(() => vi.fn(() => Promise.resolve()));
const mockResetConfigToDefaults = vi.hoisted(() => vi.fn(() => Promise.resolve()));
const mockGetZoneRefreshIntervalOptions = vi.hoisted(() =>
  vi.fn(() =>
    Promise.resolve([
      { value: 'FiveMinutes', label: '5 Minutes' },
      { value: 'OneHour', label: '1 Hour' },
      { value: 'SevenDays', label: '7 Days' },
    ]),
  ),
);

vi.mock('@/utils/tauri', () => ({
  tauriUtils: {
    getConfig: mockGetConfig,
    updateConfig: mockUpdateConfig,
    resetConfigToDefaults: mockResetConfigToDefaults,
    getZoneRefreshIntervalOptions: mockGetZoneRefreshIntervalOptions,
  },
}));

describe('SettingsForm', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders loading state initially', async () => {
    render(<SettingsForm />);

    expect(screen.getByText('Loading configuration...')).toBeInTheDocument();
    // Wait for async state updates to complete
    await waitFor(() => {
      expect(screen.getByText('POE Client Log Path')).toBeInTheDocument();
    });
  });

  it('renders form after loading', async () => {
    render(<SettingsForm />);

    await waitFor(() => {
      expect(screen.getByText('POE Client Log Path')).toBeInTheDocument();
    });
  });

  it('renders log level field', async () => {
    render(<SettingsForm />);

    await waitFor(() => {
      expect(screen.getByText('Log Level')).toBeInTheDocument();
    });
  });

  it('renders zone refresh interval field', async () => {
    render(<SettingsForm />);

    await waitFor(() => {
      expect(screen.getByText('Zone Refresh Interval')).toBeInTheDocument();
    });
  });

  it('renders save button', async () => {
    render(<SettingsForm />);

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Save Configuration' })).toBeInTheDocument();
    });
  });

  it('renders reset button', async () => {
    render(<SettingsForm />);

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Reset to Defaults' })).toBeInTheDocument();
    });
  });

  it('renders reload button', async () => {
    render(<SettingsForm />);

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Reload' })).toBeInTheDocument();
    });
  });

  it('loads config on mount', async () => {
    render(<SettingsForm />);

    await waitFor(() => {
      expect(mockGetConfig).toHaveBeenCalled();
    });
  });

  it('loads zone refresh options on mount', async () => {
    render(<SettingsForm />);

    await waitFor(() => {
      expect(mockGetZoneRefreshIntervalOptions).toHaveBeenCalled();
    });
  });

  it('saves config when save button is clicked', async () => {
    const user = userEvent.setup();

    render(<SettingsForm />);

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Save Configuration' })).toBeInTheDocument();
    });

    await user.click(screen.getByRole('button', { name: 'Save Configuration' }));

    await waitFor(() => {
      expect(mockUpdateConfig).toHaveBeenCalled();
    });
  });

  it('shows success message after save', async () => {
    const user = userEvent.setup();

    render(<SettingsForm />);

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Save Configuration' })).toBeInTheDocument();
    });

    await user.click(screen.getByRole('button', { name: 'Save Configuration' }));

    await waitFor(() => {
      expect(screen.getByText('Configuration saved successfully!')).toBeInTheDocument();
    });
  });

  it('calls onConfigUpdate callback after save', async () => {
    const user = userEvent.setup();
    const handleConfigUpdate = vi.fn();

    render(<SettingsForm onConfigUpdate={handleConfigUpdate} />);

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Save Configuration' })).toBeInTheDocument();
    });

    await user.click(screen.getByRole('button', { name: 'Save Configuration' }));

    await waitFor(() => {
      expect(handleConfigUpdate).toHaveBeenCalled();
    });
  });

  it('resets config when reset button is clicked', async () => {
    const user = userEvent.setup();

    render(<SettingsForm />);

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Reset to Defaults' })).toBeInTheDocument();
    });

    await user.click(screen.getByRole('button', { name: 'Reset to Defaults' }));

    await waitFor(() => {
      expect(mockResetConfigToDefaults).toHaveBeenCalled();
    });
  });

  it('shows success message after reset', async () => {
    const user = userEvent.setup();

    render(<SettingsForm />);

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Reset to Defaults' })).toBeInTheDocument();
    });

    await user.click(screen.getByRole('button', { name: 'Reset to Defaults' }));

    await waitFor(() => {
      expect(screen.getByText('Configuration reset to defaults!')).toBeInTheDocument();
    });
  });

  it('reloads config when reload button is clicked', async () => {
    const user = userEvent.setup();

    render(<SettingsForm />);

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Reload' })).toBeInTheDocument();
    });

    // Clear initial call
    mockGetConfig.mockClear();

    await user.click(screen.getByRole('button', { name: 'Reload' }));

    await waitFor(() => {
      expect(mockGetConfig).toHaveBeenCalled();
    });
  });

  it('shows error when config fails to load', async () => {
    mockGetConfig.mockRejectedValueOnce(new Error('Network error'));

    render(<SettingsForm />);

    await waitFor(() => {
      expect(screen.getByText('Failed to load configuration: Network error')).toBeInTheDocument();
    });
  });

  it('shows error when save fails', async () => {
    const user = userEvent.setup();
    mockUpdateConfig.mockRejectedValueOnce(new Error('Save failed'));

    render(<SettingsForm />);

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Save Configuration' })).toBeInTheDocument();
    });

    await user.click(screen.getByRole('button', { name: 'Save Configuration' }));

    await waitFor(() => {
      expect(screen.getByText('Failed to save configuration: Save failed')).toBeInTheDocument();
    });
  });

  it('shows warning for invalid POE path', async () => {
    mockGetConfig.mockResolvedValueOnce({
      poe_client_log_path: 'invalid/path',
      log_level: 'info',
      zone_refresh_interval: 'SevenDays',
    });

    render(<SettingsForm />);

    await waitFor(() => {
      expect(
        screen.getByText(/This path doesn't look like a typical POE client log file/),
      ).toBeInTheDocument();
    });
  });

  it('displays correct log level from config', async () => {
    render(<SettingsForm />);

    await waitFor(() => {
      // The dropdown should show the current log level
      expect(screen.getByRole('button', { name: /info/i })).toBeInTheDocument();
    });
  });
});
