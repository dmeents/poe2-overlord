import { useCallback } from 'react';
import type { AppError } from '@/types/error';
import {
  formatErrorMessage,
  isFileSystemError,
  isNetworkError,
  isValidationError,
} from '@/utils/error-handling';

/**
 * Hook for handling errors with user feedback.
 *
 * NOTE: This implementation uses console logging for error feedback.
 * For production, integrate a toast notification library like sonner:
 * - Install: pnpm add sonner
 * - Import: import { toast } from 'sonner'
 * - Replace console.error with toast.error()
 */
export function useErrorHandler() {
  const handleError = useCallback((error: AppError) => {
    const message = formatErrorMessage(error);

    if (isNetworkError(error)) {
      console.error('[Network Error]', message);
    } else if (isValidationError(error)) {
      console.error('[Validation Error]', message);
    } else if (isFileSystemError(error)) {
      console.error('[Filesystem Error]', message);
    } else {
      console.error('[Error]', message);
    }
  }, []);

  return { handleError };
}
