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
