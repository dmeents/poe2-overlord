import type { ReactNode } from 'react';

interface SectionHeaderProps {
  title: string;
  icon?: ReactNode;
  className?: string;
}

export function SectionHeader({ title, icon, className = '' }: SectionHeaderProps) {
  return (
    <div className={`my-2 ${className}`}>
      <div className="flex items-center space-x-2 text-zinc-300">
        {icon && <span className="text-zinc-400">{icon}</span>}
        <h3 className="text-sm font-medium uppercase tracking-wide">{title}</h3>
      </div>
    </div>
  );
}
