export interface ProcessInfo {
  running: boolean;
  pid?: number;
  startTime?: string;
}

export interface ProcessStatusProps {
  gameRunning: boolean;
  processInfo: ProcessInfo | null;
  onRefresh: () => void;
}
