import { useEffect, useState } from 'react';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { listenToAppEvent } from '@/utils/events/listener';
import type {
  AppEventPayload,
  AppEventRegistry,
} from '@/utils/events/registry';

/**
 * React hook for listening to AppEvent variants with automatic lifecycle management.
 */
export function useAppEventListener<K extends keyof AppEventRegistry>(
  eventType: K,
  handler: (payload: AppEventPayload<K>) => void,
  deps: React.DependencyList = []
) {
  const [isListening, setIsListening] = useState(false);

  useEffect(() => {
    let unlistenFn: UnlistenFn | null = null;

    const setup = async () => {
      try {
        unlistenFn = await listenToAppEvent(eventType, handler);
        setIsListening(true);
      } catch (error) {
        console.error(`Failed to set up listener for ${eventType}:`, error);
        setIsListening(false);
      }
    };

    setup();

    return () => {
      if (unlistenFn) {
        try {
          unlistenFn();
        } catch (error) {
          console.error(`Error cleaning up listener for ${eventType}:`, error);
        }
      }
      setIsListening(false);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [eventType, ...deps]);

  return { isListening };
}

/** React hook for listening to multiple AppEvent variants simultaneously. */
export function useAppEventListeners(
  listeners: Array<{
    eventType: keyof AppEventRegistry;
    handler: (payload: unknown) => void;
  }>,
  deps: React.DependencyList = []
) {
  const [isListening, setIsListening] = useState(false);

  useEffect(() => {
    const unlistenFns: UnlistenFn[] = [];

    const setup = async () => {
      try {
        const promises = listeners.map(({ eventType, handler }) =>
          listenToAppEvent(
            eventType as keyof AppEventRegistry,
            handler as (
              payload: AppEventPayload<keyof AppEventRegistry>
            ) => void
          )
        );

        const results = await Promise.all(promises);
        unlistenFns.push(...results);
        setIsListening(true);
      } catch (error) {
        console.error('Failed to set up event listeners:', error);
        setIsListening(false);
      }
    };

    setup();

    return () => {
      unlistenFns.forEach(unlisten => {
        try {
          unlisten();
        } catch (error) {
          console.error('Error cleaning up event listener:', error);
        }
      });
      setIsListening(false);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [...deps]);

  return { isListening };
}
