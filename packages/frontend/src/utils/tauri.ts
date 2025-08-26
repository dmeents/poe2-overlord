import { invoke } from '@tauri-apps/api/core';
import type { ProcessInfo } from '../types';
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
};
