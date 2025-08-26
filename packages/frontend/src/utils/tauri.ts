import type { ProcessInfo, AppConfig } from '@/types';
import { invoke } from '@tauri-apps/api/core';
import { POE2_CONFIG } from './constants';

export const tauriUtils = {
  async checkPoe2Process(): Promise<ProcessInfo> {
    try {
      return await invoke<ProcessInfo>(POE2_CONFIG.COMMAND_NAME);
    } catch (error) {
      console.error('Failed to check POE2 process:', error);
      throw error;
    }
  },

  // Configuration commands
  async getConfig(): Promise<AppConfig> {
    try {
      return await invoke<AppConfig>('get_config');
    } catch (error) {
      console.error('Failed to get config:', error);
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

  async getPoeClientLogPath(): Promise<string> {
    try {
      return await invoke<string>('get_poe_client_log_path');
    } catch (error) {
      console.error('Failed to get POE client log path:', error);
      throw error;
    }
  },

  async setPoeClientLogPath(path: string): Promise<void> {
    try {
      await invoke('set_poe_client_log_path', { path });
    } catch (error) {
      console.error('Failed to set POE client log path:', error);
      throw error;
    }
  },

  async getAutoStartMonitoring(): Promise<boolean> {
    try {
      return await invoke<boolean>('get_auto_start_monitoring');
    } catch (error) {
      console.error('Failed to get auto-start monitoring setting:', error);
      throw error;
    }
  },

  async setAutoStartMonitoring(enabled: boolean): Promise<void> {
    try {
      await invoke('set_auto_start_monitoring', { enabled });
    } catch (error) {
      console.error('Failed to set auto-start monitoring setting:', error);
      throw error;
    }
  },

  async getLogLevel(): Promise<string> {
    try {
      return await invoke<string>('get_log_level');
    } catch (error) {
      console.error('Failed to get log level:', error);
      throw error;
    }
  },

  async setLogLevel(level: string): Promise<void> {
    try {
      await invoke('set_log_level', { level });
    } catch (error) {
      console.error('Failed to set log level:', error);
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
};
