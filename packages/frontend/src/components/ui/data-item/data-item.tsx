import type { DataItemProps } from './data-item.types';

export function DataItem({
  label,
  value,
  subValue,
  className = '',
  color,
  icon,
}: DataItemProps) {
  return (
    <div
      className={`flex items-center justify-between h-12 px-2 border-l-2 border-zinc-700 hover:border-zinc-700 hover:bg-zinc-900/20 transition-all  odd:bg-zinc-900/60 ${className}`}
      style={{ borderLeftColor: color }}
    >
      <div className='flex items-center gap-2'>
        {icon && (
          <div className='flex-shrink-0 text-zinc-400 text-sm'>{icon}</div>
        )}
        <div className='text-zinc-300 text-sm'>{label}</div>
      </div>
      <div className='text-right pl-3'>
        <div className='text-zinc-100 text-sm font-semibold'>{value}</div>
        {subValue && (
          <div className='text-xs text-zinc-500 leading-tight'>{subValue}</div>
        )}
      </div>
    </div>
  );
}
