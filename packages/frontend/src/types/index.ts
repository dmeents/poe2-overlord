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
