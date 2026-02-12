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

/**
 * Safely extracts an error message from an unknown error type.
 * Handles Error objects, objects with message property, strings, and other types.
 */
function getErrorMessage(error: unknown): string {
  if (error instanceof Error) {
    return error.message;
  }

  if (typeof error === 'string') {
    return error;
  }

  // Handle objects that have a message property (common API error shapes)
  if (
    error !== null &&
    typeof error === 'object' &&
    'message' in error &&
    typeof (error as { message: unknown }).message === 'string'
  ) {
    return (error as { message: string }).message;
  }

  return 'An unknown error occurred';
}

export function ErrorState({
  title = 'Error Loading Data',
  message,
  error,
  icon,
  action,
  className = '',
}: ErrorStateProps) {
  // Format the error message, preferring explicit message prop
  const errorMessage = message || getErrorMessage(error);

  return (
    <div className={`text-center py-8 ${className}`}>
      <div className="text-red-400 mb-4">
        <div className="mx-auto h-12 w-12">{icon || <ExclamationTriangleIcon />}</div>
      </div>
      <h3 className="text-lg font-semibold text-red-400 mb-2">{title}</h3>
      <p className="text-sm text-zinc-400">{errorMessage}</p>
      {action && <div className="mt-4">{action}</div>}
    </div>
  );
}
