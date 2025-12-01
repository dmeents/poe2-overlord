export interface ServerConnectionEvent {
  ip_address: string;
  port: number;
  timestamp: string;
}

export interface ServerStatus {
  ip_address: string;
  port: number;
  is_online: boolean;
  latency_ms: number | null;
  timestamp: string;
}

// Type for the ServerStatusChanged event payload from backend
// This matches the inner structure of the AppEvent::ServerStatusChanged variant
export interface ServerStatusChangedPayload {
  old_status: ServerStatus | null;
  new_status: ServerStatus;
  timestamp: string;
}

// Type for the full AppEvent enum wrapper as it comes from Tauri
// The backend sends AppEvent as a tagged union, so we need this wrapper
export interface ServerStatusChangedEvent {
  ServerStatusChanged: ServerStatusChangedPayload;
}
