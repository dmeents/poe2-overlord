import type { ReactNode } from 'react';
import { Card, EmptyState } from '../';

interface DataCardProps {
  title: string;
  icon: ReactNode;
  subtitle?: string;
  className?: string;
  isLoading?: boolean;
  isEmpty?: boolean;
  emptyTitle?: string;
  emptyDescription?: string;
  emptyIcon?: ReactNode;
  children: ReactNode;
}

export function DataCard({
  title,
  icon,
  subtitle,
  className = '',
  isLoading = false,
  isEmpty = false,
  emptyTitle = 'No Data Available',
  emptyDescription = 'No data to display at this time.',
  emptyIcon,
  children,
}: DataCardProps) {
  if (isLoading) {
    return (
      <Card title={title} subtitle={subtitle} icon={icon} className={className}>
        <div className='grid grid-cols-2 gap-4'>
          {[...Array(6)].map((_, i) => (
            <div key={i} className='animate-pulse'>
              <div className='h-4 bg-zinc-700 rounded mb-2'></div>
              <div className='h-6 bg-zinc-700 rounded'></div>
            </div>
          ))}
        </div>
      </Card>
    );
  }

  if (isEmpty) {
    return (
      <Card title={title} subtitle={subtitle} icon={icon} className={className}>
        <EmptyState
          icon={emptyIcon}
          title={emptyTitle}
          description={emptyDescription}
        />
      </Card>
    );
  }

  return (
    <Card title={title} subtitle={subtitle} icon={icon} className={className}>
      {children}
    </Card>
  );
}
