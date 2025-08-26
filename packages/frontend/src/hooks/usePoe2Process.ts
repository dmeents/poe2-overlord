import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import type { ProcessInfo } from "../types";
import { POE2_CONFIG, tauriUtils } from "../utils";

export const usePoe2Process = () => {
  const [processInfo, setProcessInfo] = useState<ProcessInfo | null>(null);
  const [poe2Running, setPoe2Running] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    // Listen for POE2 process status updates from Rust backend
    const unsubscribe = listen<ProcessInfo>(POE2_CONFIG.EVENT_NAME, (event) => {
      setProcessInfo(event.payload);
      setPoe2Running(event.payload.running);
    });

    // Initial process check
    checkPoe2Process();

    return () => {
      unsubscribe.then((fn) => fn());
    };
  }, []);

  const checkPoe2Process = async () => {
    try {
      setIsLoading(true);
      const info = await tauriUtils.checkPoe2Process();
      setProcessInfo(info);
      setPoe2Running(info.running);
    } catch (error) {
      console.error("Failed to check POE2 process:", error);
    } finally {
      setIsLoading(false);
    }
  };

  return {
    processInfo,
    poe2Running,
    isLoading,
    checkPoe2Process,
  };
};
