export interface ProcessInfo {
  name: string;
  pid: number;
  running: boolean;
}

// Type for the GameProcessStatusChanged event payload from backend
export interface GameProcessStatusChangedEvent {
  old_status: GameProcessStatus | null;
  new_status: GameProcessStatus;
  is_state_change: boolean;
  timestamp: string;
}

// Type for the GameProcessStatus from backend
export interface GameProcessStatus {
  name: string;
  pid: number;
  running: boolean;
  detected_at: string;
}

export interface ProcessStatusProps {
  gameRunning: boolean;
  processInfo: ProcessInfo | null;
  onRefresh: () => void;
}
