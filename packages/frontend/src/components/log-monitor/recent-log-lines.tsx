interface RecentLogLinesProps {
  lastLines: string[];
}

export function RecentLogLines({ lastLines }: RecentLogLinesProps) {
  return (
    <div className='bg-zinc-900/50 p-4 border border-zinc-800'>
      <h3 className='text-lg font-semibold text-white mb-3'>
        Recent Log Lines
      </h3>
      <div className='space-y-1 max-h-32 overflow-y-auto'>
        {lastLines.length > 0 ? (
          lastLines.map((line, index) => (
            <div key={index} className='text-sm text-zinc-300 font-mono'>
              {line.length > 60 ? `${line.substring(0, 60)}...` : line}
            </div>
          ))
        ) : (
          <div className='text-zinc-500 text-sm'>No recent lines</div>
        )}
      </div>
    </div>
  );
}
