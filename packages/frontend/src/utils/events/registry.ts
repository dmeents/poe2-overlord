import type { GameProcessStatusChangedEvent } from '@/types/process';
import type { ServerStatusChangedEvent } from '@/types/server';

/**
 * Centralized registry mapping AppEvent variants to their Tauri event names.
 * Must match the backend event names exactly.
 */
export const APP_EVENTS = {
  GameProcessStatusChanged: 'game-process-status-changed',
  ServerStatusChanged: 'server-status-changed',
  CharacterTrackingDataUpdated: 'character-tracking-data-updated',
  LocationStateChanged: 'location-state-changed',
  SceneChangeDetected: 'scene-change-detected',
  ActChangeDetected: 'act-change-detected',
  ZoneChangeDetected: 'zone-change-detected',
  HideoutChangeDetected: 'hideout-change-detected',
  ConfigurationChanged: 'configuration-changed',
  ServerPingCompleted: 'server-ping-completed',
  WalkthroughProgressUpdated: 'walkthrough-progress-updated',
  WalkthroughStepCompleted: 'walkthrough-step-completed',
  WalkthroughStepAdvanced: 'walkthrough-step-advanced',
  WalkthroughCampaignCompleted: 'walkthrough-campaign-completed',
  SystemError: 'system-error',
  SystemShutdown: 'system-shutdown',
} as const;

/** Extracts the payload type from an AppEvent tagged union wrapper. */
export type ExtractPayload<T> = T extends { [K in keyof T]: infer P }
  ? P
  : never;

/** Maps AppEvent variant names to their TypeScript types. */
export type AppEventRegistry = {
  GameProcessStatusChanged: GameProcessStatusChangedEvent;
  ServerStatusChanged: ServerStatusChangedEvent;
  // Add other event types as they're defined
};

/** Gets the payload type for a specific event variant. */
export type AppEventPayload<K extends keyof AppEventRegistry> = ExtractPayload<
  AppEventRegistry[K]
>;

export function isAppEventKey(key: string): key is keyof typeof APP_EVENTS {
  return key in APP_EVENTS;
}
