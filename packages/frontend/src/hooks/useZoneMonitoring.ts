import type { ZoneChangeEvent } from '@/types';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

export const useZoneMonitoring = () => {
  const [currentZone, setCurrentZone] = useState<string | null>(null);
  const [lastZoneChange, setLastZoneChange] = useState<string | null>(null);
  const [isMonitoring, setIsMonitoring] = useState(false);

  useEffect(() => {
    // Listen for zone change events
    const unlistenZone = listen<ZoneChangeEvent>('log-zone-change', event => {
      const { zone_name, timestamp } = event.payload;
      setCurrentZone(zone_name);
      setLastZoneChange(timestamp);
      setIsMonitoring(true);
    });

    // Listen for process status to know when monitoring is active
    const unlistenProcess = listen('poe2-process-status', event => {
      const processInfo = event.payload as { running: boolean };
      setIsMonitoring(processInfo.running);

      // Clear zone when process stops
      if (!processInfo.running) {
        setCurrentZone(null);
        setLastZoneChange(null);
      }
    });

    // Cleanup listeners
    return () => {
      unlistenZone.then(f => f());
      unlistenProcess.then(f => f());
    };
  }, []);

  return {
    currentZone,
    lastZoneChange,
    isMonitoring,
  };
};
