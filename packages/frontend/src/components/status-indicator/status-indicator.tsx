import type { ReactNode } from 'react';
import { statusIndicatorStyles } from './status-indicator.styles';

type StatusType = 'success' | 'warning' | 'error' | 'info';

interface StatusIndicatorProps {
  status: StatusType;
  icon: ReactNode;
  size?: 'sm' | 'md' | 'lg';
  className?: string;
}

export const StatusIndicator = ({
  status,
  icon,
  size = 'md',
  className = '',
}: StatusIndicatorProps) => {
  return (
    <div
      className={`
        ${statusIndicatorStyles.sizeClasses[size]} 
        ${statusIndicatorStyles.statusClasses[status]}
        ${status !== 'success' ? statusIndicatorStyles.animated : ''}
        ${className}
      `}
    >
      {icon}
    </div>
  );
};
