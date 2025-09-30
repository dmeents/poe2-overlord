import React, { useCallback, useState } from 'react';
import { ErrorType, type StandardError } from './useErrorHandling';

/**
 * Error boundary state for React error boundaries
 */
export interface ErrorBoundaryState {
  hasError: boolean;
  error: StandardError | null;
  errorInfo: React.ErrorInfo | null;
}

/**
 * Hook for creating error boundary functionality
 *
 * This hook provides error boundary state management and recovery mechanisms
 * that can be used with React error boundaries or custom error handling.
 *
 * @returns Object containing error boundary state and functions
 *
 * @example
 * ```typescript
 * const { hasError, error, errorInfo, handleError, resetError } = useErrorBoundary();
 *
 * // Use in error boundary component
 * if (hasError) {
 *   return <ErrorFallback error={error} resetError={resetError} />;
 * }
 * ```
 */
export function useErrorBoundary() {
  const [state, setState] = useState<ErrorBoundaryState>({
    hasError: false,
    error: null,
    errorInfo: null,
  });

  /**
   * Handle an error in the error boundary
   */
  const handleError = useCallback(
    (error: Error, errorInfo: React.ErrorInfo) => {
      const standardError: StandardError = {
        type: ErrorType.UNKNOWN,
        message: error.message,
        originalError: error,
        timestamp: new Date(),
        context: 'React Error Boundary',
      };

      setState({
        hasError: true,
        error: standardError,
        errorInfo,
      });

      // Log error for debugging
      console.error('Error Boundary caught an error:', error, errorInfo);
    },
    []
  );

  /**
   * Reset the error boundary state
   */
  const resetError = useCallback(() => {
    setState({
      hasError: false,
      error: null,
      errorInfo: null,
    });
  }, []);

  /**
   * Check if the error is recoverable
   */
  const isRecoverable = useCallback((error: StandardError): boolean => {
    return [ErrorType.NETWORK, ErrorType.SERVER].includes(error.type);
  }, []);

  return {
    hasError: state.hasError,
    error: state.error,
    errorInfo: state.errorInfo,
    handleError,
    resetError,
    isRecoverable,
  };
}

/**
 * Higher-order component for error boundary functionality
 *
 * @param Component - The component to wrap with error boundary
 * @returns Component wrapped with error boundary
 */
export function withErrorBoundary<P extends object>(
  Component: React.ComponentType<P>
) {
  return function ErrorBoundaryWrapper(props: P) {
    const { hasError, error, resetError } = useErrorBoundary();

    if (hasError && error) {
      return (
        <div className='flex flex-col items-center justify-center min-h-[200px] p-6'>
          <div className='text-center'>
            <h2 className='text-xl font-semibold text-red-600 mb-2'>
              Something went wrong
            </h2>
            <p className='text-gray-600 mb-4'>
              {error.message || 'An unexpected error occurred'}
            </p>
            <button
              onClick={resetError}
              className='px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors'
            >
              Try again
            </button>
          </div>
        </div>
      );
    }

    return <Component {...props} />;
  };
}
