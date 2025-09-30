export { useCharacterConfig } from './useCharacterConfig';
export { useCharacterDataFiltering } from './useCharacterDataFiltering';
export { useCharacterFilters } from './useCharacterFilterState';
export { useCharacterManagement } from './useCharacterManagement';
export { useGameProcessEvents } from './useGameProcessEvents';
export { useServerStatusEvents as useServerStatus } from './useServerStatusEvents';
export { useWalkthroughEvents } from './useWalkthroughEvents';
export { useWalkthroughGuide } from './useWalkthroughGuide';
export { useZoneDataFiltering } from './useZoneDataFiltering';
export { useZoneFilters } from './useZoneFilterState';

// Focused character hooks
export { useCharacterData } from './useCharacterData';
export { useCharacterEvents } from './useCharacterEvents';
export { useCharacterMutations } from './useCharacterMutations';

// Cache invalidation utilities
export { useCacheInvalidation } from './useCacheInvalidation';

// Error handling utilities
export {
  useErrorBoundary,
  withErrorBoundary,
  type ErrorBoundaryState,
} from './useErrorBoundary';
export {
  CRUD_ERROR_CONFIG,
  DEFAULT_ERROR_CONFIG,
  EVENT_ERROR_CONFIG,
  ErrorType,
  useErrorHandling,
  type ErrorHandlingConfig,
  type StandardError,
} from './useErrorHandling';

// Generic hooks
export {
  createCRUDOperationsConfig,
  useCRUDOperations,
} from './useCRUDOperations';
export {
  FilterHelpers,
  SortHelpers,
  createDataFilteringConfig,
  useDataFiltering,
} from './useDataFiltering';
export { createFilterStateConfig, useFilterState } from './useFilterState';
export {
  createEventListenerConfig,
  createMultiEventListenerConfig,
  useMultiTauriEventListener,
  useTauriEventListener,
} from './useTauriEventListener';

// React Query hooks
export {
  useActiveCharacter,
  useCharacter,
  useCharacters,
  useCreateCharacter,
  useDeleteCharacter,
  useSetActiveCharacter,
  useUpdateCharacter,
} from './useCharacterQueries';
