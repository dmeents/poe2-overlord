import type { ReactNode } from 'react';

interface SectionHeaderProps {
  title: string;
  icon: ReactNode;
  children: ReactNode;
  className?: string;
}

export function SectionHeader({
  title,
  icon,
  children,
  className = '',
}: SectionHeaderProps) {
  return (
    <div className={`mt-6 space-y-4 ${className}`}>
      <div className='flex items-center space-x-2 text-zinc-300'>
        <span className='text-zinc-400'>{icon}</span>
        <h3 className='text-sm font-medium uppercase tracking-wide'>{title}</h3>
      </div>
      {children}
    </div>
  );
}
