export interface ProcessInfo {
  name: string;
  pid: number;
  running: boolean;
}

export interface OverlayState {
  visible: boolean;
  poe2Running: boolean;
  processInfo: ProcessInfo | null;
  isDragging: boolean;
  isMinimized: boolean;
}

export interface WindowControlsProps {
  isMinimized: boolean;
  onToggleMinimize: () => void;
  onMinimize: () => void;
  onClose: () => void;
}

export interface TitleBarProps {
  poe2Running: boolean;
  processInfo: ProcessInfo | null;
  windowControls: WindowControlsProps;
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
