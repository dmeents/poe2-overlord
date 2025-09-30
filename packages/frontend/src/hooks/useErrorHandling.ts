import { useCallback, useState } from 'react';

/**
 * Standard error types for consistent error handling across hooks
 */
export enum ErrorType {
  NETWORK = 'NETWORK',
  VALIDATION = 'VALIDATION',
  PERMISSION = 'PERMISSION',
  NOT_FOUND = 'NOT_FOUND',
  SERVER = 'SERVER',
  UNKNOWN = 'UNKNOWN',
}

/**
 * Standardized error object with consistent structure
 */
export interface StandardError {
  type: ErrorType;
  message: string;
  originalError?: Error;
  timestamp: Date;
  context?: string;
}

/**
 * Error handling configuration for hooks
 */
export interface ErrorHandlingConfig {
  enableLogging?: boolean;
  enableRecovery?: boolean;
  customErrorMessage?: (error: Error, context?: string) => string;
}

/**
 * Hook for standardized error handling across all hooks
 *
 * Provides consistent error handling patterns, logging, and recovery mechanisms
 * for all hooks in the application.
 *
 * @param config - Error handling configuration
 * @returns Object containing error state and error handling functions
 *
 * @example
 * ```typescript
 * const { error, handleError, clearError } = useErrorHandling({
 *   enableLogging: true,
 *   enableRecovery: true
 * });
 *
 * const handleAsyncOperation = async () => {
 *   try {
 *     await someAsyncOperation();
 *   } catch (err) {
 *     handleError(err, 'Failed to perform operation');
 *   }
 * };
 * ```
 */
export function useErrorHandling(config: ErrorHandlingConfig = {}) {
  const {
    enableLogging = true,
    enableRecovery = false,
    customErrorMessage,
  } = config;

  const [error, setError] = useState<StandardError | null>(null);

  /**
   * Create a standardized error from any error input
   */
  const createStandardError = useCallback(
    (input: unknown, context?: string): StandardError => {
      let type: ErrorType = ErrorType.UNKNOWN;
      let message: string;
      let originalError: Error | undefined;

      if (input instanceof Error) {
        originalError = input;
        message = customErrorMessage
          ? customErrorMessage(input, context)
          : input.message;

        // Determine error type based on error message or name
        if (
          input.message.includes('network') ||
          input.message.includes('fetch')
        ) {
          type = ErrorType.NETWORK;
        } else if (
          input.message.includes('validation') ||
          input.message.includes('invalid')
        ) {
          type = ErrorType.VALIDATION;
        } else if (
          input.message.includes('permission') ||
          input.message.includes('unauthorized')
        ) {
          type = ErrorType.PERMISSION;
        } else if (
          input.message.includes('not found') ||
          input.message.includes('404')
        ) {
          type = ErrorType.NOT_FOUND;
        } else if (
          input.message.includes('server') ||
          input.message.includes('500')
        ) {
          type = ErrorType.SERVER;
        }
      } else if (typeof input === 'string') {
        message = customErrorMessage
          ? customErrorMessage(new Error(input), context)
          : input;
      } else {
        message = customErrorMessage
          ? customErrorMessage(new Error('Unknown error'), context)
          : 'An unexpected error occurred';
      }

      return {
        type,
        message,
        originalError,
        timestamp: new Date(),
        context,
      };
    },
    [customErrorMessage]
  );

  /**
   * Handle an error with standardized processing
   */
  const handleError = useCallback(
    (input: unknown, context?: string) => {
      const standardError = createStandardError(input, context);

      setError(standardError);

      if (enableLogging) {
        console.error(`[${standardError.type}] ${standardError.message}`, {
          context: standardError.context,
          originalError: standardError.originalError,
          timestamp: standardError.timestamp,
        });
      }
    },
    [createStandardError, enableLogging]
  );

  /**
   * Clear the current error
   */
  const clearError = useCallback(() => {
    setError(null);
  }, []);

  /**
   * Handle async operations with automatic error handling
   */
  const handleAsyncOperation = useCallback(
    async <T>(
      operation: () => Promise<T>,
      context?: string
    ): Promise<T | null> => {
      try {
        clearError();
        return await operation();
      } catch (err) {
        handleError(err, context);
        return null;
      }
    },
    [handleError, clearError]
  );

  /**
   * Get user-friendly error message
   */
  const getUserFriendlyMessage = useCallback((error: StandardError): string => {
    switch (error.type) {
      case ErrorType.NETWORK:
        return 'Network connection failed. Please check your internet connection and try again.';
      case ErrorType.VALIDATION:
        return 'Please check your input and try again.';
      case ErrorType.PERMISSION:
        return 'You do not have permission to perform this action.';
      case ErrorType.NOT_FOUND:
        return 'The requested resource was not found.';
      case ErrorType.SERVER:
        return 'Server error occurred. Please try again later.';
      default:
        return (
          error.message || 'An unexpected error occurred. Please try again.'
        );
    }
  }, []);

  /**
   * Check if error is recoverable
   */
  const isRecoverable = useCallback(
    (error: StandardError): boolean => {
      if (!enableRecovery) return false;

      return [ErrorType.NETWORK, ErrorType.SERVER].includes(error.type);
    },
    [enableRecovery]
  );

  return {
    error,
    handleError,
    clearError,
    handleAsyncOperation,
    getUserFriendlyMessage,
    isRecoverable,
    createStandardError,
  };
}

/**
 * Default error handling configuration for most hooks
 */
export const DEFAULT_ERROR_CONFIG: ErrorHandlingConfig = {
  enableLogging: true,
  enableRecovery: true,
};

/**
 * Error handling configuration for event listeners
 */
export const EVENT_ERROR_CONFIG: ErrorHandlingConfig = {
  enableLogging: true,
  enableRecovery: false,
  customErrorMessage: (error, context) =>
    `Event error${context ? ` in ${context}` : ''}: ${error.message}`,
};

/**
 * Error handling configuration for CRUD operations
 */
export const CRUD_ERROR_CONFIG: ErrorHandlingConfig = {
  enableLogging: true,
  enableRecovery: true,
  customErrorMessage: (error, context) => {
    const operation = context?.split(' ')[0] || 'operation';
    return `Failed to ${operation}: ${error.message}`;
  },
};
