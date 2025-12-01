import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import {
  APP_EVENTS,
  type AppEventPayload,
  type AppEventRegistry,
} from './registry';

/**
 * Type-safe wrapper around Tauri's listen that automatically unwraps AppEvent variants.
 */
export async function listenToAppEvent<K extends keyof AppEventRegistry>(
  eventType: K,
  handler: (payload: AppEventPayload<K>) => void
): Promise<UnlistenFn> {
  const eventName = APP_EVENTS[eventType];

  return listen<AppEventRegistry[K]>(eventName, event => {
    // Cast through unknown to handle the tagged union structure
    const payload = event.payload as unknown as Record<string, unknown>;
    const variantPayload = payload[eventType];

    if (variantPayload) {
      handler(variantPayload as AppEventPayload<K>);
    }
  });
}

/** Sets up multiple event listeners at once. */
export async function setupAppEventListeners<K extends keyof AppEventRegistry>(
  listeners: Array<{
    eventType: K;
    handler: (payload: AppEventPayload<K>) => void;
  }>
): Promise<UnlistenFn[]> {
  const unlistenPromises = listeners.map(({ eventType, handler }) =>
    listenToAppEvent(eventType, handler)
  );

  return Promise.all(unlistenPromises);
}

/** Safely cleans up all unlisten functions. */
export function cleanupListeners(unlistenFns: UnlistenFn[]): void {
  unlistenFns.forEach(unlisten => {
    try {
      unlisten();
    } catch (error) {
      console.error('Error cleaning up event listener:', error);
    }
  });
}
