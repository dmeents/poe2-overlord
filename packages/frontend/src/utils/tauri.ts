import { invoke } from '@tauri-apps/api/core';
import type {
  AppConfig,
  BackgroundImageOption,
  ZoneRefreshInterval,
  ZoneRefreshIntervalOption,
} from '@/types/app-config';
import { parseError } from '@/utils/error-handling';

export const tauriUtils = {
  // Configuration commands
  async getConfig(): Promise<AppConfig> {
    try {
      return await invoke<AppConfig>('get_config');
    } catch (err) {
      const error = parseError(err);
      console.error('Failed to get config:', error.message);
      throw error;
    }
  },

  async getDefaultConfig(): Promise<AppConfig> {
    try {
      return await invoke<AppConfig>('get_default_config');
    } catch (err) {
      const error = parseError(err);
      console.error('Failed to get default config:', error.message);
      throw error;
    }
  },

  async updateConfig(config: AppConfig): Promise<void> {
    try {
      await invoke('update_config', { newConfig: config });
    } catch (err) {
      const error = parseError(err);
      console.error('Failed to update config:', error.message);
      throw error;
    }
  },

  async resetConfigToDefaults(): Promise<void> {
    try {
      await invoke('reset_config_to_defaults');
    } catch (err) {
      const error = parseError(err);
      console.error('Failed to reset config to defaults:', error.message);
      throw error;
    }
  },

  // Helper methods for common config operations
  async getPoeClientLogPath(): Promise<string> {
    try {
      const config = await this.getConfig();
      return config.poe_client_log_path;
    } catch (err) {
      const error = parseError(err);
      console.error('Failed to get POE client log path:', error.message);
      throw error;
    }
  },

  async setPoeClientLogPath(path: string): Promise<void> {
    try {
      const config = await this.getConfig();
      const updatedConfig = { ...config, poe_client_log_path: path };
      await this.updateConfig(updatedConfig);
    } catch (err) {
      const error = parseError(err);
      console.error('Failed to set POE client log path:', error.message);
      throw error;
    }
  },

  async getLogLevel(): Promise<string> {
    try {
      const config = await this.getConfig();
      return config.log_level;
    } catch (err) {
      const error = parseError(err);
      console.error('Failed to get log level:', error.message);
      throw error;
    }
  },

  async setLogLevel(level: string): Promise<void> {
    try {
      const config = await this.getConfig();
      const updatedConfig = { ...config, log_level: level };
      await this.updateConfig(updatedConfig);
    } catch (err) {
      const error = parseError(err);
      console.error('Failed to set log level:', error.message);
      throw error;
    }
  },

  async getDefaultPoeClientLogPath(): Promise<string> {
    try {
      const defaultConfig = await this.getDefaultConfig();
      return defaultConfig.poe_client_log_path;
    } catch (err) {
      const error = parseError(err);
      console.error('Failed to get default POE client log path:', error.message);
      throw error;
    }
  },

  async resetPoeClientLogPathToDefault(): Promise<void> {
    try {
      const config = await this.getConfig();
      const defaultConfig = await this.getDefaultConfig();
      const updatedConfig = {
        ...config,
        poe_client_log_path: defaultConfig.poe_client_log_path,
      };
      await this.updateConfig(updatedConfig);
    } catch (err) {
      const error = parseError(err);
      console.error('Failed to reset POE client log path to default:', error.message);
      throw error;
    }
  },

  // Zone refresh interval commands
  async getZoneRefreshInterval(): Promise<ZoneRefreshInterval> {
    try {
      return await invoke<ZoneRefreshInterval>('get_zone_refresh_interval');
    } catch (err) {
      const error = parseError(err);
      console.error('Failed to get zone refresh interval:', error.message);
      throw error;
    }
  },

  async setZoneRefreshInterval(interval: ZoneRefreshInterval): Promise<void> {
    try {
      await invoke('set_zone_refresh_interval', { interval });
    } catch (err) {
      const error = parseError(err);
      console.error('Failed to set zone refresh interval:', error.message);
      throw error;
    }
  },

  async getZoneRefreshIntervalOptions(): Promise<ZoneRefreshIntervalOption[]> {
    try {
      return await invoke<ZoneRefreshIntervalOption[]>('get_zone_refresh_interval_options');
    } catch (err) {
      const error = parseError(err);
      console.error('Failed to get zone refresh interval options:', error.message);
      throw error;
    }
  },

  async getBackgroundImageOptions(): Promise<BackgroundImageOption[]> {
    try {
      return await invoke<BackgroundImageOption[]>('get_background_image_options');
    } catch (err) {
      const error = parseError(err);
      console.error('Failed to get background image options:', error.message);
      throw error;
    }
  },
};
