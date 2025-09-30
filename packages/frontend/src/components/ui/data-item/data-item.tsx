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
      className={`flex items-center justify-between p-3 bg-zinc-900/80 border border-zinc-700/50 ${className}`}
    >
      <div className='flex items-start gap-3'>
        {icon && <div className='flex-shrink-0 mt-0.5'>{icon}</div>}
        {color && !icon && (
          <div
            className='w-3 h-3 rounded-sm flex-shrink-0 mt-1'
            style={{ backgroundColor: color }}
          />
        )}
        <div className='text-zinc-300 font-medium'>{label}</div>
      </div>
      <div className='text-right'>
        <div className='text-zinc-400 text-sm'>{value}</div>
        {subValue && <div className='text-xs text-zinc-500'>{subValue}</div>}
      </div>
    </div>
  );
}
