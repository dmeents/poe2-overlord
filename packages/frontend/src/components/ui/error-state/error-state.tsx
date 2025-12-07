import { ExclamationTriangleIcon } from '@heroicons/react/24/outline';
import type { ReactNode } from 'react';

interface ErrorStateProps {
  title?: string;
  message?: string;
  error?: Error | string | unknown;
  icon?: ReactNode;
  action?: ReactNode;
  className?: string;
}

export function ErrorState({
  title = 'Error Loading Data',
  message,
  error,
  icon,
  action,
  className = '',
}: ErrorStateProps) {
  // Format the error message
  const errorMessage =
    message ||
    (error instanceof Error
      ? error.message
      : error
        ? String(error)
        : 'An unknown error occurred');

  return (
    <div className={`text-center py-8 ${className}`}>
      <div className='text-red-400 mb-4'>
        <div className='mx-auto h-12 w-12'>
          {icon || <ExclamationTriangleIcon />}
        </div>
      </div>
      <h3 className='text-lg font-semibold text-red-400 mb-2'>{title}</h3>
      <p className='text-sm text-zinc-400'>{errorMessage}</p>
      {action && <div className='mt-4'>{action}</div>}
    </div>
  );
}
