import type { ReactNode } from 'react';

export interface DataCardProps {
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
