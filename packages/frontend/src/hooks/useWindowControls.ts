import { useState } from "react";
import { tauriUtils } from "../utils/tauri";

export const useWindowControls = () => {
  const [isMinimized, setIsMinimized] = useState(false);

  const toggleMinimize = () => {
    setIsMinimized(!isMinimized);
  };

  const minimizeWindow = async () => {
    try {
      await tauriUtils.minimizeWindow();
      setIsMinimized(true);
    } catch (error) {
      console.error("Failed to minimize window:", error);
    }
  };

  const closeWindow = async () => {
    try {
      await tauriUtils.closeWindow();
    } catch (error) {
      console.error("Failed to close window:", error);
    }
  };

  return {
    isMinimized,
    toggleMinimize,
    minimizeWindow,
    closeWindow,
  };
};
