import React from 'react';

export interface ProcessInfo {
  name: string;
  pid: number;
  running: boolean;
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
  auto_start_monitoring: boolean;
  log_level: string;
}
