import type { ReactNode } from 'react';

interface SectionHeaderProps {
  title: string;
  icon?: ReactNode;
  className?: string;
}

export function SectionHeader({
  title,
  icon,
  className = '',
}: SectionHeaderProps) {
  return (
    <h4
      className={`text-sm font-medium text-zinc-300 mb-3 flex items-center ${className}`}
    >
      {icon && <span className='w-4 h-4 mr-2 text-zinc-400'>{icon}</span>}
      {title}
    </h4>
  );
}
