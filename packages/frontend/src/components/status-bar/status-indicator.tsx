import type { ReactNode } from 'react';

interface StatusIndicatorProps {
  status: boolean;
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

  const statusClasses = status ? 'text-green-800' : 'text-red-800';

  return (
    <div
      className={`
        ${sizeClasses[size]} 
        transition-all duration-200 ease-in-out
        ${statusClasses}
        ${status ? 'animate-pulse' : ''}
        ${className}
      `}
    >
      {icon}
    </div>
  );
};
