import type { ConfigurationChangedEvent as ConfigurationChangedEventData } from '@/types/app-config';
import type { CharacterData } from '@/types/character';
import type { LevelingStats } from '@/types/leveling';
import type { GameProcessStatusChangedEvent } from '@/types/process';
import type { ServerStatusChangedEvent } from '@/types/server';
import type { WalkthroughStepResult } from '@/types/walkthrough';

/**
 * Centralized registry mapping AppEvent variants to their Tauri event names.
 * Must match the backend event names exactly.
 */
export const APP_EVENTS = {
  GameProcessStatusChanged: 'game-process-status-changed',
  ServerStatusChanged: 'server-status-changed',
  CharacterUpdated: 'character-updated',
  CharacterDeleted: 'character-deleted',
  ConfigurationChanged: 'configuration-changed',
  LevelingStatsUpdated: 'leveling-stats-updated',

  WalkthroughStepCompleted: 'walkthrough-step-completed',
  WalkthroughStepAdvanced: 'walkthrough-step-advanced',
  WalkthroughCampaignCompleted: 'walkthrough-campaign-completed',
  SystemError: 'system-error',
  SystemShutdown: 'system-shutdown',
} as const;

/**
 * Type-safe event keys for use with useAppEventListener.
 * Maps each event to its PascalCase key name.
 * The `satisfies` clause ensures this stays in sync with APP_EVENTS.
 */
export const EVENT_KEYS = {
  GameProcessStatusChanged: 'GameProcessStatusChanged',
  ServerStatusChanged: 'ServerStatusChanged',
  CharacterUpdated: 'CharacterUpdated',
  CharacterDeleted: 'CharacterDeleted',
  ConfigurationChanged: 'ConfigurationChanged',
  LevelingStatsUpdated: 'LevelingStatsUpdated',
  WalkthroughStepCompleted: 'WalkthroughStepCompleted',
  WalkthroughStepAdvanced: 'WalkthroughStepAdvanced',
  WalkthroughCampaignCompleted: 'WalkthroughCampaignCompleted',
  SystemError: 'SystemError',
  SystemShutdown: 'SystemShutdown',
} as const satisfies { [K in keyof typeof APP_EVENTS]: K };

/** Extracts the payload type from an AppEvent tagged union wrapper. */
export type ExtractPayload<T> = T extends { [K in keyof T]: infer P } ? P : never;

/** Event type for character updates */
export type CharacterUpdatedEvent = {
  CharacterUpdated: {
    character_id: string;
    data: CharacterData;
    timestamp: string;
  };
};

/**
 * Event type for character deletion.
 */
export type CharacterDeletedEvent = {
  CharacterDeleted: {
    character_id: string;
    timestamp: string;
  };
};

/** Event type for configuration changes */
export type ConfigurationChangedEvent = {
  ConfigurationChanged: ConfigurationChangedEventData;
};

/** Event type for leveling stats updates */
export type LevelingStatsUpdatedEvent = {
  LevelingStatsUpdated: {
    character_id: string;
    stats: LevelingStats;
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
  CharacterUpdated: CharacterUpdatedEvent;
  CharacterDeleted: CharacterDeletedEvent;
  ConfigurationChanged: ConfigurationChangedEvent;
  LevelingStatsUpdated: LevelingStatsUpdatedEvent;

  WalkthroughStepCompleted: WalkthroughStepCompletedEvent;
  WalkthroughStepAdvanced: WalkthroughStepAdvancedEvent;
  WalkthroughCampaignCompleted: WalkthroughCampaignCompletedEvent;
};

/** Gets the payload type for a specific event variant. */
export type AppEventPayload<K extends keyof AppEventRegistry> = ExtractPayload<AppEventRegistry[K]>;
