import { ExclamationTriangleIcon } from '@heroicons/react/24/outline';
import type { ReactNode } from 'react';
import type { AppError } from '@/types/error';
import { formatErrorMessage, parseError } from '@/utils/error-handling';
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
 * Safely extracts an error message from an unknown error type.
 * Parses the error into an AppError and formats it for display.
 */
function getErrorMessage(error: unknown): string {
  const appError: AppError = parseError(error);
  return formatErrorMessage(appError);
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
