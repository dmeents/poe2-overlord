import { invoke } from '@tauri-apps/api/core';
import { useCallback, useEffect, useState } from 'react';

export interface ServerStatus {
  ip_address: string;
  port: number;
  is_online: boolean;
  last_ping_ms: number | null;
  last_seen: string;
  last_checked: string;
}

export function useServerStatus() {
  const [serverStatus, setServerStatus] = useState<ServerStatus | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Get current server status
  const getServerStatus = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);

      const status = await invoke<ServerStatus | null>('get_server_status');
      setServerStatus(status);
    } catch (err) {
      setError(
        err instanceof Error ? err.message : 'Failed to get server status'
      );
      console.error('Error getting server status:', err);
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Get last known server address
  const getLastKnownServer = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);

      const serverInfo = await invoke<[string, number] | null>(
        'get_last_known_server'
      );
      return serverInfo;
    } catch (err) {
      setError(
        err instanceof Error ? err.message : 'Failed to get last known server'
      );
      console.error('Error getting last known server:', err);
      return null;
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Ping the current server
  const pingServer = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);

      const pingMs = await invoke<number | null>('ping_server');

      if (pingMs !== null) {
        // Update the local state with the new ping result
        setServerStatus(prev =>
          prev
            ? {
                ...prev,
                is_online: true,
                last_ping_ms: pingMs,
                last_checked: new Date().toISOString(),
              }
            : null
        );
      }

      return pingMs;
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to ping server');
      console.error('Error pinging server:', err);

      // Update status to offline if ping fails
      setServerStatus(prev =>
        prev
          ? {
              ...prev,
              is_online: false,
              last_ping_ms: null,
              last_checked: new Date().toISOString(),
            }
          : null
      );

      return null;
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Start server monitoring
  const startServerMonitoring = useCallback(async () => {
    try {
      setError(null);
      await invoke('start_server_monitoring');
    } catch (err) {
      setError(
        err instanceof Error ? err.message : 'Failed to start server monitoring'
      );
      console.error('Error starting server monitoring:', err);
    }
  }, []);

  // Refresh server status
  const refreshStatus = useCallback(async () => {
    await getServerStatus();
  }, [getServerStatus]);

  // Auto-refresh status every 30 seconds
  useEffect(() => {
    const interval = setInterval(() => {
      if (serverStatus) {
        pingServer();
      }
    }, 30000);

    return () => clearInterval(interval);
  }, [pingServer, serverStatus]);

  // Initial load
  useEffect(() => {
    getServerStatus();
  }, [getServerStatus]);

  return {
    serverStatus,
    isLoading,
    error,
    getServerStatus,
    getLastKnownServer,
    pingServer,
    startServerMonitoring,
    refreshStatus,
  };
}
