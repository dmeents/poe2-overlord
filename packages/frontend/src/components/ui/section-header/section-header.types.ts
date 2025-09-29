import type { ReactNode } from 'react';

export interface SectionHeaderProps {
  title: string;
  icon: ReactNode;
  children: ReactNode;
  className?: string;
}
