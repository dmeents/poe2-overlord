/* eslint-disable react-refresh/only-export-components */

import { invoke } from '@tauri-apps/api/core';
import { createContext, useContext, useEffect, useState } from 'react';
import { useAppEventListener } from '@/hooks/useAppEventListener';
import type { ServerStatus, ServerStatusChangedEvent } from '@/types/server';
import { parseError } from '@/utils/error-handling';
import { EVENT_KEYS, type ExtractPayload } from '@/utils/events/registry';

interface ServerStatusContextValue {
  serverStatus: ServerStatus | null;
}

const ServerStatusContext = createContext<ServerStatusContextValue | undefined>(undefined);

export function ServerStatusProvider({ children }: React.PropsWithChildren) {
  const [serverStatus, setServerStatus] = useState<ServerStatus | null>(null);

  // Fetch initial server status on mount to avoid waiting for the first ping cycle
  useEffect(() => {
    const fetchInitialStatus = async () => {
      try {
        const status = await invoke<ServerStatus | null>('get_server_status');
        if (status) {
          setServerStatus(status);
        }
      } catch (err) {
        const error = parseError(err);
        console.error('Failed to fetch initial server status:', error.message);
      }
    };

    fetchInitialStatus();
  }, []);

  useAppEventListener([
    {
      eventType: EVENT_KEYS.ServerStatusChanged,
      handler: (payload: unknown) => {
        const { new_status } = payload as ExtractPayload<ServerStatusChangedEvent>;
        setServerStatus(new_status);
      },
    },
  ]);

  return (
    <ServerStatusContext.Provider value={{ serverStatus }}>{children}</ServerStatusContext.Provider>
  );
}

export function useServerStatus() {
  const context = useContext(ServerStatusContext);
  if (context === undefined) {
    throw new Error('useServerStatus must be used within ServerStatusProvider');
  }
  return context;
}
