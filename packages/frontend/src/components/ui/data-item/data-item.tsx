interface DataItemProps {
  label: string;
  value: string | number;
  subValue?: string;
  className?: string;
}

export function DataItem({
  label,
  value,
  subValue,
  className = '',
}: DataItemProps) {
  return (
    <div
      className={`flex items-center justify-between p-3 bg-zinc-900/80 border border-zinc-700/50 ${className}`}
    >
      <span className='text-zinc-300 font-medium'>{label}</span>
      <div className='text-right'>
        <div className='text-zinc-400 text-sm'>{value}</div>
        {subValue && <div className='text-xs text-zinc-500'>{subValue}</div>}
      </div>
    </div>
  );
}
