import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { ProcessInfo } from "../types";
import { POE2_CONFIG } from "./constants";

export const tauriUtils = {
  async checkPoe2Process(): Promise<ProcessInfo> {
    try {
      return await invoke<ProcessInfo>(POE2_CONFIG.COMMAND_NAME);
    } catch (error) {
      console.error("Failed to check POE2 process:", error);
      throw error;
    }
  },

  async minimizeWindow(): Promise<void> {
    try {
      const window = getCurrentWindow();
      await window.minimize();
    } catch (error) {
      console.error("Failed to minimize window:", error);
      throw error;
    }
  },

  async closeWindow(): Promise<void> {
    try {
      const window = getCurrentWindow();
      await window.close();
    } catch (error) {
      console.error("Failed to close window:", error);
      throw error;
    }
  },
};
