import React from 'react';

export interface ProcessInfo {
  name: string;
  pid: number;
  running: boolean;
}

export interface ZoneChangeEvent {
  zone_name: string;
  timestamp: string;
}

export interface ActChangeEvent {
  act_name: string;
  timestamp: string;
}

export interface SceneChangeEvent {
  type: 'Zone' | 'Act';
  zone_name?: string;
  act_name?: string;
  timestamp: string;
}

// Legacy compatibility - create SceneChangeEvent from ZoneChangeEvent
export const createZoneSceneEvent = (
  event: ZoneChangeEvent
): SceneChangeEvent => ({
  type: 'Zone',
  zone_name: event.zone_name,
  timestamp: event.timestamp,
});

// Legacy compatibility - create SceneChangeEvent from ActChangeEvent
export const createActSceneEvent = (
  event: ActChangeEvent
): SceneChangeEvent => ({
  type: 'Act',
  act_name: event.act_name,
  timestamp: event.timestamp,
});

export interface ProcessStatusProps {
  poe2Running: boolean;
  processInfo: ProcessInfo | null;
  onRefresh: () => void;
}

export interface QuickActionProps {
  icon: React.ReactNode;
  label: string;
  onClick?: () => void;
}

export interface InfoPanelProps {
  title: string;
  description: string;
  icon: React.ReactNode;
}

export interface FooterProps {
  version: string;
  technology: string;
}

export interface AppConfig {
  poe_client_log_path: string;
  log_level: string;
}
