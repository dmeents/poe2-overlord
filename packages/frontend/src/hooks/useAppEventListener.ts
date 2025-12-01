import { useEffect, useState } from 'react';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { listenToAppEvent } from '@/utils/events/listener';
import type {
  AppEventPayload,
  AppEventRegistry,
} from '@/utils/events/registry';

export function useAppEventListener(
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
