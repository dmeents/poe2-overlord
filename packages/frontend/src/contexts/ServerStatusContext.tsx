/* eslint-disable react-refresh/only-export-components */

import { createContext, useContext, useState } from 'react';
import { useAppEventListener } from '@/hooks/useAppEventListener';
import type { ServerStatus, ServerStatusChangedEvent } from '@/types/server';
import { EVENT_KEYS, type ExtractPayload } from '@/utils/events/registry';

interface ServerStatusContextValue {
  serverStatus: ServerStatus | null;
  isListening: boolean;
}

const ServerStatusContext = createContext<ServerStatusContextValue | undefined>(undefined);

export function ServerStatusProvider({ children }: React.PropsWithChildren) {
  const [serverStatus, setServerStatus] = useState<ServerStatus | null>(null);

  const { isListening } = useAppEventListener([
    {
      eventType: EVENT_KEYS.ServerStatusChanged,
      handler: (payload: unknown) => {
        const { new_status } = payload as ExtractPayload<ServerStatusChangedEvent>;
        setServerStatus(new_status);
      },
    },
  ]);

  return (
    <ServerStatusContext.Provider value={{ serverStatus, isListening }}>
      {children}
    </ServerStatusContext.Provider>
  );
}

export function useServerStatus() {
  const context = useContext(ServerStatusContext);
  if (context === undefined) {
    throw new Error('useServerStatus must be used within ServerStatusProvider');
  }
  return context;
}
