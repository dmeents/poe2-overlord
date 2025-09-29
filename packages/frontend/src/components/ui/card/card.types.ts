import type { ReactNode } from 'react';

export interface CardProps {
  children: ReactNode;
  title?: string;
  icon?: ReactNode;
  className?: string;
  variant?: 'default' | 'insight' | 'featured';
}
