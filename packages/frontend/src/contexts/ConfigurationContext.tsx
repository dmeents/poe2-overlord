/* eslint-disable react-refresh/only-export-components */

import { createContext, useCallback, useContext, useEffect, useMemo, useState } from 'react';
import { useAppEventListener } from '@/hooks/useAppEventListener';
import type { AppConfig } from '@/types/app-config';
import {
  type ConfigurationChangedEvent,
  EVENT_KEYS,
  type ExtractPayload,
} from '@/utils/events/registry';
import { tauriUtils } from '@/utils/tauri';

interface ConfigurationContextValue {
  config: AppConfig | null;
  isLoading: boolean;
  updateConfig: (partial: Partial<AppConfig>) => Promise<void>;
}

const ConfigurationContext = createContext<ConfigurationContextValue | undefined>(undefined);

export function ConfigurationProvider({ children }: React.PropsWithChildren) {
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    tauriUtils
      .getConfig()
      .then(setConfig)
      .catch(err => console.error('Failed to load config:', err))
      .finally(() => setIsLoading(false));
  }, []);

  useAppEventListener(
    [
      {
        eventType: EVENT_KEYS.ConfigurationChanged,
        handler: (payload: unknown) => {
          const { new_config } = payload as ExtractPayload<ConfigurationChangedEvent>;
          setConfig(new_config);
        },
      },
    ],
    [],
  );

  const updateConfig = useCallback(
    async (partial: Partial<AppConfig>) => {
      if (!config) return;
      const updated = { ...config, ...partial };
      setConfig(updated);
      await tauriUtils.updateConfig(updated);
    },
    [config],
  );

  const value = useMemo(
    () => ({ config, isLoading, updateConfig }),
    [config, isLoading, updateConfig],
  );

  return <ConfigurationContext.Provider value={value}>{children}</ConfigurationContext.Provider>;
}

export function useConfiguration() {
  const context = useContext(ConfigurationContext);

  if (context === undefined) {
    throw new Error('useConfiguration must be used within ConfigurationProvider');
  }

  return context;
}
