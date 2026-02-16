export type ErrorCode =
  | 'filesystem'
  | 'validation'
  | 'internal'
  | 'network'
  | 'serialization'
  | 'security';

export interface SerializableError {
  code: ErrorCode;
  message: string;
}

/**
 * Frontend-specific wrapper for errors with additional metadata
 */
export interface AppError extends SerializableError {
  timestamp: Date;
  operation?: string;
}
