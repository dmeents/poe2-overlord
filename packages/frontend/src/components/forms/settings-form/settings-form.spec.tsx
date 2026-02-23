import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { SettingsForm } from './settings-form';

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
const mockUseConfiguration = vi.hoisted(() => vi.fn());

vi.mock('@/utils/tauri', () => ({
  tauriUtils: {
    getConfig: vi.fn(() => Promise.resolve({})),
    updateConfig: mockUpdateConfig,
    resetConfigToDefaults: mockResetConfigToDefaults,
    getZoneRefreshIntervalOptions: mockGetZoneRefreshIntervalOptions,
  },
}));

vi.mock('@/contexts/ConfigurationContext', () => ({
  useConfiguration: mockUseConfiguration,
}));

const defaultConfig = {
  config_version: 1,
  poe_client_log_path: 'C:\\Games\\Path of Exile\\logs\\client.txt',
  log_level: 'info',
  zone_refresh_interval: 'SevenDays',
  hide_optional_objectives: false,
  hide_league_start_objectives: false,
  hide_flavor_text: false,
  hide_objective_descriptions: false,
};

describe('SettingsForm', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockUseConfiguration.mockReturnValue({
      config: defaultConfig,
      isLoading: false,
      updateConfig: vi.fn(),
    });
  });

  describe('Initial Loading and Rendering', () => {
    it('renders loading state then loads and displays all form fields', async () => {
      mockUseConfiguration.mockReturnValue({ config: null, isLoading: true, updateConfig: vi.fn() });
      const { rerender } = render(<SettingsForm />);

      expect(screen.getByText('Loading configuration...')).toBeInTheDocument();

      mockUseConfiguration.mockReturnValue({
        config: defaultConfig,
        isLoading: false,
        updateConfig: vi.fn(),
      });
      rerender(<SettingsForm />);

      await waitFor(() => {
        expect(screen.getByText('POE Client Log Path')).toBeInTheDocument();
      });

      expect(screen.getByText('Log Level')).toBeInTheDocument();
      expect(screen.getByText('Zone Refresh Interval')).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Save Configuration' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Reset to Defaults' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Reload' })).toBeInTheDocument();
      expect(mockGetZoneRefreshIntervalOptions).toHaveBeenCalled();
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

      await user.click(screen.getByRole('button', { name: 'Reload' }));

      // After reload, form should still render correctly
      expect(screen.getByText('POE Client Log Path')).toBeInTheDocument();
    });
  });

  describe('Validation', () => {
    it('shows warning for invalid POE path', async () => {
      mockUseConfiguration.mockReturnValue({
        config: {
          ...defaultConfig,
          poe_client_log_path: 'invalid/path',
        },
        isLoading: false,
        updateConfig: vi.fn(),
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
