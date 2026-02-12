import { useCallback, useEffect, useRef, useState } from 'react';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { Tooltip } from '@/components/ui/tooltip/tooltip';
import { useAppEventListener } from '@/hooks/useAppEventListener';
import type { AppConfig, ZoneRefreshIntervalOption } from '@/types/app-config';
import {
  type ConfigurationChangedEvent,
  EVENT_KEYS,
  type ExtractPayload,
} from '@/utils/events/registry';
import { tauriUtils } from '@/utils/tauri';
import { Button } from '../../ui/button/button';
import { AlertMessage } from '../form-alert-message/form-alert-message';
import { FormField } from '../form-field/form-field';
import { Input } from '../form-input/form-input';
import { Select } from '../form-select/form-select';
import { settingsFormStyles } from './settings-form.styles';

/** Valid log levels matching backend validation */
const VALID_LOG_LEVELS = ['trace', 'debug', 'info', 'warn', 'error'];

/**
 * Extract a user-friendly error message from any error type.
 * Provides consistent error handling across all form operations.
 */
function extractErrorMessage(err: unknown): string {
  const rawMessage = err instanceof Error ? err.message : String(err);
  return rawMessage;
}

/**
 * Format a configuration error with context-specific messages.
 * Maps backend error patterns to user-friendly messages.
 */
function formatConfigError(
  err: unknown,
  operation: 'save' | 'load' | 'reset',
  context?: { logLevel?: string },
): string {
  const message = extractErrorMessage(err);

  // Map common error patterns to user-friendly messages
  if (message.includes('Invalid log level')) {
    const level = context?.logLevel || 'unknown';
    return `Invalid log level: "${level}". Valid levels: ${VALID_LOG_LEVELS.join(', ')}`;
  }
  if (message.includes('cannot be empty')) {
    return 'POE client log path cannot be empty';
  }
  if (message.includes('path traversal') || message.includes('not allowed')) {
    return 'Invalid path: The path contains invalid characters or attempts to access restricted locations';
  }
  if (message.includes('file not found') || message.includes('ENOENT')) {
    return 'The specified file path does not exist. Please check the path and try again.';
  }

  // Default: include operation context
  return `Failed to ${operation} configuration: ${message}`;
}

interface SettingsFormProps {
  onConfigUpdate?: (config: AppConfig) => void;
}

export function SettingsForm({ onConfigUpdate }: SettingsFormProps) {
  const [config, setConfig] = useState<AppConfig>({
    poe_client_log_path: '',
    log_level: 'info',
    zone_refresh_interval: 'SevenDays',
  });
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [zoneRefreshOptions, setZoneRefreshOptions] = useState<ZoneRefreshIntervalOption[]>([]);

  // Track timeout refs for cleanup to prevent memory leaks
  const successTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  // Clear success message with proper cleanup
  const clearSuccessAfterDelay = useCallback(() => {
    // Clear any existing timeout
    if (successTimeoutRef.current) {
      clearTimeout(successTimeoutRef.current);
    }
    successTimeoutRef.current = setTimeout(() => {
      setSuccess(null);
      successTimeoutRef.current = null;
    }, 3000);
  }, []);

  // Cleanup timeout on unmount
  useEffect(() => {
    return () => {
      if (successTimeoutRef.current) {
        clearTimeout(successTimeoutRef.current);
      }
    };
  }, []);

  // Listen for configuration changes from backend to keep UI in sync
  useAppEventListener(
    [
      {
        eventType: EVENT_KEYS.ConfigurationChanged,
        handler: (payload: unknown) => {
          const { new_config } = payload as ExtractPayload<ConfigurationChangedEvent>;
          // Update local state with the new config from the event
          setConfig(new_config);
          // Notify parent component if callback provided
          onConfigUpdate?.(new_config);
        },
      },
    ],
    [], // No dependencies - using functional updates
  );

  const loadConfig = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      const loadedConfig = await tauriUtils.getConfig();
      setConfig(loadedConfig);
    } catch (err) {
      setError(formatConfigError(err, 'load'));
    } finally {
      setIsLoading(false);
    }
  }, []);

  const loadZoneRefreshOptions = useCallback(async () => {
    try {
      const options = await tauriUtils.getZoneRefreshIntervalOptions();
      setZoneRefreshOptions(options);
    } catch (err) {
      // Non-critical - use default options, but log for debugging
      console.warn(
        'Failed to load zone refresh options, using defaults:',
        extractErrorMessage(err),
      );
    }
  }, []);

  // Load configuration and options on component mount
  useEffect(() => {
    loadConfig();
    loadZoneRefreshOptions();
  }, [loadConfig, loadZoneRefreshOptions]);

  const handleSave = async () => {
    // Pre-validate before backend call
    const pathValid = validatePoeClientLogPath(config.poe_client_log_path);
    if (!pathValid) {
      setError('Please enter a valid POE client log path before saving');
      return;
    }

    if (!VALID_LOG_LEVELS.includes(config.log_level.toLowerCase())) {
      setError(
        `Invalid log level: ${config.log_level}. Valid levels: ${VALID_LOG_LEVELS.join(', ')}`,
      );
      return;
    }

    try {
      setIsSaving(true);
      setError(null);
      setSuccess(null);

      await tauriUtils.updateConfig(config);
      setSuccess('Configuration saved successfully!');
      onConfigUpdate?.(config);

      clearSuccessAfterDelay();
    } catch (err) {
      setError(formatConfigError(err, 'save', { logLevel: config.log_level }));
    } finally {
      setIsSaving(false);
    }
  };

  const handleReset = async () => {
    try {
      setIsSaving(true);
      setError(null);
      setSuccess(null);

      await tauriUtils.resetConfigToDefaults();
      // State will be updated by ConfigurationChanged event listener
      // which also handles onConfigUpdate callback
      setSuccess('Configuration reset to defaults!');

      clearSuccessAfterDelay();
    } catch (err) {
      setError(formatConfigError(err, 'reset'));
    } finally {
      setIsSaving(false);
    }
  };

  const handleInputChange = (field: keyof AppConfig, value: string | boolean) => {
    setConfig(prev => ({
      ...prev,
      [field]: value,
    }));
  };

  const validatePoeClientLogPath = (path: string): boolean => {
    // Basic validation - check if it's not empty
    if (!path.trim()) return false;

    const pathLower = path.toLowerCase();

    // Must have valid extension (.txt or .log)
    const validExtensions = ['.txt', '.log'];
    const hasValidExtension = validExtensions.some(ext => pathLower.endsWith(ext));

    if (!hasValidExtension) return false;

    // Should contain POE-related keywords (strengthened from OR to more specific matching)
    const hasPoeIndicators =
      pathLower.includes('path of exile') ||
      pathLower.includes('poe2') ||
      pathLower.includes('poe 2') ||
      (pathLower.includes('client') && pathLower.includes('log'));

    return hasPoeIndicators;
  };

  if (isLoading) {
    return <LoadingSpinner message="Loading configuration..." />;
  }

  const isPoePathValid = validatePoeClientLogPath(config.poe_client_log_path);

  const logLevelOptions = [
    { value: 'trace', label: 'Trace' },
    { value: 'debug', label: 'Debug' },
    { value: 'info', label: 'Info' },
    { value: 'warn', label: 'Warn' },
    { value: 'error', label: 'Error' },
  ];

  return (
    <div className={settingsFormStyles.container}>
      {/* Error and Success Messages */}
      <div className={settingsFormStyles.messagesContainer}>
        <AlertMessage type="error" message={error || ''} />
        <AlertMessage type="success" message={success || ''} />
      </div>

      {/* POE Client Log Path */}
      <FormField
        label={
          <Tooltip
            content={
              <div>
                <p className="mb-2">
                  <strong>POE Client Log Path:</strong> This should point to the client.txt file in
                  your Path of Exile installation directory.
                </p>
                <p>
                  The file is typically located at:{' '}
                  <code className="bg-zinc-700 px-1 text-zinc-200">
                    [POE Install]/logs/client.txt
                  </code>
                </p>
                <p className="mt-2 text-zinc-300">Common locations:</p>
                <ul className="list-disc list-inside text-zinc-300 mt-1 space-y-1">
                  <li>
                    Steam:{' '}
                    <code className="bg-zinc-700 px-1 text-zinc-200">
                      C:\Program Files (x86)\Steam\steamapps\common\Path of Exile\logs\client.txt
                    </code>
                  </li>
                  <li>
                    Standalone:{' '}
                    <code className="bg-zinc-700 px-1 text-zinc-200">
                      C:\Games\Path of Exile\logs\client.txt
                    </code>
                  </li>
                </ul>
              </div>
            }>
            POE Client Log Path
          </Tooltip>
        }
        htmlFor="poe-client-log-path">
        <Input
          id="poe-client-log-path"
          value={config.poe_client_log_path}
          onChange={value => handleInputChange('poe_client_log_path', value as string)}
          type="text"
          placeholder="Enter path to client.txt file (e.g., C:\Games\Path of Exile\logs\client.txt)"
          isValid={isPoePathValid}
          warningMessage="This path doesn't look like a typical POE client log file. Please verify the path."
        />
      </FormField>

      {/* Log Level */}
      <FormField
        label={
          <Tooltip
            content={
              <div>
                <p className="mb-2">
                  <strong>Log Level:</strong> Controls the verbosity of application logging.
                </p>
                <div className="space-y-1 text-zinc-300">
                  <p>
                    <strong>Trace:</strong> Most detailed logging, shows every operation
                  </p>
                  <p>
                    <strong>Debug:</strong> Detailed logging for troubleshooting
                  </p>
                  <p>
                    <strong>Info:</strong> General information about application operation
                    (recommended)
                  </p>
                  <p>
                    <strong>Warn:</strong> Only warnings and errors
                  </p>
                  <p>
                    <strong>Error:</strong> Only error messages
                  </p>
                </div>
                <p className="mt-2 text-zinc-300">
                  Use "Info" for normal operation, "Debug" for troubleshooting, or "Error" for
                  minimal logging.
                </p>
              </div>
            }>
            Log Level
          </Tooltip>
        }
        htmlFor="log-level">
        <Select
          id="log-level"
          value={config.log_level}
          onChange={value => handleInputChange('log_level', value)}
          options={logLevelOptions}
          variant="dropdown"
        />
      </FormField>

      {/* Zone Refresh Interval */}
      <FormField
        label={
          <Tooltip
            content={
              <div>
                <p className="mb-2">
                  <strong>Zone Refresh Interval:</strong> Controls how often zone metadata should be
                  refreshed from the POE wiki.
                </p>
                <div className="space-y-1 text-zinc-300">
                  <p>
                    <strong>5 Minutes:</strong> Very frequent updates (useful for testing)
                  </p>
                  <p>
                    <strong>1 Hour:</strong> Frequent updates for active development
                  </p>
                  <p>
                    <strong>12 Hours:</strong> Twice daily updates
                  </p>
                  <p>
                    <strong>24 Hours:</strong> Daily updates
                  </p>
                  <p>
                    <strong>3 Days:</strong> Occasional updates
                  </p>
                  <p>
                    <strong>7 Days:</strong> Weekly updates (recommended for normal use)
                  </p>
                </div>
                <p className="mt-2 text-zinc-300">
                  Use shorter intervals for testing zone data updates, or longer intervals to reduce
                  wiki requests.
                </p>
              </div>
            }>
            Zone Refresh Interval
          </Tooltip>
        }
        htmlFor="zone-refresh-interval"
        className="last-form-item">
        <Select
          id="zone-refresh-interval"
          value={config.zone_refresh_interval}
          onChange={value => handleInputChange('zone_refresh_interval', value)}
          options={zoneRefreshOptions.map(opt => ({
            value: opt.value,
            label: opt.label,
          }))}
          variant="dropdown"
        />
      </FormField>

      {/* Action Buttons */}
      <div className={settingsFormStyles.actionButtons}>
        <Button onClick={handleSave} disabled={isSaving} variant="primary" size="md">
          {isSaving ? 'Saving...' : 'Save Configuration'}
        </Button>

        <Button onClick={handleReset} disabled={isSaving} variant="secondary" size="md">
          Reset to Defaults
        </Button>

        <Button onClick={loadConfig} disabled={isSaving} variant="outline" size="md">
          Reload
        </Button>
      </div>
    </div>
  );
}
