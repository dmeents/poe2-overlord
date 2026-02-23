import { ExclamationTriangleIcon } from '@heroicons/react/24/outline';
import type { ReactNode } from 'react';
import { errorStateStyles } from './error-state.styles';

interface ErrorStateProps {
  title?: string;
  message?: string;
  error?: unknown;
  icon?: ReactNode;
  action?: ReactNode;
  className?: string;
}

/**
 * Safely extracts a user-friendly message from any unknown error type.
 */
function getErrorMessage(error: unknown): string {
  if (error instanceof Error) {
    return error.message;
  }
  if (typeof error === 'string') {
    return error;
  }
  if (
    typeof error === 'object' &&
    error !== null &&
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
    <div className={`${errorStateStyles.container} ${className}`}>
      <div className={errorStateStyles.iconContainer}>
        <div className={errorStateStyles.icon}>{icon || <ExclamationTriangleIcon />}</div>
      </div>
      <h3 className={errorStateStyles.title}>{title}</h3>
      <p className={errorStateStyles.message}>{errorMessage}</p>
      {action && <div className={errorStateStyles.actionContainer}>{action}</div>}
    </div>
  );
}
