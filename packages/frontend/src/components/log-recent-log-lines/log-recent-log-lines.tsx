import { Button } from '../button';
import { logRecentLogLinesStyles } from './log-recent-log-lines.styles';

interface RecentLogLinesProps {
  lastLines: string[];
  onRefresh: () => void;
}

export function RecentLogLines({ lastLines, onRefresh }: RecentLogLinesProps) {
  return (
    <div className={logRecentLogLinesStyles.container}>
      <div className={logRecentLogLinesStyles.header}>
        <h3 className={logRecentLogLinesStyles.title}>Recent Log Lines</h3>
        <Button onClick={onRefresh} variant='outline' size='sm'>
          Refresh
        </Button>
      </div>
      <div className={logRecentLogLinesStyles.logLinesContainer}>
        {lastLines.length > 0 ? (
          lastLines.map((line, index) => (
            <div key={index} className={logRecentLogLinesStyles.logLine}>
              {line}
            </div>
          ))
        ) : (
          <div className={logRecentLogLinesStyles.emptyState}>
            No recent lines
          </div>
        )}
      </div>
    </div>
  );
}
