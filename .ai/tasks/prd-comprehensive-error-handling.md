# PRD: Comprehensive Error Handling System

## Context

This PRD captures the architectural improvements identified during Issue #31 (Error handling pattern consistency). Issue #31 implemented a focused fix for the settings form, but a full solution requires more extensive changes.

**Origin**: Issue #31 analysis revealed inconsistent error handling patterns across the codebase
**Scope**: Comprehensive error handling system spanning backend and frontend
**Priority**: MEDIUM (architectural improvement)
**Estimated Effort**: 15-19 hours

## Problem Statement

Current error handling has several issues:
1. **No centralized error handling utility** - Each component implements its own pattern
2. **Inconsistent error extraction** - Multiple ways to get error messages
3. **Silent failures** - Many operations fail without user feedback
4. **No error boundary** - React Error Boundaries not implemented
5. **Mixed state management** - Some use local state, some use Query state, some ignore errors
6. **No error type discrimination** - Can't differentiate between network, validation, security errors
7. **Loss of context** - Backend AppError structure lost when converted to String for IPC

## Proposed Solution

### Phase 1: Backend Foundation

#### 1.1 Add SerializableError struct
**File**: `packages/backend/src/errors.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableError {
    pub code: String,       // Error type (Validation, FileSystem, etc)
    pub message: String,    // Human-readable message
    pub operation: String,  // What operation failed
    pub details: Option<HashMap<String, String>>, // Additional context
}

impl From<AppError> for SerializableError {
    fn from(err: AppError) -> Self { ... }
}
```

#### 1.2 Update CommandResult type
Change `CommandResult<T>` from `Result<T, String>` to `Result<T, SerializableError>`

### Phase 2: Frontend Foundation

#### 2.1 Create error type definitions
**File**: `packages/frontend/src/types/error.ts`

```typescript
export enum ErrorType {
  FileSystem = 'FileSystem',
  Validation = 'Validation',
  Internal = 'Internal',
  Network = 'Network',
  Serialization = 'Serialization',
  Security = 'Security',
  ConcurrentModification = 'ConcurrentModification',
  Unknown = 'Unknown',
}

export interface AppError {
  code: ErrorType;
  message: string;
  operation: string;
  details?: Record<string, string>;
}
```

#### 2.2 Create error utilities
**File**: `packages/frontend/src/utils/error-handling.ts`

```typescript
export function parseError(error: unknown): AppError;
export function formatErrorMessage(error: AppError): string;
export function isValidationError(error: AppError): boolean;
export function isNetworkError(error: AppError): boolean;
export function createMutationErrorHandler(onError?: (error: AppError) => void);
```

#### 2.3 Create error state hook
**File**: `packages/frontend/src/hooks/useErrorHandler.ts`

```typescript
export function useErrorHandler() {
  const [error, setError] = useState<AppError | null>(null);
  const handleError = useCallback((err: unknown) => { ... }, []);
  const clearError = useCallback(() => setError(null), []);
  return { error, handleError, clearError };
}
```

#### 2.4 Create React Error Boundary
**File**: `packages/frontend/src/components/ui/error-boundary/error-boundary.tsx`

### Phase 3: UI Component Updates

1. **ErrorState component** - Accept AppError type, show contextual information
2. **AlertMessage component** - Support AppError objects
3. **Global error toast** - For non-blocking errors

### Phase 4: Migrate Existing Components

1. `settings-form.tsx` - Already started, extend pattern
2. `characters.tsx` - Replace empty catch blocks
3. `character-form-modal.tsx` - Use error hook
4. `walkthrough-guide.tsx` - Add user feedback

### Phase 5: Query Integration

1. Add default onError handlers to mutation hooks
2. Integrate with TanStack Query error states
3. Global error listener in app root

## Files to Change

### Backend (2 files)
- `packages/backend/src/errors.rs`
- `packages/backend/src/lib.rs`

### Frontend - New Files (4 files)
- `packages/frontend/src/types/error.ts`
- `packages/frontend/src/utils/error-handling.ts`
- `packages/frontend/src/hooks/useErrorHandler.ts`
- `packages/frontend/src/components/ui/error-boundary/error-boundary.tsx`

### Frontend - Updates (8+ files)
- Various components and routes

## Risk Mitigation

1. **Breaking Changes**: Backend changes affect all Tauri commands - need careful migration
2. **Incomplete Context**: parseError() must handle unknown types gracefully
3. **User Confusion**: formatErrorMessage() must translate technical errors

## Success Criteria

- [ ] Backend SerializableError struct implemented
- [ ] All Tauri commands return structured errors
- [ ] Frontend parseError() handles all error types
- [ ] useErrorHandler() hook available
- [ ] ErrorBoundary wraps app routes
- [ ] No silent failures in major flows
- [ ] User-facing error messages are clear
- [ ] 100% test coverage for error utilities

## Notes

This PRD is for future implementation. The focused fix in Issue #31 provides consistency for the settings form. This comprehensive solution addresses the full codebase.

## References

- Original analysis: Issue #31 implementation planner output
- Related: `settings-form.tsx` error handling patterns
- Existing: `ErrorState` component, `AlertMessage` component
