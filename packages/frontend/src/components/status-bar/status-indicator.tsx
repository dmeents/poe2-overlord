import type { ReactNode } from 'react';

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
  const sizeClasses = {
    sm: 'w-4 h-4',
    md: 'w-5 h-5',
    lg: 'w-6 h-6',
  };

  const statusClasses = {
    success: 'text-green-800',
    warning: 'text-yellow-600',
    error: 'text-red-800',
    info: 'text-zinc-500',
  }[status];

  return (
    <div
      className={`
        ${sizeClasses[size]} 
        ${statusClasses}
        ${status === 'error' ? 'animate-pulse' : ''}
        ${className}
      `}
    >
      {icon}
    </div>
  );
};
