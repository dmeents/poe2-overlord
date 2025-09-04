import { Button } from '../button';

interface RecentLogLinesProps {
  lastLines: string[];
  onRefresh: () => void;
}

export function RecentLogLines({ lastLines, onRefresh }: RecentLogLinesProps) {
  return (
    <div className='bg-zinc-900/50 p-4 border border-zinc-800'>
      <div className='flex items-center justify-between mb-3'>
        <h3 className='text-lg font-semibold text-white'>Recent Log Lines</h3>
        <Button onClick={onRefresh} variant='outline' size='sm'>
          Refresh
        </Button>
      </div>
      <div className='space-y-1 max-h-32 overflow-auto'>
        {lastLines.length > 0 ? (
          lastLines.map((line, index) => (
            <div key={index} className='text-sm text-zinc-300 font-mono'>
              {line}
            </div>
          ))
        ) : (
          <div className='text-zinc-500 text-sm'>No recent lines</div>
        )}
      </div>
    </div>
  );
}
