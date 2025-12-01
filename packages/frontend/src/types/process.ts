// Type for the GameProcessStatus from backend
export interface GameProcessStatus {
  name: string;
  pid: number;
  running: boolean;
  detected_at: string;
}

// Type for the GameProcessStatusChanged event payload from backend
// This matches the inner structure of the AppEvent::GameProcessStatusChanged variant
export interface GameProcessStatusChangedPayload {
  old_status: GameProcessStatus | null;
  new_status: GameProcessStatus;
  is_state_change: boolean;
  timestamp: string;
}

// Type for the full AppEvent enum wrapper as it comes from Tauri
// The backend sends AppEvent as a tagged union, so we need this wrapper
export interface GameProcessStatusChangedEvent {
  GameProcessStatusChanged: GameProcessStatusChangedPayload;
}
