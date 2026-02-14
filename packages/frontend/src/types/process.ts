// Type for the GameProcessStatus from backend
export interface GameProcessStatus {
  name: string;
  pid: number;
  running: boolean;
  detected_at: string;
}

// Type for the full AppEvent enum wrapper as it comes from Tauri
// The backend sends AppEvent as a tagged union, so we need this wrapper
export interface GameProcessStatusChangedEvent {
  GameProcessStatusChanged: {
    old_status: GameProcessStatus | null;
    new_status: GameProcessStatus;
    is_state_change: boolean;
    timestamp: string;
  };
}
