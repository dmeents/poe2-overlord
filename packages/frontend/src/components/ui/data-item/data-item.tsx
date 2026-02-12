import type { ReactNode } from 'react';

export interface DataItemProps {
  label: string | ReactNode;
  value: string | number;
  subValue?: string;
  className?: string;
  color?: string;
  icon?: ReactNode;
}

export function DataItem({ label, value, subValue, className = '', color, icon }: DataItemProps) {
  return (
    <div
      className={`
        flex items-center justify-between
        px-4 h-12
        border-l-2 border-transparent
        transition-all
        hover:bg-zinc-800/70
        odd:bg-zinc-900/60 even:bg-zinc-900/30
        ${className}
      `}
      style={color ? { borderLeftColor: color } : undefined}>
      <div className="flex items-center gap-2 min-w-0">
        {icon && <div className="flex-shrink-0 w-3.5 h-3.5 text-zinc-400">{icon}</div>}
        <span className="text-zinc-200 text-sm truncate">{label}</span>
      </div>
      <div className="text-right flex-shrink-0 pl-3">
        <div className="text-zinc-200 text-sm font-semibold">{value}</div>
        {subValue && (
          <div className="text-xs text-zinc-400 leading-tight font-mono">{subValue}</div>
        )}
      </div>
    </div>
  );
}
