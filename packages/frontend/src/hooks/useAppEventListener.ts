import type { UnlistenFn } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';
import { listenToAppEvent } from '@/utils/events/listener';
import type { AppEventPayload, AppEventRegistry } from '@/utils/events/registry';

export function useAppEventListener(
  listeners: Array<{
    eventType: keyof AppEventRegistry;
    handler: (payload: unknown) => void;
  }>,
  deps: React.DependencyList = [],
) {
  const [isListening, setIsListening] = useState(false);

  useEffect(() => {
    let cancelled = false;
    const unlistenFns: UnlistenFn[] = [];

    const setup = async () => {
      try {
        const results = await Promise.all(
          listeners.map(({ eventType, handler }) =>
            listenToAppEvent(
              eventType as keyof AppEventRegistry,
              handler as (payload: AppEventPayload<keyof AppEventRegistry>) => void,
            ),
          ),
        );

        if (cancelled) {
          // biome-ignore lint/suspicious/useIterableCallbackReturn: unlisten fns return void, forEach is correct here
          results.forEach(fn => fn());
          return;
        }

        unlistenFns.push(...results);
        setIsListening(true);
      } catch (error) {
        console.error('Failed to set up event listeners:', error);
        setIsListening(false);
      }
    };

    setup();

    return () => {
      cancelled = true;
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
    // biome-ignore lint/correctness/useExhaustiveDependencies: intentional dynamic deps pattern — callers control the dependency array
  }, deps);

  return { isListening };
}
