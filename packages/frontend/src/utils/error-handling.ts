import type { AppError, ErrorCode, SerializableError } from '@/types/error';

/**
 * Parse error from Tauri invoke rejection
 */
export function parseError(error: unknown): AppError {
  if (isSerializableError(error)) {
    return {
      ...error,
      timestamp: new Date(),
    };
  }

  // Fallback for unexpected error formats
  return {
    code: 'internal',
    message: String(error),
    timestamp: new Date(),
  };
}

function isSerializableError(error: unknown): error is SerializableError {
  return (
    typeof error === 'object' &&
    error !== null &&
    'code' in error &&
    'message' in error &&
    typeof error.code === 'string' &&
    typeof error.message === 'string'
  );
}

/**
 * Format error message for display
 */
export function formatErrorMessage(error: AppError): string {
  return error.message;
}

/**
 * Check if error is a specific type
 */
export function isErrorType(error: AppError, code: ErrorCode): boolean {
  return error.code === code;
}

export function isValidationError(error: AppError): boolean {
  return error.code === 'validation';
}

export function isNetworkError(error: AppError): boolean {
  return error.code === 'network';
}

export function isFileSystemError(error: AppError): boolean {
  return error.code === 'filesystem';
}
