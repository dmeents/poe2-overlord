import type { AppConfig } from '@/types/app-config';
import { invoke } from '@tauri-apps/api/core';

export const tauriUtils = {
  // Configuration commands
  async getConfig(): Promise<AppConfig> {
    try {
      return await invoke<AppConfig>('get_config');
    } catch (error) {
      console.error('Failed to get config:', error);
      throw error;
    }
  },

  async getDefaultConfig(): Promise<AppConfig> {
    try {
      return await invoke<AppConfig>('get_default_config');
    } catch (error) {
      console.error('Failed to get default config:', error);
      throw error;
    }
  },

  async updateConfig(config: AppConfig): Promise<void> {
    try {
      await invoke('update_config', { newConfig: config });
    } catch (error) {
      console.error('Failed to update config:', error);
      throw error;
    }
  },

  async resetConfigToDefaults(): Promise<void> {
    try {
      await invoke('reset_config_to_defaults');
    } catch (error) {
      console.error('Failed to reset config to defaults:', error);
      throw error;
    }
  },

  // Helper methods for common config operations
  async getPoeClientLogPath(): Promise<string> {
    try {
      const config = await this.getConfig();
      return config.poe_client_log_path;
    } catch (error) {
      console.error('Failed to get POE client log path:', error);
      throw error;
    }
  },

  async setPoeClientLogPath(path: string): Promise<void> {
    try {
      const config = await this.getConfig();
      const updatedConfig = { ...config, poe_client_log_path: path };
      await this.updateConfig(updatedConfig);
    } catch (error) {
      console.error('Failed to set POE client log path:', error);
      throw error;
    }
  },

  async getLogLevel(): Promise<string> {
    try {
      const config = await this.getConfig();
      return config.log_level;
    } catch (error) {
      console.error('Failed to get log level:', error);
      throw error;
    }
  },

  async setLogLevel(level: string): Promise<void> {
    try {
      const config = await this.getConfig();
      const updatedConfig = { ...config, log_level: level };
      await this.updateConfig(updatedConfig);
    } catch (error) {
      console.error('Failed to set log level:', error);
      throw error;
    }
  },

  async getDefaultPoeClientLogPath(): Promise<string> {
    try {
      const defaultConfig = await this.getDefaultConfig();
      return defaultConfig.poe_client_log_path;
    } catch (error) {
      console.error('Failed to get default POE client log path:', error);
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
    } catch (error) {
      console.error('Failed to reset POE client log path to default:', error);
      throw error;
    }
  },
};
