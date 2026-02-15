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

  describe('Initial Loading and Rendering', () => {
    it('renders loading state then loads and displays all form fields', async () => {
      render(<SettingsForm />);

      // Loading state
      expect(screen.getByText('Loading configuration...')).toBeInTheDocument();

      // Wait for form to load
      await waitFor(() => {
        expect(screen.getByText('POE Client Log Path')).toBeInTheDocument();
      });

      // All form fields should be present
      expect(screen.getByText('Log Level')).toBeInTheDocument();
      expect(screen.getByText('Zone Refresh Interval')).toBeInTheDocument();

      // All buttons should be present
      expect(screen.getByRole('button', { name: 'Save Configuration' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Reset to Defaults' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Reload' })).toBeInTheDocument();

      // Config should be loaded
      expect(mockGetConfig).toHaveBeenCalled();
      expect(mockGetZoneRefreshIntervalOptions).toHaveBeenCalled();

      // Log level should display correctly
      expect(screen.getByRole('button', { name: /info/i })).toBeInTheDocument();
    });
  });

  describe('Save Configuration', () => {
    it('saves config and shows success message', async () => {
      const user = userEvent.setup();

      render(<SettingsForm />);

      await waitFor(() => {
        expect(screen.getByRole('button', { name: 'Save Configuration' })).toBeInTheDocument();
      });

      await user.click(screen.getByRole('button', { name: 'Save Configuration' }));

      await waitFor(() => {
        expect(mockUpdateConfig).toHaveBeenCalled();
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
  });

  describe('Reset Configuration', () => {
    it('resets config and shows success message', async () => {
      const user = userEvent.setup();

      render(<SettingsForm />);

      await waitFor(() => {
        expect(screen.getByRole('button', { name: 'Reset to Defaults' })).toBeInTheDocument();
      });

      await user.click(screen.getByRole('button', { name: 'Reset to Defaults' }));

      await waitFor(() => {
        expect(mockResetConfigToDefaults).toHaveBeenCalled();
        expect(screen.getByText('Configuration reset to defaults!')).toBeInTheDocument();
      });
    });
  });

  describe('Reload Configuration', () => {
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
  });

  describe('Error Handling', () => {
    it('shows error when config fails to load', async () => {
      mockGetConfig.mockRejectedValueOnce(new Error('Network error'));

      render(<SettingsForm />);

      await waitFor(() => {
        expect(screen.getByText('Failed to load configuration: Network error')).toBeInTheDocument();
      });
    });
  });

  describe('Validation', () => {
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
  });
});
