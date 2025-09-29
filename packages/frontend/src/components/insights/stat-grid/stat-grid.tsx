interface StatItem {
  value: string | number;
  label: string;
  subtext?: string;
}

interface StatGridProps {
  stats: StatItem[];
  columns?: 2 | 3 | 4;
  className?: string;
}

export function StatGrid({
  stats,
  columns = 2,
  className = '',
}: StatGridProps) {
  const gridClasses = {
    2: 'grid grid-cols-2 gap-4',
    3: 'grid grid-cols-3 gap-4',
    4: 'grid grid-cols-4 gap-4',
  };

  return (
    <div className={`${gridClasses[columns]} ${className}`}>
      {stats.map((stat, index) => (
        <div
          key={index}
          className='text-center p-4 bg-zinc-900/80 border border-zinc-700/50'
        >
          <div className='text-2xl font-bold text-white mb-1'>{stat.value}</div>
          <div className='text-sm text-zinc-400 uppercase tracking-wide'>
            {stat.label}
          </div>
          {stat.subtext && (
            <div className='text-xs text-zinc-500 mt-1'>{stat.subtext}</div>
          )}
        </div>
      ))}
    </div>
  );
}
