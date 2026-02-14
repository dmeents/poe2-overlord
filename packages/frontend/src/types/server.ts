export interface ServerStatus {
  ip_address: string;
  port: number;
  is_online: boolean;
  latency_ms: number | null;
  timestamp: string;
}

// Type for the full AppEvent enum wrapper as it comes from Tauri
// The backend sends AppEvent as a tagged union, so we need this wrapper
export interface ServerStatusChangedEvent {
  ServerStatusChanged: {
    old_status: ServerStatus | null;
    new_status: ServerStatus;
    timestamp: string;
  };
}
