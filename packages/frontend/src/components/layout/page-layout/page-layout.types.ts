import type { ReactNode } from 'react';

export interface PageLayoutProps {
  children?: ReactNode;
  leftColumn: ReactNode;
  rightColumn: ReactNode;
  className?: string;
}
