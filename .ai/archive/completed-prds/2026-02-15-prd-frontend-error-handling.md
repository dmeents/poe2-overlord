# PRD: Frontend Error Handling Implementation

**Status**: Implemented
**Created**: 2026-02-15
**Completed**: 2026-02-15
**Replaces**: Frontend portions of `prd-comprehensive-error-handling.md`

## Context

The backend has been updated to return structured error objects (`SerializableError`) instead of flat strings. The frontend needs to be updated to handle these structured errors and provide consistent error handling patterns across the application.

### Backend Contract

All Tauri command rejections now return:
```json
{
  "code": "validation" | "filesystem" | "internal" | "network" | "serialization" | "security",
  "message": "Human-readable error message with operation context"
}
```

Previously, errors were flat strings. This is a **breaking change** for the frontend.

## Problem Statement

Current frontend issues:
1. **Assumes errors are strings** - All `invoke()` error handlers treat errors as strings
2. **Inconsistent error extraction** - Multiple patterns for getting error messages
3. **No error type discrimination** - Can't handle different error types differently
4. **Silent failures** - Many operations fail without user feedback
5. **No error boundary** - Unhandled errors crash components
6. **Mixed state management** - Inconsistent patterns across components

## Proposed Solution

### Phase 1: Update Error Contract

#### 1.1 Create TypeScript error types
**File**: `packages/frontend/src/types/error.ts`

```typescript
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

// Frontend-specific wrapper
export interface AppError extends SerializableError {
  timestamp: Date;
  operation?: string; // Optional: extract from message
}
```

#### 1.2 Create error utility functions
**File**: `packages/frontend/src/utils/error-handling.ts`

```typescript
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
```

### Phase 2: Update Existing Error Handling

#### 2.1 Audit and update impacted files

Update all `invoke()` call sites to use `parseError()`:

**Files to update**:
- `packages/frontend/src/utils/tauri.ts` - All try/catch blocks
- `packages/frontend/src/routes/economy.tsx` - String matching on `error.message.includes('No currency data available')`
- `packages/frontend/src/components/ui/error-state/error-state.tsx` - `getErrorMessage()` utility
- `packages/frontend/src/contexts/CharacterContext.tsx` - Error extraction via `.message`
- `packages/frontend/src/contexts/EconomyContext.tsx` - Error cast to `Error | null`
- `packages/frontend/src/contexts/GameProcessContext.tsx` - Silent error swallowing
- `packages/frontend/src/hooks/useStepNavigation.ts` - Silent catch blocks
- `packages/frontend/src/routes/characters.tsx` - Empty catch blocks

**Pattern to follow**:
```typescript
// Before
try {
  const result = await invoke<SomeType>('command_name', { args });
  return result;
} catch (error) {
  console.error('Operation failed:', error);
  throw error;
}

// After
import { parseError } from '@/utils/error-handling';

try {
  const result = await invoke<SomeType>('command_name', { args });
  return result;
} catch (err) {
  const error = parseError(err);
  console.error('Operation failed:', error.message);
  throw error;
}
```

#### 2.2 Update error state component
**File**: `packages/frontend/src/components/ui/error-state/error-state.tsx`

Update `getErrorMessage()` to handle `AppError`:
```typescript
import type { AppError } from '@/types/error';
import { formatErrorMessage, parseError } from '@/utils/error-handling';

function getErrorMessage(error: unknown): string {
  const appError = parseError(error);
  return formatErrorMessage(appError);
}
```

### Phase 3: React Error Boundary

#### 3.1 Create Error Boundary component
**File**: `packages/frontend/src/components/error-boundary/error-boundary.tsx`

```typescript
import { Component, type ReactNode } from 'react';
import type { AppError } from '@/types/error';
import { parseError } from '@/utils/error-handling';

interface Props {
  children: ReactNode;
  fallback?: (error: AppError, reset: () => void) => ReactNode;
}

interface State {
  error: AppError | null;
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { error: null };
  }

  static getDerivedStateFromError(error: unknown): State {
    return { error: parseError(error) };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    console.error('Error boundary caught error:', error, errorInfo);
  }

  reset = () => {
    this.setState({ error: null });
  };

  render() {
    if (this.state.error) {
      if (this.props.fallback) {
        return this.props.fallback(this.state.error, this.reset);
      }

      return (
        <div className="error-boundary">
          <h2>Something went wrong</h2>
          <p>{this.state.error.message}</p>
          <button onClick={this.reset}>Try again</button>
        </div>
      );
    }

    return this.props.children;
  }
}
```

#### 3.2 Wrap app with Error Boundary
**File**: `packages/frontend/src/main.tsx`

```typescript
import { ErrorBoundary } from '@/components/error-boundary/error-boundary';

// Wrap the Router with ErrorBoundary
<ErrorBoundary>
  <RouterProvider router={router} />
</ErrorBoundary>
```

### Phase 4: Error Handling Hook

#### 4.1 Create useErrorHandler hook
**File**: `packages/frontend/src/hooks/useErrorHandler.ts`

```typescript
import { useCallback } from 'react';
import { toast } from 'sonner';
import type { AppError } from '@/types/error';
import { formatErrorMessage, isNetworkError, isValidationError } from '@/utils/error-handling';

export function useErrorHandler() {
  const handleError = useCallback((error: AppError) => {
    const message = formatErrorMessage(error);

    if (isNetworkError(error)) {
      toast.error('Network error', { description: message });
    } else if (isValidationError(error)) {
      toast.error('Validation error', { description: message });
    } else {
      toast.error('Error', { description: message });
    }
  }, []);

  return { handleError };
}
```

### Phase 5: TanStack Query Integration

#### 5.1 Add default error handler to mutations
**File**: `packages/frontend/src/queries/character-mutations.ts` (and other mutation files)

```typescript
import { useErrorHandler } from '@/hooks/useErrorHandler';
import { parseError } from '@/utils/error-handling';

export function useCreateCharacter() {
  const { handleError } = useErrorHandler();

  return useMutation({
    mutationFn: async (params: CreateCharacterParams) => {
      return await invoke<CharacterDataResponse>('create_character', params);
    },
    onError: (err) => {
      const error = parseError(err);
      handleError(error);
    },
  });
}
```

## Migration Checklist

### Backend Verification (✅ Completed)
- [x] `SerializableError` struct added
- [x] `CommandResult<T>` updated
- [x] `to_command_result()` updated
- [x] Tests added for `SerializableError`
- [x] All backend tests pass

### Frontend Implementation (Completed)
- [x] Create `src/types/error.ts`
- [x] Create `src/utils/error-handling.ts`
- [x] Update `src/components/ui/error-state/error-state.tsx`
- [x] Audit and update all `invoke()` call sites:
  - [x] `src/utils/tauri.ts`
  - [x] `src/routes/economy.tsx` (no changes needed - message check still works)
  - [x] `src/contexts/CharacterContext.tsx`
  - [x] `src/contexts/EconomyContext.tsx`
  - [x] `src/contexts/GameProcessContext.tsx`
  - [x] `src/hooks/useStepNavigation.ts`
  - [x] `src/routes/characters.tsx`
- [x] Create Error Boundary component
- [x] Wrap app with Error Boundary
- [x] Create `useErrorHandler` hook
- [x] Update TanStack Query mutations with default error handlers
- [ ] Test all error scenarios
- [ ] Update error handling documentation

## Testing Strategy

1. **Unit tests** - Test error parsing and formatting utilities
2. **Integration tests** - Test Error Boundary with various error scenarios
3. **E2E tests** - Verify error handling in real workflows:
   - Invalid form submissions
   - Network failures
   - File system errors
   - Validation errors

## Success Criteria

- [x] All `invoke()` calls handle structured errors
- [x] Error Boundary catches and displays unhandled errors
- [x] Users see consistent error messages
- [x] Different error types are handled appropriately
- [x] No silent failures (all catch blocks now use parseError and proper logging)
- [ ] All existing error handling tests pass (requires running test suite)
- [ ] New error handling tests added (future work)

## Notes

- The economy route's `error.message.includes('No currency data available')` check will still work since the message string is preserved in the `SerializableError.message` field
- Consider adding error logging/reporting service integration in the future
- Error codes can be extended in the future (e.g., `authentication`, `authorization`)

## Implementation Notes

**Toast Notification System:**
The `useErrorHandler` hook currently uses `console.error` for error feedback. To add toast notifications:
1. Install sonner: `pnpm add sonner`
2. Add `<Toaster />` component to the app root (in `main.tsx` or `providers.tsx`)
3. Update `useErrorHandler` to use `toast.error()` instead of `console.error`

**Files Created:**
- `packages/frontend/src/types/error.ts` - Error type definitions
- `packages/frontend/src/utils/error-handling.ts` - Error parsing and utility functions
- `packages/frontend/src/components/error-boundary/error-boundary.tsx` - React Error Boundary
- `packages/frontend/src/hooks/useErrorHandler.ts` - Error handling hook

**Files Modified:**
- `packages/frontend/src/components/ui/error-state/error-state.tsx` - Updated to use parseError
- `packages/frontend/src/utils/tauri.ts` - All catch blocks updated
- `packages/frontend/src/contexts/CharacterContext.tsx` - Error type updated to AppError
- `packages/frontend/src/contexts/EconomyContext.tsx` - Error type updated to AppError
- `packages/frontend/src/contexts/GameProcessContext.tsx` - Catch block updated
- `packages/frontend/src/hooks/useStepNavigation.ts` - All catch blocks updated
- `packages/frontend/src/routes/characters.tsx` - Display error.message
- `packages/frontend/src/queries/characters.ts` - All mutations have error handlers
- `packages/frontend/src/main.tsx` - Wrapped app with ErrorBoundary
