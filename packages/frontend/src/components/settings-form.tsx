import { LoadingSpinner, Tooltip } from '@/components';
import type { AppConfig } from '@/types';
import { tauriUtils } from '@/utils/tauri';
import { useEffect, useState } from 'react';
import { AlertMessage, FormField, SelectInput, TextInput } from './form';

interface SettingsFormProps {
  onConfigUpdate?: (config: AppConfig) => void;
}

export function SettingsForm({ onConfigUpdate }: SettingsFormProps) {
  const [config, setConfig] = useState<AppConfig>({
    poe_client_log_path: '',
    log_level: 'info',
  });
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  // Load configuration on component mount
  useEffect(() => {
    loadConfig();
  }, []);

  const loadConfig = async () => {
    try {
      setIsLoading(true);
      setError(null);
      const loadedConfig = await tauriUtils.getConfig();
      setConfig(loadedConfig);
    } catch (err) {
      setError('Failed to load configuration');
      console.error('Error loading config:', err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSave = async () => {
    try {
      setIsSaving(true);
      setError(null);
      setSuccess(null);

      await tauriUtils.updateConfig(config);
      setSuccess('Configuration saved successfully!');
      onConfigUpdate?.(config);

      // Clear success message after 3 seconds
      setTimeout(() => setSuccess(null), 3000);
    } catch (err) {
      setError('Failed to save configuration');
      console.error('Error saving config:', err);
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
      const defaultConfig = await tauriUtils.getConfig();
      setConfig(defaultConfig);
      setSuccess('Configuration reset to defaults!');
      onConfigUpdate?.(defaultConfig);

      // Clear success message after 3 seconds
      setTimeout(() => setSuccess(null), 3000);
    } catch (err) {
      setError('Failed to reset configuration');
      console.error('Error resetting config:', err);
    } finally {
      setIsSaving(false);
    }
  };

  const handleInputChange = (
    field: keyof AppConfig,
    value: string | boolean
  ) => {
    setConfig(prev => ({
      ...prev,
      [field]: value,
    }));
  };

  const validatePoeClientLogPath = (path: string): boolean => {
    // Basic validation - check if it's not empty and has a reasonable structure
    if (!path.trim()) return false;

    // Check if it ends with .txt or .log
    const validExtensions = ['.txt', '.log'];
    const hasValidExtension = validExtensions.some(ext =>
      path.toLowerCase().endsWith(ext)
    );

    // Check if it contains common POE path indicators
    const hasPoeIndicators =
      path.toLowerCase().includes('poe') ||
      path.toLowerCase().includes('path of exile') ||
      path.toLowerCase().includes('client');

    return hasValidExtension || hasPoeIndicators;
  };

  if (isLoading) {
    return <LoadingSpinner message='Loading configuration...' />;
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
    <div className='space-y-0'>
      {/* Error and Success Messages */}
      <div className='mb-6'>
        <AlertMessage type='error' message={error || ''} />
        <AlertMessage type='success' message={success || ''} />
      </div>

      {/* POE Client Log Path */}
      <FormField
        label={
          <Tooltip
            content={
              <div>
                <p className='mb-2'>
                  <strong>POE Client Log Path:</strong> This should point to the
                  client.txt file in your Path of Exile installation directory.
                </p>
                <p>
                  The file is typically located at:{' '}
                  <code className='bg-zinc-700 px-1 text-zinc-200'>
                    [POE Install]/logs/client.txt
                  </code>
                </p>
                <p className='mt-2 text-zinc-300'>Common locations:</p>
                <ul className='list-disc list-inside text-zinc-300 mt-1 space-y-1'>
                  <li>
                    Steam:{' '}
                    <code className='bg-zinc-700 px-1 text-zinc-200'>
                      C:\Program Files (x86)\Steam\steamapps\common\Path of
                      Exile\logs\client.txt
                    </code>
                  </li>
                  <li>
                    Standalone:{' '}
                    <code className='bg-zinc-700 px-1 text-zinc-200'>
                      C:\Games\Path of Exile\logs\client.txt
                    </code>
                  </li>
                </ul>
              </div>
            }
          >
            POE Client Log Path
          </Tooltip>
        }
        htmlFor='poe-client-log-path'
      >
        <TextInput
          id='poe-client-log-path'
          value={config.poe_client_log_path}
          onChange={value => handleInputChange('poe_client_log_path', value)}
          placeholder='Enter path to client.txt file (e.g., C:\Games\Path of Exile\logs\client.txt)'
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
                <p className='mb-2'>
                  <strong>Log Level:</strong> Controls the verbosity of
                  application logging.
                </p>
                <div className='space-y-1 text-zinc-300'>
                  <p>
                    <strong>Trace:</strong> Most detailed logging, shows every
                    operation
                  </p>
                  <p>
                    <strong>Debug:</strong> Detailed logging for troubleshooting
                  </p>
                  <p>
                    <strong>Info:</strong> General information about application
                    operation (recommended)
                  </p>
                  <p>
                    <strong>Warn:</strong> Only warnings and errors
                  </p>
                  <p>
                    <strong>Error:</strong> Only error messages
                  </p>
                </div>
                <p className='mt-2 text-zinc-300'>
                  Use "Info" for normal operation, "Debug" for troubleshooting,
                  or "Error" for minimal logging.
                </p>
              </div>
            }
          >
            Log Level
          </Tooltip>
        }
        htmlFor='log-level'
        className='last-form-item'
      >
        <SelectInput
          id='log-level'
          value={config.log_level}
          onChange={value => handleInputChange('log_level', value)}
          options={logLevelOptions}
        />
      </FormField>

      {/* Action Buttons */}
      <div className='flex space-x-3 pt-6'>
        <button
          onClick={handleSave}
          disabled={isSaving}
          className='px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed'
        >
          {isSaving ? 'Saving...' : 'Save Configuration'}
        </button>

        <button
          onClick={handleReset}
          disabled={isSaving}
          className='px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed'
        >
          Reset to Defaults
        </button>

        <button
          onClick={loadConfig}
          disabled={isSaving}
          className='px-4 py-2 border border-gray-600 text-gray-300 rounded hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed'
        >
          Reload
        </button>
      </div>
    </div>
  );
}
