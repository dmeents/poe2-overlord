import type { SceneChangeEvent } from '@/types';
import { getSceneEventTimestamp } from '@/types';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

export const useZoneMonitoring = () => {
  const [currentZone, setCurrentZone] = useState<string | null>(null);
  const [currentAct, setCurrentAct] = useState<string | null>(null);
  const [lastZoneChange, setLastZoneChange] = useState<string | null>(null);
  const [lastActChange, setLastActChange] = useState<string | null>(null);
  const [isMonitoring, setIsMonitoring] = useState(false);

  useEffect(() => {
    // Listen for unified scene change events
    const unlistenScene = listen<SceneChangeEvent>(
      'log-scene-change',
      event => {
        const sceneEvent = event.payload;
        setIsMonitoring(true);

        const timestamp = getSceneEventTimestamp(sceneEvent);

        switch (sceneEvent.type) {
          case 'Zone':
            setCurrentZone(sceneEvent.zone_name);
            setLastZoneChange(timestamp);
            break;
          case 'Act':
            setCurrentAct(sceneEvent.act_name);
            setLastActChange(timestamp);
            break;
          case 'Hideout':
            // For hideouts, we might want to clear the current zone/act
            // or handle it differently depending on requirements
            setCurrentZone(sceneEvent.hideout_name);
            setLastZoneChange(timestamp);
            break;
        }
      }
    );

    // Listen for process status to know when monitoring is active
    const unlistenProcess = listen('game-process-status', event => {
      const processInfo = event.payload as { running: boolean };
      setIsMonitoring(processInfo.running);

      // Clear zone and act when process stops
      if (!processInfo.running) {
        setCurrentZone(null);
        setCurrentAct(null);
        setLastZoneChange(null);
        setLastActChange(null);
      }
    });

    // Cleanup listeners
    return () => {
      unlistenScene.then(f => f());
      unlistenProcess.then(f => f());
    };
  }, []);

  return {
    currentZone,
    currentAct,
    lastZoneChange,
    lastActChange,
    isMonitoring,
  };
};
