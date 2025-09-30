import type { ReactNode } from 'react';

export interface DataItemProps {
  label: string | ReactNode;
  value: string | number;
  subValue?: string;
  className?: string;
  color?: string;
  icon?: ReactNode;
}
