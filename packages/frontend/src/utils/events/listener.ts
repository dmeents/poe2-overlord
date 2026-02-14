import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { APP_EVENTS, type AppEventPayload, type AppEventRegistry } from './registry';

/**
 * Type-safe wrapper around Tauri's listen that automatically unwraps AppEvent variants.
 */
export async function listenToAppEvent<K extends keyof AppEventRegistry>(
  eventType: K,
  handler: (payload: AppEventPayload<K>) => void,
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
