import type { ProcessInfo } from '@/types';
import { StatusDot } from './status-dot.component';

interface ProcessStatusComponentProps {
  processInfo: ProcessInfo | null;
  onRefresh: () => void;
}

export function ProcessStatusComponent({
  processInfo,
  onRefresh,
}: ProcessStatusComponentProps) {
  return (
    <div>
      <div>
        <div>
          <StatusDot isOnline={processInfo?.running || false} />
          <span>Path of Exile 2</span>
        </div>
        <div>
          <span>{processInfo?.running ? 'Running' : 'Stopped'}</span>
        </div>
      </div>
      <div>
        <span>PID: {processInfo?.pid || 'N/A'}</span>
        <span>Name: {processInfo?.name || 'N/A'}</span>
      </div>
    </div>
  );
}
