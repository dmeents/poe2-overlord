import type { GameProcessStatusChangedEvent } from '@/types/process';
import type { ServerStatusChangedEvent } from '@/types/server';
import type { CharacterData } from '@/types/character';
import type { WalkthroughStepResult } from '@/types/walkthrough';

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

/** Event type for character tracking data updates */
export type CharacterTrackingDataUpdatedEvent = {
  CharacterTrackingDataUpdated: {
    character_id: string;
    data: CharacterData;
    timestamp: string;
  };
};

/** Event type for walkthrough step completed */
export type WalkthroughStepCompletedEvent = {
  WalkthroughStepCompleted: {
    character_id: string;
    step: WalkthroughStepResult;
    timestamp: string;
  };
};

/** Event type for walkthrough step advanced */
export type WalkthroughStepAdvancedEvent = {
  WalkthroughStepAdvanced: {
    character_id: string;
    from_step_id: string | null;
    to_step_id: string | null;
    timestamp: string;
  };
};

/** Event type for walkthrough campaign completed */
export type WalkthroughCampaignCompletedEvent = {
  WalkthroughCampaignCompleted: {
    character_id: string;
    timestamp: string;
  };
};

/** Maps AppEvent variant names to their TypeScript types. */
export type AppEventRegistry = {
  GameProcessStatusChanged: GameProcessStatusChangedEvent;
  ServerStatusChanged: ServerStatusChangedEvent;
  CharacterTrackingDataUpdated: CharacterTrackingDataUpdatedEvent;

  WalkthroughStepCompleted: WalkthroughStepCompletedEvent;
  WalkthroughStepAdvanced: WalkthroughStepAdvancedEvent;
  WalkthroughCampaignCompleted: WalkthroughCampaignCompletedEvent;
  // Add other event types as they're defined
};

/** Gets the payload type for a specific event variant. */
export type AppEventPayload<K extends keyof AppEventRegistry> = ExtractPayload<
  AppEventRegistry[K]
>;

export function isAppEventKey(key: string): key is keyof typeof APP_EVENTS {
  return key in APP_EVENTS;
}
